use crate::api_error::ApiError;
use crate::helpers::try_get_connection;
use crate::models::group_model::Group;
use crate::models::jwt_model::JWTInternal;
use crate::models::user_model::{NewUser, User};
use crate::StorageState;
use actix_web::web;
use serde::{Deserialize, Serialize};

pub(crate) async fn create_user(
    form_data: web::Json<NewUser>,
    db: web::Data<StorageState>,
) -> Result<&'static str, ApiError> {
    let mut db = try_get_connection(&db)?;
    User::create(&mut db, &form_data.0)?;
    Ok("created.")
}

pub(crate) async fn all_users(
    db: web::Data<StorageState>,
) -> Result<web::Json<Vec<User>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let all_users = User::read_all(&mut db)?;
    Ok(web::Json(all_users))
}

pub(crate) async fn one_user(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<web::Json<User>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let uid = path.into_inner();
    let user = User::read_by_id(&mut db, uid)?;
    Ok(web::Json(user))
}

#[derive(Serialize, Deserialize)]
pub(crate) struct UserUpdatePayload {
    new_login: Option<String>,
    new_hash: Option<String>,
}
pub(crate) async fn update_user(
    db: web::Data<StorageState>,
    user_update_payload: web::Json<UserUpdatePayload>,
    path: web::Path<i32>,
) -> Result<&'static str, ApiError> {
    let mut db = try_get_connection(&db)?;
    let uid = path.into_inner();
    let mut user_retrieved = User::read_by_id(&mut db, uid)?;
    if let Some(new_login) = user_update_payload.new_login.clone() {
        user_retrieved.login = new_login;
    };
    if let Some(new_hash) = user_update_payload.new_hash.clone() {
        user_retrieved.hash = new_hash;
    };
    User::update_user(&mut db, &user_retrieved)?;
    Ok("updated.")
}

pub(crate) async fn delete_user(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<&'static str, ApiError> {
    let mut db = try_get_connection(&db)?;
    let uid = path.into_inner();
    let user = User::read_by_id(&mut db, uid)?;
    JWTInternal::invalidate_user(&mut db, &user)?;
    User::delete_user(&mut db, &user)?;
    Ok("deleted.")
}

pub(crate) async fn get_user_groups(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<web::Json<Vec<Group>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let id = path.into_inner();
    let user = User::read_by_id(&mut db, id)?;
    Ok(web::Json(User::get_groups(&mut db, &user)?))
}
