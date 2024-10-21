/// Connection parameters for SMB protocol
#[derive(Debug, Clone)]
pub struct SmbParams {
    pub address: String,
    #[cfg(posix)]
    pub port: u16,
    pub share: String,
    pub username: Option<String>,
    pub password: Option<String>,
    #[cfg(posix)]
    pub workgroup: Option<String>,
}

// -- SMB params

impl SmbParams {
    /// Instantiates a new `AwsS3Params` struct
    pub fn new<S: AsRef<str>>(address: S, share: S) -> Self {
        Self {
            address: address.as_ref().to_string(),
            #[cfg(posix)]
            port: 445,
            share: share.as_ref().to_string(),
            username: None,
            password: None,
            #[cfg(posix)]
            workgroup: None,
        }
    }

    #[cfg(posix)]
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn username(mut self, username: Option<impl ToString>) -> Self {
        self.username = username.map(|x| x.to_string());
        self
    }

    pub fn password(mut self, password: Option<impl ToString>) -> Self {
        self.password = password.map(|x| x.to_string());
        self
    }

    #[cfg(posix)]
    pub fn workgroup(mut self, workgroup: Option<impl ToString>) -> Self {
        self.workgroup = workgroup.map(|x| x.to_string());
        self
    }

    /// Returns whether a password is supposed to be required for this protocol params.
    /// The result true is returned ONLY if the supposed secret is MISSING!!!
    pub fn password_missing(&self) -> bool {
        self.password.is_none()
    }

    /// Set password
    #[cfg(posix)]
    pub fn set_default_secret(&mut self, secret: String) {
        self.password = Some(secret);
    }

    #[cfg(win)]
    pub fn set_default_secret(&mut self, _secret: String) {}
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_init_smb_params() {
        let params = SmbParams::new("localhost", "temp");
        assert_eq!(&params.address, "localhost");

        #[cfg(posix)]
        assert_eq!(params.port, 445);
        assert_eq!(&params.share, "temp");

        #[cfg(posix)]
        assert!(params.username.is_none());
        #[cfg(posix)]
        assert!(params.password.is_none());
        #[cfg(posix)]
        assert!(params.workgroup.is_none());
    }

    #[test]
    #[cfg(posix)]
    fn should_init_smb_params_with_optionals() {
        let params = SmbParams::new("localhost", "temp")
            .port(3456)
            .username(Some("foo"))
            .password(Some("bar"))
            .workgroup(Some("baz"));

        assert_eq!(&params.address, "localhost");
        assert_eq!(params.port, 3456);
        assert_eq!(&params.share, "temp");
        assert_eq!(params.username.as_deref().unwrap(), "foo");
        assert_eq!(params.password.as_deref().unwrap(), "bar");
        assert_eq!(params.workgroup.as_deref().unwrap(), "baz");
    }

    #[test]
    #[cfg(win)]
    fn should_init_smb_params_with_optionals() {
        let params = SmbParams::new("localhost", "temp")
            .username(Some("foo"))
            .password(Some("bar"));

        assert_eq!(&params.address, "localhost");
        assert_eq!(&params.share, "temp");
        assert_eq!(params.username.as_deref().unwrap(), "foo");
        assert_eq!(params.password.as_deref().unwrap(), "bar");
    }
}
