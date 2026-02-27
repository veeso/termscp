//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::PathBuf;

use remotefs::fs::Metadata;

use super::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    /// Create a new empty file via the active tab's pane.
    pub(crate) fn action_newfile(&mut self, input: String) {
        // Check if file exists in current explorer listing
        if self
            .browser
            .explorer()
            .iter_files_all()
            .any(|file| input == file.name())
        {
            self.log_and_alert(LogLevel::Warn, format!("File \"{input}\" already exists"));
            return;
        }

        let file_path = PathBuf::from(input.as_str());
        let writer = match self
            .browser
            .fs_pane_mut()
            .fs
            .create_file(file_path.as_path(), &Metadata::default())
        {
            Ok(f) => f,
            Err(err) => {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not create file \"{}\": {}", file_path.display(), err),
                );
                return;
            }
        };
        // finalize write
        if let Err(err) = self.browser.fs_pane_mut().fs.finalize_write(writer) {
            self.log_and_alert(
                LogLevel::Error,
                format!("Could not write file \"{}\": {}", file_path.display(), err),
            );
            return;
        }

        self.log(
            LogLevel::Info,
            format!("Created file \"{}\"", file_path.display()),
        );
    }
}
