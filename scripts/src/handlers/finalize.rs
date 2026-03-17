use crate::models::Meeting;
use worker::*;

pub async fn lock(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = ctx.param("id").unwrap();
    let kv = ctx.env.kv("MOM_STORE")?;
    
    let mut m: Meeting = kv.get(id).json().await?.ok_or("Meeting not found")?;
    
    if m.status == "Finalized" {
        return Response::error("Meeting is already finalized and locked", 400);
    }

    m.status = "Finalized".to_string();
    // Force stop the watch if it was running
    m.stopwatch.is_running = false;
    
    kv.put(id, serde_json::to_string(&m)?)?.execute().await?;
    Response::from_json(&m)
}
