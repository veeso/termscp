//! `crypto` provides utilities for encrypting and decrypting strings.
//!
//! New data is encrypted with AES-128-GCM (authenticated encryption).
//! Legacy data encrypted by `magic-crypt` (AES-128-CBC with MD5 key derivation
//! and a zero IV) is transparently decrypted via a fallback path.

use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockDecryptMut, KeyIvInit};
use aes_gcm::aead::{Aead, AeadCore};
use aes_gcm::{Aes128Gcm, KeyInit, Nonce};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use md5::{Digest, Md5};

/// Nonce length for AES-128-GCM (96 bits).
const GCM_NONCE_LEN: usize = 12;

/// Encrypt a string with AES-128-GCM. The output is `nonce || ciphertext`,
/// encoded as standard Base64.
pub fn aes128_b64_crypt(key: &str, input: &str) -> String {
    let derived = derive_gcm_key(key);
    let cipher = Aes128Gcm::new(&derived.into());
    let nonce_bytes = Aes128Gcm::generate_nonce(&mut aes_gcm::aead::OsRng);
    let ciphertext = cipher
        .encrypt(&nonce_bytes, input.as_bytes())
        .expect("AES-GCM encryption must not fail for valid inputs");

    let mut combined = Vec::with_capacity(GCM_NONCE_LEN + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    B64.encode(combined)
}

/// Decrypt a Base64-encoded string. Tries AES-128-GCM first; on failure falls
/// back to the legacy AES-128-CBC format produced by `magic-crypt`.
pub fn aes128_b64_decrypt(key: &str, secret: &str) -> Result<String, CryptoError> {
    // Try modern AES-128-GCM first
    if let Ok(plaintext) = decrypt_gcm(key, secret) {
        return Ok(plaintext);
    }
    // Fall back to legacy AES-128-CBC (magic-crypt compat)
    decrypt_legacy_cbc(key, secret)
}

/// AES-128-GCM decryption. Expects `nonce (12 bytes) || ciphertext` in the
/// Base64-decoded payload.
fn decrypt_gcm(key: &str, secret: &str) -> Result<String, CryptoError> {
    let raw = B64.decode(secret)?;
    if raw.len() < GCM_NONCE_LEN {
        return Err(CryptoError::InvalidData);
    }
    let (nonce_bytes, ciphertext) = raw.split_at(GCM_NONCE_LEN);
    let nonce = Nonce::from_slice(nonce_bytes);
    let derived = derive_gcm_key(key);
    let cipher = Aes128Gcm::new(&derived.into());
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::AesGcm)?;
    String::from_utf8(plaintext).map_err(|_| CryptoError::InvalidData)
}

/// Legacy AES-128-CBC decryption compatible with `magic-crypt` v4.
/// Key is derived via MD5; IV is 16 zero bytes; padding is PKCS7.
fn decrypt_legacy_cbc(key: &str, secret: &str) -> Result<String, CryptoError> {
    type CbcDec = cbc::Decryptor<aes::Aes128>;

    let key_bytes: [u8; 16] = Md5::digest(key.as_bytes()).into();
    let iv = [0u8; 16];
    let raw = B64.decode(secret)?;
    let decryptor = CbcDec::new(&key_bytes.into(), &iv.into());
    let plaintext = decryptor
        .decrypt_padded_vec_mut::<Pkcs7>(&raw)
        .map_err(|_| CryptoError::InvalidData)?;
    String::from_utf8(plaintext).map_err(|_| CryptoError::InvalidData)
}

/// Derive a 128-bit key for AES-GCM by hashing the input with MD5.
fn derive_gcm_key(key: &str) -> [u8; 16] {
    let digest = Md5::digest(key.as_bytes());
    let mut out = [0u8; 16];
    out.copy_from_slice(&digest);
    out
}

/// Errors that can occur during crypto operations.
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("base64 decode error: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("AES-GCM decryption failed")]
    AesGcm,
    #[error("invalid encrypted data")]
    InvalidData,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = "MYSUPERSECRETKEY";
        let input = "Hello world!";
        let secret = aes128_b64_crypt(key, input);
        let decrypted = aes128_b64_decrypt(key, &secret).unwrap();
        assert_eq!(decrypted, input);
    }

    #[test]
    fn test_decrypt_legacy_magic_crypt_ciphertext() {
        // This ciphertext was produced by magic-crypt v4:
        //   new_magic_crypt!("MYSUPERSECRETKEY", 128).encrypt_str_to_base64("Hello world!")
        let key = "MYSUPERSECRETKEY";
        let legacy_secret = "z4Z6LpcpYqBW4+bkIok+5A==";
        let decrypted = aes128_b64_decrypt(key, legacy_secret).unwrap();
        assert_eq!(decrypted, "Hello world!");
    }

    #[test]
    fn test_different_encryptions_produce_different_ciphertexts() {
        let key = "MYSUPERSECRETKEY";
        let input = "Hello world!";
        let secret1 = aes128_b64_crypt(key, input);
        let secret2 = aes128_b64_crypt(key, input);
        // Due to random nonces, ciphertexts must differ
        assert_ne!(secret1, secret2);
        // But both decrypt to the same plaintext
        assert_eq!(aes128_b64_decrypt(key, &secret1).unwrap(), input);
        assert_eq!(aes128_b64_decrypt(key, &secret2).unwrap(), input);
    }

    #[test]
    fn test_wrong_key_fails() {
        let secret = aes128_b64_crypt("correct-key", "sensitive data");
        assert!(aes128_b64_decrypt("wrong-key", &secret).is_err());
    }

    #[test]
    fn test_invalid_base64_fails() {
        assert!(aes128_b64_decrypt("key", "not-valid-base64!!!").is_err());
    }
}
