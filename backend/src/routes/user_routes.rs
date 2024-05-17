use crate::api_error::ApiError;
use crate::helpers::try_get_connection;
use crate::models::group_model::Group;
use crate::models::group_user_model::GroupUser;
use crate::models::jwt_model::{Claims, JWTInternal};
use crate::models::role_model::Role;
use crate::models::role_user_model::RoleUser;
use crate::models::user_model::{NewUser, SafeUser, User};
use crate::StorageState;
use actix_web::{web, HttpMessage, HttpRequest};
use log::error;
use serde::{Deserialize, Serialize};

pub(crate) async fn create_user(
    form_data: web::Json<NewUser>,
    db: web::Data<StorageState>,
) -> Result<web::Json<User>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let user = User::create(&mut db, &form_data.0)?;
    let public_group = &Group::get(&mut db, 1)?;
    GroupUser::add_user_to_group(&mut db, &user, public_group)?; // group public with id 0 should always exist
    Ok(web::Json(user))
}

pub(crate) async fn all_users(
    db: web::Data<StorageState>,
) -> Result<web::Json<Vec<User>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let all_users = User::get_all(&mut db)?;
    Ok(web::Json(all_users))
}

pub(crate) async fn one_user(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<web::Json<User>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let uid = path.into_inner();
    let user = User::get(&mut db, uid)?;
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
    let mut user_retrieved = User::get(&mut db, uid)?;
    if let Some(new_login) = user_update_payload.new_login.clone() {
        user_retrieved.login = new_login;
    };
    if let Some(new_hash) = user_update_payload.new_hash.clone() {
        if new_hash.len() < 4 {
            return Err(ApiError::User);
        }
        user_retrieved.hash = match bcrypt::hash(new_hash, 12) {
            Ok(new_hash) => new_hash,
            Err(e) => {
                error!("{e:?}");
                return Err(ApiError::Internal);
            }
        };
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
    let user = User::get(&mut db, uid)?;

    if RoleUser::roles_from_user(&mut db, &user)? == Role::from("root")? {
        Err(ApiError::CantDeleteRoot)
    } else {
        JWTInternal::invalidate_user(&mut db, &user)?;
        User::delete_user(&mut db, &user)?;
        Ok("deleted.")
    }
}

pub(crate) async fn get_user_groups(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<web::Json<Vec<Group>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let id = path.into_inner();
    let user = User::get(&mut db, id)?;
    Ok(web::Json(User::get_groups(&mut db, &user)?))
}

#[derive(Serialize, Deserialize)]
pub struct UserDataResponsePayload {
    user: SafeUser,
    role: Role,
    groups: Vec<Group>,
}

pub(crate) async fn get_user_data<'a>(
    req: HttpRequest,
) -> Result<web::Json<UserDataResponsePayload>, ApiError> {
    let claims = match req.extensions().get::<Claims>() {
        Some(claims) => claims.clone(),
        None => return Err(ApiError::Internal),
    };
    Ok(web::Json(UserDataResponsePayload {
        user: claims.user.into(),
        groups: claims.groups,
        role: claims.role,
    }))
}
