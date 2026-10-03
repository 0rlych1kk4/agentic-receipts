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

#[test]
fn receipt_without_signature_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.signature_hex = None;

    assert!(matches!(
        verify_receipt(&signed),
        Err(agentic_receipts::ReceiptError::MissingSignature)
    ));
}

#[test]
fn receipt_with_malformed_public_key_hex_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.public_key_hex = "not-valid-hex".to_string();

    assert!(matches!(
        verify_receipt(&signed),
        Err(agentic_receipts::ReceiptError::InvalidPublicKey)
    ));
}

#[test]
fn receipt_with_short_public_key_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.public_key_hex = hex::encode([0u8; 31]);

    assert!(matches!(
        verify_receipt(&signed),
        Err(agentic_receipts::ReceiptError::InvalidPublicKey)
    ));
}

#[test]
fn receipt_with_long_public_key_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.public_key_hex = hex::encode([0u8; 33]);

    assert!(matches!(
        verify_receipt(&signed),
        Err(agentic_receipts::ReceiptError::InvalidPublicKey)
    ));
}

#[test]
fn receipt_with_malformed_signature_hex_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.signature_hex = Some("not-valid-hex".to_string());

    assert!(matches!(
        verify_receipt(&signed),
        Err(agentic_receipts::ReceiptError::InvalidSignature)
    ));
}

#[test]
fn receipt_with_short_signature_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.signature_hex = Some(hex::encode([0u8; 63]));

    assert!(matches!(
        verify_receipt(&signed),
        Err(agentic_receipts::ReceiptError::InvalidSignature)
    ));
}

#[test]
fn receipt_with_long_signature_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.signature_hex = Some(hex::encode([0u8; 65]));

    assert!(matches!(
        verify_receipt(&signed),
        Err(agentic_receipts::ReceiptError::InvalidSignature)
    ));
}

#[test]
fn tampered_agent_id_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.agent_id = "attacker-agent".to_string();

    assert!(verify_receipt(&signed).is_err());
}

#[test]
fn tampered_task_id_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.task_id = "tampered-task".to_string();

    assert!(verify_receipt(&signed).is_err());
}

#[test]
fn tampered_input_hash_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.input_hash = sha256_hex(b"tampered-input");

    assert!(verify_receipt(&signed).is_err());
}

#[test]
fn tampered_action_type_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.action_type = ActionType::ApiCall;

    assert!(verify_receipt(&signed).is_err());
}

#[test]
fn tampered_tool_name_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.tool_name = Some("tampered-tool".to_string());

    assert!(verify_receipt(&signed).is_err());
}

#[test]
fn tampered_model_id_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.model_id = Some("tampered-model".to_string());

    assert!(verify_receipt(&signed).is_err());
}

#[test]
fn tampered_started_at_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.started_at += chrono::Duration::seconds(1);

    assert!(verify_receipt(&signed).is_err());
}

#[test]
fn tampered_completed_at_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.completed_at += chrono::Duration::seconds(1);

    assert!(verify_receipt(&signed).is_err());
}

#[test]
fn tampered_latency_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.latency_ms += 1;

    assert!(verify_receipt(&signed).is_err());
}

#[test]
fn tampered_nonce_fails_verification() {
    let (signing_key, _) = generate_keypair();
    let receipt = sample_receipt();

    let mut signed = sign_receipt(receipt, &signing_key).unwrap();
    signed.nonce = "tampered-nonce".to_string();

    assert!(verify_receipt(&signed).is_err());
}
