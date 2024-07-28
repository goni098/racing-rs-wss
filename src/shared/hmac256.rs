use crate::error::Error;
use hmac::{Hmac, Mac};
use sha2::{digest::CtOutput, Sha256};

type HmacSha256 = Hmac<Sha256>;

pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<CtOutput<HmacSha256>, Error> {
    let mut mac = HmacSha256::new_from_slice(key)?;

    mac.update(data);

    let result = mac.finalize();

    Ok(result)
}
