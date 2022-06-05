use std::collections::HashMap;

use axum::{extract::Query, response::IntoResponse, Extension, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbConn, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

use crate::model::claims::Claims;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub token: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Deserialize)]
pub struct FileParams {
    page: Option<usize>,
    posts_per_page: Option<usize>,
    filename: Option<String>,
    upload_time_begin: Option<String>,
    upload_time_end: Option<String>,
}

pub async fn register(
    Json(data): Json<HashMap<String, String>>,
    Extension(ref conn): Extension<DbConn>,
) -> Result<Json<UserResponse>, &'static str> {
    let name = data.get("username").unwrap();
    let password = data.get("password").unwrap();

    let inserter = crate::db::user::ActiveModel {
        name: Set(name.to_string()),
        password: Set(password.to_string()),
        ..Default::default()
    };

    let model = inserter.insert(conn).await.expect("insert user failed");
    let claims = Claims::new(model.id, model.name.clone());
    let token = claims.generate();

    Ok(Json(UserResponse {
        user: User {
            id: model.id,
            name: model.name,
            token: Some(token),
        },
    }))
}

pub async fn login(
    Json(data): Json<HashMap<String, String>>,
    Extension(ref conn): Extension<DbConn>,
) -> Result<Json<UserResponse>, &'static str> {
    use crate::db::user::Column;

    let name = data.get("username").unwrap().to_string();
    let password = data.get("password").unwrap().to_string();

    let user = crate::db::user::Entity::find()
        .filter(
            Condition::all()
                .add(Column::Name.eq(name))
                .add(Column::Password.eq(password)),
        )
        .one(conn)
        .await
        .unwrap();

    if let Some(user) = user {
        return Ok(Json(UserResponse {
            user: User {
                id: user.id,
                name: user.name.clone(),
                token: Some(Claims::new(user.id, user.name).generate()),
            },
        }));
    };
    Err("No user")
}

#[derive(Serialize, Debug)]
pub struct FileListResponse {
    file_name: String,
    update_time: String,
    operator: String,
    size: u32,
}

pub async fn file_list(
    _claims: Claims,
    Query(params): Query<FileParams>,
    Extension(ref conn): Extension<DbConn>,
) -> impl IntoResponse {
    use crate::db::file::Column;

    let page = params.page.unwrap_or(1);
    let posts_per_page = params.posts_per_page.unwrap_or(20);
    let mut conditions = Condition::all();
    // TODO find a way to search update time start and end
    if let Some(name) = params.filename {
        conditions = conditions.add(Column::Name.like(&name));
    }

    let paginator = crate::db::file::Entity::find()
        .filter(conditions)
        .order_by_asc(Column::UploadTime)
        .paginate(conn, posts_per_page);

    let _num_pages = paginator.num_pages().await.ok().unwrap();

    let lists = paginator
        .fetch_page(page - 1)
        .await
        .expect("could not retrieve posts");

    let mut result = Vec::new();
    for m in lists {
        result.push(FileListResponse {
            file_name: m.name,
            update_time: m.upload_time.to_string(),
            operator: m.operator,
            size: m.size,
        });
    }

    return Json(result);
}

pub async fn me(
    claims: Claims,
    Extension(ref conn): Extension<DbConn>,
) -> Json<Option<UserResponse>> {
    let id = claims.id;
    match crate::db::user::Entity::find_by_id(id)
        .one(conn)
        .await
        .unwrap()
    {
        Some(m) => Json(Some(UserResponse {
            user: User {
                id: m.id,
                name: m.name,
                token: None,
            },
        })),
        None => Json(None),
    }
}
