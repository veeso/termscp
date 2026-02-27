use std::env;
use std::path::{Path, PathBuf};

use super::super::{ConfigClient, FileTransferActivity};
use crate::filetransfer::{HostBridgeParams, ProtocolParams};
use crate::system::environment;
use crate::utils::path;

impl FileTransferActivity {
    /// Initialize configuration client if possible.
    /// This function doesn't return errors.
    pub(in crate::ui::activities::filetransfer) fn init_config_client() -> ConfigClient {
        match environment::init_config_dir() {
            Ok(termscp_dir) => match termscp_dir {
                Some(termscp_dir) => {
                    // Make configuration file path and ssh keys path
                    let (config_path, ssh_keys_path): (PathBuf, PathBuf) =
                        environment::get_config_paths(termscp_dir.as_path());
                    match ConfigClient::new(config_path.as_path(), ssh_keys_path.as_path()) {
                        Ok(config_client) => config_client,
                        Err(_) => ConfigClient::degraded(),
                    }
                }
                None => ConfigClient::degraded(),
            },
            Err(_) => ConfigClient::degraded(),
        }
    }

    /// Set text editor to use
    pub(in crate::ui::activities::filetransfer) fn setup_text_editor(&self) {
        unsafe {
            env::set_var("EDITOR", self.config().get_text_editor());
        }
    }

    /// Convert a path to absolute according to host explorer
    pub(in crate::ui::activities::filetransfer) fn host_bridge_to_abs_path(
        &self,
        path: &Path,
    ) -> PathBuf {
        path::absolutize(self.host_bridge().wrkdir.as_path(), path)
    }

    /// Convert a path to absolute according to remote explorer
    pub(in crate::ui::activities::filetransfer) fn remote_to_abs_path(
        &self,
        path: &Path,
    ) -> PathBuf {
        path::absolutize(self.remote().wrkdir.as_path(), path)
    }

    /// Get remote hostname
    pub(in crate::ui::activities::filetransfer) fn get_remote_hostname(&self) -> String {
        let ft_params = self.context().remote_params().unwrap();
        self.get_hostname(&ft_params.params)
    }

    pub(in crate::ui::activities::filetransfer) fn get_hostbridge_hostname(&self) -> String {
        let host_bridge_params = self.context().host_bridge_params().unwrap();
        match host_bridge_params {
            HostBridgeParams::Localhost(_) => {
                let hostname = match hostname::get() {
                    Ok(h) => h,
                    Err(_) => return String::from("localhost"),
                };
                let hostname: String = hostname.as_os_str().to_string_lossy().to_string();
                let tokens: Vec<&str> = hostname.split('.').collect();
                String::from(*tokens.first().unwrap_or(&"localhost"))
            }
            HostBridgeParams::Remote(_, params) => self.get_hostname(params),
        }
    }

    fn get_hostname(&self, params: &ProtocolParams) -> String {
        match params {
            ProtocolParams::Generic(params) => params.address.clone(),
            ProtocolParams::AwsS3(params) => params.bucket_name.clone(),
            ProtocolParams::Kube(params) => {
                params.namespace.clone().unwrap_or("default".to_string())
            }
            ProtocolParams::Smb(params) => params.address.clone(),
            ProtocolParams::WebDAV(params) => params.uri.clone(),
        }
    }

    /// Get connection message to show to client
    pub(in crate::ui::activities::filetransfer) fn get_connection_msg(
        params: &ProtocolParams,
    ) -> String {
        match params {
            ProtocolParams::Generic(params) => {
                info!(
                    "Client is not connected to remote; connecting to {}:{}",
                    params.address, params.port
                );
                format!("Connecting to {}:{}…", params.address, params.port)
            }
            ProtocolParams::AwsS3(params) => {
                info!(
                    "Client is not connected to remote; connecting to {}{} ({})",
                    params.endpoint.as_deref().unwrap_or(""),
                    params.bucket_name,
                    params.region.as_deref().unwrap_or("custom")
                );
                format!("Connecting to {}…", params.bucket_name)
            }
            ProtocolParams::Kube(params) => {
                let namespace = params.namespace.as_deref().unwrap_or("default");
                info!("Client is not connected to remote; connecting to namespace {namespace}",);
                format!("Connecting to Kube namespace {namespace}…",)
            }
            ProtocolParams::Smb(params) => {
                info!(
                    "Client is not connected to remote; connecting to {}:{}",
                    params.address, params.share
                );
                format!("Connecting to \\\\{}\\{}…", params.address, params.share)
            }
            ProtocolParams::WebDAV(params) => {
                info!(
                    "Client is not connected to remote; connecting to {}",
                    params.uri
                );
                format!("Connecting to {}…", params.uri)
            }
        }
    }
}
