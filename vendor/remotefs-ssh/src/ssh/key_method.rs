/// [`KeyMethod`] method type for SSH key exchange.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodType {
    Kex,
    HostKey,
    CryptClientServer,
    CryptServerClient,
    MacClientServer,
    MacServerClient,
    CompClientServer,
    CompServerClient,
    LangClientServer,
    LangServerClient,
    SignAlgo,
}

#[cfg(feature = "libssh2")]
impl From<MethodType> for ssh2::MethodType {
    fn from(t: MethodType) -> Self {
        match t {
            MethodType::Kex => ssh2::MethodType::Kex,
            MethodType::HostKey => ssh2::MethodType::HostKey,
            MethodType::CryptClientServer => ssh2::MethodType::CryptCs,
            MethodType::CryptServerClient => ssh2::MethodType::CryptSc,
            MethodType::MacClientServer => ssh2::MethodType::MacCs,
            MethodType::MacServerClient => ssh2::MethodType::MacSc,
            MethodType::CompClientServer => ssh2::MethodType::CompCs,
            MethodType::CompServerClient => ssh2::MethodType::CompSc,
            MethodType::LangClientServer => ssh2::MethodType::LangCs,
            MethodType::LangServerClient => ssh2::MethodType::LangSc,
            MethodType::SignAlgo => ssh2::MethodType::SignAlgo,
        }
    }
}

#[cfg(feature = "libssh2")]
impl From<ssh2::MethodType> for MethodType {
    fn from(t: ssh2::MethodType) -> Self {
        match t {
            ssh2::MethodType::Kex => MethodType::Kex,
            ssh2::MethodType::HostKey => MethodType::HostKey,
            ssh2::MethodType::CryptCs => MethodType::CryptClientServer,
            ssh2::MethodType::CryptSc => MethodType::CryptServerClient,
            ssh2::MethodType::MacCs => MethodType::MacClientServer,
            ssh2::MethodType::MacSc => MethodType::MacServerClient,
            ssh2::MethodType::CompCs => MethodType::CompClientServer,
            ssh2::MethodType::CompSc => MethodType::CompServerClient,
            ssh2::MethodType::LangCs => MethodType::LangClientServer,
            ssh2::MethodType::LangSc => MethodType::LangServerClient,
            ssh2::MethodType::SignAlgo => MethodType::SignAlgo,
        }
    }
}

/// Ssh key method.
/// Defined by [`MethodType`] (see ssh2 docs) and the list of supported algorithms.
pub struct KeyMethod {
    pub(crate) method_type: MethodType,
    algos: Vec<String>,
}

impl KeyMethod {
    /// Instantiates a new [`KeyMethod`]
    pub fn new(method_type: MethodType, algos: &[String]) -> Self {
        Self {
            method_type,
            algos: algos.to_vec(),
        }
    }

    /// Get preferred algos in ssh protocol syntax
    pub(crate) fn prefs(&self) -> String {
        self.algos.join(",")
    }

    #[cfg(feature = "libssh")]
    pub fn ssh_opts(&self) -> Option<libssh_rs::SshOption> {
        let values = self.algos.join(",");

        match self.method_type {
            MethodType::Kex => Some(libssh_rs::SshOption::KeyExchange(values)),
            MethodType::HostKey => Some(libssh_rs::SshOption::HostKeys(values)),
            MethodType::CryptClientServer => Some(libssh_rs::SshOption::CiphersCS(values)),
            MethodType::CryptServerClient => Some(libssh_rs::SshOption::CiphersSC(values)),
            MethodType::MacClientServer => Some(libssh_rs::SshOption::HmacCS(values)),
            MethodType::MacServerClient => Some(libssh_rs::SshOption::HmacSC(values)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "libssh2")]
    use ssh2::MethodType as Ssh2MethodType;

    #[cfg(feature = "libssh2")]
    use super::*;

    #[test]
    #[cfg(feature = "libssh2")]
    fn test_should_convert_method_type() {
        assert_eq!(MethodType::from(Ssh2MethodType::Kex), MethodType::Kex);
        assert_eq!(
            MethodType::from(Ssh2MethodType::HostKey),
            MethodType::HostKey
        );
        assert_eq!(
            MethodType::from(Ssh2MethodType::CryptCs),
            MethodType::CryptClientServer
        );
        assert_eq!(
            MethodType::from(Ssh2MethodType::CryptSc),
            MethodType::CryptServerClient
        );
        assert_eq!(
            MethodType::from(Ssh2MethodType::MacCs),
            MethodType::MacClientServer
        );
        assert_eq!(
            MethodType::from(Ssh2MethodType::MacSc),
            MethodType::MacServerClient
        );
        assert_eq!(
            MethodType::from(Ssh2MethodType::CompCs),
            MethodType::CompClientServer
        );
        assert_eq!(
            MethodType::from(Ssh2MethodType::CompSc),
            MethodType::CompServerClient
        );
        assert_eq!(
            MethodType::from(Ssh2MethodType::LangCs),
            MethodType::LangClientServer
        );
        assert_eq!(
            MethodType::from(Ssh2MethodType::LangSc),
            MethodType::LangServerClient
        );
        assert_eq!(
            MethodType::from(Ssh2MethodType::SignAlgo),
            MethodType::SignAlgo
        );
    }

    #[test]
    #[cfg(feature = "libssh2")]
    fn test_should_convert_method_type_back() {
        assert!(matches!(
            Ssh2MethodType::from(MethodType::Kex),
            Ssh2MethodType::Kex
        ));
        assert!(matches!(
            Ssh2MethodType::from(MethodType::HostKey),
            Ssh2MethodType::HostKey
        ));
        assert!(matches!(
            Ssh2MethodType::from(MethodType::CryptClientServer),
            Ssh2MethodType::CryptCs
        ));
        assert!(matches!(
            Ssh2MethodType::from(MethodType::CryptServerClient),
            Ssh2MethodType::CryptSc
        ));
        assert!(matches!(
            Ssh2MethodType::from(MethodType::MacClientServer),
            Ssh2MethodType::MacCs
        ));
        assert!(matches!(
            Ssh2MethodType::from(MethodType::MacServerClient),
            Ssh2MethodType::MacSc
        ));
        assert!(matches!(
            Ssh2MethodType::from(MethodType::CompClientServer),
            Ssh2MethodType::CompCs
        ));
        assert!(matches!(
            Ssh2MethodType::from(MethodType::CompServerClient),
            Ssh2MethodType::CompSc
        ));
        assert!(matches!(
            Ssh2MethodType::from(MethodType::LangClientServer),
            Ssh2MethodType::LangCs
        ));
        assert!(matches!(
            Ssh2MethodType::from(MethodType::LangServerClient),
            Ssh2MethodType::LangSc
        ));
        assert!(matches!(
            Ssh2MethodType::from(MethodType::SignAlgo),
            Ssh2MethodType::SignAlgo
        ));
    }
}
