use std::path::Path;

use super::{File, FileTransferActivity};
use crate::ui::activities::filetransfer::lib::browser::FileExplorerTab;

impl FileTransferActivity {
    pub(crate) fn action_scan(&mut self, p: &Path) -> Result<Vec<File>, String> {
        match self.browser.tab() {
            FileExplorerTab::Local | FileExplorerTab::FindLocal => self
                .host
                .list_dir(p)
                .map_err(|e| format!("Failed to list directory: {}", e)),
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => self
                .client
                .list_dir(p)
                .map_err(|e| format!("Failed to list directory: {}", e)),
        }
    }
}
