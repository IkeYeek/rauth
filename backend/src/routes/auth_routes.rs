use crate::api_error::ApiError;
use crate::helpers::try_get_connection;
use crate::models::group_model::Groups;
use crate::models::group_user_model::GroupUser;
use crate::models::jwt_model::JWTInternal;
use crate::models::role_model::Role;
use crate::models::user_model::User;
use crate::{KeySet, StorageState};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use actix_web::{web, HttpRequest, HttpResponse};
use log::{error, info};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
pub(crate) struct AuthPayload {
    login: String,
    password: String,
}
#[derive(Serialize)]
struct AuthResponse {
    jwt: String,
}

#[inline]
fn add_close_window_js_script_to_response() -> String {
    "
    you can now close this window.
    <script>
    window.close();
    </script>
    "
    .to_string()
}

pub(crate) async fn auth(
    db: web::Data<StorageState>,
    payload: web::Form<AuthPayload>,
    key_set: web::Data<KeySet>,
) -> Result<HttpResponse, ApiError> {
    let mut db = try_get_connection(&db)?;
    let user = User::lookup(&mut db, &payload.login, &payload.password)?;
    let new_jwt = JWTInternal::create(&mut db, &user, &key_set.encoding)?;
    JWTInternal::register(&mut db, &new_jwt)?;
    let jwt_cookie = Cookie::build("jwt", &new_jwt.token)
        .path("/")
        .domain(".localhost.dummy")
        .max_age(Duration::weeks(1))
        .finish();
    let mut response = HttpResponse::Ok()
        .insert_header(actix_web::http::header::ContentType::html())
        .body(format!(
            "logged in. {}",
            add_close_window_js_script_to_response()
        ));
    match response.add_cookie(&jwt_cookie) {
        Ok(()) => Ok(response),
        Err(e) => {
            error!("{e:?}");
            Err(ApiError::Internal)
        }
    }
}

pub(crate) async fn logout(
    db: web::Data<StorageState>,
    req: HttpRequest,
    key_set: web::Data<KeySet>,
) -> Result<HttpResponse, ApiError> {
    let mut db = try_get_connection(&db)?;
    let Some(jwt) = req.cookie("jwt") else {
        return Err(ApiError::Jwt);
    };
    let claims = match JWTInternal::validate_jwt(&mut db, jwt.value(), &key_set.decoding) {
        Ok(claims) => claims,
        Err(e) => {
            error!("{e:?}");
            return Err(ApiError::Internal);
        }
    };
    match JWTInternal::delete(&mut db, &claims.jti) {
        Ok(()) => (),
        Err(e) => {
            error!("{e:?}");
            return Err(ApiError::Internal);
        }
    }

    let mut response = HttpResponse::Ok()
        .insert_header(actix_web::http::header::ContentType::html())
        .body(format!(
            "logged out.{}",
            add_close_window_js_script_to_response()
        ));
    response.del_cookie("jwt");
    Ok(response)
}

#[derive(Serialize, Deserialize)]
pub(crate) struct AccessQS {
    origin: String,
}
pub(crate) async fn has_access(
    db: web::Data<StorageState>,
    access_data: web::Query<AccessQS>,
    role: Role,
    groups: Groups,
) -> Result<HttpResponse, ApiError> {
    let mut db = try_get_connection(&db)?;
    if role == Role::from("root").unwrap() {
        return Ok(HttpResponse::Ok().body("granted my dear looord"));
    }
    match Url::parse(&access_data.origin) {
        Ok(parsed_url) => match parsed_url.host_str() {
            Some(origin_host) => {
                GroupUser::user_allowed_to_origin(
                    &mut db,
                    &access_data.origin,
                    origin_host,
                    &groups.0.iter().map(|g| g.id).collect::<Vec<i32>>(),
                )?;
                Ok(HttpResponse::Ok().body("granted"))
            }
            None => Err(ApiError::Internal),
        },
        Err(e) => {
            info!("bad api usage {e:?} - {:?}", access_data.origin);
            Err(ApiError::User)
        }
    }
}

pub(crate) async fn is_auth() -> Result<&'static str, ApiError> {
    Ok("authed")
}
