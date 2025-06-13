//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
// ext
use std::path::{Path, PathBuf};

use super::{File, FileTransferActivity, LogLevel, SelectedFile, TransferPayload};

impl FileTransferActivity {
    /// Open local file
    pub(crate) fn action_open_local(&mut self) {
        let entries: Vec<File> = match self.get_local_selected_entries() {
            SelectedFile::One(entry) => vec![entry],
            SelectedFile::Many(entries) => entries.into_iter().map(|(f, _)| f).collect(),
            SelectedFile::None => vec![],
        };
        entries
            .iter()
            .for_each(|x| self.action_open_local_file(x, None));

        // clear selection
        self.host_bridge_mut().clear_queue();
        self.reload_host_bridge_filelist();
    }

    /// Open local file
    pub(crate) fn action_open_remote(&mut self) {
        let entries: Vec<File> = match self.get_remote_selected_entries() {
            SelectedFile::One(entry) => vec![entry],
            SelectedFile::Many(entries) => entries.into_iter().map(|(f, _)| f).collect(),
            SelectedFile::None => vec![],
        };
        entries
            .iter()
            .for_each(|x| self.action_open_remote_file(x, None));

        // clear selection
        self.remote_mut().clear_queue();
        self.reload_remote_filelist();
    }

    /// Perform open lopcal file
    pub(crate) fn action_open_local_file(&mut self, entry: &File, open_with: Option<&str>) {
        if self.host_bridge.is_localhost() {
            self.open_path_with(entry.path(), open_with);
        } else {
            self.open_bridged_file(entry, open_with);
        }
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
            SelectedFile::Many(entries) => entries.into_iter().map(|(f, _)| f).collect(),
            SelectedFile::None => vec![],
        };
        // Open all entries
        entries
            .iter()
            .for_each(|x| self.action_open_local_file(x, Some(with)));

        // clear selection
        self.host_bridge_mut().clear_queue();
    }

    /// Open selected file with provided application
    pub(crate) fn action_remote_open_with(&mut self, with: &str) {
        let entries: Vec<File> = match self.get_remote_selected_entries() {
            SelectedFile::One(entry) => vec![entry],
            SelectedFile::Many(entries) => entries.into_iter().map(|(f, _)| f).collect(),
            SelectedFile::None => vec![],
        };
        // Open all entries
        entries
            .iter()
            .for_each(|x| self.action_open_remote_file(x, Some(with)));

        // clear selection
        self.remote_mut().clear_queue();
        self.reload_remote_filelist();
    }

    fn open_bridged_file(&mut self, entry: &File, open_with: Option<&str>) {
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

        let tmpfile = cache.join(tmpfile);

        // open from host bridge
        let mut reader = match self.host_bridge.open_file(entry.path()) {
            Ok(reader) => reader,
            Err(err) => {
                self.log(
                    LogLevel::Error,
                    format!("Failed to open bridged entry: {err}"),
                );
                return;
            }
        };

        // write to file
        let mut writer = match std::fs::File::create(tmpfile.as_path()) {
            Ok(writer) => writer,
            Err(err) => {
                self.log(LogLevel::Error, format!("Failed to create file: {err}"));
                return;
            }
        };

        if let Err(err) = std::io::copy(&mut reader, &mut writer) {
            self.log(LogLevel::Error, format!("Failed to write file: {err}"));
            return;
        }

        if tmpfile.exists() {
            self.open_path_with(tmpfile.as_path(), open_with);
        }
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
                format!("Failed to open file `{}`: {}", p.display(), err),
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
