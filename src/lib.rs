use serde::{Deserialize, Serialize};
use worker::*;
use chrono::Utc;

#[derive(Serialize, Deserialize, Clone)]
struct Stopwatch {
    is_running: bool,
    last_start_time: i64,      // Unix timestamp
    accumulated_ms: i64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Meeting {
    id: String,
    title: String,
    attendees: Vec<String>,
    created_at: String,
    stopwatch: Stopwatch,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        // CREATE/UPDATE meeting metadata
        .post_async("/meeting", |mut req, ctx| async move {
            let mut meeting: Meeting = req.json().await?;
            meeting.created_at = Utc::now().to_rfc3339();
            
            let kv = ctx.env.kv("MOM_METADATA")?;
            kv.put(&meeting.id, serde_json::to_string(&meeting)?)?.execute().await?;
            
            Response::ok(format!("Meeting {} initialized", meeting.id))
        })

        // GET meeting details
        .get_async("/meeting/:id", |_, ctx| async move {
            let id = ctx.param("id").unwrap();
            let kv = ctx.env.kv("MOM_METADATA")?;
            
            match kv.get(id).json::<Meeting>().await? {
                Some(m) => Response::from_json(&m),
                None => Response::error("Meeting not found", 404),
            }
        })

        // TOGGLE Stopwatch (Start/Stop)
        .post_async("/meeting/:id/stopwatch", |_, ctx| async move {
            let id = ctx.param("id").unwrap();
            let kv = ctx.env.kv("MOM_METADATA")?;
            let mut meeting = kv.get(id).json::<Meeting>().await?.ok_or("Not Found")?;

            let now = Utc::now().timestamp_millis();

            if meeting.stopwatch.is_running {
                // Stop: Add diff to accumulated
                let diff = now - meeting.stopwatch.last_start_time;
                meeting.stopwatch.accumulated_ms += diff;
                meeting.stopwatch.is_running = false;
            } else {
                // Start: Mark start time
                meeting.stopwatch.last_start_time = now;
                meeting.stopwatch.is_running = true;
            }

            kv.put(id, serde_json::to_string(&meeting)?)?.execute().await?;
            Response::from_json(&meeting.stopwatch)
        })

        // ATTACHMENTS: Upload binary file (PDF, Images, etc.)
        .put_async("/meeting/:id/files/:filename", |mut req, ctx| async move {
            let id = ctx.param("id").unwrap();
            let filename = ctx.param("filename").unwrap();
            let bucket = ctx.env.r2("MOM_ATTACHMENTS")?;
            
            let bytes = req.bytes().await?;
            let path = format!("{}/{}", id, filename);
            
            bucket.put(path, bytes).execute().await?;
            Response::ok("File attached successfully")
        })

        .run(req, env)
        .await
}
