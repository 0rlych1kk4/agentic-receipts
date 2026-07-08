use agentic_receipts::{
    generate_keypair, receipt_hash, sha256_hex, sign_receipt, verify_receipt, ActionType,
    AgenticReceipt,
};
use chrono::Utc;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (signing_key, _verifying_key) = generate_keypair();

    let input = br#"{"prompt":"summarize this risk report"}"#;
    let output = br#"{"summary":"high-level risk summary"}"#;

    let started_at = Utc::now();
    let completed_at = Utc::now();

    let receipt = AgenticReceipt {
        version: "0.1.0".to_string(),
        receipt_id: Uuid::new_v4(),

        agent_id: "risk-agent-001".to_string(),
        task_id: "task-001".to_string(),
        action_type: ActionType::ToolCall,

        tool_name: Some("risk_summarizer".to_string()),
        model_id: Some("local-llm-v1".to_string()),

        input_hash: sha256_hex(input),
        output_hash: sha256_hex(output),

        started_at,
        completed_at,
        latency_ms: 120,

        nonce: Uuid::new_v4().to_string(),
        previous_receipt_hash: None,

        signature_algorithm: "ed25519".to_string(),
        public_key_hex: String::new(),
        signature_hex: None,
    };

    let signed = sign_receipt(receipt, &signing_key)?;

    verify_receipt(&signed)?;

    println!("Receipt verified successfully.");
    println!("Receipt hash: {}", receipt_hash(&signed)?);
    println!("{}", serde_json::to_string_pretty(&signed)?);

    Ok(())
}
