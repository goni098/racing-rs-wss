use axum::{routing::get, Router};
use racer::{extractors::state::AppState, handlers};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("fail to load env");

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("msg");
    let redis_url = "redis://127.0.0.1/";

    let sate = AppState::init(&db_url, redis_url)
        .await
        .expect("fail to init app state");

    let user_router = Router::new();

    let gas_socket_router = Router::new()
        .route("/gas-channel", get(handlers::gas_channel))
        .with_state(sate);

    let app = Router::new()
        .route("/api", get(|| async { "hello racer, 🦀!" }))
        .merge(user_router)
        .merge(gas_socket_router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!(
        "🦀 server is listening on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
