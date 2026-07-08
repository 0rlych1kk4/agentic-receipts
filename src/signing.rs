use crate::error::ReceiptError;
use crate::receipt::AgenticReceipt;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use rand_core::OsRng;

pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

pub fn signing_payload(receipt: &AgenticReceipt) -> Result<Vec<u8>, ReceiptError> {
    let mut unsigned = receipt.clone();
    unsigned.signature_hex = None;
    Ok(serde_json::to_vec(&unsigned)?)
}

pub fn sign_receipt(
    mut receipt: AgenticReceipt,
    signing_key: &SigningKey,
) -> Result<AgenticReceipt, ReceiptError> {
    receipt.signature_algorithm = "ed25519".to_string();
    receipt.public_key_hex = hex::encode(signing_key.verifying_key().to_bytes());
    receipt.signature_hex = None;

    let payload = signing_payload(&receipt)?;
    let signature = signing_key.sign(&payload);

    receipt.signature_hex = Some(hex::encode(signature.to_bytes()));

    Ok(receipt)
}
