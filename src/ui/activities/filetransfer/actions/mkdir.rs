//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::PathBuf;

use remotefs::fs::UnixPex;

use super::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    pub(crate) fn action_mkdir(&mut self, input: String) {
        let path = PathBuf::from(input.as_str());
        let result: Result<(), String> = if self.is_local_tab() {
            self.host_bridge
                .mkdir(path.as_path())
                .map_err(|e| e.to_string())
        } else {
            self.client
                .as_mut()
                .create_dir(path.as_path(), UnixPex::from(0o755))
                .map_err(|e| e.to_string())
        };
        match result {
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
        match self.host_bridge.mkdir(path.as_path()) {
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
        match self
            .client
            .as_mut()
            .create_dir(path.as_path(), UnixPex::from(0o755))
        {
            Ok(_) => self.log(LogLevel::Info, format!("Created directory \"{input}\"")),
            Err(err) => self.log_and_alert(
                LogLevel::Error,
                format!("Could not create directory \"{input}\": {err}"),
            ),
        }
    }
}
