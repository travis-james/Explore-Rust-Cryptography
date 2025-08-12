pub fn encipher(input: &[u8], key: u8) -> Vec<u8> {
    // iter iterates over input, BORROWING each value, it is not the actual value.
    // map applies a function on each borrowed value. |x| is like func(x), what comes after is like the function body.
    // imagine now we're at the end of the operation, we have this newly modified value, need to catch all these new values into a COLLECTION, hence collect. Rust implicitly knows the type in this case.
    input.iter().map(|x| x.wrapping_add(key)).collect()
}

pub fn decipher(input: &[u8], key: u8) -> Vec<u8> {
    input.iter().map(|x| x.wrapping_sub(key)).collect()
}

pub fn crack(cipher_text: &[u8], crib: &[u8]) -> Option<u8> {
    for i in 0..=256 {
        let ferk = decipher(&cipher_text[..crib.len()], i as u8);
        if ferk != crib {
            continue;
        }
        return Some(i as u8);
    }
    None
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
            let got = encipher(&input, key);
            assert_eq!(
                expected, got,
                "test name: {}, key: {}, expected {:?}, got {:?}",
                name, key, expected, got
            );
        }
    }

    #[test]
    fn test_decipher_transforms() {
        let tests = vec![
            ("IBM to HAL", 1u8, b"IBM".to_vec(), b"HAL".to_vec()),
            ("URGE to SPEC", 2u8, b"URGE".to_vec(), b"SPEC".to_vec()),
            ("SHUN to PERK", 3u8, b"SHUN".to_vec(), b"PERK".to_vec()),
            ("LOOP to BEEF", 10u8, b"LOOP".to_vec(), b"BEEF".to_vec()),
        ];
        for (name, key, input, expected) in tests {
            let got = decipher(&input, key);
            assert_eq!(
                expected, got,
                "test name: {}, key: {}, expected {:?}, got {:?}",
                name, key, expected, got
            );
        }
    }

    #[test]
    fn test_crack() {
        let tests = vec![
            ("HAL to IBM", 1u8, b"HAL".to_vec(), b"IBM".to_vec()),
            ("SPEC to URGE", 2u8, b"SPEC".to_vec(), b"URGE".to_vec()),
            ("PERK to SHUN", 3u8, b"PERK".to_vec(), b"SHUN".to_vec()),
            ("BEEF to LOOP", 10u8, b"BEEF".to_vec(), b"LOOP".to_vec()),
        ];
        for (name, expected_key, plain_text, cipher_text) in tests {
            let got = crack(&cipher_text, &plain_text[..3]).expect("Failed to crack cipher");
            assert_eq!(
                expected_key, got,
                "test name: {}, expected {:?}, got {:?}",
                name, expected_key, got
            )
        }
    }

    #[test]
    fn test_crack_fail() {
        let cipher_text = b"anything";
        let plain_text = b"beryllium";
        let got = crack(cipher_text, &plain_text[..3]);
        assert!(got.is_none())
    }
}
