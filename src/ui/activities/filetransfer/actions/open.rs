//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use super::{File, FileTransferActivity, LogLevel, SelectedFile, TransferPayload};
// ext
use std::path::{Path, PathBuf};

impl FileTransferActivity {
    /// Open local file
    pub(crate) fn action_open_local(&mut self) {
        let entries: Vec<File> = match self.get_local_selected_entries() {
            SelectedFile::One(entry) => vec![entry],
            SelectedFile::Many(entries) => entries,
            SelectedFile::None => vec![],
        };
        entries
            .iter()
            .for_each(|x| self.action_open_local_file(x, None));
    }

    /// Open local file
    pub(crate) fn action_open_remote(&mut self) {
        let entries: Vec<File> = match self.get_remote_selected_entries() {
            SelectedFile::One(entry) => vec![entry],
            SelectedFile::Many(entries) => entries,
            SelectedFile::None => vec![],
        };
        entries
            .iter()
            .for_each(|x| self.action_open_remote_file(x, None));
    }

    /// Perform open lopcal file
    pub(crate) fn action_open_local_file(&mut self, entry: &File, open_with: Option<&str>) {
        self.open_path_with(entry.path(), open_with);
    }

    /// Open remote file. The file is first downloaded to a temporary directory on localhost
    pub(crate) fn action_open_remote_file(&mut self, entry: &File, open_with: Option<&str>) {
        // Download file
        let tmpfile: String =
            match self.get_cache_tmp_name(&entry.name(), entry.extension().as_deref()) {
                None => {
                    self.log(LogLevel::Error, String::from("Could not create tempdir"));
                    return;
                }
                Some(p) => p,
            };
        let cache: PathBuf = match self.cache.as_ref() {
            None => {
                self.log(LogLevel::Error, String::from("Could not create tempdir"));
                return;
            }
            Some(p) => p.path().to_path_buf(),
        };
        match self.filetransfer_recv(
            TransferPayload::Any(entry.clone()),
            cache.as_path(),
            Some(tmpfile.clone()),
        ) {
            Ok(_) => {
                // Make file and open if file exists
                let mut tmp: PathBuf = cache;
                tmp.push(tmpfile.as_str());
                if tmp.exists() {
                    self.open_path_with(tmp.as_path(), open_with);
                }
            }
            Err(err) => {
                self.log(
                    LogLevel::Error,
                    format!("Failed to download remote entry: {err}"),
                );
            }
        }
    }

    /// Open selected file with provided application
    pub(crate) fn action_local_open_with(&mut self, with: &str) {
        let entries: Vec<File> = match self.get_local_selected_entries() {
            SelectedFile::One(entry) => vec![entry],
            SelectedFile::Many(entries) => entries,
            SelectedFile::None => vec![],
        };
        // Open all entries
        entries
            .iter()
            .for_each(|x| self.action_open_local_file(x, Some(with)));
    }

    /// Open selected file with provided application
    pub(crate) fn action_remote_open_with(&mut self, with: &str) {
        let entries: Vec<File> = match self.get_remote_selected_entries() {
            SelectedFile::One(entry) => vec![entry],
            SelectedFile::Many(entries) => entries,
            SelectedFile::None => vec![],
        };
        // Open all entries
        entries
            .iter()
            .for_each(|x| self.action_open_remote_file(x, Some(with)));
    }

    /// Common function which opens a path with default or specified program.
    fn open_path_with(&mut self, p: &Path, with: Option<&str>) {
        // Open file
        let result = match with {
            None => open::that(p),
            Some(with) => open::with(p, with),
        };
        // Log result
        match result {
            Ok(_) => self.log(LogLevel::Info, format!("Opened file `{}`", p.display())),
            Err(err) => self.log(
                LogLevel::Error,
                format!("Failed to open filoe `{}`: {}", p.display(), err),
            ),
        }
        // NOTE: clear screen in order to prevent crap on stderr
        if let Some(ctx) = self.context.as_mut() {
            // Clear screen
            if let Err(err) = ctx.terminal().clear_screen() {
                error!("Could not clear screen screen: {}", err);
            }
        }
    }
}
