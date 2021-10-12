//! ## Config
//!
//! `config` is the module which provides access to termscp configuration

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// Locals
use crate::filetransfer::FileTransferProtocol;

// Ext
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub const DEFAULT_NOTIFICATION_TRANSFER_THRESHOLD: u64 = 536870912; // 512MB

#[derive(Deserialize, Serialize, std::fmt::Debug)]
/// ## UserConfig
///
/// UserConfig contains all the configurations for the user,
/// supported by termscp
pub struct UserConfig {
    pub user_interface: UserInterfaceConfig,
    pub remote: RemoteConfig,
}

#[derive(Deserialize, Serialize, std::fmt::Debug)]
/// ## UserInterfaceConfig
///
/// UserInterfaceConfig provides all the keys to configure the user interface
pub struct UserInterfaceConfig {
    pub text_editor: PathBuf,
    pub default_protocol: String,
    pub show_hidden_files: bool,
    pub check_for_updates: Option<bool>,      // @! Since 0.3.3
    pub prompt_on_file_replace: Option<bool>, // @! Since 0.7.0; Default True
    pub group_dirs: Option<String>,
    pub file_fmt: Option<String>, // Refers to local host (for backward compatibility)
    pub remote_file_fmt: Option<String>, // @! Since 0.5.0
    pub notifications: Option<bool>, // @! Since 0.7.0; Default true
    pub notification_threshold: Option<u64>, // @! Since 0.7.0; Default 512MB
}

#[derive(Deserialize, Serialize, std::fmt::Debug)]
/// ## RemoteConfig
///
/// Contains configuratio related to remote hosts
pub struct RemoteConfig {
    pub ssh_keys: HashMap<String, PathBuf>, // Association between host name and path to private key
}

impl Default for UserConfig {
    fn default() -> Self {
        UserConfig {
            user_interface: UserInterfaceConfig::default(),
            remote: RemoteConfig::default(),
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

impl Default for RemoteConfig {
    fn default() -> Self {
        RemoteConfig {
            ssh_keys: HashMap::new(),
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
        let remote: RemoteConfig = RemoteConfig { ssh_keys: keys };
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
            remote: remote,
        };
        assert_eq!(
            *cfg.remote
                .ssh_keys
                .get(&String::from("192.168.1.31"))
                .unwrap(),
            PathBuf::from("/tmp/private.key")
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
