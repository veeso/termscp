//! ## Kubernetes Parameters
//!
//! Defines the runtime parameters used to construct Kubernetes-backed file
//! transfer clients.

use remotefs_kube::Config;

/// Protocol params used by Kubernetes connections.
#[derive(Debug, Clone)]
pub struct KubeProtocolParams {
    /// Optional namespace for the default pod context.
    pub namespace: Option<String>,
    /// Optional Kubernetes API URL.
    pub cluster_url: Option<String>,
    /// Optional username override.
    pub username: Option<String>,
    /// Optional client certificate path.
    pub client_cert: Option<String>,
    /// Optional client key path.
    pub client_key: Option<String>,
}

impl KubeProtocolParams {
    /// Kubernetes connections do not use the shared password secret flow.
    pub fn set_default_secret(&mut self, _secret: String) {}

    /// Kubernetes params never require the generic password prompt.
    pub fn password_missing(&self) -> bool {
        false
    }

    /// Converts bookmark/runtime parameters into a `remotefs_kube` config.
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
