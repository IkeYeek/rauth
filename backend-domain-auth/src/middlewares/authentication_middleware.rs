use crate::api_error::ApiError;
use crate::helpers::try_get_connection;
use crate::models::jwt_model::{Claims, JWTInternal};
use crate::{KeySet, StorageState};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{web, HttpMessage};
use futures::future::LocalBoxFuture;
use futures::FutureExt;
use log::error;
use std::future::{ready, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse<actix_web::body::BoxBody>,
            Error = actix_web::Error,
        > + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut db = if let Some(storage) = req.app_data::<web::Data<StorageState>>() {
            match try_get_connection(storage) {
                Ok(db) => db,
                Err(e) => return Box::pin(ready(Err(actix_web::Error::from(e)))),
            }
        } else {
            error!("couldn't access storage");
            return Box::pin(ready(Err(actix_web::Error::from(ApiError::Internal))));
        };
        let Some(key_set) = req.app_data::<web::Data<KeySet>>() else {
            error!("couldn't access key set");
            return Box::pin(ready(Err(actix_web::Error::from(ApiError::Internal))));
        };

        let token = match req.cookie("jwt") {
            Some(auth) => auth.value().to_string(),
            None => {
                return Box::pin(ready(Err(actix_web::Error::from(ApiError::Jwt))));
            }
        };

        let mut refresh_cookie = false; // booooooh blatant side effect boooooooh

        let claims = match JWTInternal::validate_jwt(&mut db, &token, &key_set.decoding) {
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
                        return Box::pin(ready(Err(actix_web::Error::from(e))));
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
                            Err(e) => return Box::pin(ready(Err(actix_web::Error::from(e)))),
                        },
                        Err(e) => return Box::pin(ready(Err(actix_web::Error::from(e)))),
                    }
                } else {
                    match JWTInternal::from(&claims, &key_set.encoding) {
                        Ok(token) => token,
                        Err(e) => return Box::pin(ready(Err(actix_web::Error::from(e)))),
                    }
                }
            }
            Err(e) => return Box::pin(ready(Err(actix_web::Error::from(e)))),
        };
        drop(db);

        let srv = Rc::clone(&self.service);
        async move {
            req.extensions_mut().insert::<Claims>(claims.claims);
            let mut resp: ServiceResponse = srv.call(req).await?;
            if refresh_cookie {
                let jwt_cookie = Cookie::build("jwt", &claims.token)
                    .domain(".localhost.dummy")
                    .max_age(Duration::weeks(1))
                    .finish();
                if let Err(e) = resp.response_mut().add_cookie(&jwt_cookie) {
                    return Err(actix_web::Error::from(e));
                }
            }
            Ok(resp)
        }
        .boxed_local()
    }
}

pub struct RequireAuth;

impl<S> Transform<S, ServiceRequest> for RequireAuth
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse<actix_web::body::BoxBody>,
            Error = actix_web::Error,
        > + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}
