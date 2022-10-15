//! ## Crypto
//!
//! `crypto` is the module which provides utilities for crypting

// Ext
use magic_crypt::MagicCryptTrait;

/// ### aes128_b64_crypt
///
/// Crypt a string using AES128; output is returned as a BASE64 string
pub fn aes128_b64_crypt(key: &str, input: &str) -> String {
    let crypter = new_magic_crypt!(key, 128);
    crypter.encrypt_str_to_base64(input)
}

/// ### aes128_b64_decrypt
///
/// Decrypt a string using AES128
pub fn aes128_b64_decrypt(key: &str, secret: &str) -> Result<String, magic_crypt::MagicCryptError> {
    let crypter = new_magic_crypt!(key, 128);
    crypter.decrypt_base64_to_string(secret)
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_utils_crypto_aes128() {
        let key: &str = "MYSUPERSECRETKEY";
        let input: &str = "Hello world!";
        let secret: String = aes128_b64_crypt(key, input);
        assert_eq!(secret.as_str(), "z4Z6LpcpYqBW4+bkIok+5A==");
        assert_eq!(
            aes128_b64_decrypt(key, secret.as_str())
                .ok()
                .unwrap()
                .as_str(),
            input
        );
    }
}
