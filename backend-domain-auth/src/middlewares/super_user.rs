use crate::api_error::ApiError;
use crate::models::jwt_model::{Claims};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Handler, HttpMessage};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};
use crate::models::role_model::Role;

pub struct RequireSuperUserMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for RequireSuperUserMiddleware<S>
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
       let binding = req.extensions();
       let claims = match binding.get::<Claims>() {
           Some(claims) => claims,
           None => return Box::pin(ready(Err(actix_web::Error::from(ApiError::JWT))))
       };
        let is_super = match Role::superior_to(claims.clone().role, match Role::from("user") {
            Ok(r) => r,
            Err(e) => return Box::pin(ready(Err(actix_web::Error::from(e))))
        }) {
            Ok(is_super) => is_super,
            Err(e) => return Box::pin(ready(Err(actix_web::Error::from(e))))
        };
        drop(binding);
        return if is_super {
            let srv = self.service.clone();
            Box::pin(async move {
                let res = srv.call(req).await?;
                Ok(res)
            })
        } else {
            Box::pin(ready(Err(actix_web::Error::from(ApiError::JWT))))
        }
    }
}

pub struct RequireSuperUser;

impl<S> Transform<S, ServiceRequest> for RequireSuperUser
    where
        S: Service<
            ServiceRequest,
            Response = ServiceResponse<actix_web::body::BoxBody>,
            Error = actix_web::Error,
        > + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Transform = RequireSuperUserMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequireSuperUserMiddleware {
            service: Rc::new(service),
        }))
    }
}
