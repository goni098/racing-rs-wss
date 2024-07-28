use axum::http::StatusCode;
use axum_derive_error::ErrorResponse;

#[allow(dead_code)]
#[derive(ErrorResponse, thiserror::Error)]
pub enum Error {
    // http rejected error
    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    Validation(#[from] validator::ValidationErrors),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    PathRejection(#[from] axum::extract::rejection::PathRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    FormRejection(#[from] axum::extract::rejection::FormRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    QueryRejection(#[from] axum::extract::rejection::QueryRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    BodyRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error(transparent)]
    #[status(StatusCode::UNAUTHORIZED)]
    TypedHeaderRejection(#[from] axum_extra::typed_header::TypedHeaderRejection),

    #[error("{0:#?}")]
    #[status(StatusCode::BAD_REQUEST)]
    BadRequest(String),

    #[error("{0:#?}")]
    #[status(StatusCode::UNAUTHORIZED)]
    Unauthorized(String),

    #[error("{0:#?}")]
    Internal(String),

    // exection error
    #[error(transparent)]
    MissingEnv(#[from] std::env::VarError),

    #[error("Custom error: {0:#?}")]
    Custom(String),

    #[error("SerdeJson error: {0:#?}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Database error: {0:#?}")]
    Database(#[from] sea_orm::error::DbErr),

    #[error("IO error: {0:#?}")]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    HmacInvalidLength(#[from] hmac::digest::InvalidLength),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    Hex(#[from] hex::FromHexError),

    #[error("{0}")]
    Env(String),
}
