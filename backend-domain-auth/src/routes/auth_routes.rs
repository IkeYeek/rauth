use crate::api_error::ApiError;
use crate::helpers::try_get_connection;
use crate::models::group_user_model::GroupUser;
use crate::models::jwt_model::JWTInternal;
use crate::models::user_model::User;
use crate::{KeySet, StorageState};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use actix_web::{web, HttpRequest, HttpResponse};
use log::error;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
pub(crate) struct AuthPayload {
    login: String,
    hash: String,
}
pub(crate) async fn auth(
    db: web::Data<StorageState>,
    payload: web::Json<AuthPayload>,
    key_set: web::Data<KeySet>,
) -> Result<HttpResponse, ApiError> {
    let mut db = try_get_connection(&db)?;
    let user = User::lookup(&mut *db, &payload.login, &payload.hash)?;
    let new_jwt = JWTInternal::create(&mut *db, &user, &key_set.encoding)?;
    JWTInternal::register(&mut *db, &new_jwt)?;
    let jwt_cookie = Cookie::build("jwt", &new_jwt.token)
        .domain(".localhost.dummy")
        .max_age(Duration::weeks(1))
        .finish();
    let mut response = HttpResponse::Ok().body("authed.");
    match response.add_cookie(&jwt_cookie) {
        Ok(()) => Ok(response),
        Err(e) => {
            error!("{e:?}");
            Err(ApiError::Internal)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct AccessQS {
    origin: String,
}
pub(crate) async fn has_access(
    request: HttpRequest,
    db: web::Data<StorageState>,
    access_data: web::Query<AccessQS>,
    key_set: web::Data<KeySet>,
) -> Result<HttpResponse, ApiError> {
    let mut db = try_get_connection(&db)?;
    match (Url::parse(&access_data.origin), request.cookie("jwt")) {
        (Ok(parsed_url), Some(user_jwt)) => {
            let mut res = HttpResponse::Ok().body("granted");
            let req_jwt = JWTInternal::validate_jwt(&mut *db, user_jwt.value(), &key_set.decoding)?;
            if JWTInternal::needs_refresh(&mut *db, &req_jwt)? {
                let refresh_token =
                    JWTInternal::refresh(&mut *db, &req_jwt.user, &req_jwt.jti, &key_set.encoding)?;
                let jwt_cookie = Cookie::build("jwt", &refresh_token.token)
                    .domain(".localhost.dummy")
                    .max_age(Duration::weeks(1))
                    .finish();
                if let Err(e) = res.add_cookie(&jwt_cookie) {
                    error!("{e:?}");
                    return Err(ApiError::Internal);
                }
            }
            match parsed_url.host_str() {
                Some(origin_host) => {
                    if req_jwt.role.role == "root" {
                        return Ok(res);
                    }
                    let group_ids: Vec<i32> = req_jwt.groups.iter().map(|g| g.id).collect();

                    GroupUser::user_allowed_to_origin(
                        &mut *db,
                        &access_data.origin,
                        origin_host,
                        &group_ids,
                    )?;
                    Ok(res)
                }
                None => Err(ApiError::JWT),
            }
        }
        _ => Err(ApiError::JWT),
    }
}

pub(crate) async fn is_auth(
    req: HttpRequest,
    db: web::Data<StorageState>,
    key_set: web::Data<KeySet>,
) -> Result<&'static str, ApiError> {
    let mut db = try_get_connection(&db)?;
    match req.cookie("jwt") {
        Some(jwt_cookie) => {
            JWTInternal::validate_jwt(&mut db, jwt_cookie.value(), &key_set.decoding)?;
            Ok("authed.")
        }
        None => Err(ApiError::JWT),
    }
}
