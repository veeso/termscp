use std::path::Path;

use super::{File, FileTransferActivity};

impl FileTransferActivity {
    /// List directory contents via the active tab's pane.
    pub(crate) fn action_scan(&mut self, p: &Path) -> Result<Vec<File>, String> {
        self.browser
            .fs_pane_mut()
            .fs
            .list_dir(p)
            .map_err(|e| format!("Failed to list directory: {}", e))
    }
}
