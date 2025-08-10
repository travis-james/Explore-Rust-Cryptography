pub fn encipher(input: &[u8], key: &u8) -> Vec<u8> {
    // iter iterates over input, BORROWING each value, it is not the actual value.
    // map applies a function on each borrowed value. |x| is like func(x), what comes after is like the function body.
    // imagine now we're at the end of the operation, we have this newly modified value, need to catch all these new values into a COLLECTION, hence collect. Rust implicitly knows the type in this case.
    return input.iter().map(|x| x + key).collect();
}

// Conditional compilation attribute, only compile when running tests.
#[cfg(test)]
mod tests {
    // Use mod to keep tests scoped and organized.
    use super::*;
    #[test] // Rust's test runner will be able to find and run this as a test.
    fn test_encipher_transforms() {
        let tests = vec![
            ("HAL to IBM", 1u8, b"HAL".to_vec(), b"IBM".to_vec()),
            ("SPEC to URGE", 2u8, b"SPEC".to_vec(), b"URGE".to_vec()),
            ("PERK to SHUN", 3u8, b"PERK".to_vec(), b"SHUN".to_vec()),
            ("BEEF to LOOP", 10u8, b"BEEF".to_vec(), b"LOOP".to_vec()),
        ];
        for (name, key, input, expected) in tests {
            let got = encipher(&input, &key);
            assert_eq!(
                expected, got,
                "test name: {}, key: {}, expected {:?}, got {:?}",
                name, key, expected, got
            );
        }
    }
}
