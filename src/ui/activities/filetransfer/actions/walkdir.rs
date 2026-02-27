//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::{Path, PathBuf};

use super::{File, FileTransferActivity};
use crate::ui::activities::filetransfer::lib::walkdir::WalkdirStates;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalkdirError {
    Aborted,
    Error(String),
}

impl FileTransferActivity {
    pub(crate) fn action_walkdir(&mut self) -> Result<Vec<File>, WalkdirError> {
        let mut acc = Vec::with_capacity(32_768);

        let pwd = if self.is_local_tab() {
            self.host_bridge
                .pwd()
                .map_err(|e| WalkdirError::Error(e.to_string()))?
        } else {
            self.client
                .pwd()
                .map_err(|e| WalkdirError::Error(e.to_string()))?
        };

        self.walkdir(&mut acc, &pwd, |activity, path| {
            if activity.is_local_tab() {
                activity
                    .host_bridge
                    .list_dir(path)
                    .map_err(|e| e.to_string())
            } else {
                activity.client.list_dir(path).map_err(|e| e.to_string())
            }
        })?;

        Ok(acc)
    }

    fn walkdir<F>(
        &mut self,
        acc: &mut Vec<File>,
        path: &Path,
        list_dir_fn: F,
    ) -> Result<(), WalkdirError>
    where
        F: Fn(&mut Self, &Path) -> Result<Vec<File>, String> + Copy,
    {
        // init acc if empty
        if acc.is_empty() {
            self.init_walkdir();
        }

        // list current directory
        let dir_entries = list_dir_fn(self, path).map_err(WalkdirError::Error)?;

        // get dirs to scan later
        let dirs = dir_entries
            .iter()
            .filter(|entry| entry.is_dir())
            .map(|entry| entry.path.clone())
            .collect::<Vec<PathBuf>>();

        // extend acc
        acc.extend(dir_entries.clone());
        // update view
        self.update_walkdir_entries(acc.len());

        // check aborted
        self.check_aborted()?;

        for dir in dirs {
            self.walkdir(acc, &dir, list_dir_fn)?;
        }

        Ok(())
    }

    fn check_aborted(&mut self) -> Result<(), WalkdirError> {
        // read events
        self.tick();

        // check if the user wants to abort
        if self.walkdir.aborted {
            return Err(WalkdirError::Aborted);
        }

        Ok(())
    }

    fn init_walkdir(&mut self) {
        self.walkdir = WalkdirStates::default();
    }
}
