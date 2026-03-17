use crate::models::Meeting;
use worker::*;
use serde::Serialize;

#[derive(Serialize)]
struct MeetingSummary {
    id: String,
    title: String,
    date: String,
}

pub async fn list_all(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let kv = ctx.env.kv("MOM_STORE")?;
    let list = kv.list().execute().await?;
    
    let mut summaries = Vec::new();
    for key in list.keys {
        if let Some(m) = kv.get(&key.name).json::<Meeting>().await? {
            summaries.push(MeetingSummary {
                id: m.id,
                title: m.title,
                date: m.created_at.to_rfc3339(),
            });
        }
    }
    
    Response::from_json(&summaries)
}
