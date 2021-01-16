//! ## Crypto
//!
//! `crypto` is the module which provides utilities for crypting

/*
*
*   Copyright (C) 2020-2021 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "TermSCP"
*
*   TermSCP is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   TermSCP is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with TermSCP.  If not, see <http://www.gnu.org/licenses/>.
*
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
