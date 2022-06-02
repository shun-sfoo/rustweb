use std::collections::HashMap;

use axum::{extract::Query, Extension, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbConn, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

use crate::model::claims::Claims;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
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
    update_begin: Option<String>,
    upload_end: Option<String>,
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
            username: model.name,
            token: Some(token),
        },
    }))
}

pub async fn file_list(
    _claims: Claims,
    Query(params): Query<FileParams>,
    Extension(ref conn): Extension<DbConn>,
) {
    use crate::db::file::Column;

    let page = params.page.unwrap_or(1);
    let posts_per_page = params.posts_per_page.unwrap_or(20);
    let mut conditions = Condition::all();
    if let Some(name) = params.filename {
        conditions.add(Column::Name.like(&name));
    }

    // if params.update_begin.is_some() && params.update_begin.is_some() {
    //     conditions.add(
    //         Column::UploadTime.between(params.update_begin.unwrap(), params.upload_end.unwrap()),
    //     );
    // }
    //

    let paginator = crate::db::file::Entity::find()
        // .filter()
        .order_by_asc(Column::UploadTime)
        .paginate(conn, posts_per_page);

    todo!()
}

pub async fn me(claims: Claims, Extension(ref conn): Extension<DbConn>) -> Json<Option<User>> {
    let id = claims.id;
    match crate::db::user::Entity::find_by_id(id)
        .one(conn)
        .await
        .unwrap()
    {
        Some(m) => Json(Some(User {
            id: m.id,
            username: m.name,
            token: None,
        })),
        None => Json(None),
    }
}
