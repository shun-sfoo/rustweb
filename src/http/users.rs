use crate::http::{Error, Result};
use anyhow::Context;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash,
};
use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;

use super::{extractor::AuthUser, ApiContext};

use crate::db::users::ActiveModel as UserModel;
use crate::db::users::Column as UserColumn;
use crate::db::users::Entity as UserEntity;

pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route("/api/users", post(create_user))
        .route("/api/users/login", post(login_user))
        .route("/api/user", get(get_current_user).put(update_user))
}

#[derive(Serialize, Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(Deserialize)]
struct NewUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    token: String,
}

#[derive(Deserialize, Default, PartialEq, Eq)]
struct UpdateUser {
    password: Option<String>,
}

async fn create_user(
    ctx: Extension<ApiContext>,
    Json(req): Json<UserBody<NewUser>>,
) -> Result<Json<UserBody<User>>> {
    let password_hash = hash_password(req.user.password).await?;

    let user_id = Uuid::new_v4().to_string();

    let insert_user = UserModel {
        username: Set(req.user.username.clone()),
        password_hash: Set(password_hash),
        user_id: Set(user_id.clone()),
    };

    insert_user
        .insert(&ctx.db)
        .await
        .map_err(|e| Error::SeaOrm(e))?;

    Ok(Json(UserBody {
        user: User {
            username: req.user.username,
            token: AuthUser { user_id }.to_jwt(&ctx),
        },
    }))
}

async fn login_user(
    ctx: Extension<ApiContext>,
    Json(req): Json<UserBody<NewUser>>,
) -> Result<Json<UserBody<User>>> {
    let user = UserEntity::find()
        .filter(UserColumn::Username.eq(req.user.username))
        .one(&ctx.db)
        .await
        .map_err(|e| {
            debug!("find use by username error :{:?}", e);
            Error::SeaOrm(e)
        })?;

    if let Some(u) = user {
        verify_password(req.user.password, u.password_hash).await?;

        return Ok(Json(UserBody {
            user: User {
                username: u.username,
                token: AuthUser { user_id: u.user_id }.to_jwt(&ctx),
            },
        }));
    }

    Err(Error::unprocessable_entity([(
        "username",
        "does not exist",
    )]))
}

async fn get_current_user(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
) -> Result<Json<UserBody<User>>> {
    let user = UserEntity::find_by_id(auth_user.user_id)
        .one(&ctx.db)
        .await
        .map_err(|e| {
            debug!("find use by id error :{:?}", e);
            Error::SeaOrm(e)
        })?;

    if let Some(u) = user {
        return Ok(Json(UserBody {
            user: User {
                username: u.username,
                token: AuthUser { user_id: u.user_id }.to_jwt(&ctx),
            },
        }));
    }

    Err(Error::unprocessable_entity([("user_id", "does not exist")]))
}

async fn update_user(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
    Json(req): Json<UserBody<UpdateUser>>,
) -> Result<Json<UserBody<User>>> {
    if req.user == UpdateUser::default() {
        return get_current_user(auth_user, ctx).await;
    }

    todo!()
}

async fn hash_password(password: String) -> Result<String> {
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_str())
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("panic in generating password hash")??)
}

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    Ok(tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash")??)
}
