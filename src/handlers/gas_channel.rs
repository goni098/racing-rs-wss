use crate::{
    extractors::{
        security::WsAuth,
        state::{Params, WsState},
    },
    shared::handle_gas_timing,
};
use axum::{extract::State, response::IntoResponse};
use axum_typed_websockets::{Message, WebSocket, WebSocketUpgrade};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::sleep};

pub async fn gas_channel(
    State(state): State<WsState>,
    WsAuth(user): WsAuth,
    ws: WebSocketUpgrade<ServerMsg, ClientMsg>,
) -> impl IntoResponse {
    let user_params = UserParams {
        base_points: 10,
        max_gas: 10,
        telegram_id: user.id,
    };

    ws.on_upgrade(|connection| ping_pong_socket(connection, state, user_params))
}

async fn ping_pong_socket(
    connection: WebSocket<ServerMsg, ClientMsg>,
    state: WsState,
    user_params: UserParams,
) {
    let (mut sender, mut receiver) = connection.split();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Signal>(100);

    let params = {
        let mut state = state.write().await;

        let params = if let Some(params) = state.get_mut(&user_params.telegram_id) {
            params
        } else {
            state.insert(user_params.telegram_id, Params::defualt(10, 10));

            state.get_mut(&user_params.telegram_id).unwrap()
        };

        params.active = true;
        params.base_points = user_params.base_points;
        params.max_gas = user_params.max_gas;

        Arc::new(RwLock::new(params.clone()))
    };

    let params_channel = params.clone();
    let tx_channel = tx.clone();
    let channel = tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(1000)).await;

            let mut params = params_channel.write().await;

            handle_gas_timing(&mut params);

            let _ = tx_channel
                .send(Signal::GasStaus(GasStatus {
                    gas: params.gas,
                    refilling_in: params.refilling_in,
                }))
                .await;
        }
    });

    let mut winning_streak = 0;
    let params_channel = params.clone();
    tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Item(msg) => match msg {
                    ClientMsg::Lose | ClientMsg::Win => {
                        let gas = {
                            let mut params = params.write().await;

                            if params.gas < 1 {
                                0
                            } else {
                                params.gas -= 1;
                                params.gas
                            }
                        };

                        if gas > 0 {
                            let result = if let ClientMsg::Lose = msg {
                                winning_streak = 0;
                                0
                            } else {
                                winning_streak += 1;
                                1
                            };

                            let _ = tx
                                .send(Signal::GuessingResult(GuessingResult {
                                    result,
                                    winning_streak,
                                }))
                                .await;
                        }
                    }
                    ClientMsg::Reload => {
                        let _ = tx.send(Signal::Text("reloaded")).await;
                    }
                    ClientMsg::Refill => {
                        let _ = tx.send(Signal::Text("refilled")).await;
                    }
                },
                Message::Close(_) => {
                    let _ = tx.send(Signal::CloseConnection).await;
                    channel.abort();
                }
                _ => {}
            }
        }
    });

    tokio::spawn(async move {
        while let Some(signal) = rx.recv().await {
            match signal {
                Signal::GasStaus(GasStatus { gas, refilling_in }) => {
                    let _ = sender
                        .send(Message::Item(ServerMsg::GasStatus(GasStatus {
                            gas,
                            refilling_in,
                        })))
                        .await;
                }
                Signal::GuessingResult(result) => {
                    let _ = sender
                        .send(Message::Item(ServerMsg::GuessingResult(result)))
                        .await;
                }
                Signal::CloseConnection => {
                    let mut state = state.write().await;
                    let mut params = params_channel.read().await.clone();
                    params.active = false;
                    state.insert(user_params.telegram_id, params);
                }
                Signal::Text(msg) => {
                    let _ = sender.send(Message::Item(ServerMsg::Text(msg))).await;
                }
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

struct UserParams {
    telegram_id: u64,
    max_gas: u8,
    base_points: u8,
}

enum Signal {
    GasStaus(GasStatus),
    GuessingResult(GuessingResult),
    CloseConnection,
    Text(&'static str),
}
