use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Params {
    pub gas: u8,
    pub refilling_in: u8,
    pub max_gas: u8,
    pub base_points: u8,
    pub active: bool,
}

pub type WsState = Arc<RwLock<HashMap<u64, Params>>>;

impl Params {
    pub fn defualt(max_gas: u8, base_points: u8) -> Self {
        Self {
            gas: max_gas,
            refilling_in: 0,
            max_gas,
            base_points,
            active: true,
        }
    }
}
