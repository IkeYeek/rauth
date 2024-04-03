use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ApiError {
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
    Jwt,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::GroupCreation
            | ApiError::Role
            | ApiError::Group
            | ApiError::DomainRule
            | ApiError::URLRule
            | ApiError::UserCreation
            | ApiError::User => StatusCode::BAD_REQUEST,
            ApiError::Jwt => StatusCode::FORBIDDEN,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
