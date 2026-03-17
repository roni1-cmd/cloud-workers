use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stopwatch {
    pub is_running: bool,
    pub last_started_at: Option<i64>,
    pub total_ms: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Meeting {
    pub id: String,
    pub title: String,
    pub attendees: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub stopwatch: Stopwatch,
}
