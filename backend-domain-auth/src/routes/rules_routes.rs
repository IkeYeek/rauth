use actix_web::web;
use diesel::RunQueryDsl;
use crate::api_error::ApiError;
use crate::helpers::try_get_connection;
use crate::models::domain_rule_model::{DomainRule, NewDomainRule};
use crate::models::url_rule_model::{NewURLRule, URLRule};
use crate::StorageState;

pub(crate) async fn add_domain_rule(db: web::Data<StorageState>, payload: web::Json<NewDomainRule>) -> Result<DomainRule, ApiError> {
    let mut db = try_get_connection(&db)?;
    DomainRule::create(&mut db, &payload.0)
}

pub(crate) async fn remove_domain_rule(db: web::Data<StorageState>, path: web::Path<i32>) -> Result<(), ApiError> {
    let mut db = try_get_connection(&db)?;
    let rule_id = path.into_inner();
    DomainRule::delete(&mut db, rule_id)
}

pub(crate) async fn list_domain_rules(db: web::Data<StorageState>) -> Result<Vec<DomainRule>, ApiError> {
    let mut db = try_get_connection(&db)?;
    DomainRule::get_all(&mut db)
}

pub(crate) async fn domain_rule(db: web::Data<StorageState>, path: web::Path<i32>) -> Result<DomainRule, ApiError> {
    let mut db = try_get_connection(&db)?;
    let rule_id = path.into_inner();
    DomainRule::get(&mut db, rule_id)
}

pub(crate) async fn add_url_rule(db: web::Data<StorageState>, payload: web::Json<NewURLRule>) -> Result<URLRule, ApiError> {
    let mut db = try_get_connection(&db)?;
    URLRule::create(&mut db, &payload.0)
}

pub(crate) async fn remove_url_rule(db: web::Data<StorageState>, path: web::Path<i32>) -> Result<(), ApiError> {
    let mut db = try_get_connection(&db)?;
    let rule_id = path.into_inner();
    URLRule::delete(&mut db, rule_id)
}

pub(crate) async fn list_url_rules(db: web::Data<StorageState>) -> Result<Vec<URLRule>, ApiError> {
    let mut db = try_get_connection(&db)?;
    URLRule::get_all(&mut db)
}

pub(crate) async fn url_rule(db: web::Data<StorageState>, path: web::Path<i32>) -> Result<URLRule, ApiError> {
    let mut db = try_get_connection(&db)?;
    let rule_id = path.into_inner();
    URLRule::get(&mut db, rule_id)
}