//! ## Config
//!
//! `config` is the module which provides access to termscp configuration

// Locals
use crate::filetransfer::FileTransferProtocol;

// Ext
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub const DEFAULT_NOTIFICATION_TRANSFER_THRESHOLD: u64 = 536870912; // 512MB

#[derive(Deserialize, Serialize, Debug, Default)]
/// UserConfig contains all the configurations for the user,
/// supported by termscp
pub struct UserConfig {
    pub user_interface: UserInterfaceConfig,
    pub remote: RemoteConfig,
}

#[derive(Deserialize, Serialize, Debug)]
/// UserInterfaceConfig provides all the keys to configure the user interface
pub struct UserInterfaceConfig {
    pub text_editor: PathBuf,
    pub default_protocol: String,
    pub show_hidden_files: bool,
    pub check_for_updates: Option<bool>,      // @! Since 0.3.3
    pub prompt_on_file_replace: Option<bool>, // @! Since 0.7.0; Default True
    pub group_dirs: Option<String>,
    /// file fmt. Refers to local host (for backward compatibility)
    pub file_fmt: Option<String>,
    pub remote_file_fmt: Option<String>,     // @! Since 0.5.0
    pub notifications: Option<bool>,         // @! Since 0.7.0; Default true
    pub notification_threshold: Option<u64>, // @! Since 0.7.0; Default 512MB
}

#[derive(Deserialize, Serialize, Debug)]
/// Contains configuratio related to remote hosts
pub struct RemoteConfig {
    /// Ssh configuration path. If NONE, won't be read
    pub ssh_config: Option<String>,
    /// Association between host name and path to private key
    /// NOTE: this parameter must stay as last: <https://github.com/alexcrichton/toml-rs/issues/142>
    pub ssh_keys: HashMap<String, PathBuf>,
}

impl Default for RemoteConfig {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/root"));
        let mut ssh_config_path = "~/.ssh/config".to_string();
        ssh_config_path = ssh_config_path.replacen('~', &home_dir.to_string_lossy(), 1);

        Self {
            ssh_config: Some(ssh_config_path),
            ssh_keys: HashMap::default(),
        }
    }
}

impl Default for UserInterfaceConfig {
    fn default() -> Self {
        UserInterfaceConfig {
            text_editor: match edit::get_editor() {
                Ok(p) => p,
                Err(_) => PathBuf::from("nano"), // Default to nano
            },
            default_protocol: FileTransferProtocol::Sftp.to_string(),
            show_hidden_files: false,
            check_for_updates: Some(true),
            prompt_on_file_replace: Some(true),
            group_dirs: None,
            file_fmt: None,
            remote_file_fmt: None,
            notifications: Some(true),
            notification_threshold: Some(DEFAULT_NOTIFICATION_TRANSFER_THRESHOLD),
        }
    }
}

// Tests

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_config_mod_new() {
        let mut keys: HashMap<String, PathBuf> = HashMap::with_capacity(1);
        keys.insert(
            String::from("192.168.1.31"),
            PathBuf::from("/tmp/private.key"),
        );
        let remote: RemoteConfig = RemoteConfig {
            ssh_keys: keys,
            ssh_config: Some(String::from("~/.ssh/config")),
        };
        let ui: UserInterfaceConfig = UserInterfaceConfig {
            default_protocol: String::from("SFTP"),
            text_editor: PathBuf::from("nano"),
            show_hidden_files: true,
            check_for_updates: Some(true),
            prompt_on_file_replace: Some(true),
            group_dirs: Some(String::from("first")),
            file_fmt: Some(String::from("{NAME}")),
            remote_file_fmt: Some(String::from("{USER}")),
            notifications: Some(true),
            notification_threshold: Some(DEFAULT_NOTIFICATION_TRANSFER_THRESHOLD),
        };
        assert_eq!(ui.default_protocol, String::from("SFTP"));
        assert_eq!(ui.text_editor, PathBuf::from("nano"));
        assert_eq!(ui.show_hidden_files, true);
        assert_eq!(ui.check_for_updates, Some(true));
        assert_eq!(ui.prompt_on_file_replace, Some(true));
        assert_eq!(ui.group_dirs, Some(String::from("first")));
        assert_eq!(ui.file_fmt, Some(String::from("{NAME}")));
        let cfg: UserConfig = UserConfig {
            user_interface: ui,
            remote,
        };
        assert_eq!(
            *cfg.remote
                .ssh_keys
                .get(&String::from("192.168.1.31"))
                .unwrap(),
            PathBuf::from("/tmp/private.key")
        );
        assert_eq!(
            cfg.remote.ssh_config.as_deref().unwrap(),
            String::from("~/.ssh/config")
        );
        assert_eq!(cfg.user_interface.default_protocol, String::from("SFTP"));
        assert_eq!(cfg.user_interface.text_editor, PathBuf::from("nano"));
        assert_eq!(cfg.user_interface.show_hidden_files, true);
        assert_eq!(cfg.user_interface.check_for_updates, Some(true));
        assert_eq!(cfg.user_interface.prompt_on_file_replace, Some(true));
        assert_eq!(cfg.user_interface.group_dirs, Some(String::from("first")));
        assert_eq!(cfg.user_interface.file_fmt, Some(String::from("{NAME}")));
        assert_eq!(
            cfg.user_interface.remote_file_fmt,
            Some(String::from("{USER}"))
        );
        assert_eq!(cfg.user_interface.notifications, Some(true));
        assert_eq!(
            cfg.user_interface.notification_threshold,
            Some(DEFAULT_NOTIFICATION_TRANSFER_THRESHOLD)
        );
    }
}
