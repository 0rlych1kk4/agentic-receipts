//! Signed, tamper-evident execution receipts for AI agents, tool calls,
//! GPU workloads, and distributed jobs.
//!
//! `agentic-receipts` provides:
//!
//! - SHA-256 input, output, and receipt hashing;
//! - Ed25519 receipt signing;
//! - receipt signature verification;
//! - JSON-serializable execution receipt structures;
//! - optional receipt chaining.
//!
//! # Security status
//!
//! This crate is experimental and has not undergone an independent security
//! audit. Review its implementation and threat model before using it for
//! high-stakes production, financial, or compliance-sensitive workloads.

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
