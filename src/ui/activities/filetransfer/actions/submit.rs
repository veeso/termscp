//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use super::{File, FileTransferActivity};

enum SubmitAction {
    ChangeDir,
    None,
}

impl FileTransferActivity {
    /// Decides which action to perform on submit, dispatching via the active tab's pane.
    pub(crate) fn action_submit(&mut self, entry: File) {
        let (action, entry) = if entry.is_dir() {
            (SubmitAction::ChangeDir, entry)
        } else if entry.metadata().symlink.is_some() {
            // Stat symlink target via the active pane
            let symlink = entry.metadata().symlink.as_ref().unwrap();
            match self.browser.fs_pane_mut().fs.stat(symlink.as_path()) {
                Ok(e) => (SubmitAction::ChangeDir, e),
                Err(err) => {
                    warn!(
                        "Could not stat file pointed by {} ({}): {}",
                        entry.path().display(),
                        symlink.display(),
                        err
                    );
                    (SubmitAction::ChangeDir, entry)
                }
            }
        } else {
            (SubmitAction::None, entry)
        };
        if let (SubmitAction::ChangeDir, entry) = (action, entry) {
            self.action_enter_dir(entry)
        }
    }
}
