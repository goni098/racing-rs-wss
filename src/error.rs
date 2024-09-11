use axum::http::StatusCode;
use axum_derive_error::ErrorResponse;

#[allow(dead_code)]
#[derive(ErrorResponse, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    PathRejectionError(#[from] axum::extract::rejection::PathRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    FormRejectionError(#[from] axum::extract::rejection::FormRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    QueryRejectionError(#[from] axum::extract::rejection::QueryRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    BodyRejectionError(#[from] axum::extract::rejection::JsonRejection),

    #[error(transparent)]
    #[status(StatusCode::UNAUTHORIZED)]
    TypedHeaderRejectionError(#[from] axum_extra::typed_header::TypedHeaderRejection),

    #[error("{0:#?}")]
    #[status(StatusCode::BAD_REQUEST)]
    BadRequest(String),

    #[error("{0:#?}")]
    #[status(StatusCode::UNAUTHORIZED)]
    Unauthorized(String),

    #[error("{0:#?}")]
    Internal(String),

    #[error(transparent)]
    MissingEnvError(#[from] std::env::VarError),

    #[error("{0}")]
    Env(String),

    #[error("Custom error: {0:#?}")]
    Custom(String),

    #[error("SerdeJson error: {0:#?}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Database error: {0:#?}")]
    DatabaseError(#[from] sea_orm::error::DbErr),

    #[error("IO error: {0:#?}")]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    HmacInvalidLengthError(#[from] hmac::digest::InvalidLength),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    HexError(#[from] hex::FromHexError),

    #[error("Redis pool error: {0:#?}")]
    RedisPoolError(#[from] deadpool_redis::PoolError),

    #[error("Redis create pool error: {0:#?}")]
    RedisCreatePoolError(#[from] deadpool_redis::CreatePoolError),

    #[error("Redis error: {0:#?}")]
    RedisError(#[from] deadpool_redis::redis::RedisError),
}
