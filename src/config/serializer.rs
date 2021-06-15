//! ## Serializer
//!
//! `serializer` is the module which provides the serializer/deserializer for configuration

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
use super::{SerializerError, SerializerErrorKind, UserConfig};

use std::io::{Read, Write};

pub struct ConfigSerializer;

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
        trace!("Serialized new configuration data: {}", data);
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
        trace!("Read configuration from file: {}", data);
        // Deserialize
        match toml::de::from_str(data.as_str()) {
            Ok(config) => {
                debug!("Read config from file {:?}", config);
                Ok(config)
            }
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

    use pretty_assertions::assert_eq;
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
        assert!(cfg.is_ok());
        let cfg: UserConfig = cfg.ok().unwrap();
        // Verify configuration
        // Verify ui
        assert_eq!(cfg.user_interface.default_protocol, String::from("SCP"));
        assert_eq!(cfg.user_interface.text_editor, PathBuf::from("vim"));
        assert_eq!(cfg.user_interface.show_hidden_files, true);
        assert_eq!(cfg.user_interface.check_for_updates.unwrap(), true);
        assert_eq!(cfg.user_interface.group_dirs, Some(String::from("last")));
        assert_eq!(
            cfg.user_interface.file_fmt,
            Some(String::from("{NAME} {PEX}"))
        );
        assert_eq!(
            cfg.user_interface.remote_file_fmt,
            Some(String::from("{NAME} {USER}")),
        );
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
        assert!(cfg.is_ok());
        let cfg: UserConfig = cfg.ok().unwrap();
        // Verify configuration
        // Verify ui
        assert_eq!(cfg.user_interface.default_protocol, String::from("SCP"));
        assert_eq!(cfg.user_interface.text_editor, PathBuf::from("vim"));
        assert_eq!(cfg.user_interface.show_hidden_files, true);
        assert_eq!(cfg.user_interface.group_dirs, None);
        assert!(cfg.user_interface.check_for_updates.is_none());
        assert!(cfg.user_interface.file_fmt.is_none());
        assert!(cfg.user_interface.remote_file_fmt.is_none());
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

    #[test]
    fn test_config_serializer_fail_write() {
        let toml_file: tempfile::NamedTempFile = tempfile::NamedTempFile::new().ok().unwrap();
        let writer: Box<dyn Write> = Box::new(std::fs::File::open(toml_file.path()).unwrap());
        // Try to write unexisting file
        let serializer: ConfigSerializer = ConfigSerializer {};
        let cfg: UserConfig = UserConfig::default();
        assert!(serializer.serialize(writer, &cfg).is_err());
    }

    #[test]
    fn test_config_serializer_fail_read() {
        let toml_file: tempfile::NamedTempFile = tempfile::NamedTempFile::new().ok().unwrap();
        let reader: Box<dyn Read> = Box::new(std::fs::File::open(toml_file.path()).unwrap());
        // Try to write unexisting file
        let serializer: ConfigSerializer = ConfigSerializer {};
        assert!(serializer.deserialize(reader).is_err());
    }

    fn create_good_toml() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        let file_content: &str = r#"
        [user_interface]
        default_protocol = "SCP"
        text_editor = "vim"
        show_hidden_files = true
        check_for_updates = true
        group_dirs = "last"
        file_fmt = "{NAME} {PEX}"
        remote_file_fmt = "{NAME} {USER}"

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
