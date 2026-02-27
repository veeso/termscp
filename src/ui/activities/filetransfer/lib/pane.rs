use crate::explorer::FileExplorer;
use crate::host::HostBridge;

/// One side of the dual-pane file browser.
///
/// Holds the file explorer state, connection tracking, and the filesystem
/// client (`HostBridge`) used to interact with the underlying storage
/// (local or remote).
pub struct Pane {
    /// File explorer state (directory listing, sorting, filtering, transfer queue)
    pub(crate) explorer: FileExplorer,
    /// Whether this pane has been connected at least once
    pub(crate) connected: bool,
    /// Filesystem client for this pane
    pub(crate) fs: Box<dyn HostBridge>,
}

impl Pane {
    /// Create a new Pane.
    pub fn new(explorer: FileExplorer, connected: bool, fs: Box<dyn HostBridge>) -> Self {
        Self {
            explorer,
            connected,
            fs,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::explorer::builder::FileExplorerBuilder;
    use crate::host::Localhost;

    fn make_pane() -> Pane {
        let wrkdir = std::env::temp_dir();
        let explorer = FileExplorerBuilder::new().build();
        let fs = Localhost::new(wrkdir).unwrap();
        Pane::new(explorer, false, Box::new(fs))
    }

    #[test]
    fn test_pane_new() {
        let pane = make_pane();
        assert!(!pane.connected);
        assert!(pane.fs.is_localhost());
    }

    #[test]
    fn test_pane_pwd() {
        let mut pane = make_pane();
        let pwd = pane.fs.pwd().unwrap();
        assert_eq!(pwd, PathBuf::from(std::env::temp_dir()));
    }
}
