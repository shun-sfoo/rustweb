use std::{collections::HashMap, env, io::Write};

use axum::{
    body::{Body, StreamBody},
    extract::{Multipart, Path, Query},
    headers::HeaderName,
    http::{HeaderMap, HeaderValue, Request, StatusCode},
    response::{IntoResponse, IntoResponseParts, ResponseParts},
    Extension, Json,
};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, Condition, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set, UpdateResult,
};
use serde::{Deserialize, Serialize};
use tokio_util::io::ReaderStream;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tracing::{debug, info};

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

// This will parse query strings like `?page=2&per_page=30` into `Pagination`
#[derive(Deserialize, Debug)]
pub struct Pagination {
    page: usize,
    per_page: usize,
}

#[derive(Serialize, Debug)]
pub struct FileUploadResponse {
    id: Option<i32>,
}

// http://localhost:8080/files?filter={"name":"1","upload_begin":"2022-06-16"}&range=[0,9]&sort=["id","ASC"]

#[derive(Deserialize, Debug)]
pub struct FileQuery {
    filter: Option<String>,
    range: Option<String>,
    sort: Option<String>,
}

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
    let mut conditions = Condition::all();
    conditions = conditions.add(Column::IsDelete.eq(0));

    if params.contains_key("filter") {
        let filter = serde_json::from_str::<Filter>(&params["filter"]).unwrap();

        if let Some(name) = filter.name {
            conditions = conditions.add(Column::Name.like(&format!("%{}%", name)));
        }

        match (filter.upload_begin, filter.upload_end) {
            (Some(begin), Some(end)) => {
                let b = format!("{} 00:00:00", begin);
                let e = format!("{} 23:59:59", end);
                let b = chrono::NaiveDateTime::parse_from_str(&b, "%Y-%m-%d %H:%M:%S").unwrap();
                let e = chrono::NaiveDateTime::parse_from_str(&e, "%Y-%m-%d %H:%M:%S").unwrap();

                conditions =
                    conditions.add(Column::UploadTime.between(b.timestamp(), e.timestamp()));
            }
            (Some(begin), None) => {
                let b = format!("{} 00:00:00", begin);
                let b = chrono::NaiveDateTime::parse_from_str(&b, "%Y-%m-%d %H:%M:%S").unwrap();
                conditions = conditions.add(Column::UploadTime.gt(b.timestamp()));
            }
            (None, Some(end)) => {
                let e = format!("{} 23:59:59", end);
                let e = chrono::NaiveDateTime::parse_from_str(&e, "%Y-%m-%d %H:%M:%S").unwrap();
                conditions = conditions.add(Column::UploadTime.lt(e.timestamp()));
            }
            (None, None) => {}
        };
    }

    let list = crate::db::file::Entity::find()
        .filter(conditions)
        .all(conn)
        .await
        .unwrap();

    let total = list.len();

    let mut data = Vec::new();

    let mut headers = HeaderMap::new();
    headers.insert("content-range", format!("/{}", total).parse().unwrap());

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

pub async fn file_list(
    // _claims: Claims,
    Query(params): Query<HashMap<String, String>>,
    Extension(ref conn): Extension<DbConn>,
) -> impl IntoResponse {
    debug!(?params);
    // debug!(?pagination);
    use crate::db::file::Column;

    let mut conditions = Condition::all();

    if params.contains_key("filter") {
        let filter = serde_json::from_str::<Filter>(params.get("filter").unwrap()).unwrap();
        if let Some(name) = filter.name {
            conditions = conditions.add(Column::Name.eq(name));
        }
        // todo update time
    }

    let mut start_index = 0;
    let mut end_index = 0;

    if params.contains_key("range") {
        let (start, end) =
            serde_json::from_str::<(i32, i32)>(params.get("range").unwrap()).unwrap();

        start_index = start;
        end_index = end;
    }

    params.get("filter").map(|filter| {
        serde_json::from_str::<Filter>(filter).unwrap();
    });

    params.get("range").map(|range| {
        let range = serde_json::from_str::<(u32, u32)>(range).unwrap();
        info!(?range);
    });

    params.get("sort").map(|sort| {
        let sort = serde_json::from_str::<(String, String)>(sort).unwrap();
        info!(?sort);
    });

    // let page = params.page.unwrap_or(1);
    let page = 1;
    // let posts_per_page = params.posts_per_page.unwrap_or(20);
    let posts_per_page = 20;
    // TODO find a way to search update time start and end
    conditions = conditions.add(Column::IsDelete.eq(0));
    // if let Some(name) = params.name {
    //     conditions = conditions.add(Column::Name.like(&format!("%{}%", &name)));
    // }

    // conditions = conditions.add(Column::UploadTime.between(
    //     params.upload_time_begin.unwrap_or(0),
    //     params.upload_time_end.unwrap_or(get_epoch()),
    // ));

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
            id: m.id,
            name: m.name,
            upload_time: m.upload_time.to_string(),
            operator: m.operator,
            location: m.location,
            size: m.size,
        });
    }
    // Content-Range: posts 0-24/319
    // let mut headers = HeaderMap::new();
    // headers.insert("content-range", "files 0-24/319".parse().unwrap());

    return (SetHeader("content-range", "files 0-24/319"), Json(result));
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
                location: Set(format!("http://192.168.1.23:8080/{}", name)),
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

pub async fn download(req: Request<Body>) -> impl IntoResponse {
    debug!(?req);
    // image_dir 地址为 /home/user/images/
    // path 为 images/name
    // 根据fallback的原理 找不到的路由会调用到这里
    // 如果文件的路径示 /name
    // 会调用 / 路由，从而产生错误
    // 确保 ServeDir::new(image_path).oneshot(req) 能找到正确的地址
    let image_dir = env::var("HOME").expect("NO IMAGE_PATH");
    info!(?image_dir);
    let path = req.uri().path().to_string();
    info!(?path);

    return match ServeDir::new(image_dir).oneshot(req).await {
        Ok(res) => Ok(res.map(axum::body::boxed)),
        Err(e) => Err(format!("{}", e)),
    };
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

pub async fn handler() -> impl IntoResponse {
    // `File` implements `AsyncRead`
    let file = match tokio::fs::File::open("Cargo.toml").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let headers = SetHeader("content-disposition", "attachment; filename=\"Cargo.toml\"");

    Ok((headers, body))
}
