use crate::api_error::ApiError;
use crate::helpers::try_get_connection;
use crate::models::domain_rule_model::{DomainRule, NewDomainRule};
use crate::models::url_rule_model::{NewURLRule, URLRule};
use crate::StorageState;
use actix_web::web;

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
