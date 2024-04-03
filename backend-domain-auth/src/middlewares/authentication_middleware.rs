use crate::api_error::ApiError;
use crate::helpers::try_get_connection;
use crate::models::jwt_model::{Claims, JWTInternal};
use crate::{KeySet, StorageState};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{web, Handler, HttpMessage};
use futures::future::LocalBoxFuture;
use futures::FutureExt;
use log::error;
use std::future::{ready, Ready};
use std::io::Error;
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
        let mut db = match req.app_data::<web::Data<StorageState>>() {
            Some(storage) => match try_get_connection(storage) {
                Ok(db) => db,
                Err(e) => return Box::pin(ready(Err(actix_web::Error::from(e)))),
            },
            None => {
                error!("couldn't access storage");
                return Box::pin(ready(Err(actix_web::Error::from(ApiError::Internal))));
            }
        };
        let key_set = match req.app_data::<web::Data<KeySet>>() {
            Some(key_set) => key_set,
            None => {
                error!("couldn't access key set");
                return Box::pin(ready(Err(actix_web::Error::from(ApiError::Internal))));
            }
        };

        let token = match req.headers().get("Authorization") {
            Some(auth) => match auth.to_str() {
                Ok(auth) => &auth[7..auth.len()],
                Err(e) => {
                    error!("{e:?}");
                    return Box::pin(ready(Err(actix_web::Error::from(actix_web::Error::from(
                        ApiError::Internal,
                    )))));
                }
            },
            None => {
                return Box::pin(ready(Err(actix_web::Error::from(ApiError::JWT))));
            }
        };

        let claims = match JWTInternal::validate_jwt(&mut *db, token, &key_set.decoding) {
            Ok(claims) => claims,
            Err(e) => return Box::pin(ready(Err(actix_web::Error::from(e)))),
        };
        drop(db);

        let srv = Rc::clone(&self.service);
        async move {
            req.extensions_mut().insert::<Claims>(claims);
            let res = srv.call(req).await?;
            Ok(res)
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
