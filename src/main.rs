mod error;
mod extractors;
mod handlers;
mod shared;
mod telegram;

use axum::{routing::get, Router};
use extractors::state::WsState;
use shared::handle_gas_timing;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::sleep};

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    dotenv::dotenv().ok();

    let ws_state: WsState = Arc::new(RwLock::new(HashMap::new()));

    let memo = ws_state.clone();

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(1000)).await;

            let mut memo = memo.write().await;

            memo.retain(|_, params| {
                if !params.active {
                    handle_gas_timing(params);
                    params.gas != params.max_gas
                } else {
                    true
                }
            });
        }
    });

    let user_router = Router::new();

    let gas_socket_router = Router::new()
        .route("/gas-channel", get(handlers::gas_channel))
        .with_state(ws_state);

    let app = Router::new()
        .route("/api", get(|| async { "Hello racing, 🦀!" }))
        .merge(user_router)
        .merge(gas_socket_router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8098").await?;

    println!("🦀 server listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
