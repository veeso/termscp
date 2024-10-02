use remotefs_kube::Config;

/// Protocol params used by WebDAV
#[derive(Debug, Clone)]
pub struct KubeProtocolParams {
    pub namespace: Option<String>,
    pub cluster_url: Option<String>,
    pub username: Option<String>,
    pub client_cert: Option<String>,
    pub client_key: Option<String>,
}

impl KubeProtocolParams {
    pub fn set_default_secret(&mut self, _secret: String) {}

    pub fn password_missing(&self) -> bool {
        false
    }

    pub fn config(self) -> Option<Config> {
        if let Some(cluster_url) = self.cluster_url {
            let mut config = Config::new(cluster_url.parse().unwrap_or_default());
            config.auth_info.username = self.username;
            config.auth_info.client_certificate = self.client_cert;
            config.auth_info.client_key = self.client_key;
            if let Some(namespace) = self.namespace {
                config.default_namespace = namespace;
            }

            Some(config)
        } else {
            None
        }
    }
}
