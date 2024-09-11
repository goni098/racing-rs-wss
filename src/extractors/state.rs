use crate::error::Error;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use deadpool_redis::{Config, Runtime};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub type RedisConnection = deadpool_redis::Connection;

#[allow(dead_code)]
pub struct Redis(pub RedisConnection);

#[allow(dead_code)]
pub struct Pg(pub DatabaseConnection);

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis_pool: deadpool_redis::Pool,
}

#[async_trait]
impl<S> FromRequestParts<S> for Pg
where
    S: Send + Sync,
    DatabaseConnection: FromRef<S>,
{
    type Rejection = Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let connection = DatabaseConnection::from_ref(state);

        Ok(Self(connection))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Redis
where
    S: Send + Sync,
    deadpool_redis::Pool: FromRef<S>,
{
    type Rejection = Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let connection = deadpool_redis::Pool::from_ref(state).get().await?;

        Ok(Self(connection))
    }
}

impl FromRef<AppState> for DatabaseConnection {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.db.clone()
    }
}

impl FromRef<AppState> for deadpool_redis::Pool {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.redis_pool.clone()
    }
}

impl AppState {
    pub async fn init(db_url: &str, redis_url: &str) -> Result<Self, Error> {
        let mut opt = ConnectOptions::new(db_url);

        opt.sqlx_logging(false);

        let database_connection = Database::connect(opt).await?;

        let redis_pool = Config::from_url(redis_url).create_pool(Some(Runtime::Tokio1))?;

        Ok(Self {
            db: database_connection,
            redis_pool,
        })
    }
}
