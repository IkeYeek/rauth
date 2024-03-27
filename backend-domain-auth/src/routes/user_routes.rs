use crate::api_error::ApiError;
use crate::models::group_model::Group;
use crate::models::user_model::{NewUser, User};
use crate::AppDatabaseState;
use actix_web::{delete, get, patch, post, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct CustomResponse<'a> {
    message: &'a str,
}

#[post("/")]
pub(crate) async fn create_user(
    form_data: web::Json<NewUser>,
    db: web::Data<AppDatabaseState>,
) -> Result<&'static str, ApiError> {
    match db.db.lock() {
        Ok(mut db) => {
            let _ = User::create(&mut db, &form_data.0)?;
            Ok("created.")
        }
        Err(e) => {
            eprintln!("{e:?}");
            return Err(ApiError::Internal);
        }
    }
}

#[get("/")]
pub(crate) async fn all_users(
    db: web::Data<AppDatabaseState>,
) -> Result<web::Json<Vec<User>>, ApiError> {
    match db.db.lock() {
        Ok(mut db) => {
            let all_users = User::read_all(&mut db)?;
            Ok(web::Json(all_users))
        }
        Err(e) => {
            eprintln!("{e:?}");
            return Err(ApiError::User);
        }
    }
}

#[get("/{user}")]
pub(crate) async fn one_user(
    db: web::Data<AppDatabaseState>,
    path: web::Path<i32>,
) -> Result<web::Json<User>, ApiError> {
    match db.db.lock() {
        Ok(mut db) => {
            let uid = path.into_inner();
            let user = User::read_by_id(&mut db, uid)?;
            Ok(web::Json(user))
        }
        Err(e) => {
            eprintln!("{e:?}");
            Err(ApiError::Internal)
        }
    }
}

#[derive(Serialize, Deserialize)]
struct UserUpdatePayload {
    new_login: Option<String>,
    new_hash: Option<String>,
}
#[patch("/{user}")]
async fn update_user(
    db: web::Data<AppDatabaseState>,
    user_update_payload: web::Json<UserUpdatePayload>,
    path: web::Path<i32>,
) -> Result<&'static str, ApiError> {
    let uid = path.into_inner();
    return match db.db.lock() {
        Ok(mut db) => {
            let mut user_retrieved = User::read_by_id(&mut db, uid)?;
            if let Some(new_login) = user_update_payload.new_login.clone() {
                user_retrieved.login = new_login;
            };
            if let Some(new_hash) = user_update_payload.new_hash.clone() {
                user_retrieved.hash = new_hash;
            };
            User::update_user(&mut db, &user_retrieved)?;
            Ok("updated.")
        }
        Err(e) => {
            eprintln!("{e:?}");
            return Err(ApiError::Internal);
        }
    };
}

#[delete("/{user}")]
async fn delete_user(
    db: web::Data<AppDatabaseState>,
    path: web::Path<i32>,
) -> Result<&'static str, ApiError> {
    match db.db.lock() {
        Ok(mut db) => {
            let uid = path.into_inner();
            let user = User::read_by_id(&mut db, uid)?;
            User::delete_user(&mut db, &user)?;
            Ok("deleted.")
        }
        Err(e) => {
            eprintln!("{e:?}");
            Err(ApiError::Internal)
        }
    }
}

#[get("/{user}/groups")]
pub(crate) async fn get_user_groups(
    db: web::Data<AppDatabaseState>,
    path: web::Path<i32>,
) -> Result<web::Json<Vec<Group>>, ApiError> {
    match db.db.lock() {
        Ok(mut db) => {
            let id = path.into_inner();
            let user = User::read_by_id(&mut db, id)?;
            Ok(web::Json(User::get_groups(&mut db, &user)?))
        }
        Err(e) => {
            eprintln!("{e:?}");
            Err(ApiError::Internal)
        }
    }
}
