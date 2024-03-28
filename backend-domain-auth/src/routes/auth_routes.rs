use crate::api_error::ApiError;
use crate::models::jwt::JWT;
use crate::models::role_user_model::RoleUser;
use crate::models::user_model::User;
use crate::{AppDatabaseState, KeySet};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AuthPayload {
    login: String,
    hash: String,
}
#[post("/")]
async fn auth(
    db: web::Data<AppDatabaseState>,
    payload: web::Json<AuthPayload>,
    key_set: web::Data<KeySet>,
) -> Result<HttpResponse, ApiError> {
    match db.db.lock() {
        Ok(mut db) => {
            let user = User::lookup(&mut *db, &payload.login, &payload.hash)?;
            let user_groups = User::get_groups(&mut *db, &user)?;
            let role = RoleUser::roles_from_user(&mut *db, &user)?;
            let jwt = JWT::create(&role, &user_groups, &key_set.encoding)?;
            JWT::register(&mut *db, &jwt)?;
            let jwt_cookie = Cookie::build("jwt", &jwt.token)
                .domain(".localhost.dummy")
                .max_age(Duration::weeks(1))
                .finish();
            let mut response = HttpResponse::Ok().body("authed.");
            match response.add_cookie(&jwt_cookie) {
                Ok(()) => Ok(response),
                Err(e) => {
                    eprintln!("{e:?}");
                    Err(ApiError::Internal)
                }
            }
        }
        Err(e) => {
            eprintln!("{e:?}");
            Err(ApiError::Internal)
        }
    }
}

#[get("/")]
pub(crate) async fn is_auth(
    req: HttpRequest,
    db: web::Data<AppDatabaseState>,
    key_set: web::Data<KeySet>,
) -> Result<&'static str, ApiError> {
    match db.db.lock() {
        Ok(mut db) => match req.cookie("jwt") {
            Some(jwt_cookie) => {
                JWT::validate_jwt(&mut db, jwt_cookie.value(), &key_set.decoding)?;
                Ok("authed.")
            }
            None => Err(ApiError::JWT),
        },
        Err(e) => {
            eprintln!("{e:?}");
            Err(ApiError::Internal)
        }
    }
}
