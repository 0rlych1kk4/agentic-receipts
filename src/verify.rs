use crate::error::ReceiptError;
use crate::receipt::AgenticReceipt;
use crate::signing::signing_payload;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

pub fn verify_receipt(receipt: &AgenticReceipt) -> Result<(), ReceiptError> {
    let public_key_bytes =
        hex::decode(&receipt.public_key_hex).map_err(|_| ReceiptError::InvalidPublicKey)?;

    let public_key_array: [u8; 32] = public_key_bytes
        .try_into()
        .map_err(|_| ReceiptError::InvalidPublicKey)?;

    let verifying_key =
        VerifyingKey::from_bytes(&public_key_array).map_err(|_| ReceiptError::InvalidPublicKey)?;

    let signature_hex = receipt
        .signature_hex
        .as_ref()
        .ok_or(ReceiptError::MissingSignature)?;

    let signature_bytes = hex::decode(signature_hex).map_err(|_| ReceiptError::InvalidSignature)?;

    let signature_array: [u8; 64] = signature_bytes
        .try_into()
        .map_err(|_| ReceiptError::InvalidSignature)?;

    let signature = Signature::from_bytes(&signature_array);
    let payload = signing_payload(receipt)?;

    verifying_key
        .verify(&payload, &signature)
        .map_err(|_| ReceiptError::Signature)?;

    Ok(())
}
