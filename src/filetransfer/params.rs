//! ## Params
//!
//! file transfer parameters

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
use super::FileTransferProtocol;

use std::path::{Path, PathBuf};

/// ### FileTransferParams
///
/// Holds connection parameters for file transfers
#[derive(Clone)]
pub struct FileTransferParams {
    pub address: String,
    pub port: u16,
    pub protocol: FileTransferProtocol,
    pub username: Option<String>,
    pub password: Option<String>,
    pub entry_directory: Option<PathBuf>,
}

impl FileTransferParams {
    /// ### new
    ///
    /// Instantiates a new `FileTransferParams`
    pub fn new<S: AsRef<str>>(address: S) -> Self {
        Self {
            address: address.as_ref().to_string(),
            port: 22,
            protocol: FileTransferProtocol::Sftp,
            username: None,
            password: None,
            entry_directory: None,
        }
    }

    /// ### port
    ///
    /// Set port for params
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// ### protocol
    ///
    /// Set protocol for params
    pub fn protocol(mut self, protocol: FileTransferProtocol) -> Self {
        self.protocol = protocol;
        self
    }

    /// ### username
    ///
    /// Set username for params
    pub fn username<S: AsRef<str>>(mut self, username: Option<S>) -> Self {
        self.username = username.map(|x| x.as_ref().to_string());
        self
    }

    /// ### password
    ///
    /// Set password for params
    pub fn password<S: AsRef<str>>(mut self, password: Option<S>) -> Self {
        self.password = password.map(|x| x.as_ref().to_string());
        self
    }

    /// ### entry_directory
    ///
    /// Set entry directory
    pub fn entry_directory<P: AsRef<Path>>(mut self, dir: Option<P>) -> Self {
        self.entry_directory = dir.map(|x| x.as_ref().to_path_buf());
        self
    }
}

impl Default for FileTransferParams {
    fn default() -> Self {
        Self::new("localhost")
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_filetransfer_params() {
        let params: FileTransferParams = FileTransferParams::new("test.rebex.net")
            .port(2222)
            .protocol(FileTransferProtocol::Scp)
            .username(Some("omar"))
            .password(Some("foobar"))
            .entry_directory(Some(&Path::new("/tmp")));
        assert_eq!(params.address.as_str(), "test.rebex.net");
        assert_eq!(params.port, 2222);
        assert_eq!(params.protocol, FileTransferProtocol::Scp);
        assert_eq!(params.username.as_ref().unwrap(), "omar");
        assert_eq!(params.password.as_ref().unwrap(), "foobar");
    }

    #[test]
    fn test_filetransfer_params_default() {
        let params: FileTransferParams = FileTransferParams::default();
        assert_eq!(params.address.as_str(), "localhost");
        assert_eq!(params.port, 22);
        assert_eq!(params.protocol, FileTransferProtocol::Sftp);
        assert!(params.username.is_none());
        assert!(params.password.is_none());
    }
}
