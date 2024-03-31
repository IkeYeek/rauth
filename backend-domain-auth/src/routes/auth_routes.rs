use crate::api_error::ApiError;
use crate::models::group_user_model::GroupUser;
use crate::models::jwt_model::JWT;
use crate::models::role_user_model::RoleUser;
use crate::models::user_model::User;
use crate::{AppDatabaseState, KeySet};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
    TextExpressionMethods,
};
use serde::{Deserialize, Serialize};
use url::Url;

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
            let new_jwt = JWT::create(&role, &user_groups, &key_set.encoding)?;
            JWT::register(&mut *db, &new_jwt)?;
            let jwt_cookie = Cookie::build("jwt", &new_jwt.token)
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

#[derive(Serialize, Deserialize)]
struct AccessQS {
    origin: String,
}
#[get("/has_access")]
pub(crate) async fn has_access(
    request: HttpRequest,
    db: web::Data<AppDatabaseState>,
    access_data: web::Query<AccessQS>,
    key_set: web::Data<KeySet>,
) -> Result<&'static str, ApiError> {
    match (
        db.db.lock(),
        Url::parse(&access_data.origin),
        request.cookie("jwt"),
    ) {
        (Ok(mut db), Ok(parsed_url), Some(user_jwt)) => {
            let req_jwt = JWT::validate_jwt(&mut *db, user_jwt.value(), &key_set.decoding)?;
            match parsed_url.host_str() {
                Some(origin_host) => {
                    if req_jwt.roles.role == "root" {
                        return Ok("granted my dear lord.");
                    }
                    let group_ids: Vec<i32> = req_jwt.groups.iter().map(|g| g.id).collect();

                    GroupUser::user_allowed_to_origin(
                        &mut *db,
                        &access_data.origin,
                        origin_host,
                        &group_ids,
                    )?;
                    Ok("granted.")
                }
                None => Err(ApiError::JWT),
            }
        }
        _ => Err(ApiError::JWT),
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
