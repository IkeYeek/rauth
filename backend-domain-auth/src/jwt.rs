use crate::api_error::ApiError;
use crate::models::group_model::Group;
use crate::models::role_model::Role;
use diesel::{insert_into, ExpressionMethods, Insertable, QueryDsl, RunQueryDsl, SqliteConnection};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Claims {
    pub(crate) company: String,
    pub(crate) exp: i64,
    pub(crate) jti: String,
    pub(crate) roles: Role,
    pub(crate) groups: Vec<Group>,
}

pub(crate) struct JWT {
    pub(crate) claims: Claims,
    pub(crate) token: String,
}
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::jwt)]
struct JWIId {
    jwt_id: String,
}

impl JWT {
    pub(crate) fn from(claims: Claims, key: &EncodingKey) -> Result<Self, ApiError> {
        let token = encode(&Header::new(Algorithm::EdDSA), &claims, key);
        if let Ok(token) = token {
            return Ok(JWT { token, claims });
        };
        return Err(ApiError::Internal);
    }
    pub(crate) fn create(r: &Role, g: &Vec<Group>, key: &EncodingKey) -> Result<Self, ApiError> {
        let id = Uuid::new_v4();
        let claims = Claims {
            company: String::from("I.K.E"),
            exp: chrono::Utc::now().timestamp() + 3600 * 24 * 7,
            jti: id.to_string(),
            roles: Role {
                role: r.role.clone(),
                id: r.id,
            },
            groups: g.to_vec(),
        };
        Ok(Self::from(claims, key)?)
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

    pub(crate) fn register(db: &mut SqliteConnection, token: &JWT) -> Result<(), ApiError> {
        let insertable_jwt = JWIId {
            jwt_id: token.claims.jti.clone(),
        };
        insert_into(crate::schema::jwt::dsl::jwt)
            .values(&insertable_jwt)
            .execute(db)
            .map_err(|e| {
                eprintln!("{e:?}");
                ApiError::Internal
            })?;
        Ok(())
    }
}
