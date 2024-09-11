use crate::database::repositories::user;
use crate::error::Error;
use crate::extractors::{
    security::Auth,
    state::{AppState, RedisConnection},
};
use axum::extract::State;
use axum_typed_websockets::{Message, WebSocket, WebSocketUpgrade};
use chrono::Utc;
use deadpool_redis::redis::AsyncCommands;
use futures::{SinkExt, StreamExt};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;
use tokio::time::sleep;

pub async fn gas_channel(
    Auth(user): Auth,
    State(state): State<AppState>,
    ws: WebSocketUpgrade<ServerMsg, ClientMsg>,
) -> Result<(), Error> {
    let db = state.db;
    let mut redis_conn = state.redis_pool.get().await?;

    let params = Params::init(&mut redis_conn, &db, user.id).await?;

    ws.on_upgrade(|connection| ping_pong_socket(connection, db, redis_conn, params));

    Ok(())
}

async fn ping_pong_socket(
    connection: WebSocket<ServerMsg, ClientMsg>,
    db: DatabaseConnection,
    redis_conn: RedisConnection,
    params: Params,
) {
    let (sender, mut receiver) = connection.split();
    let sender = Arc::new(RwLock::new(sender));

    let params = Arc::new(RwLock::new(params));
    let params_ = params.clone();

    let sender_ = sender.clone();

    let gas_timing = tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(1000)).await;

            let mut params = params_.write().await;

            handle_gas_timing(&mut params);

            let mut sender = sender_.write().await;

            let _ = sender
                .send(Message::Item(ServerMsg::GasStatus(GasStatus {
                    gas: params.gas,
                    refilling_in: params.refilling_in,
                })))
                .await;
        }
    });

    tokio::spawn(async move {
        let mut winning_streak = 0;

        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Item(msg) => match msg {
                    ClientMsg::Lose | ClientMsg::Win => {
                        let mut params = params.write().await;

                        if params.gas > 0 {
                            params.gas -= 1;

                            let result = if let ClientMsg::Lose = msg {
                                winning_streak = 0;
                                0
                            } else {
                                winning_streak += 1;
                                1
                            };

                            let mut sender = sender.write().await;

                            let _ = sender
                                .send(Message::Item(ServerMsg::GuessingResult(GuessingResult {
                                    result,
                                    winning_streak,
                                })))
                                .await;
                        }
                    }
                    ClientMsg::Reload => {}
                    ClientMsg::Refill => {}
                },
                Message::Close(_) => {
                    gas_timing.abort();
                }
                _ => {}
            }
        }
    });
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ClientMsg {
    Win,
    Lose,
    Refill,
    Reload,
}

#[derive(Serialize)]
pub struct GuessingResult {
    result: u8,
    winning_streak: u8,
}

#[derive(Serialize)]
pub struct GasStatus {
    gas: u8,
    refilling_in: u8,
}

#[derive(Serialize)]
pub enum ServerMsg {
    Text(&'static str),
    GuessingResult(GuessingResult),
    GasStatus(GasStatus),
}

#[derive(Debug, Clone)]
struct Params {
    gas: u8,
    refilling_in: u8,
    max_gas: u8,
    base_points: u8,
}

impl Params {
    async fn init(
        redis_conn: &mut RedisConnection,
        db: &DatabaseConnection,
        telegram_id: u64,
    ) -> Result<Self, Error> {
        let now = Utc::now().timestamp() as u64;

        let last_active_time = redis_conn
            .get::<String, Option<u64>>(format!("{}_lastime_active", telegram_id))
            .await?
            .unwrap_or(now);

        let last_gas = redis_conn
            .get::<String, Option<u8>>(format!("{}_lastime_active", telegram_id))
            .await?
            .unwrap_or(10);

        let user = user::find_by_telegram_id(db, telegram_id)
            .await?
            .ok_or(Error::Custom("user not found".to_string()))?;

        let duration = now - last_active_time;

        let gas = last_gas + (duration / 90) as u8;
        let refilling_in = (duration % 90) as u8;

        Ok(Self {
            gas,
            refilling_in,
            max_gas: calculate_max_gas(user.fuel_tank_lv as u8),
            base_points: calculate_base_points(user.turbo_changer_lv as u8),
        })
    }
}

fn handle_gas_timing(params: &mut Params) {
    if params.gas == params.max_gas {
        params.refilling_in = 0;
    } else if params.refilling_in == 0 {
        params.refilling_in = 90;
    } else {
        params.refilling_in -= 1;
        if params.refilling_in == 0 {
            params.gas += 1
        };
    }
}

fn calculate_max_gas(fuel_tank_lv: u8) -> u8 {
    10 + fuel_tank_lv * 2
}

fn calculate_base_points(turbo_changer_lv: u8) -> u8 {
    10 + turbo_changer_lv * 10
}

fn points_by_streak(points: u16, streak: u8) -> u16 {
    points * 2_u16.pow(streak as u32 - 1)
}
