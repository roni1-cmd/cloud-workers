use worker::*;

pub async fn upload(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = ctx.param("id").unwrap();
    let name = ctx.param("name").unwrap();
    let bucket = ctx.env.r2("MOM_FILES")?;
    
    let bytes = req.bytes().await?;
    bucket.put(format!("{}/{}", id, name), bytes).execute().await?;
    Response::ok("Uploaded")
}

pub async fn download(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = ctx.param("id").unwrap();
    let name = ctx.param("name").unwrap();
    let bucket = ctx.env.r2("MOM_FILES")?;
    
    let object = bucket.get(format!("{}/{}", id, name)).execute().await?;
    match object {
        Some(obj) => Response::from_body(obj.body().unwrap()),
        None => Response::error("File not found", 404),
    }
}
