//! ## builder
//!
//! Remotefs client builder

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
use super::params::{AwsS3Params, GenericProtocolParams};
use super::{FileTransferProtocol, ProtocolParams};
use crate::system::config_client::ConfigClient;
use crate::system::sshkey_storage::SshKeyStorage;

use remotefs::RemoteFs;
use remotefs_aws_s3::AwsS3Fs;
use remotefs_ftp::FtpFs;
use remotefs_ssh::{ScpFs, SftpFs, SshOpts};
use std::path::PathBuf;

/// Remotefs builder
pub struct Builder;

impl Builder {
    /// Build RemoteFs client from protocol and params.
    ///
    /// if protocol and parameters are inconsistent, the function will panic.
    pub fn build(
        protocol: FileTransferProtocol,
        params: ProtocolParams,
        config_client: &ConfigClient,
    ) -> Box<dyn RemoteFs> {
        match (protocol, params) {
            (FileTransferProtocol::AwsS3, ProtocolParams::AwsS3(params)) => {
                Box::new(Self::aws_s3_client(params))
            }
            (FileTransferProtocol::Ftp(secure), ProtocolParams::Generic(params)) => {
                Box::new(Self::ftp_client(params, secure))
            }
            (FileTransferProtocol::Scp, ProtocolParams::Generic(params)) => {
                Box::new(Self::scp_client(params, config_client))
            }
            (FileTransferProtocol::Sftp, ProtocolParams::Generic(params)) => {
                Box::new(Self::sftp_client(params, config_client))
            }
            (protocol, params) => {
                error!("Invalid params for protocol '{:?}'", protocol);
                panic!(
                    "Invalid protocol '{:?}' with parameters of type {:?}",
                    protocol, params
                )
            }
        }
    }

    /// Build aws s3 client from parameters
    fn aws_s3_client(params: AwsS3Params) -> AwsS3Fs {
        let mut client = AwsS3Fs::new(params.bucket_name, params.region);
        if let Some(profile) = params.profile {
            client = client.profile(profile);
        }
        client
    }

    /// Build ftp client from parameters
    fn ftp_client(params: GenericProtocolParams, secure: bool) -> FtpFs {
        let mut client = FtpFs::new(params.address, params.port).passive_mode();
        if let Some(username) = params.username {
            client = client.username(username);
        }
        if let Some(password) = params.password {
            client = client.password(password);
        }
        if secure {
            client = client.secure(true, true);
        }
        client
    }

    /// Build scp client
    fn scp_client(params: GenericProtocolParams, config_client: &ConfigClient) -> ScpFs {
        Self::build_ssh_opts(params, config_client).into()
    }

    /// Build sftp client
    fn sftp_client(params: GenericProtocolParams, config_client: &ConfigClient) -> SftpFs {
        Self::build_ssh_opts(params, config_client).into()
    }

    /// Build ssh options from generic protocol params and client configuration
    fn build_ssh_opts(params: GenericProtocolParams, config_client: &ConfigClient) -> SshOpts {
        let mut opts = SshOpts::new(params.address)
            .key_storage(Box::new(Self::make_ssh_storage(config_client)))
            .port(params.port);
        if let Some(username) = params.username {
            opts = opts.username(username);
        }
        if let Some(password) = params.password {
            opts = opts.password(password);
        }
        if let Some(config_path) = config_client.get_ssh_config() {
            opts = opts.config_file(PathBuf::from(config_path));
        }
        opts
    }

    /// Make ssh storage from `ConfigClient` if possible, empty otherwise (empty is implicit if degraded)
    fn make_ssh_storage(config_client: &ConfigClient) -> SshKeyStorage {
        SshKeyStorage::storage_from_config(config_client)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use std::path::{Path, PathBuf};
    use tempfile::TempDir;

    #[test]
    fn should_build_aws_s3_fs() {
        let params = ProtocolParams::AwsS3(AwsS3Params::new("omar", "eu-west-1", Some("test")));
        let config_client = get_config_client();
        let _ = Builder::build(FileTransferProtocol::AwsS3, params, &config_client);
    }

    #[test]
    fn should_build_ftp_fs() {
        let params = ProtocolParams::Generic(
            GenericProtocolParams::default()
                .address("127.0.0.1")
                .port(21)
                .username(Some("omar"))
                .password(Some("qwerty123")),
        );
        let config_client = get_config_client();
        let _ = Builder::build(FileTransferProtocol::Ftp(true), params, &config_client);
    }

    #[test]
    fn should_build_scp_fs() {
        let params = ProtocolParams::Generic(
            GenericProtocolParams::default()
                .address("127.0.0.1")
                .port(22)
                .username(Some("omar"))
                .password(Some("qwerty123")),
        );
        let config_client = get_config_client();
        let _ = Builder::build(FileTransferProtocol::Scp, params, &config_client);
    }

    #[test]
    fn should_build_sftp_fs() {
        let params = ProtocolParams::Generic(
            GenericProtocolParams::default()
                .address("127.0.0.1")
                .port(22)
                .username(Some("omar"))
                .password(Some("qwerty123")),
        );
        let config_client = get_config_client();
        let _ = Builder::build(FileTransferProtocol::Sftp, params, &config_client);
    }

    #[test]
    #[should_panic]
    fn should_not_build_fs() {
        let params = ProtocolParams::Generic(
            GenericProtocolParams::default()
                .address("127.0.0.1")
                .port(22)
                .username(Some("omar"))
                .password(Some("qwerty123")),
        );
        let config_client = get_config_client();
        let _ = Builder::build(FileTransferProtocol::AwsS3, params, &config_client);
    }

    fn get_config_client() -> ConfigClient {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, ssh_keys_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        ConfigClient::new(cfg_path.as_path(), ssh_keys_path.as_path())
            .ok()
            .unwrap()
    }

    /// Get paths for configuration and keys directory
    fn get_paths(dir: &Path) -> (PathBuf, PathBuf) {
        let mut k: PathBuf = PathBuf::from(dir);
        let mut c: PathBuf = k.clone();
        k.push("ssh-keys/");
        c.push("config.toml");
        (c, k)
    }
}
