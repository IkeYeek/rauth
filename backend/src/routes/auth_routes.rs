use crate::api_error::ApiError;
use crate::helpers::try_get_connection;
use crate::models::group_model::{Group, Groups};
use crate::models::group_user_model::GroupUser;
use crate::models::jwt_model::{Claims, JWTInternal};
use crate::models::role_model::Role;
use crate::models::user_model::User;
use crate::{KeySet, StorageState};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use log::info;
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
pub(crate) async fn auth(
    db: web::Data<StorageState>,
    payload: web::Json<AuthPayload>,
    key_set: web::Data<KeySet>,
) -> Result<HttpResponse, ApiError> {
    let mut db = try_get_connection(&db)?;
    let user = User::lookup(&mut db, &payload.login, &payload.password)?;
    let new_jwt = JWTInternal::create(&mut db, &user, &key_set.encoding)?;
    JWTInternal::register(&mut db, &new_jwt)?;
    /*let jwt_cookie = Cookie::build("jwt", &new_jwt.token)
    .domain(".localhost.dummy")
    .max_age(Duration::weeks(1))
    .finish();*/
    let response = HttpResponse::Ok().json(AuthResponse { jwt: new_jwt.token });
    Ok(response)
    /*match response.add_cookie(&jwt_cookie) {
        Ok(()) => Ok(response),
        Err(e) => {
            error!("{e:?}");
            Err(ApiError::Internal)
        }
    }*/
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