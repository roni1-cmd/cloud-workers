use crate::models::Meeting;
use worker::*;
use chrono::Utc;

pub async fn toggle(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = ctx.param("id").unwrap();
    let kv = ctx.env.kv("MOM_STORE")?;
    let mut m: Meeting = kv.get(id).json().await?.ok_or("Meeting not found")?;
    let now = Utc::now().timestamp_millis();

    if m.stopwatch.is_running {
        let start = m.stopwatch.last_started_at.unwrap_or(now);
        m.stopwatch.total_ms += now - start;
        m.stopwatch.is_running = false;
        m.stopwatch.last_started_at = None;
    } else {
        m.stopwatch.is_running = true;
        m.stopwatch.last_started_at = Some(now);
    }

    kv.put(id, serde_json::to_string(&m)?)?.execute().await?;
    Response::from_json(&m.stopwatch)
}
