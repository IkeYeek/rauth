use crate::api_error::ApiError;
use crate::models::group_model::Group;
use crate::models::role_model::Role;
use crate::models::role_user_model::RoleUser;
use crate::models::user_model::User;
use diesel::{
    insert_into, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SqliteConnection,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::error;
use serde::{Deserialize, Serialize};
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

pub(crate) struct JWTInternal {
    pub(crate) claims: Claims,
    pub(crate) token: String,
}
#[derive(Insertable, Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name = crate::schema::jwt)]
struct Jwt {
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
        Err(ApiError::Internal)
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
            role: RoleUser::roles_from_user(db, user)?,
            groups: User::get_groups(db, user)?,
        };
        Self::from(&claims, key)
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
            Ok(res) =>  Ok(res > 0),
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
        let refreshed_jwt = JWTInternal::create(db, user, key)?;
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
        if let Ok(claims) = decode::<Claims>(raw_token, key, &validation) {
            let claims = claims.claims;
            if claims.exp < chrono::Utc::now().timestamp() || !Self::is_valid_jti(db, &claims.jti) {
                return Err(ApiError::Jwt);
            }
            return Ok(claims.clone());
        }
        Err(ApiError::Jwt)
    }

    pub(crate) fn register(db: &mut SqliteConnection, token: &JWTInternal) -> Result<(), ApiError> {
        let insertable_jwt = Jwt {
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
