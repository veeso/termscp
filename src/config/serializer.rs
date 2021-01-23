//! ## Serializer
//!
//! `serializer` is the module which provides the serializer/deserializer for configuration

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

use super::{SerializerError, SerializerErrorKind, UserConfig};

use std::io::{Read, Write};

pub struct ConfigSerializer {}

impl ConfigSerializer {
    /// ### serialize
    ///
    /// Serialize `UserConfig` into TOML and write content to writable
    pub fn serialize(
        &self,
        mut writable: Box<dyn Write>,
        cfg: &UserConfig,
    ) -> Result<(), SerializerError> {
        // Serialize content
        let data: String = match toml::ser::to_string(cfg) {
            Ok(dt) => dt,
            Err(err) => {
                return Err(SerializerError::new_ex(
                    SerializerErrorKind::SerializationError,
                    err.to_string(),
                ))
            }
        };
        // Write file
        match writable.write_all(data.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(SerializerError::new_ex(
                SerializerErrorKind::IoError,
                err.to_string(),
            )),
        }
    }

    /// ### deserialize
    ///
    /// Read data from readable and deserialize its content as TOML
    pub fn deserialize(&self, mut readable: Box<dyn Read>) -> Result<UserConfig, SerializerError> {
        // Read file content
        let mut data: String = String::new();
        if let Err(err) = readable.read_to_string(&mut data) {
            return Err(SerializerError::new_ex(
                SerializerErrorKind::IoError,
                err.to_string(),
            ));
        }
        // Deserialize
        match toml::de::from_str(data.as_str()) {
            Ok(hosts) => Ok(hosts),
            Err(err) => Err(SerializerError::new_ex(
                SerializerErrorKind::SyntaxError,
                err.to_string(),
            )),
        }
    }
}

// Tests

#[cfg(test)]
mod tests {

    use super::*;

    use std::io::{Seek, SeekFrom};
    use std::path::PathBuf;

    #[test]
    fn test_config_serializer_deserialize_ok() {
        let toml_file: tempfile::NamedTempFile = create_good_toml();
        toml_file.as_file().sync_all().unwrap();
        toml_file.as_file().seek(SeekFrom::Start(0)).unwrap();
        // Parse
        let deserializer: ConfigSerializer = ConfigSerializer {};
        let cfg = deserializer.deserialize(Box::new(toml_file));
        println!("{:?}", cfg);
        assert!(cfg.is_ok());
        let cfg: UserConfig = cfg.ok().unwrap();
        // Verify configuration
        // Verify ui
        assert_eq!(cfg.user_interface.default_protocol, String::from("SCP"));
        assert_eq!(cfg.user_interface.text_editor, PathBuf::from("vim"));
        assert_eq!(cfg.user_interface.show_hidden_files, true);
        assert_eq!(cfg.user_interface.group_dirs, Some(String::from("last")));
        assert_eq!(cfg.user_interface.file_fmt, Some(String::from("{NAME} {PEX}")));
        // Verify keys
        assert_eq!(
            *cfg.remote
                .ssh_keys
                .get(&String::from("192.168.1.31"))
                .unwrap(),
            PathBuf::from("/home/omar/.ssh/raspberry.key")
        );
        assert_eq!(
            *cfg.remote
                .ssh_keys
                .get(&String::from("192.168.1.32"))
                .unwrap(),
            PathBuf::from("/home/omar/.ssh/beaglebone.key")
        );
        assert!(cfg.remote.ssh_keys.get(&String::from("1.1.1.1")).is_none());
    }

    #[test]
    fn test_config_serializer_deserialize_ok_no_opts() {
        let toml_file: tempfile::NamedTempFile = create_good_toml_no_opts();
        toml_file.as_file().sync_all().unwrap();
        toml_file.as_file().seek(SeekFrom::Start(0)).unwrap();
        // Parse
        let deserializer: ConfigSerializer = ConfigSerializer {};
        let cfg = deserializer.deserialize(Box::new(toml_file));
        println!("{:?}", cfg);
        assert!(cfg.is_ok());
        let cfg: UserConfig = cfg.ok().unwrap();
        // Verify configuration
        // Verify ui
        assert_eq!(cfg.user_interface.default_protocol, String::from("SCP"));
        assert_eq!(cfg.user_interface.text_editor, PathBuf::from("vim"));
        assert_eq!(cfg.user_interface.show_hidden_files, true);
        assert_eq!(cfg.user_interface.group_dirs, None);
        assert_eq!(cfg.user_interface.file_fmt, None);
        // Verify keys
        assert_eq!(
            *cfg.remote
                .ssh_keys
                .get(&String::from("192.168.1.31"))
                .unwrap(),
            PathBuf::from("/home/omar/.ssh/raspberry.key")
        );
        assert_eq!(
            *cfg.remote
                .ssh_keys
                .get(&String::from("192.168.1.32"))
                .unwrap(),
            PathBuf::from("/home/omar/.ssh/beaglebone.key")
        );
        assert!(cfg.remote.ssh_keys.get(&String::from("1.1.1.1")).is_none());
    }

    #[test]
    fn test_config_serializer_deserialize_nok() {
        let toml_file: tempfile::NamedTempFile = create_bad_toml();
        toml_file.as_file().sync_all().unwrap();
        toml_file.as_file().seek(SeekFrom::Start(0)).unwrap();
        // Parse
        let deserializer: ConfigSerializer = ConfigSerializer {};
        assert!(deserializer.deserialize(Box::new(toml_file)).is_err());
    }

    #[test]
    fn test_config_serializer_serialize() {
        let mut cfg: UserConfig = UserConfig::default();
        let toml_file: tempfile::NamedTempFile = tempfile::NamedTempFile::new().ok().unwrap();
        // Insert key
        cfg.remote.ssh_keys.insert(
            String::from("192.168.1.31"),
            PathBuf::from("/home/omar/.ssh/id_rsa"),
        );
        // Serialize
        let serializer: ConfigSerializer = ConfigSerializer {};
        let writer: Box<dyn Write> = Box::new(std::fs::File::create(toml_file.path()).unwrap());
        assert!(serializer.serialize(writer, &cfg).is_ok());
        // Reload configuration and check if it's ok
        toml_file.as_file().sync_all().unwrap();
        toml_file.as_file().seek(SeekFrom::Start(0)).unwrap();
        assert!(serializer.deserialize(Box::new(toml_file)).is_ok());
    }

    fn create_good_toml() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        let file_content: &str = r#"
        [user_interface]
        default_protocol = "SCP"
        text_editor = "vim"
        show_hidden_files = true
        group_dirs = "last"
        file_fmt = "{NAME} {PEX}"

        [remote.ssh_keys]
        "192.168.1.31" = "/home/omar/.ssh/raspberry.key"
        "192.168.1.32" = "/home/omar/.ssh/beaglebone.key"
        "#;
        tmpfile.write_all(file_content.as_bytes()).unwrap();
        tmpfile
    }

    fn create_good_toml_no_opts() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        let file_content: &str = r#"
        [user_interface]
        default_protocol = "SCP"
        text_editor = "vim"
        show_hidden_files = true

        [remote.ssh_keys]
        "192.168.1.31" = "/home/omar/.ssh/raspberry.key"
        "192.168.1.32" = "/home/omar/.ssh/beaglebone.key"
        "#;
        tmpfile.write_all(file_content.as_bytes()).unwrap();
        tmpfile
    }

    fn create_bad_toml() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        let file_content: &str = r#"
        [user_interface]
        default_protocol = "SFTP"

        [remote.ssh_keys]
        "192.168.1.31" = "/home/omar/.ssh/raspberry.key"
        "#;
        tmpfile.write_all(file_content.as_bytes()).unwrap();
        tmpfile
    }
}
