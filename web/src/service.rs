use std::{collections::HashMap, env, io::Write};

use axum::{
    body::StreamBody,
    extract::{Multipart, Path, Query},
    headers::HeaderName,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, IntoResponseParts, ResponseParts},
    Extension, Json,
};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, Condition, DbConn, EntityTrait, QueryFilter,
    QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use tokio_util::io::ReaderStream;
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

#[derive(Serialize, Debug)]
pub struct FileUploadResponse {
    id: Option<i32>,
}

// http://localhost:8080/files?filter={"name":"1","upload_begin":"2022-06-16"}&range=[0,9]&sort=["id","ASC"]

#[derive(Deserialize, Debug)]
pub struct Filter {
    name: Option<String>,
    upload_begin: Option<String>,
    upload_end: Option<String>,
}

struct SetHeader<'a>(&'a str, &'a str);

impl<'a> IntoResponseParts for SetHeader<'a> {
    type Error = StatusCode;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        let name = self
            .0
            .parse::<HeaderName>()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let value = self
            .1
            .parse::<HeaderValue>()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        res.headers_mut().insert(name, value);

        Ok(res)
    }
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
    id: i32,
    name: String,
    #[serde(rename = "uploadTime")]
    upload_time: String,
    operator: String,
    location: String,
    size: u32,
}

pub async fn list(
    Query(params): Query<HashMap<String, String>>,
    Extension(ref conn): Extension<DbConn>,
) -> impl IntoResponse {
    debug!(?params);
    use crate::db::file::Column;
    use crate::db::file::Entity;
    use chrono::NaiveDateTime;

    let mut conditions = Condition::all();
    conditions = conditions.add(Column::IsDelete.eq(false));

    let mut offset = 0;
    let mut limit = 10;
    let mut end_index = limit;

    if params.contains_key("filter") {
        let filter = serde_json::from_str::<Filter>(&params["filter"]).unwrap();

        if let Some(name) = filter.name {
            conditions = conditions.add(Column::Name.like(&format!("%{}%", name)));
        }

        match (filter.upload_begin, filter.upload_end) {
            (Some(begin), Some(end)) => {
                let b = format!("{} 00:00:00", begin);
                let e = format!("{} 23:59:59", end);
                let b = NaiveDateTime::parse_from_str(&b, "%Y-%m-%d %H:%M:%S").unwrap();
                let e = NaiveDateTime::parse_from_str(&e, "%Y-%m-%d %H:%M:%S").unwrap();

                conditions =
                    conditions.add(Column::UploadTime.between(b.timestamp(), e.timestamp()));
            }

            (Some(begin), None) => {
                let b = format!("{} 00:00:00", begin);
                let b = NaiveDateTime::parse_from_str(&b, "%Y-%m-%d %H:%M:%S").unwrap();
                conditions = conditions.add(Column::UploadTime.gte(b.timestamp()));
            }

            (None, Some(end)) => {
                let e = format!("{} 23:59:59", end);
                let e = NaiveDateTime::parse_from_str(&e, "%Y-%m-%d %H:%M:%S").unwrap();
                conditions = conditions.add(Column::UploadTime.lte(e.timestamp()));
            }

            (None, None) => {}
        };
    }

    if params.contains_key("range") {
        let (start, end) = serde_json::from_str::<(u64, u64)>(&params["range"]).unwrap();
        offset = start;
        limit = end - start + 1;
        end_index = end;
    }

    let total = Entity::find()
        .filter(Column::IsDelete.eq(false))
        .all(conn)
        .await
        .unwrap();

    let list = Entity::find()
        .filter(conditions)
        .offset(offset)
        .limit(limit)
        .all(conn)
        .await
        .unwrap();

    let mut data = Vec::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        "content-range",
        format!("files {}-{}/{}", offset, end_index, total.len())
            .parse()
            .unwrap(),
    );

    for m in list {
        data.push(FileListResponse {
            id: m.id,
            name: m.name,
            upload_time: m.upload_time.to_string(),
            operator: "admin".to_string(),
            location: m.location,
            size: m.size,
        });
    }

    (headers, Json(data))
}

#[derive(Deserialize, Debug)]
pub struct Id {
    id: Vec<i32>,
}

pub async fn delete_file(
    Query(ids): Query<HashMap<String, String>>,
    Extension(ref conn): Extension<DbConn>,
) -> impl IntoResponse {
    debug!(?ids);

    let id_str = ids.get("filter").unwrap();

    debug!(?id_str);
    let ids: Id = serde_json::from_str(id_str).unwrap();
    debug!(?ids);

    let d = crate::db::file::Entity::update_many()
        .col_expr(crate::db::file::Column::IsDelete, Expr::value(true))
        .filter(crate::db::file::Column::Id.is_in(ids.id))
        .exec(conn)
        .await
        .unwrap();

    Json(vec![d.rows_affected])
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
    // claims: Claims,
    mut multipart: Multipart,
    Extension(ref conn): Extension<DbConn>,
) -> Json<FileUploadResponse> {
    // let id = claims.id;
    if let Some(field) = multipart.next_field().await.unwrap() {
        debug!(?field);
        // debug!(?claims);
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
                name: Set(name.clone()),
                size: Set(0),
                is_delete: Set(false),
                operator: Set("1".to_string()),
                location: Set(format!("http://39.107.88.89:8081/{}", name)),
                upload_time: Set(now),
                ..Default::default()
            };

            let result = crate::db::file::Entity::insert(model)
                .exec(conn)
                .await
                .unwrap();

            return Json(FileUploadResponse {
                id: Some(result.last_insert_id),
            });
        }
    }

    return Json(FileUploadResponse { id: None });
}

pub async fn download_file(Path(filename): Path<String>) -> impl IntoResponse {
    debug!(?filename);

    let path = env::var("HOME").expect("No home");

    let file_path = format!("{}/{}", path, filename);

    let file = match tokio::fs::File::open(file_path.clone()).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();

    let attachment = format!("attachment; filename={}", filename);

    headers.insert("content-disposition", attachment.parse().unwrap());

    Ok((headers, body))
}
