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

#[derive(Serialize, Deserialize, Debug)]
pub struct PayloadDomainRulesForDomain {
    domain: String,
}
pub(crate) async fn domain_rules_for_domain(
    db: web::Data<StorageState>,
    payload: web::Json<PayloadDomainRulesForDomain>,
) -> Result<web::Json<Vec<DomainRule>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    Ok(web::Json(DomainRule::for_domain(&mut db, &payload.domain)?))
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PayloadDomainRulesForGroup {
    group_id: i32,
}
pub(crate) async fn domain_rules_for_group(
    db: web::Data<StorageState>,
    payload: web::Json<PayloadDomainRulesForGroup>,
) -> Result<web::Json<Vec<DomainRule>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let group = Group::read_by_id(&mut db, payload.group_id)?;
    Ok(web::Json(DomainRule::for_group(&mut db, &group)?))
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PayloadDomainRulesForUser {
    user_id: i32,
}
pub(crate) async fn domain_rules_for_user(
    db: web::Data<StorageState>,
    payload: web::Json<PayloadDomainRulesForUser>,
) -> Result<web::Json<Vec<DomainRule>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let user = User::read_by_id(&mut db, payload.user_id)?;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PayloadURLRulesForURL {
    url: String,
}
pub(crate) async fn url_rules_for_url(
    db: web::Data<StorageState>,
    payload: web::Json<crate::routes::rules_routes::PayloadURLRulesForURL>,
) -> Result<web::Json<Vec<URLRule>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    Ok(web::Json(URLRule::for_url(&mut db, &payload.url)?))
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PayloadURLRulesForGroup {
    group_id: i32,
}
pub(crate) async fn url_rules_for_group(
    db: web::Data<StorageState>,
    payload: web::Json<crate::routes::rules_routes::PayloadURLRulesForGroup>,
) -> Result<web::Json<Vec<URLRule>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let group = Group::read_by_id(&mut db, payload.group_id)?;
    Ok(web::Json(URLRule::for_group(&mut db, &group)?))
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PayloadURLRulesForUser {
    user_id: i32,
}
pub(crate) async fn url_rules_for_user(
    db: web::Data<StorageState>,
    payload: web::Json<crate::routes::rules_routes::PayloadURLRulesForUser>,
) -> Result<web::Json<Vec<URLRule>>, ApiError> {
    let mut db = try_get_connection(&db)?;
    let user = User::read_by_id(&mut db, payload.user_id)?;
    Ok(web::Json(URLRule::for_user(&mut db, &user)?))
}
