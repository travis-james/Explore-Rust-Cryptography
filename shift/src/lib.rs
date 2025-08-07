pub fn encipher(input: &[u8]) -> Vec<u8> {
    // iter iterates over input, BORROWING each value, it is not the actual value.
    // map applies a function on each borrowed value. |x| is like func(x), what comes after is like the function body.
    // imagine now we're at the end of the operation, we have this newly modified value, need to catch all these new values into a COLLECTION, hence collect. Rust implicitly knows the type in this case.
    return input.iter().map(|x| x + 1).collect();
}

// Conditional compilation attribute, only compile when running tests.
#[cfg(test)]
mod tests {
    // Use mod to keep tests scoped and organized.
    use super::*;
    #[test] // Rust's test runner will be able to find and run this as a test.
    fn test_encipher_transforms() {
        let tests = vec![
            ("HAL to IBM", b"HAL".to_vec(), b"IBM".to_vec()),
            ("ADD to BEE", b"ADD".to_vec(), b"BEE".to_vec()),
        ];
        for (name, input, expected) in tests {
            let got = encipher(&input);
            assert_eq!(
                expected, got,
                "tests: {}, expected {:?}, got {:?}",
                name, expected, got
            );
        }
    }
}
