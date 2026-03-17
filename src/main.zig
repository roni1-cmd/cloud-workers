const std = @import("std");

// Exported to be called from the Rust or JS layer
export fn validate_voter_id(id_ptr: [*]const u8, len: usize) bool {
    const id = id_ptr[0..len];
    // Example: Check if ID follows the 'CTU-XXXX-XXXX' format
    if (len != 13) return false;
    if (!std.mem.eql(u8, id[0..4], "CTU-")) return false;
    return true;
}
