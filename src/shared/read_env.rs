use std::env::VarError;

use crate::error::Error;

pub fn read_env(config: &str) -> Result<String, Error> {
    std::env::var(config).map_err(|error| {
        let reasons = match error {
            VarError::NotPresent => format!("missing env::{config}"),
            _ => error.to_string(),
        };

        Error::Env(reasons)
    })
}
