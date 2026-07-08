use sha2::{Digest, Sha256};

pub fn sha256_hex(data: &[u8]) -> String {
    let hash = Sha256::digest(data);
    format!("sha256:{}", hex::encode(hash))
}

pub fn receipt_hash<T: serde::Serialize>(receipt: &T) -> Result<String, serde_json::Error> {
    let bytes = serde_json::to_vec(receipt)?;
    Ok(sha256_hex(&bytes))
}
