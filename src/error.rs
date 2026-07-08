use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReceiptError {
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("signature error")]
    Signature,

    #[error("missing signature")]
    MissingSignature,

    #[error("invalid public key")]
    InvalidPublicKey,

    #[error("invalid signature")]
    InvalidSignature,
}
