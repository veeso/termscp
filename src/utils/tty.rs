//! ## Utils
//!
//! `Utils` implements utilities functions to work with layouts

/// Read a secret from tty with customisable prompt
pub fn read_secret_from_tty(prompt: &str) -> std::io::Result<Option<String>> {
    match rpassword::read_password_from_tty(Some(prompt)) {
        Ok(p) if p.is_empty() => Ok(None),
        Ok(p) => Ok(Some(p)),
        Err(err) => Err(err),
    }
}
