use std::path::{Path, PathBuf};

use crate::explorer::FileExplorer;
use crate::host::{HostBridge, HostResult};

/// One side of the dual-pane file browser.
///
/// Both local and remote sides have the same shape:
/// a filesystem client (`HostBridge`) paired with a `FileExplorer`
/// that tracks directory state, sorting, and the transfer queue.
pub struct Pane {
    /// Unified filesystem operations (Localhost or RemoteBridged)
    pub(crate) fs: Box<dyn HostBridge>,
    /// File explorer state (directory listing, sorting, filtering, transfer queue)
    pub(crate) explorer: FileExplorer,
    /// Whether this pane has been connected at least once
    pub(crate) connected: bool,
}

impl Pane {
    /// Create a new Pane.
    pub fn new(fs: Box<dyn HostBridge>, explorer: FileExplorer, connected: bool) -> Self {
        Self {
            fs,
            explorer,
            connected,
        }
    }

    /// Whether the underlying filesystem is connected.
    pub fn is_connected(&mut self) -> bool {
        self.fs.is_connected()
    }

    /// Whether this is a localhost pane.
    pub fn is_localhost(&self) -> bool {
        self.fs.is_localhost()
    }

    /// Connect the filesystem.
    pub fn connect(&mut self) -> HostResult<()> {
        self.fs.connect()?;
        self.connected = true;
        Ok(())
    }

    /// Disconnect the filesystem.
    pub fn disconnect(&mut self) -> HostResult<()> {
        self.fs.disconnect()
    }

    /// Absolutize a relative path against the current working directory.
    pub fn to_abs_path(&self, path: &Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            let mut abs = self.explorer.wrkdir.clone();
            abs.push(path);
            abs
        }
    }
}
