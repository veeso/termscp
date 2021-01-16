//! ## Config
//!
//! `config` is the module which provides access to termscp configuration

/*
*
*   Copyright (C) 2020-2021 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "TermSCP"
*
*   TermSCP is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   TermSCP is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with TermSCP.  If not, see <http://www.gnu.org/licenses/>.
*
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
    pub group_dirs: Option<String>,
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
            group_dirs: None,
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
#[derive(std::fmt::Debug, PartialEq)]
pub enum SerializerErrorKind {
    IoError,
    SerializationError,
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
        let err: String = match &self.kind {
            SerializerErrorKind::IoError => String::from("IO error"),
            SerializerErrorKind::SerializationError => String::from("Serialization error"),
            SerializerErrorKind::SyntaxError => String::from("Syntax error"),
        };
        match &self.msg {
            Some(msg) => write!(f, "{} ({})", err, msg),
            None => write!(f, "{}", err),
        }
    }
}

// Tests

#[cfg(test)]
mod tests {

    use super::*;
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
            group_dirs: Some(String::from("first")),
        };
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
        assert_eq!(cfg.user_interface.group_dirs, Some(String::from("first")));
    }

    #[test]
    fn test_config_mod_new_default() {
        // Force vim editor
        env::set_var(String::from("EDITOR"), String::from("vim"));
        // Get default
        let cfg: UserConfig = UserConfig::default();
        assert_eq!(cfg.user_interface.default_protocol, String::from("SFTP"));
        assert_eq!(cfg.user_interface.text_editor, PathBuf::from("vim"));
        assert_eq!(cfg.remote.ssh_keys.len(), 0);
    }

    #[test]
    fn test_config_mod_errors() {
        let error: SerializerError = SerializerError::new(SerializerErrorKind::SyntaxError);
        assert_eq!(error.kind, SerializerErrorKind::SyntaxError);
        assert!(error.msg.is_none());
        assert_eq!(format!("{}", error), String::from("Syntax error"));
        let error: SerializerError =
            SerializerError::new_ex(SerializerErrorKind::SyntaxError, String::from("bad syntax"));
        assert_eq!(error.kind, SerializerErrorKind::SyntaxError);
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
