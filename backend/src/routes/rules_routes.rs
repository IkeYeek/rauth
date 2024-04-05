use crate::api_error::ApiError;
use crate::helpers::try_get_connection;
use crate::models::domain_rule_model::{DomainRule, NewDomainRule};
use crate::models::group_model::Group;
use crate::models::url_rule_model::{NewURLRule, URLRule};
use crate::models::user_model::User;
use crate::StorageState;
use actix_web::web;
use serde::{Deserialize, Serialize};

pub(crate) async fn add_domain_rule(
    db: web::Data<StorageState>,
    payload: web::Json<NewDomainRule>,
) -> Result<web::Json<DomainRule>, ApiError> {
    let mut db = try_get_connection(&db)?;
    Ok(web::Json(DomainRule::create(&mut db, &payload.0)?))
}

pub(crate) async fn delete_domain_rule(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<&'static str, ApiError> {
    let mut db = try_get_connection(&db)?;
    let rule_id = path.into_inner();
    DomainRule::delete(&mut db, rule_id)?;
    Ok("deleted.")
}

pub(crate) async fn list_domain_rules(
    db: web::Data<StorageState>,
) -> Result<web::Json<Vec<DomainRule>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    Ok(web::Json(DomainRule::get_all(&mut db)?))
}

pub(crate) async fn domain_rule(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<web::Json<DomainRule>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let rule_id = path.into_inner();
    Ok(web::Json(DomainRule::get(&mut db, rule_id)?))
}

pub(crate) async fn domain_rules_for_domain(
    db: web::Data<StorageState>,
    path: web::Path<String>,
) -> Result<web::Json<Vec<DomainRule>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let domain = path.into_inner();
    Ok(web::Json(DomainRule::for_domain(&mut db, &domain)?))
}
pub(crate) async fn domain_rules_for_group(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<web::Json<Vec<DomainRule>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let group_id = path.into_inner();
    let group = Group::get(&mut db, group_id)?;
    Ok(web::Json(DomainRule::for_group(&mut db, &group)?))
}
pub(crate) async fn domain_rules_for_user(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<web::Json<Vec<DomainRule>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let user_id = path.into_inner();
    let user = User::get(&mut db, user_id)?;
    Ok(web::Json(DomainRule::for_user(&mut db, &user)?))
}
pub(crate) async fn add_url_rule(
    db: web::Data<StorageState>,
    payload: web::Json<NewURLRule>,
) -> Result<web::Json<URLRule>, ApiError> {
    let mut db = try_get_connection(&db)?;
    Ok(web::Json(URLRule::create(&mut db, &payload.0)?))
}

pub(crate) async fn delete_url_rule(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<&'static str, ApiError> {
    let mut db = try_get_connection(&db)?;
    let rule_id = path.into_inner();
    URLRule::delete(&mut db, rule_id)?;
    Ok("deleted.")
}

pub(crate) async fn list_url_rules(
    db: web::Data<StorageState>,
) -> Result<web::Json<Vec<URLRule>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    Ok(web::Json(URLRule::get_all(&mut db)?))
}

pub(crate) async fn url_rule(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<web::Json<URLRule>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let rule_id = path.into_inner();
    Ok(web::Json(URLRule::get(&mut db, rule_id)?))
}

pub(crate) async fn url_rules_for_url(
    db: web::Data<StorageState>,
    path: web::Path<String>,
) -> Result<web::Json<Vec<URLRule>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let url = path.into_inner();
    Ok(web::Json(URLRule::for_url(&mut db, &url)?))
}
pub(crate) async fn url_rules_for_group(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<web::Json<Vec<URLRule>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let group_id = path.into_inner();
    let group = Group::get(&mut db, group_id)?;
    Ok(web::Json(URLRule::for_group(&mut db, &group)?))
}
pub(crate) async fn url_rules_for_user(
    db: web::Data<StorageState>,
    path: web::Path<i32>,
) -> Result<web::Json<Vec<URLRule>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let user_id = path.into_inner();
    let user = User::get(&mut db, user_id)?;
    Ok(web::Json(URLRule::for_user(&mut db, &user)?))
}
