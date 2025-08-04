pub fn encipher(_input: &[u8]) -> Vec<u8> {
    return Vec::new();
}

// Conditional compilation attribute, only compile when running tests.
#[cfg(test)]
mod tests {
    // Use mod to keep tests scoped and organized.
    use super::*;
    #[test] // Rust's test runner will be able to find and run this as a test.
    fn test_encipher_transforms_HAL_to_IBM() {
        let got = encipher(b"HAL");
        let expected = b"IBM".to_vec();
        assert_eq!(expected, got, "expected {:?}, got {:?}", expected, got);
    }
}
