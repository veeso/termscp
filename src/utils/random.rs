//! ## Random
//!
//! `random` is the module which provides utilities for rand

// Ext
use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// Generate a random alphanumeric string with provided length
pub fn random_alphanumeric_with_len(len: usize) -> String {
    let mut rng = thread_rng();
    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_utils_random_alphanumeric_with_len() {
        assert_eq!(random_alphanumeric_with_len(256).len(), 256);
    }
}
