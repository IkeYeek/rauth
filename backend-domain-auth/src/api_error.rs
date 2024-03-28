use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub(crate) enum ApiError {
    #[display(fmt = "Couldn't create such group.")]
    GroupCreation,
    #[display(fmt = "Error with group.")]
    Group,

    #[display(fmt = "Couldn't create such user.")]
    UserCreation,
    #[display(fmt = "Error with user.")]
    User,

    #[display(fmt = "Error with domain rule.")]
    DomainRule,

    #[display(fmt = "Error with url rule.")]
    URLRule,

    #[display(fmt = "Error with role.")]
    Role,

    #[display(fmt = "Internal server error")]
    Internal,

    #[display(fmt = "Couldn't validate jwt")]
    JWT,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::GroupCreation => StatusCode::BAD_REQUEST,
            ApiError::Role => StatusCode::BAD_REQUEST,
            ApiError::Group => StatusCode::BAD_REQUEST,
            ApiError::DomainRule => StatusCode::BAD_REQUEST,
            ApiError::JWT => StatusCode::FORBIDDEN,
            ApiError::URLRule => StatusCode::BAD_REQUEST,
            ApiError::UserCreation => StatusCode::BAD_REQUEST,
            ApiError::User => StatusCode::BAD_REQUEST,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
