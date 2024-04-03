use crate::api_error::ApiError;
use crate::models::jwt_model::Claims;
use crate::models::role_model::Role;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::HttpMessage;
use futures::future::LocalBoxFuture;
use futures::FutureExt;
use log::error;
use std::future::{ready, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct TargetUserOrSuperUserMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for TargetUserOrSuperUserMiddleware<S>
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
            Some(claims) => claims.clone(),
            None => return Box::pin(ready(Err(actix_web::Error::from(ApiError::Jwt)))),
        };
        let pattern_path_split = match req.match_pattern() {
            Some(path) => path
                .split('/')
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
            None => return Box::pin(ready(Err(actix_web::Error::from(ApiError::Internal)))),
        };
        let uri_path_split = req.path().split('/').collect::<Vec<&str>>();
        let Some(user_pos_in_pattern) = pattern_path_split.iter().position(|elem| *elem == "{user}") else { return Box::pin(ready(Err(actix_web::Error::from(ApiError::Internal)))) };
        let user_id = match uri_path_split.get(user_pos_in_pattern) {
            Some(raw_user_id) => match raw_user_id.parse::<i32>() {
                Ok(user_id) => user_id,
                Err(e) => {
                    error!("{e:?}");
                    return Box::pin(ready(Err(actix_web::Error::from(ApiError::Internal))));
                }
            },
            None => return Box::pin(ready(Err(actix_web::Error::from(ApiError::Internal)))),
        };
        let claims = claims.clone();
        let is_super = match Role::superior_to(
            &claims.role,
            &match Role::from("user") {
                Ok(r) => r,
                Err(e) => return Box::pin(ready(Err(actix_web::Error::from(e)))),
            },
        ) {
            Ok(is_super) => is_super,
            Err(e) => return Box::pin(ready(Err(actix_web::Error::from(e)))),
        };
        drop(binding);
        let is_user = claims.user.id == user_id;
        return if is_super || is_user {
            let srv = self.service.clone();
            async move {
                let resp = srv.call(req).await?;
                Ok(resp)
            }
            .boxed_local()
        } else {
            return Box::pin(ready(Err(actix_web::Error::from(ApiError::Jwt))));
        };
    }
}

pub struct TargetUserOrSuperUser;

impl<S> Transform<S, ServiceRequest> for TargetUserOrSuperUser
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse<actix_web::body::BoxBody>,
            Error = actix_web::Error,
        > + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Transform = TargetUserOrSuperUserMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TargetUserOrSuperUserMiddleware {
            service: Rc::new(service),
        }))
    }
}
