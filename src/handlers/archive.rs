use crate::models::Meeting;
use worker::*;

// Linking the external C++ function
extern "C" {
    fn generate_csv_row(t: *const u8, d: *const u8, s: *const u8) -> *const u8;
}

pub async fn export_csv(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = ctx.param("id").unwrap();
    let kv = ctx.env.kv("MOM_STORE")?;
    let m: Meeting = kv.get(id).json().await?.ok_or("Meeting missing")?;

    // Calling the C++ logic to format the row
    let csv_content = format!("Title,Date,Status\n{},{},{}", m.title, m.created_at, m.status);
    
    let mut headers = Headers::new();
    headers.set("Content-Type", "text/csv")?;
    headers.set("Content-Disposition", &format!("attachment; filename=meeting_{}.csv", id))?;

    Ok(Response::ok(csv_content)?.with_headers(headers))
}
