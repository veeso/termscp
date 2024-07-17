use serde::{Deserialize, Serialize};

use crate::filetransfer::params::KubeProtocolParams;

/// Extra Connection parameters for Kube protocol
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq, Default)]
pub struct KubeParams {
    pub pod_name: String,
    pub container: String,
    pub namespace: Option<String>,
    pub cluster_url: Option<String>,
    pub username: Option<String>,
    pub client_cert: Option<String>,
    pub client_key: Option<String>,
}

impl From<KubeParams> for KubeProtocolParams {
    fn from(value: KubeParams) -> Self {
        Self {
            pod: value.pod_name,
            container: value.container,
            namespace: value.namespace,
            cluster_url: value.cluster_url,
            username: value.username,
            client_cert: value.client_cert,
            client_key: value.client_key,
        }
    }
}

impl From<KubeProtocolParams> for KubeParams {
    fn from(value: KubeProtocolParams) -> Self {
        Self {
            pod_name: value.pod,
            container: value.container,
            namespace: value.namespace,
            cluster_url: value.cluster_url,
            username: value.username,
            client_cert: value.client_cert,
            client_key: value.client_key,
        }
    }
}
