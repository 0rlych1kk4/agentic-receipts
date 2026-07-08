pub mod error;
pub mod hashing;
pub mod receipt;
pub mod signing;
pub mod verify;

pub use error::ReceiptError;
pub use hashing::{receipt_hash, sha256_hex};
pub use receipt::{ActionType, AgenticReceipt};
pub use signing::{generate_keypair, sign_receipt};
pub use verify::verify_receipt;
