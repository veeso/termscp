//! # String
//!
//! String related utilities

/// Get a substring considering utf8 characters
pub fn secure_substring(string: &str, start: usize, end: usize) -> String {
    assert!(end >= start);
    string.chars().take(end).skip(start).collect()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_get_secure_substring() {
        assert_eq!(secure_substring("christian", 2, 5).as_str(), "ris");
        assert_eq!(secure_substring("россия", 3, 5).as_str(), "си");
    }
}
