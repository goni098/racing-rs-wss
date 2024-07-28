use crate::{error::Error, extractors::security::MiniappUser, shared::hmac_sha256};

pub fn verify_init_data(init_data: &str, bot_token: &str) -> Result<MiniappUser, Error> {
    let mut pairs = Vec::new();
    let mut hash = None;
    let mut user = None;

    for (key, value) in url::form_urlencoded::parse(init_data.as_bytes()) {
        if key == "hash" {
            hash = Some(value);
        } else {
            if key == "user" {
                user = Some(serde_json::from_str::<MiniappUser>(&value)?);
            }

            pairs.push(format!("{key}={value}"));
        }
    }

    let hash = hash.ok_or(Error::Unauthorized("Missing hash".to_string()))?;
    let user = user.ok_or(Error::Unauthorized("Missing user".to_string()))?;

    pairs.sort();

    let data_check = pairs.join("\n");

    let secret = hmac_sha256(b"WebAppData", bot_token.as_bytes())?.into_bytes();
    let decoded_hash = hmac_sha256(&secret, data_check.as_bytes())?.into_bytes();

    if hex::encode(decoded_hash).ne(&hash) {
        return Err(Error::Unauthorized("Hash is not valid".to_string()));
    }

    Ok(user)
}
