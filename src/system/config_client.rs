//! ## ConfigClient
//!
//! `config_client` is the module which provides an API between the Config module and the system

// Locals
use crate::config::{
    params::{UserConfig, DEFAULT_NOTIFICATION_TRANSFER_THRESHOLD},
    serialization::{deserialize, serialize, SerializerError, SerializerErrorKind},
};
use crate::explorer::GroupDirs;
use crate::filetransfer::FileTransferProtocol;
// Ext
use std::fs::{create_dir, remove_file, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::string::ToString;

// Types
pub type SshHost = (String, String, PathBuf); // 0: host, 1: username, 2: RSA key path

/// ConfigClient provides a high level API to communicate with the termscp configuration
pub struct ConfigClient {
    config: UserConfig,   // Configuration loaded
    config_path: PathBuf, // Configuration TOML Path
    ssh_key_dir: PathBuf, // SSH Key storage directory
    degraded: bool,       // Indicates the `ConfigClient` is working in degraded mode
}

impl ConfigClient {
    /// Instantiate a new `ConfigClient` with provided path
    pub fn new(config_path: &Path, ssh_key_dir: &Path) -> Result<Self, SerializerError> {
        // Initialize a default configuration
        let default_config: UserConfig = UserConfig::default();
        info!(
            "Setting up config client with config path {} and SSH key directory {}",
            config_path.display(),
            ssh_key_dir.display()
        );
        // Create client
        let mut client: ConfigClient = ConfigClient {
            config: default_config,
            config_path: PathBuf::from(config_path),
            ssh_key_dir: PathBuf::from(ssh_key_dir),
            degraded: false,
        };
        // If ssh key directory doesn't exist, create it
        if !ssh_key_dir.exists() {
            if let Err(err) = create_dir(ssh_key_dir) {
                error!("Failed to create SSH key dir: {}", err);
                return Err(SerializerError::new_ex(
                    SerializerErrorKind::Io,
                    format!(
                        "Could not create SSH key directory \"{}\": {}",
                        ssh_key_dir.display(),
                        err
                    ),
                ));
            }
            debug!("Created SSH key directory");
        }
        // If Config file doesn't exist, create it
        if !config_path.exists() {
            if let Err(err) = client.write_config() {
                error!("Couldn't create configuration file: {}", err);
                return Err(err);
            }
            debug!("Config file didn't exist; created file");
        } else {
            // otherwise Load configuration from file
            if let Err(err) = client.read_config() {
                error!("Couldn't read configuration file: {}", err);
                return Err(err);
            }
            debug!("Read configuration file");
        }
        Ok(client)
    }

    /// Instantiate a ConfigClient in degraded mode.
    /// When in degraded mode, the configuration in use will be the default configuration
    /// and the IO operation on configuration won't be available
    pub fn degraded() -> Self {
        Self {
            config: UserConfig::default(),
            config_path: PathBuf::default(),
            ssh_key_dir: PathBuf::default(),
            degraded: true,
        }
    }

    // Text editor

    /// Get text editor from configuration
    pub fn get_text_editor(&self) -> PathBuf {
        self.config.user_interface.text_editor.clone()
    }

    /// Set text editor path
    pub fn set_text_editor(&mut self, path: PathBuf) {
        self.config.user_interface.text_editor = path;
    }

    // Default protocol

    /// Get default protocol from configuration
    pub fn get_default_protocol(&self) -> FileTransferProtocol {
        match FileTransferProtocol::from_str(self.config.user_interface.default_protocol.as_str()) {
            Ok(p) => p,
            Err(_) => FileTransferProtocol::Sftp,
        }
    }

    /// Set default protocol to configuration
    pub fn set_default_protocol(&mut self, proto: FileTransferProtocol) {
        self.config.user_interface.default_protocol = proto.to_string();
    }

    /// Get value of `show_hidden_files`
    pub fn get_show_hidden_files(&self) -> bool {
        self.config.user_interface.show_hidden_files
    }

    /// Set new value for `show_hidden_files`
    pub fn set_show_hidden_files(&mut self, value: bool) {
        self.config.user_interface.show_hidden_files = value;
    }

    /// Get value of `check_for_updates`
    pub fn get_check_for_updates(&self) -> bool {
        self.config.user_interface.check_for_updates.unwrap_or(true)
    }

    /// Set new value for `check_for_updates`
    pub fn set_check_for_updates(&mut self, value: bool) {
        self.config.user_interface.check_for_updates = Some(value);
    }

    /// Get value of `prompt_on_file_replace`
    pub fn get_prompt_on_file_replace(&self) -> bool {
        self.config
            .user_interface
            .prompt_on_file_replace
            .unwrap_or(true)
    }

    /// Set new value for `prompt_on_file_replace`
    pub fn set_prompt_on_file_replace(&mut self, value: bool) {
        self.config.user_interface.prompt_on_file_replace = Some(value);
    }

    /// Get GroupDirs value from configuration (will be converted from string)
    pub fn get_group_dirs(&self) -> Option<GroupDirs> {
        // Convert string to `GroupDirs`
        match &self.config.user_interface.group_dirs {
            None => None,
            Some(val) => match GroupDirs::from_str(val.as_str()) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
        }
    }

    /// Set value for group_dir in configuration.
    /// Provided value, if `Some` will be converted to `GroupDirs`
    pub fn set_group_dirs(&mut self, val: Option<GroupDirs>) {
        self.config.user_interface.group_dirs = val.map(|val| val.to_string());
    }

    /// Get current file fmt for local host
    pub fn get_local_file_fmt(&self) -> Option<String> {
        self.config.user_interface.file_fmt.clone()
    }

    /// Set file fmt parameter for local host
    pub fn set_local_file_fmt(&mut self, s: String) {
        self.config.user_interface.file_fmt = match s.is_empty() {
            true => None,
            false => Some(s),
        };
    }

    /// Get current file fmt for remote host
    pub fn get_remote_file_fmt(&self) -> Option<String> {
        self.config.user_interface.remote_file_fmt.clone()
    }

    /// Set file fmt parameter for remote host
    pub fn set_remote_file_fmt(&mut self, s: String) {
        self.config.user_interface.remote_file_fmt = match s.is_empty() {
            true => None,
            false => Some(s),
        };
    }

    /// Get value of `notifications`
    pub fn get_notifications(&self) -> bool {
        self.config.user_interface.notifications.unwrap_or(true)
    }

    /// Set new value for `notifications`
    pub fn set_notifications(&mut self, value: bool) {
        self.config.user_interface.notifications = Some(value);
    }

    /// Get value of `notification_threshold`
    pub fn get_notification_threshold(&self) -> u64 {
        self.config
            .user_interface
            .notification_threshold
            .unwrap_or(DEFAULT_NOTIFICATION_TRANSFER_THRESHOLD)
    }

    /// Set new value for `notification_threshold`
    pub fn set_notification_threshold(&mut self, value: u64) {
        self.config.user_interface.notification_threshold = Some(value);
    }

    // Remote params

    /// Get ssh config path
    pub fn get_ssh_config(&self) -> Option<&str> {
        self.config.remote.ssh_config.as_deref()
    }

    /// Set ssh config path
    pub fn set_ssh_config(&mut self, p: Option<String>) {
        self.config.remote.ssh_config = p;
    }

    // SSH Keys

    /// Save a SSH key into configuration.
    /// This operation also creates the key file in `ssh_key_dir`
    /// and also commits changes to configuration, to prevent incoerent data
    pub fn add_ssh_key(
        &mut self,
        host: &str,
        username: &str,
        ssh_key: &str,
    ) -> Result<(), SerializerError> {
        if self.degraded {
            return Err(SerializerError::new_ex(
                SerializerErrorKind::Generic,
                String::from("Configuration won't be saved, since in degraded mode"),
            ));
        }
        let host_name: String = Self::make_ssh_host_key(host, username);
        // Get key path
        let ssh_key_path: PathBuf = {
            let mut p: PathBuf = self.ssh_key_dir.clone();
            p.push(format!("{host_name}.key"));
            p
        };
        info!(
            "Writing SSH file to {} for host {}",
            ssh_key_path.display(),
            host_name
        );
        // Write key to file
        let mut f: File = match File::create(ssh_key_path.as_path()) {
            Ok(f) => f,
            Err(err) => return Self::make_io_err(err),
        };
        if let Err(err) = f.write_all(ssh_key.as_bytes()) {
            error!("Failed to write SSH key to file: {}", err);
            return Self::make_io_err(err);
        }
        // Add host to keys
        self.config.remote.ssh_keys.insert(host_name, ssh_key_path);
        // Write config
        self.write_config()
    }

    /// Delete a ssh key from configuration, using host as key.
    /// This operation also unlinks the key file in `ssh_key_dir`
    /// and also commits changes to configuration, to prevent incoerent data
    pub fn del_ssh_key(&mut self, host: &str, username: &str) -> Result<(), SerializerError> {
        if self.degraded {
            return Err(SerializerError::new_ex(
                SerializerErrorKind::Generic,
                String::from("Configuration won't be saved, since in degraded mode"),
            ));
        }
        // Remove key from configuration and get key path
        info!("Removing key for {}@{}", host, username);
        let key_path: PathBuf = match self
            .config
            .remote
            .ssh_keys
            .remove(&Self::make_ssh_host_key(host, username))
        {
            Some(p) => p,
            None => return Ok(()), // Return ok if host doesn't exist
        };
        // Remove file
        if let Err(err) = remove_file(key_path.as_path()) {
            error!("Failed to remove key file {}: {}", key_path.display(), err);
            return Self::make_io_err(err);
        }
        // Commit changes to configuration
        self.write_config()
    }

    /// Get ssh key from host.
    /// None is returned if key doesn't exist
    /// `std::io::Error` is returned in case it was not possible to read the key file
    pub fn get_ssh_key(&self, mkey: &str) -> std::io::Result<Option<SshHost>> {
        if self.degraded {
            return Ok(None);
        }
        // Check if Key exists
        match self.config.remote.ssh_keys.get(mkey) {
            None => Ok(None),
            Some(key_path) => {
                // Get host and username
                let (host, username): (String, String) = Self::get_ssh_tokens(mkey);
                // Return key
                Ok(Some((host, username, PathBuf::from(key_path))))
            }
        }
    }

    /// Get an iterator through hosts in the ssh key storage
    pub fn iter_ssh_keys(&self) -> impl Iterator<Item = &String> + '_ {
        Box::new(self.config.remote.ssh_keys.keys())
    }

    // I/O

    /// Write configuration to file
    pub fn write_config(&self) -> Result<(), SerializerError> {
        if self.degraded {
            return Err(SerializerError::new_ex(
                SerializerErrorKind::Generic,
                String::from("Configuration won't be saved, since in degraded mode"),
            ));
        }
        // Open file
        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.config_path.as_path())
        {
            Ok(writer) => serialize(&self.config, Box::new(writer)),
            Err(err) => {
                error!("Failed to write configuration file: {}", err);
                Err(SerializerError::new_ex(
                    SerializerErrorKind::Io,
                    err.to_string(),
                ))
            }
        }
    }

    /// Read configuration from file (or reload it if already read)
    pub fn read_config(&mut self) -> Result<(), SerializerError> {
        if self.degraded {
            return Err(SerializerError::new_ex(
                SerializerErrorKind::Generic,
                String::from("Configuration won't be loaded, since in degraded mode"),
            ));
        }
        // Open bookmarks file for read
        match OpenOptions::new()
            .read(true)
            .open(self.config_path.as_path())
        {
            Ok(reader) => {
                // Deserialize
                match deserialize(Box::new(reader)) {
                    Ok(config) => {
                        self.config = config;
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => {
                error!("Failed to read configuration: {}", err);
                Err(SerializerError::new_ex(
                    SerializerErrorKind::Io,
                    err.to_string(),
                ))
            }
        }
    }

    /// Hosts are saved as `username@host` into configuration.
    /// This method creates the key name, starting from host and username
    fn make_ssh_host_key(host: &str, username: &str) -> String {
        format!("{username}@{host}")
    }

    /// Get ssh tokens starting from ssh host key
    /// Panics if key has invalid syntax
    /// Returns: (host, username)
    fn get_ssh_tokens(host_key: &str) -> (String, String) {
        let tokens: Vec<&str> = host_key.split('@').collect();
        assert!(tokens.len() >= 2);
        (String::from(tokens[1]), String::from(tokens[0]))
    }

    /// Make serializer error from `std::io::Error`
    fn make_io_err(err: std::io::Error) -> Result<(), SerializerError> {
        Err(SerializerError::new_ex(
            SerializerErrorKind::Io,
            err.to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::config::UserConfig;
    use crate::utils::random::random_alphanumeric_with_len;

    use pretty_assertions::assert_eq;
    use std::io::Read;
    use tempfile::TempDir;

    #[test]
    fn test_system_config_new() {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, ssh_keys_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let client: ConfigClient = ConfigClient::new(cfg_path.as_path(), ssh_keys_path.as_path())
            .ok()
            .unwrap();
        // Verify parameters
        let default_config: UserConfig = UserConfig::default();
        assert_eq!(client.degraded, false);
        assert_eq!(client.config.remote.ssh_keys.len(), 0);
        assert_eq!(
            client.config.user_interface.default_protocol,
            default_config.user_interface.default_protocol
        );
        assert_eq!(
            client.config.user_interface.text_editor,
            default_config.user_interface.text_editor
        );
        assert_eq!(client.config_path, cfg_path);
        assert_eq!(client.ssh_key_dir, ssh_keys_path);
    }

    #[test]
    fn test_system_config_degraded() {
        let mut client: ConfigClient = ConfigClient::degraded();
        assert_eq!(client.degraded, true);
        assert_eq!(client.config_path, PathBuf::default());
        assert_eq!(client.ssh_key_dir, PathBuf::default());
        // I/O
        assert!(client.add_ssh_key("Omar", "omar", "omar").is_err());
        assert!(client.del_ssh_key("omar", "omar").is_err());
        assert!(client.get_ssh_key("omar").ok().unwrap().is_none());
        assert!(client.write_config().is_err());
        assert!(client.read_config().is_err());
    }

    #[test]
    fn test_system_config_new_err() {
        assert!(
            ConfigClient::new(Path::new("/tmp/oifoif/omar"), Path::new("/tmp/efnnu/omar"),)
                .is_err()
        );
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, _): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        assert!(ConfigClient::new(cfg_path.as_path(), Path::new("/tmp/efnnu/omar")).is_err());
    }

    #[test]
    fn test_system_config_from_existing() {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        // Change some stuff
        client.set_text_editor(PathBuf::from("/usr/bin/vim"));
        client.set_default_protocol(FileTransferProtocol::Scp);
        assert!(client
            .add_ssh_key("192.168.1.31", "pi", "piroporopero")
            .is_ok());
        assert!(client.write_config().is_ok());
        // Istantiate a new client
        let client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        // Verify client has updated parameters
        assert_eq!(client.get_default_protocol(), FileTransferProtocol::Scp);
        assert_eq!(client.get_text_editor(), PathBuf::from("/usr/bin/vim"));
        let mut expected_key_path: PathBuf = key_path;
        expected_key_path.push("pi@192.168.1.31.key");
        assert_eq!(
            client.get_ssh_key("pi@192.168.1.31").unwrap().unwrap(),
            (
                String::from("192.168.1.31"),
                String::from("pi"),
                expected_key_path,
            )
        );
    }

    #[test]
    fn test_system_config_text_editor() {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        client.set_text_editor(PathBuf::from("mcedit"));
        assert_eq!(client.get_text_editor(), PathBuf::from("mcedit"));
    }

    #[test]
    fn test_system_config_default_protocol() {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        client.set_default_protocol(FileTransferProtocol::Ftp(true));
        assert_eq!(
            client.get_default_protocol(),
            FileTransferProtocol::Ftp(true)
        );
    }

    #[test]
    fn test_system_config_show_hidden_files() {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        client.set_show_hidden_files(true);
        assert_eq!(client.get_show_hidden_files(), true);
    }

    #[test]
    fn test_system_config_check_for_updates() {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        assert_eq!(client.get_check_for_updates(), true); // Null ?
        client.set_check_for_updates(true);
        assert_eq!(client.get_check_for_updates(), true);
        client.set_check_for_updates(false);
        assert_eq!(client.get_check_for_updates(), false);
    }

    #[test]
    fn test_system_config_prompt_on_file_replace() {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        assert_eq!(client.get_prompt_on_file_replace(), true); // Null ?
        client.set_prompt_on_file_replace(true);
        assert_eq!(client.get_prompt_on_file_replace(), true);
        client.set_prompt_on_file_replace(false);
        assert_eq!(client.get_prompt_on_file_replace(), false);
    }

    #[test]
    fn test_system_config_group_dirs() {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        client.set_group_dirs(Some(GroupDirs::First));
        assert_eq!(client.get_group_dirs(), Some(GroupDirs::First),);
        client.set_group_dirs(None);
        assert_eq!(client.get_group_dirs(), None,);
    }

    #[test]
    fn test_system_config_local_file_fmt() {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        assert_eq!(client.get_local_file_fmt(), None);
        client.set_local_file_fmt(String::from("{NAME}"));
        assert_eq!(client.get_local_file_fmt().unwrap(), String::from("{NAME}"));
        // Delete
        client.set_local_file_fmt(String::from(""));
        assert_eq!(client.get_local_file_fmt(), None);
    }

    #[test]
    fn test_system_config_remote_file_fmt() {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        assert_eq!(client.get_remote_file_fmt(), None);
        client.set_remote_file_fmt(String::from("{NAME}"));
        assert_eq!(
            client.get_remote_file_fmt().unwrap(),
            String::from("{NAME}")
        );
        // Delete
        client.set_remote_file_fmt(String::from(""));
        assert_eq!(client.get_remote_file_fmt(), None);
    }

    #[test]
    fn test_system_config_notifications() {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        assert_eq!(client.get_notifications(), true); // Null ?
        client.set_notifications(true);
        assert_eq!(client.get_notifications(), true);
        client.set_notifications(false);
        assert_eq!(client.get_notifications(), false);
    }

    #[test]
    fn test_system_config_remote_notification_threshold() {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        assert_eq!(
            client.get_notification_threshold(),
            DEFAULT_NOTIFICATION_TRANSFER_THRESHOLD
        ); // Null ?
        client.set_notification_threshold(1024);
        assert_eq!(client.get_notification_threshold(), 1024);
        client.set_notification_threshold(64);
        assert_eq!(client.get_notification_threshold(), 64);
    }

    #[test]
    fn test_system_config_remote_ssh_config() {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        assert_eq!(client.get_ssh_config(), None); // Null ?
        client.set_ssh_config(Some(String::from("/tmp/ssh_config")));
        assert_eq!(client.get_ssh_config(), Some("/tmp/ssh_config"));
        client.set_ssh_config(None);
        assert_eq!(client.get_ssh_config(), None);
    }

    #[test]
    fn test_system_config_ssh_keys() {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        // Add a new key
        let rsa_key: String = get_sample_rsa_key();
        assert!(client
            .add_ssh_key("192.168.1.31", "pi", rsa_key.as_str())
            .is_ok());
        // Iterate keys
        for key in client.iter_ssh_keys() {
            let host: SshHost = client.get_ssh_key(key).ok().unwrap().unwrap();
            assert_eq!(host.0, String::from("192.168.1.31"));
            assert_eq!(host.1, String::from("pi"));
            let mut expected_key_path: PathBuf = key_path.clone();
            expected_key_path.push("pi@192.168.1.31.key");
            assert_eq!(host.2, expected_key_path);
            // Read rsa key
            let mut key_file: File = File::open(expected_key_path.as_path()).ok().unwrap();
            // Read
            let mut key: String = String::new();
            assert!(key_file.read_to_string(&mut key).is_ok());
            // Verify rsa key
            assert_eq!(key, rsa_key);
        }
        // Unexisting key
        assert!(client.get_ssh_key("test").ok().unwrap().is_none());
        // Delete key
        assert!(client.del_ssh_key("192.168.1.31", "pi").is_ok());
    }

    #[test]
    fn test_system_config_make_key() {
        assert_eq!(
            ConfigClient::make_ssh_host_key("192.168.1.31", "pi"),
            String::from("pi@192.168.1.31")
        );
        assert_eq!(
            ConfigClient::get_ssh_tokens("pi@192.168.1.31"),
            (String::from("192.168.1.31"), String::from("pi"))
        );
    }

    #[test]
    fn test_system_config_make_io_err() {
        let err: SerializerError =
            ConfigClient::make_io_err(std::io::Error::from(std::io::ErrorKind::PermissionDenied))
                .err()
                .unwrap();
        assert_eq!(err.to_string(), "IO error (permission denied)");
    }

    /// Get paths for configuration and keys directory
    fn get_paths(dir: &Path) -> (PathBuf, PathBuf) {
        let mut k: PathBuf = PathBuf::from(dir);
        let mut c: PathBuf = k.clone();
        k.push("ssh-keys/");
        c.push("config.toml");
        (c, k)
    }

    fn get_sample_rsa_key() -> String {
        format!(
            "-----BEGIN OPENSSH PRIVATE KEY-----\n{}\n-----END OPENSSH PRIVATE KEY-----",
            random_alphanumeric_with_len(2536)
        )
    }
}
