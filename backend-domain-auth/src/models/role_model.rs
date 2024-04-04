use std::future::{Future, ready};
use std::pin::Pin;
use actix_web::{FromRequest, HttpRequest, web};
use actix_web::dev::Payload;
use futures::future::{err, ok};
use log::error;
use crate::api_error::ApiError;
use serde::{Deserialize, Serialize};
use crate::helpers::try_get_connection;
use crate::models::jwt_model::JWTInternal;
use crate::{KeySet, StorageState};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Role {
    pub(crate) role: String,
}

impl FromRequest for Role {
    type Error = ApiError;
    type Future =  Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let mut db = if let Some(storage) = req.app_data::<web::Data<StorageState>>() {
            match try_get_connection(storage) {
                Ok(db) => db,
                Err(e) => return Box::pin(ready(Err(e))),
            }
        } else {
            error!("couldn't access storage");
            return Box::pin(ready(Err(ApiError::Internal)));
        };
        let Some(key_set) = req.app_data::<web::Data<KeySet>>() else {
            error!("couldn't access key set");
            return Box::pin(ready(Err(ApiError::Internal)));
        };
        let mut refresh_cookie = false;
        let req = req.clone();
        match req.cookie("jwt") {
            Some(jwt) => {
                let claims = match JWTInternal::validate_jwt(&mut db, &jwt.value(), &key_set.decoding) {
                    Ok(claims) => {
                        let needs_refresh = match JWTInternal::needs_refresh(&mut db, &claims) {
                            Ok(needs_refresh) => {
                                if needs_refresh {
                                    refresh_cookie = true;
                                }
                                needs_refresh
                            }
                            Err(e) => {
                                error!("{e:?}");
                                return Box::pin(ready(Err(e)));
                            }
                        };
                        if needs_refresh {
                            match JWTInternal::refresh(
                                &mut db,
                                &claims.user,
                                &claims.jti,
                                &key_set.encoding,
                            ) {
                                Ok(refresh) => match JWTInternal::register(&mut db, &refresh) {
                                    Ok(()) => refresh,
                                    Err(e) => return Box::pin(ready(Err(e))),
                                },
                                Err(e) => return Box::pin(ready(Err(e))),
                            }
                        } else {
                            match JWTInternal::from(&claims, &key_set.encoding) {
                                Ok(token) => token,
                                Err(e) => return Box::pin(ready(Err(e))),
                            }
                        }
                    }
                    Err(e) => return Box::pin(ready(Err(e))),
                };
                return Box::pin(ok(claims.claims.role))
            },
            None => Box::pin(ok(match Self::from("visitor"){
                Ok(role) => role,
                Err(e) => {
                    error!("{e:?}");
                    return Box::pin(err(e));
                }
            }))
        }
    }
}

impl Role {
    /// checking if role a > role b
    pub(crate) fn superior_to(compare: &Role, to: &Role) -> Result<bool, ApiError> {
        let hierarchy = ["root", "super", "user", "visitor"];
        match (
            hierarchy.iter().position(|&r| r == compare.role),
            hierarchy.iter().position(|&r| r == to.role),
        ) {
            (Some(pos_compare_in_hierarchy), Some(pos_to_in_hierarchy)) => {
                Ok(pos_compare_in_hierarchy < pos_to_in_hierarchy)
            }
            _ => Err(ApiError::Internal),
        }
    }
    pub(crate) fn from(s: &str) -> Result<Role, crate::models::role_model::ApiError> {
        match s {
            "root" => Ok(Role {
                role: "root".into(),
            }),
            "super" => Ok(Role {
                role: "super".into(),
            }),
            "user" => Ok(Role {
                role: "user".into(),
            }),
            "visitor" => Ok(Role {
                role: "visitor".into(),
            }),
            _ => Err(ApiError::Role),
        }
    }
}
