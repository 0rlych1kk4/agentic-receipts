use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    ToolCall,
    ModelInference,
    GpuWorkload,
    RagRetrieval,
    ApiCall,
    DistributedJob,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgenticReceipt {
    pub version: String,
    pub receipt_id: Uuid,

    pub agent_id: String,
    pub task_id: String,
    pub action_type: ActionType,

    pub tool_name: Option<String>,
    pub model_id: Option<String>,

    pub input_hash: String,
    pub output_hash: String,

    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub latency_ms: u64,

    pub nonce: String,
    pub previous_receipt_hash: Option<String>,

    pub signature_algorithm: String,
    pub public_key_hex: String,
    pub signature_hex: Option<String>,
}
