//! ## WebDAV Parameters
//!
//! Defines the runtime connection parameters used to build WebDAV clients.

/// Protocol params used by WebDAV
#[derive(Debug, Clone)]
pub struct WebDAVProtocolParams {
    /// Base WebDAV endpoint URI.
    pub uri: String,
    /// Username used for authentication.
    pub username: String,
    /// Password used for authentication.
    pub password: String,
}

impl WebDAVProtocolParams {
    /// Stores the shared secret as the active WebDAV password.
    pub fn set_default_secret(&mut self, secret: String) {
        self.password = secret;
    }

    /// Returns whether the WebDAV password is currently missing.
    pub fn password_missing(&self) -> bool {
        self.password.is_empty()
    }
}
