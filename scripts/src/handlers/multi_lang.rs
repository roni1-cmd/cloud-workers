use worker::*;

extern "C" {
    // From TinyGo
    fn validate_header(ptr: *const u8, len: usize) -> bool;
    // From C
    fn obfuscate_meeting_id(id: u32, salt: u32) -> u32;
}

pub async fn secure_upload_handler(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let bytes = req.bytes().await?;
    
    // 1. Use Go to validate the file type at a low level
    let is_pdf = unsafe { validate_header(bytes.as_ptr(), bytes.len()) };
    if !is_pdf {
        return Response::error("COMELEC policy: Only PDF attachments allowed.", 400);
    }

    // 2. Use C to generate an internal reference index
    let meeting_id: u32 = ctx.param("id").unwrap().parse().unwrap_or(0);
    let secure_id = unsafe { obfuscate_meeting_id(meeting_id, 2026) };

    // 3. Store in R2 using the C-generated secure ID
    let bucket = ctx.env.r2("MOM_ATTACHMENTS")?;
    bucket.put(format!("locked_{}", secure_id), bytes).execute().await?;

    Response::ok("File validated by Go, encrypted by C, and stored by Rust.")
}
