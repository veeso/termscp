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
// Modules
pub mod serializer;

// Deps
extern crate edit;

// Locals
use crate::filetransfer::FileTransferProtocol;

// Ext
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

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
    pub check_for_updates: Option<bool>, // @! Since 0.3.3
    pub group_dirs: Option<String>,
    pub file_fmt: Option<String>, // Refers to local host (for backward compatibility)
    pub remote_file_fmt: Option<String>, // @! Since 0.5.0
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
            group_dirs: None,
            file_fmt: None,
            remote_file_fmt: None,
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

// Errors

/// ## SerializerError
///
/// Contains the error for serializer/deserializer
#[derive(std::fmt::Debug)]
pub struct SerializerError {
    kind: SerializerErrorKind,
    msg: Option<String>,
}

/// ## SerializerErrorKind
///
/// Describes the kind of error for the serializer/deserializer
#[derive(Error, Debug)]
pub enum SerializerErrorKind {
    #[error("IO error")]
    IoError,
    #[error("Serialization error")]
    SerializationError,
    #[error("Syntax error")]
    SyntaxError,
}

impl SerializerError {
    /// ### new
    ///
    /// Instantiate a new `SerializerError`
    pub fn new(kind: SerializerErrorKind) -> SerializerError {
        SerializerError { kind, msg: None }
    }

    /// ### new_ex
    ///
    /// Instantiates a new `SerializerError` with description message
    pub fn new_ex(kind: SerializerErrorKind, msg: String) -> SerializerError {
        let mut err: SerializerError = SerializerError::new(kind);
        err.msg = Some(msg);
        err
    }
}

impl std::fmt::Display for SerializerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.msg {
            Some(msg) => write!(f, "{} ({})", self.kind, msg),
            None => write!(f, "{}", self.kind),
        }
    }
}

// Tests

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::env;

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
            group_dirs: Some(String::from("first")),
            file_fmt: Some(String::from("{NAME}")),
            remote_file_fmt: Some(String::from("{USER}")),
        };
        assert_eq!(ui.default_protocol, String::from("SFTP"));
        assert_eq!(ui.text_editor, PathBuf::from("nano"));
        assert_eq!(ui.show_hidden_files, true);
        assert_eq!(ui.check_for_updates, Some(true));
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
        assert_eq!(cfg.user_interface.group_dirs, Some(String::from("first")));
        assert_eq!(cfg.user_interface.file_fmt, Some(String::from("{NAME}")));
        assert_eq!(
            cfg.user_interface.remote_file_fmt,
            Some(String::from("{USER}"))
        );
    }

    #[test]
    fn test_config_mod_new_default() {
        // Force vim editor
        env::set_var(String::from("EDITOR"), String::from("vim"));
        // Get default
        let cfg: UserConfig = UserConfig::default();
        assert_eq!(cfg.user_interface.default_protocol, String::from("SFTP"));
        // Text editor
        #[cfg(target_os = "windows")]
        assert_eq!(
            PathBuf::from(cfg.user_interface.text_editor.file_name().unwrap()), // NOTE: since edit 0.1.3 real path is used
            PathBuf::from("vim.EXE")
        );
        #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
        assert_eq!(
            PathBuf::from(cfg.user_interface.text_editor.file_name().unwrap()), // NOTE: since edit 0.1.3 real path is used
            PathBuf::from("vim")
        );
        assert_eq!(cfg.user_interface.check_for_updates.unwrap(), true);
        assert_eq!(cfg.remote.ssh_keys.len(), 0);
        assert!(cfg.user_interface.file_fmt.is_none());
        assert!(cfg.user_interface.remote_file_fmt.is_none());
    }

    #[test]
    fn test_config_mod_errors() {
        let error: SerializerError = SerializerError::new(SerializerErrorKind::SyntaxError);
        assert!(error.msg.is_none());
        assert_eq!(format!("{}", error), String::from("Syntax error"));
        let error: SerializerError =
            SerializerError::new_ex(SerializerErrorKind::SyntaxError, String::from("bad syntax"));
        assert!(error.msg.is_some());
        assert_eq!(
            format!("{}", error),
            String::from("Syntax error (bad syntax)")
        );
        // Fmt
        assert_eq!(
            format!("{}", SerializerError::new(SerializerErrorKind::IoError)),
            String::from("IO error")
        );
        assert_eq!(
            format!(
                "{}",
                SerializerError::new(SerializerErrorKind::SerializationError)
            ),
            String::from("Serialization error")
        );
    }
}
