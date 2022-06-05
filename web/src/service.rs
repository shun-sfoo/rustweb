use std::{collections::HashMap, env, io::Write};

use axum::{
    extract::{Multipart, Query},
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbConn, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use tokio::task_local;
use tracing::debug;

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

#[derive(Deserialize, Debug)]
pub struct FileParams {
    page: Option<usize>,
    posts_per_page: Option<usize>,
    name: Option<String>,
    #[serde(rename(deserialize = "uploadTimeBegin"))]
    upload_time_begin: Option<i64>,
    #[serde(rename(deserialize = "uploadTimeEnd"))]
    upload_time_end: Option<i64>,
}

fn get_epoch() -> i64 {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
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

pub async fn users(Extension(ref conn): Extension<DbConn>) -> impl IntoResponse {
    let users = crate::db::user::Entity::find().all(conn).await.unwrap();
    let mut user_list = Vec::new();

    for u in users {
        user_list.push(User {
            id: u.id,
            name: u.name,
            token: None,
        });
    }
    Json(user_list)
}

#[derive(Serialize, Debug)]
pub struct FileListResponse {
    name: String,
    #[serde(rename = "uploadTime")]
    upload_time: String,
    operator: String,
    size: u32,
}

pub async fn file_list(
    // _claims: Claims,
    Query(params): Query<FileParams>,
    Extension(ref conn): Extension<DbConn>,
) -> impl IntoResponse {
    debug!(?params);
    use crate::db::file::Column;

    let page = params.page.unwrap_or(1);
    let posts_per_page = params.posts_per_page.unwrap_or(20);
    let mut conditions = Condition::all();
    // TODO find a way to search update time start and end
    conditions = conditions.add(Column::IsDelete.eq(0));
    if let Some(name) = params.name {
        conditions = conditions.add(Column::Name.like(&format!("%{}%", &name)));
    }

    conditions = conditions.add(Column::UploadTime.between(
        params.upload_time_begin.unwrap_or(0),
        params.upload_time_end.unwrap_or(get_epoch()),
    ));

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
            name: m.name,
            upload_time: m.upload_time.to_string(),
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

pub async fn upload(
    claims: Claims,
    mut multipart: Multipart,
    Extension(ref conn): Extension<DbConn>,
) {
    let id = claims.id;
    if let Some(field) = multipart.next_field().await.unwrap() {
        debug!(?field);
        debug!(?claims);
        let name = field.file_name().unwrap().to_string();
        // let size = field.bytes().await.unwrap().len() as u32;
        debug!(?name);
        // debug!(?size);

        use crate::db::file::Column;

        let res = crate::db::file::Entity::find()
            .filter(Column::Name.eq(name.clone()))
            .one(conn)
            .await
            .unwrap();

        let location = env::var("HOME").unwrap();
        let location = format!("{}/{}", location, name.clone());
        let mut file = std::fs::File::create(location).unwrap();
        let _result = file.write_all(&field.bytes().await.unwrap());

        let now = get_epoch();
        if let None = res {
            let model = crate::db::file::ActiveModel {
                name: Set(name),
                size: Set(0),
                is_delete: Set(false),
                operator: Set(id.to_string()),
                location: Set("where".to_string()),
                upload_time: Set(now),
                ..Default::default()
            };

            crate::db::file::Entity::insert(model)
                .exec(conn)
                .await
                .unwrap();
        }
    }
}
