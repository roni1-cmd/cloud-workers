use worker::*;

pub async fn handle_failover(ctx: &RouteContext<()>, key: &str, data: Vec<u8>) -> Result<()> {
    let kv = ctx.env.kv("COMELEC")?;
    // Store small chunks (<25mb) in KV if R2 is slow
    if data.len() < 25_000_000 {
        kv.put(format!("temp:{}", key), data)?.execute().await?;
    }
    Ok(())
}
