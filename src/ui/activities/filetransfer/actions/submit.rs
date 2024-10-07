//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use super::{File, FileTransferActivity};

enum SubmitAction {
    ChangeDir,
    None,
}

impl FileTransferActivity {
    /// Decides which action to perform on submit for local explorer
    /// Return true whether the directory changed
    pub(crate) fn action_submit_local(&mut self, entry: File) {
        let (action, entry) = if entry.is_dir() {
            (SubmitAction::ChangeDir, entry)
        } else if entry.metadata().symlink.is_some() {
            // Stat file
            let symlink = entry.metadata().symlink.as_ref().unwrap();
            let stat_file = match self.host_bridge.stat(symlink.as_path()) {
                Ok(e) => e,
                Err(err) => {
                    warn!(
                        "Could not stat file pointed by {} ({}): {}",
                        entry.path().display(),
                        symlink.display(),
                        err
                    );
                    entry
                }
            };
            (SubmitAction::ChangeDir, stat_file)
        } else {
            (SubmitAction::None, entry)
        };
        if let (SubmitAction::ChangeDir, entry) = (action, entry) {
            self.action_enter_local_dir(entry)
        }
    }

    /// Decides which action to perform on submit for remote explorer
    /// Return true whether the directory changed
    pub(crate) fn action_submit_remote(&mut self, entry: File) {
        let (action, entry) = if entry.is_dir() {
            (SubmitAction::ChangeDir, entry)
        } else if entry.metadata().symlink.is_some() {
            // Stat file
            let symlink = entry.metadata().symlink.as_ref().unwrap();
            let stat_file = match self.client.stat(symlink.as_path()) {
                Ok(e) => e,
                Err(err) => {
                    warn!(
                        "Could not stat file pointed by {} ({}): {}",
                        entry.path().display(),
                        symlink.display(),
                        err
                    );
                    entry
                }
            };
            (SubmitAction::ChangeDir, stat_file)
        } else {
            (SubmitAction::None, entry)
        };
        if let (SubmitAction::ChangeDir, entry) = (action, entry) {
            self.action_enter_remote_dir(entry)
        }
    }
}
