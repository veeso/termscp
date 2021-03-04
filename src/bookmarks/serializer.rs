//! ## Serializer
//!
//! `serializer` is the module which provides the serializer/deserializer for bookmarks

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

use super::{SerializerError, SerializerErrorKind, UserHosts};

use std::io::{Read, Write};

pub struct BookmarkSerializer;

impl BookmarkSerializer {
    /// ### serialize
    ///
    /// Serialize `UserHosts` into TOML and write content to writable
    pub fn serialize(
        &self,
        mut writable: Box<dyn Write>,
        hosts: &UserHosts,
    ) -> Result<(), SerializerError> {
        // Serialize content
        let data: String = match toml::ser::to_string(hosts) {
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
    pub fn deserialize(&self, mut readable: Box<dyn Read>) -> Result<UserHosts, SerializerError> {
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

    use super::super::Bookmark;
    use super::*;

    use std::collections::HashMap;
    use std::io::{Seek, SeekFrom};

    #[test]
    fn test_bookmarks_serializer_deserialize_ok() {
        let toml_file: tempfile::NamedTempFile = create_good_toml();
        toml_file.as_file().sync_all().unwrap();
        toml_file.as_file().seek(SeekFrom::Start(0)).unwrap();
        // Parse
        let deserializer: BookmarkSerializer = BookmarkSerializer {};
        let hosts = deserializer.deserialize(Box::new(toml_file));
        assert!(hosts.is_ok());
        let hosts: UserHosts = hosts.ok().unwrap();
        // Verify hosts
        // Verify recents
        assert_eq!(hosts.recents.len(), 1);
        let host: &Bookmark = hosts.recents.get("ISO20201215T094000Z").unwrap();
        assert_eq!(host.address, String::from("172.16.104.10"));
        assert_eq!(host.port, 22);
        assert_eq!(host.protocol, String::from("SCP"));
        assert_eq!(host.username, String::from("root"));
        assert_eq!(host.password, None);
        // Verify bookmarks
        assert_eq!(hosts.bookmarks.len(), 3);
        let host: &Bookmark = hosts.bookmarks.get("raspberrypi2").unwrap();
        assert_eq!(host.address, String::from("192.168.1.31"));
        assert_eq!(host.port, 22);
        assert_eq!(host.protocol, String::from("SFTP"));
        assert_eq!(host.username, String::from("root"));
        assert_eq!(*host.password.as_ref().unwrap(), String::from("mypassword"));
        let host: &Bookmark = hosts.bookmarks.get("msi-estrem").unwrap();
        assert_eq!(host.address, String::from("192.168.1.30"));
        assert_eq!(host.port, 22);
        assert_eq!(host.protocol, String::from("SFTP"));
        assert_eq!(host.username, String::from("cvisintin"));
        assert_eq!(*host.password.as_ref().unwrap(), String::from("mysecret"));
        let host: &Bookmark = hosts.bookmarks.get("aws-server-prod1").unwrap();
        assert_eq!(host.address, String::from("51.23.67.12"));
        assert_eq!(host.port, 21);
        assert_eq!(host.protocol, String::from("FTPS"));
        assert_eq!(host.username, String::from("aws001"));
        assert_eq!(host.password, None);
    }

    #[test]
    fn test_bookmarks_serializer_deserialize_nok() {
        let toml_file: tempfile::NamedTempFile = create_bad_toml();
        toml_file.as_file().sync_all().unwrap();
        toml_file.as_file().seek(SeekFrom::Start(0)).unwrap();
        // Parse
        let deserializer: BookmarkSerializer = BookmarkSerializer {};
        assert!(deserializer.deserialize(Box::new(toml_file)).is_err());
    }

    #[test]
    fn test_bookmarks_serializer_serialize() {
        let mut bookmarks: HashMap<String, Bookmark> = HashMap::with_capacity(2);
        // Push two samples
        bookmarks.insert(
            String::from("raspberrypi2"),
            Bookmark {
                address: String::from("192.168.1.31"),
                port: 22,
                protocol: String::from("SFTP"),
                username: String::from("root"),
                password: None,
            },
        );
        bookmarks.insert(
            String::from("msi-estrem"),
            Bookmark {
                address: String::from("192.168.1.30"),
                port: 4022,
                protocol: String::from("SFTP"),
                username: String::from("cvisintin"),
                password: Some(String::from("password")),
            },
        );
        let mut recents: HashMap<String, Bookmark> = HashMap::with_capacity(1);
        recents.insert(
            String::from("ISO20201215T094000Z"),
            Bookmark {
                address: String::from("192.168.1.254"),
                port: 3022,
                protocol: String::from("SCP"),
                username: String::from("omar"),
                password: Some(String::from("aaa")),
            },
        );
        let tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        // Serialize
        let deserializer: BookmarkSerializer = BookmarkSerializer {};
        let hosts: UserHosts = UserHosts { bookmarks, recents };
        assert!(deserializer.serialize(Box::new(tmpfile), &hosts).is_ok());
    }

    fn create_good_toml() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        let file_content: &str = r#"
        [bookmarks]
        raspberrypi2 = { address = "192.168.1.31", port = 22, protocol = "SFTP", username = "root", password = "mypassword" }
        msi-estrem = { address = "192.168.1.30", port = 22, protocol = "SFTP", username = "cvisintin", password = "mysecret" }
        aws-server-prod1 = { address = "51.23.67.12", port = 21, protocol = "FTPS", username = "aws001" }

        [recents]
        ISO20201215T094000Z = { address = "172.16.104.10", port = 22, protocol = "SCP", username = "root" }
        "#;
        tmpfile.write_all(file_content.as_bytes()).unwrap();
        //write!(tmpfile, "[bookmarks]\nraspberrypi2 = {{ address = \"192.168.1.31\", port = 22, protocol = \"SFTP\", username = \"root\" }}\nmsi-estrem = {{ address = \"192.168.1.30\", port = 22, protocol = \"SFTP\", username = \"cvisintin\" }}\naws-server-prod1 = {{ address = \"51.23.67.12\", port = 21, protocol = \"FTPS\", username = \"aws001\" }}\n\n[recents]\nISO20201215T094000Z = {{ address = \"172.16.104.10\", port = 22, protocol = \"SCP\", username = \"root\" }}\n");
        tmpfile
    }

    fn create_bad_toml() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        let file_content: &str = r#"
        [bookmarks]
        raspberrypi2 = { address = "192.168.1.31", port = 22, protocol = "SFTP", username = "root"}
        msi-estrem = { address = "192.168.1.30", port = 22, protocol = "SFTP" }
        aws-server-prod1 = { address = "51.23.67.12", port = 21, protocol = "FTPS", username = "aws001" }

        [recents]
        ISO20201215T094000Z = { address = "172.16.104.10", protocol = "SCP", username = "root", port = 22 }
        "#;
        tmpfile.write_all(file_content.as_bytes()).unwrap();
        tmpfile
    }
}
