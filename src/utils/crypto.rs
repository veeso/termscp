//! ## Crypto
//!
//! `crypto` is the module which provides utilities for crypting

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// Deps
extern crate magic_crypt;

// Ext
use magic_crypt::MagicCryptTrait;

/// ### aes128_b64_crypt
///
/// Crypt a string using AES128; output is returned as a BASE64 string
pub fn aes128_b64_crypt(key: &str, input: &str) -> String {
    let crypter = new_magic_crypt!(key.to_string(), 128);
    crypter.encrypt_str_to_base64(input.to_string())
}

/// ### aes128_b64_decrypt
///
/// Decrypt a string using AES128
pub fn aes128_b64_decrypt(key: &str, secret: &str) -> Result<String, magic_crypt::MagicCryptError> {
    let crypter = new_magic_crypt!(key.to_string(), 128);
    crypter.decrypt_base64_to_string(secret.to_string())
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_utils_crypto_aes128() {
        let key: &str = "MYSUPERSECRETKEY";
        let input: &str = "Hello world!";
        let secret: String = aes128_b64_crypt(&key, input);
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
