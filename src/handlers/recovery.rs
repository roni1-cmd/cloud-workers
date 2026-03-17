use worker::*;

pub fn safety_checkpoint(ctx: &RouteContext<()>, data: &str) -> Result<()> {
    let kv = ctx.env.kv("RECOVERY_STORAGE")?;
    // Save a "dirty" copy in case the Wasm engine crashes during C/Go execution
    kv.put("last_known_state", data)?.execute()
}
