use crate::models::{Meeting, Stopwatch};
use worker::*;
use chrono::Utc;

pub async fn create(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let mut meeting: Meeting = req.json().await?;
    meeting.created_at = Utc::now();
    meeting.stopwatch = Stopwatch { is_running: false, last_started_at: None, total_ms: 0 };
    
    let kv = ctx.env.kv("MOM_STORE")?;
    kv.put(&meeting.id, serde_json::to_string(&meeting)?)?.execute().await?;
    Response::from_json(&meeting)
}

pub async fn get(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = ctx.param("id").unwrap();
    let kv = ctx.env.kv("MOM_STORE")?;
    match kv.get(id).json::<Meeting>().await? {
        Some(m) => Response::from_json(&m),
        None => Response::error("Meeting not found", 404),
    }
}
