use agentic_receipts::{
    generate_keypair, sha256_hex, sign_receipt, verify_receipt, ActionType, AgenticReceipt,
};
use chrono::Utc;
use uuid::Uuid;

fn sample_receipt() -> AgenticReceipt {
    AgenticReceipt {
        version: "0.1.0".to_string(),
        receipt_id: Uuid::new_v4(),
        agent_id: "agent-001".to_string(),
        task_id: "task-001".to_string(),
        action_type: ActionType::ToolCall,
        tool_name: Some("demo-tool".to_string()),
        model_id: None,
        input_hash: sha256_hex(b"input"),
        output_hash: sha256_hex(b"output"),
        started_at: Utc::now(),
        completed_at: Utc::now(),
        latency_ms: 10,
        nonce: Uuid::new_v4().to_string(),
        previous_receipt_hash: None,
        signature_algorithm: "ed25519".to_string(),
        public_key_hex: String::new(),
        signature_hex: None,
    }
}

#[test]
fn signed_receipt_verifies() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let signed = sign_receipt(receipt, &signing_key).unwrap();

    assert!(verify_receipt(&signed).is_ok());
}

#[test]
fn tampered_receipt_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.output_hash = sha256_hex(b"tampered-output");

    assert!(verify_receipt(&signed).is_err());
}

#[test]
fn receipt_with_wrong_public_key_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let (_, other_verifying_key) = generate_keypair();

    let receipt = sample_receipt();
    let mut signed = sign_receipt(receipt, &signing_key).unwrap();

    signed.public_key_hex = hex::encode(other_verifying_key.to_bytes());

    assert!(verify_receipt(&signed).is_err());
}
