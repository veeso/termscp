use std::path::{Path, PathBuf};

use crate::explorer::FileExplorer;

/// One side of the dual-pane file browser.
///
/// Holds the file explorer state (directory listing, sorting, filtering,
/// transfer queue) and connection tracking. The filesystem client
/// (`HostBridge` / `RemoteFs`) remains on `FileTransferActivity` for now
/// and will be moved here in a future phase.
pub struct Pane {
    /// File explorer state (directory listing, sorting, filtering, transfer queue)
    pub(crate) explorer: FileExplorer,
    /// Whether this pane has been connected at least once
    pub(crate) connected: bool,
}

impl Pane {
    /// Create a new Pane.
    pub fn new(explorer: FileExplorer, connected: bool) -> Self {
        Self {
            explorer,
            connected,
        }
    }

    /// Absolutize a relative path against the current working directory.
    #[allow(dead_code)]
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
