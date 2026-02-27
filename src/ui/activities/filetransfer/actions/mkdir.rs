//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::PathBuf;

use super::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    /// Create a directory via the active tab's pane.
    pub(crate) fn action_mkdir(&mut self, input: String) {
        let path = PathBuf::from(input.as_str());
        match self.browser.fs_pane_mut().fs.mkdir(path.as_path()) {
            Ok(_) => self.log(LogLevel::Info, format!("Created directory \"{input}\"")),
            Err(err) => self.log_and_alert(
                LogLevel::Error,
                format!("Could not create directory \"{input}\": {err}"),
            ),
        }
    }

    /// Create a directory on the local host (used by sync-browsing in change_dir).
    pub(in crate::ui::activities::filetransfer) fn action_local_mkdir(&mut self, input: String) {
        let path = PathBuf::from(input.as_str());
        match self.browser.local_pane_mut().fs.mkdir(path.as_path()) {
            Ok(_) => self.log(LogLevel::Info, format!("Created directory \"{input}\"")),
            Err(err) => self.log_and_alert(
                LogLevel::Error,
                format!("Could not create directory \"{input}\": {err}"),
            ),
        }
    }

    /// Create a directory on the remote host (used by sync-browsing in change_dir).
    pub(in crate::ui::activities::filetransfer) fn action_remote_mkdir(&mut self, input: String) {
        let path = PathBuf::from(input.as_str());
        match self.browser.remote_pane_mut().fs.mkdir(path.as_path()) {
            Ok(_) => self.log(LogLevel::Info, format!("Created directory \"{input}\"")),
            Err(err) => self.log_and_alert(
                LogLevel::Error,
                format!("Could not create directory \"{input}\": {err}"),
            ),
        }
    }
}
