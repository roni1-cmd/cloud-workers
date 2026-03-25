#[cfg(test)]
mod tests {
    #[test]
    fn test_wasm_linkage() {
        // Mocking the Wasm environment to test the bridge
        let result = unsafe { validate_voter_id("CTU-1234-5678".as_ptr(), 13) };
        assert!(result);
    }
}
