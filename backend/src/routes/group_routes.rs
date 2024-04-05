use crate::api_error::ApiError;
use crate::helpers::try_get_connection;
use crate::models::group_model::{Group, NewGroup};
use crate::models::group_user_model::GroupUser;
use crate::models::user_model::User;
use crate::schema::users::dsl::users;
use crate::StorageState;
use actix_web::web;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};

pub(crate) async fn create_group(
    db: web::Data<StorageState>,
    new_group: web::Json<NewGroup>,
) -> Result<web::Json<Group>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let g = Group::create_group(&mut db, &new_group)?;
    Ok(web::Json(g))
}

pub(crate) async fn all_groups(
    db: web::Data<StorageState>,
) -> Result<web::Json<Vec<Group>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let all_groups = Group::get_all(&mut db)?;
    Ok(web::Json(all_groups))
}

pub(crate) async fn one_group(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<web::Json<Group>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let group_id = path.into_inner();
    let group = Group::get(&mut db, group_id)?;
    Ok(web::Json(group))
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GroupUpdatePayload {
    new_name: Option<String>,
}
pub(crate) async fn update_group(
    db: web::Data<StorageState>,
    group_update_payload: web::Json<GroupUpdatePayload>,
    path: web::Path<i32>,
) -> Result<web::Json<Group>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let uid = path.into_inner();
    let mut group_retrieved = Group::get(&mut db, uid)?;
    if let Some(new_name) = group_update_payload.new_name.clone() {
        group_retrieved.name = new_name;
    };
    Group::update_group(&mut db, &group_retrieved)?;
    Ok(web::Json(group_retrieved))
}

pub(crate) async fn delete_group(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<&'static str, ApiError> {
    let mut db = try_get_connection(&db)?;
    let uid = path.into_inner();
    let group = Group::get(&mut db, uid)?;
    Group::delete_group(&mut db, &group)?;
    Ok("deleted.")
}
#[derive(Serialize, Deserialize)]
pub(crate) struct AddGroupPayload {
    user_id: i32,
}
pub(crate) async fn add_user_to_group(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
    payload: web::Json<AddGroupPayload>,
) -> Result<&'static str, ApiError> {
    let mut db = try_get_connection(&db)?;
    let path_data = path.into_inner();
    let user = users
        .filter(crate::schema::users::id.eq(payload.user_id))
        .select(User::as_select())
        .first(&mut *db);
    let group = crate::schema::groups::dsl::groups
        .filter(crate::schema::groups::dsl::id.eq(path_data))
        .select(Group::as_select())
        .first(&mut *db);
    match (user, group) {
        (Ok(user), Ok(group)) => {
            GroupUser::add_user_to_group(&mut db, &user, &group)?;
            Ok("added.")
        }
        _ => Err(ApiError::Group),
    }
}

pub(crate) async fn delete_user_from_group(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
    payload: web::Json<AddGroupPayload>,
) -> Result<&'static str, ApiError> {
    let mut db = try_get_connection(&db)?;
    let group_id = path.into_inner();
    let group = Group::get(&mut db, group_id)?;
    let user = User::get(&mut db, payload.user_id)?;
    GroupUser::remove_user_from_group(&mut db, &user, &group)?;
    Ok("removed.")
}

pub(crate) async fn list_users_from_group(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<web::Json<Vec<User>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let group_id = path.into_inner();
    let group = Group::get(&mut db, group_id)?;
    let group_users = Group::users_from_group(&mut db, &group)?;
    Ok(web::Json(group_users))
}
