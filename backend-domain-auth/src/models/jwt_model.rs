use crate::api_error::ApiError;
use crate::helpers::try_get_connection;
use crate::models::group_model::Group;
use crate::models::role_model::Role;
use crate::models::role_user_model::RoleUser;
use crate::models::user_model::User;
use crate::{KeySet, StorageState};
use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use diesel::{
    insert_into, ExpressionMethods, Insertable, QueryDsl, QueryResult, Queryable, RunQueryDsl,
    Selectable, SelectableHelper, SqliteConnection,
};
use dotenvy::Error;
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::LockResult;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Claims {
    pub(crate) company: String,
    pub(crate) exp: i64,
    pub(crate) jti: String,
    pub(crate) user: User,
    pub(crate) role: Role,
    pub(crate) groups: Vec<Group>,
}

impl FromRequest for Claims {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = match req.headers().get("Authorization") {
            Some(header) => match header.to_str() {
                Ok(header) if header.starts_with("bearer ") || header.starts_with("Bearer ") => {
                    header[7..].trim()
                }
                _ => return err(ApiError::JWT),
            },
            None => return err(ApiError::JWT),
        };

        let db = req
            .app_data::<web::Data<StorageState>>()
            .and_then(|data| data.db.lock().ok());
        let key_set = req.app_data::<web::Data<KeySet>>();

        match (db, key_set) {
            (Some(mut db), Some(key_set)) => {
                match JWTInternal::validate_jwt(&mut db, auth_header, &key_set.decoding) {
                    Ok(token) => ok(token),
                    Err(e) => err(e),
                }
            }
            _ => err(ApiError::JWT),
        }
    }
}

pub(crate) struct JWTInternal {
    pub(crate) claims: Claims,
    pub(crate) token: String,
}
#[derive(Insertable, Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name = crate::schema::jwt)]
struct JWT {
    jwt_id: String,
    needs_refresh: i32,
    user_id: i32,
}

impl JWTInternal {
    pub(crate) fn from(claims: &Claims, key: &EncodingKey) -> Result<Self, ApiError> {
        let token = encode(&Header::new(Algorithm::EdDSA), &claims, key);
        if let Ok(token) = token {
            return Ok(JWTInternal {
                token,
                claims: claims.clone(),
            });
        };
        return Err(ApiError::Internal);
    }
    pub(crate) fn create(
        db: &mut SqliteConnection,
        user: &User,
        key: &EncodingKey,
    ) -> Result<Self, ApiError> {
        let id = Uuid::new_v4();
        let claims = Claims {
            company: String::from("I.K.E"),
            exp: chrono::Utc::now().timestamp() + 3600 * 24 * 7,
            jti: id.to_string(),
            user: user.clone(),
            role: RoleUser::roles_from_user(db, &user)?,
            groups: User::get_groups(db, &user)?,
        };
        Ok(Self::from(&claims, key)?)
    }
    pub(crate) fn needs_refresh(
        db: &mut SqliteConnection,
        claims: &Claims,
    ) -> Result<bool, ApiError> {
        match crate::schema::jwt::dsl::jwt
            .filter(crate::schema::jwt::dsl::jwt_id.eq(&claims.jti))
            .filter(crate::schema::jwt::dsl::needs_refresh.eq(1))
            .count()
            .get_result::<i64>(db)
        {
            Ok(res) => return Ok(res > 0),
            Err(e) => {
                error!("{e:?}");
                Err(ApiError::Internal)
            }
        }
    }

    pub(crate) fn delete(db: &mut SqliteConnection, jti: &str) -> Result<(), ApiError> {
        if let Err(e) = diesel::delete(
            crate::schema::jwt::dsl::jwt.filter(crate::schema::jwt::dsl::jwt_id.eq(jti)),
        )
        .execute(db)
        {
            error!("{e:?}");
            return Err(ApiError::Internal);
        }
        Ok(())
    }

    pub(crate) fn refresh(
        db: &mut SqliteConnection,
        user: &User,
        jti: &str,
        key: &EncodingKey,
    ) -> Result<JWTInternal, ApiError> {
        let refreshed_jwt = JWTInternal::create(db, &user, key)?;
        JWTInternal::delete(db, jti)?;
        Ok(refreshed_jwt)
    }

    pub(crate) fn refresh_for_user(db: &mut SqliteConnection, user: &User) -> Result<(), ApiError> {
        match diesel::update(crate::schema::jwt::dsl::jwt)
            .filter(crate::schema::jwt::dsl::user_id.eq(user.id))
            .set(crate::schema::jwt::dsl::needs_refresh.eq(1))
            .execute(db)
        {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{e:?}");
                Err(ApiError::Internal)
            }
        }
    }

    pub(crate) fn invalidate_user(db: &mut SqliteConnection, user: &User) -> Result<(), ApiError> {
        match diesel::delete(
            crate::schema::jwt::dsl::jwt.filter(crate::schema::jwt::dsl::user_id.eq(user.id)),
        )
        .execute(db)
        {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{e:?}");
                Err(ApiError::Internal)
            }
        }
    }

    fn is_valid_jti(db: &mut SqliteConnection, jti: &str) -> bool {
        match crate::schema::jwt::dsl::jwt
            .filter(crate::schema::jwt::dsl::jwt_id.eq(jti))
            .count()
            .get_result::<i64>(db)
        {
            Ok(n) => n == 1,
            Err(_) => false,
        }
    }

    pub(crate) fn validate_jwt(
        db: &mut SqliteConnection,
        raw_token: &str,
        key: &DecodingKey,
    ) -> Result<Claims, ApiError> {
        let validation = Validation::new(Algorithm::EdDSA);
        if let Ok(claims) = decode::<Claims>(&raw_token, &*key, &validation) {
            let claims = claims.claims;
            if claims.exp < chrono::Utc::now().timestamp() || !Self::is_valid_jti(db, &claims.jti) {
                return Err(ApiError::JWT);
            }
            return Ok(claims.clone());
        }
        Err(ApiError::JWT)
    }

    pub(crate) fn register(db: &mut SqliteConnection, token: &JWTInternal) -> Result<(), ApiError> {
        let insertable_jwt = JWT {
            jwt_id: token.claims.jti.clone(),
            needs_refresh: 0,
            user_id: token.claims.user.id,
        };
        insert_into(crate::schema::jwt::dsl::jwt)
            .values(&insertable_jwt)
            .execute(db)
            .map_err(|e| {
                error!("{e:?}");
                ApiError::Internal
            })?;
        Ok(())
    }
}
