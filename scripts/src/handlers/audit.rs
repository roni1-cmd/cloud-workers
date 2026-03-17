use serde::{Deserialize, Serialize};
use worker::*;
use chrono::Utc;

#[derive(Serialize, Deserialize)]
struct AuditEntry {
    timestamp: String,
    action: String,
    actor: String,
    details: String,
}

pub async fn log_action(kv: &KvStore, meeting_id: &str, action: &str, actor: &str) -> Result<()> {
    let entry = AuditEntry {
        timestamp: Utc::now().to_rfc3339(),
        action: action.to_string(),
        actor: actor.to_string(),
        details: format!("Action performed on meeting {}", meeting_id),
    };
    
    let key = format!("audit:{}:{}", meeting_id, Utc::now().timestamp_nanos_opt().unwrap_or(0));
    kv.put(&key, serde_json::to_string(&entry)?)?.execute().await?;
    Ok(())
}
