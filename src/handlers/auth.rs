use worker::*;

pub fn validate_auth(req: &Request, ctx: &RouteContext<()>) -> std::result::Result<(), Response> {
    // Retrieve the secret key from the environment variables
    let secret = match ctx.secret("API_KEY") {
        Ok(s) => s.to_string(),
        Err(_) => return Err(Response::error("Internal Security Error", 500).unwrap()),
    };

    // Check the Authorization header
    let auth_header = req.headers().get("Authorization").ok().flatten();
    
    match auth_header {
        Some(val) if val == format!("Bearer {}", secret) => Ok(()),
        _ => Err(Response::error("Unauthorized: COMELEC Access Only", 401).unwrap()),
    }
}
