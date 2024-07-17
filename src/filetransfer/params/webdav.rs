/// Protocol params used by WebDAV
#[derive(Debug, Clone)]
pub struct WebDAVProtocolParams {
    pub uri: String,
    pub username: String,
    pub password: String,
}

impl WebDAVProtocolParams {
    pub fn set_default_secret(&mut self, secret: String) {
        self.password = secret;
    }

    pub fn password_missing(&self) -> bool {
        self.password.is_empty()
    }
}
