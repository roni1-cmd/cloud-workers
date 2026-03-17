use crate::models::Meeting;
use worker::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileMetadata {
    pub name: String,
    pub size: u64,
    pub content_type: String,
    pub uploaded_at: i64,
}

pub async fn register_file(ctx: &RouteContext<()>, meeting_id: &str, meta: FileMetadata) -> Result<()> {
    let kv = ctx.env.kv("MOM_STORE")?;
    let mut m: Meeting = kv.get(meeting_id).json().await?.ok_or("Meeting missing")?;
    
    // Logic to add or update metadata in a dedicated registry key or the meeting object
    let key = format!("files:{}", meeting_id);
    let mut files: Vec<FileMetadata> = kv.get(&key).json().await?.unwrap_or_default();
    
    files.push(meta);
    kv.put(&key, serde_json::to_string(&files)?)?.execute().await?;
    Ok(())
}
