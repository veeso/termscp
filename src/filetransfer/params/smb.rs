//! ## SMB Parameters
//!
//! Defines the runtime connection parameters used to build SMB remote
//! filesystem clients.

use serde::{Deserialize, Serialize};

/// SMB protocol family requested for a connection.
///
/// Each family maps to inclusive dialect bounds when the Unix client is built.
/// `Auto` negotiates SMB2 or SMB3 and never falls back to SMB1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SmbDialect {
    /// Negotiate SMB 2.0.2 through SMB 3.1.1.
    #[default]
    Auto,
    /// Force the deprecated NT1 (CIFS) dialect.
    Smb1,
    /// Negotiate SMB 2.0.2 through SMB 2.1.
    Smb2,
    /// Negotiate SMB 3.0 through SMB 3.1.1.
    Smb3,
}

/// Connection parameters for SMB protocol
#[derive(Debug, Clone)]
pub struct SmbParams {
    /// Hostname or address of the SMB server.
    pub address: String,
    #[cfg(posix)]
    /// SMB service port used on POSIX platforms.
    pub port: u16,
    /// Share name to mount.
    pub share: String,
    /// Optional username.
    pub username: Option<String>,
    /// Optional password.
    pub password: Option<String>,
    #[cfg(posix)]
    /// Optional workgroup used on POSIX platforms.
    pub workgroup: Option<String>,
    /// Requested SMB protocol family. Enforced on POSIX platforms only.
    pub dialect: SmbDialect,
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
            dialect: SmbDialect::default(),
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

    /// Sets the SMB protocol family to request.
    pub fn dialect(mut self, dialect: SmbDialect) -> Self {
        self.dialect = dialect;
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

    use super::SmbParams;
    use crate::filetransfer::params::SmbDialect;

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
    fn should_default_dialect_to_auto() {
        assert_eq!(SmbDialect::default(), SmbDialect::Auto);
        let params = SmbParams::new("localhost", "temp");
        assert_eq!(params.dialect, SmbDialect::Auto);
    }

    #[test]
    fn should_set_dialect() {
        let params = SmbParams::new("localhost", "temp").dialect(SmbDialect::Smb1);
        assert_eq!(params.dialect, SmbDialect::Smb1);
    }

    #[test]
    fn should_serialize_dialect_lowercase() {
        assert_eq!(
            toml::to_string(&Wrapper {
                dialect: SmbDialect::Auto,
            })
            .unwrap()
            .trim(),
            "dialect = \"auto\""
        );
        assert_eq!(
            toml::to_string(&Wrapper {
                dialect: SmbDialect::Smb1,
            })
            .unwrap()
            .trim(),
            "dialect = \"smb1\""
        );
        assert_eq!(
            toml::to_string(&Wrapper {
                dialect: SmbDialect::Smb2,
            })
            .unwrap()
            .trim(),
            "dialect = \"smb2\""
        );
        assert_eq!(
            toml::to_string(&Wrapper {
                dialect: SmbDialect::Smb3,
            })
            .unwrap()
            .trim(),
            "dialect = \"smb3\""
        );
    }

    #[test]
    fn should_deserialize_dialect_lowercase() {
        let w: Wrapper = toml::from_str("dialect = \"smb3\"").unwrap();
        assert_eq!(w.dialect, SmbDialect::Smb3);
        assert!(toml::from_str::<Wrapper>("dialect = \"SMB3\"").is_err());
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Wrapper {
        dialect: SmbDialect,
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
