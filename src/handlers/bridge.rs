use worker::*;

// Link the Zig-compiled function
extern "C" {
    fn validate_voter_id(ptr: *const u8, len: usize) -> bool;
}

pub async fn secure_record_vote(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let voter_id = req.headers().get("X-Voter-ID")?.unwrap_or_default();
    
    // Call Zig validation logic
    let is_valid = unsafe {
        validate_voter_id(voter_id.as_ptr(), voter_id.len())
    };

    if !is_valid {
        return Response::error("Invalid CTU Student ID Format", 400);
    }

    Response::ok("Vote Recorded Successfully")
}
