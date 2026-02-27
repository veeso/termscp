//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

use std::path::{Path, PathBuf};

use remotefs::File;
use remotefs::fs::UnixPex;
use tuirealm::{State, StateValue};

use super::browser::FileExplorerTab;
use super::lib::browser::FoundExplorerTab;
use super::{
    FileTransferActivity, Id, LogLevel, Msg, PendingActionMsg, TransferMsg, TransferOpts,
    TransferPayload, UiMsg,
};
use crate::explorer::FileExplorer;

// actions
pub(crate) mod change_dir;
pub(crate) mod chmod;
pub(crate) mod copy;
pub(crate) mod delete;
pub(crate) mod edit;
pub(crate) mod exec;
pub(crate) mod file_size;
pub(crate) mod filter;
pub(crate) mod find;
pub(crate) mod mark;
pub(crate) mod mkdir;
pub(crate) mod newfile;
pub(crate) mod open;
mod pending;
pub(crate) mod rename;
pub(crate) mod save;
pub(crate) mod scan;
pub(crate) mod submit;
pub(crate) mod symlink;
pub(crate) mod walkdir;
pub(crate) mod watcher;

#[derive(Debug)]
pub(crate) enum SelectedFile {
    One(File),
    /// List of file with their destination path
    Many(Vec<(File, PathBuf)>),
    None,
}

impl SelectedFile {
    /// Get file mode for `SelectedFile`
    /// In case is `Many` the first item mode is returned
    pub fn unix_pex(&self) -> Option<UnixPex> {
        match self {
            Self::Many(files) => files
                .iter()
                .next()
                .and_then(|(file, _)| file.metadata().mode),
            Self::One(file) => file.metadata().mode,
            Self::None => None,
        }
    }

    /// Get files as vec
    pub fn get_files(self) -> Vec<File> {
        match self {
            Self::One(file) => vec![file],
            Self::Many(files) => files.into_iter().map(|(f, _)| f).collect(),
            Self::None => vec![],
        }
    }
}

#[derive(Debug)]
enum SelectedFileIndex {
    One(usize),
    None,
}

impl From<Option<&File>> for SelectedFile {
    fn from(opt: Option<&File>) -> Self {
        match opt {
            Some(e) => SelectedFile::One(e.clone()),
            None => SelectedFile::None,
        }
    }
}

impl FileTransferActivity {
    /// Get local file entry
    pub(crate) fn get_local_selected_entries(&mut self) -> SelectedFile {
        self.get_selected_files(&Id::ExplorerHostBridge)
    }

    /// Get remote file entry
    pub(crate) fn get_remote_selected_entries(&mut self) -> SelectedFile {
        self.get_selected_files(&Id::ExplorerRemote)
    }

    /// Get remote file entry
    pub(crate) fn get_found_selected_entries(&mut self) -> SelectedFile {
        self.get_selected_files(&Id::ExplorerFind)
    }

    pub(crate) fn get_found_selected_file(&self) -> Option<File> {
        self.get_selected_file_by_id(&Id::ExplorerFind)
    }

    /// Get selected entries from whichever tab is currently active.
    pub(crate) fn get_selected_entries(&mut self) -> SelectedFile {
        let id = match self.browser.tab() {
            FileExplorerTab::HostBridge => Id::ExplorerHostBridge,
            FileExplorerTab::Remote => Id::ExplorerRemote,
            FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => Id::ExplorerFind,
        };
        self.get_selected_files(&id)
    }

    /// Get single selected file from whichever tab is currently active.
    pub(crate) fn get_selected_file(&self) -> Option<File> {
        let id = match self.browser.tab() {
            FileExplorerTab::HostBridge => Id::ExplorerHostBridge,
            FileExplorerTab::Remote => Id::ExplorerRemote,
            FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => Id::ExplorerFind,
        };
        self.get_selected_file_by_id(&id)
    }

    /// Returns whether only one entry is selected on the current tab.
    pub(crate) fn is_selected_one(&mut self) -> bool {
        matches!(self.get_selected_entries(), SelectedFile::One(_))
    }

    // -- private

    fn get_selected_index(&self, id: &Id) -> SelectedFileIndex {
        match self.app.state(id) {
            Ok(State::One(StateValue::Usize(idx))) => SelectedFileIndex::One(idx),
            _ => SelectedFileIndex::None,
        }
    }

    fn get_selected_files(&mut self, id: &Id) -> SelectedFile {
        let browser = self.browser_by_id(id);
        // if transfer queue is not empty, return that
        let transfer_queue = browser.enqueued().clone();
        if !transfer_queue.is_empty() {
            return SelectedFile::Many(
                transfer_queue
                    .iter()
                    .filter_map(|(src, dest)| {
                        let src_file = self.get_file_from_path(id, src)?;
                        Some((src_file, dest.clone()))
                    })
                    .collect(),
            );
        }

        let browser = self.browser_by_id(id);
        // if no transfer queue, return selected files
        match self.get_selected_index(id) {
            SelectedFileIndex::One(idx) => {
                let Some(f) = browser.get(idx) else {
                    return SelectedFile::None;
                };
                SelectedFile::One(f.clone())
            }
            SelectedFileIndex::None => SelectedFile::None,
        }
    }

    fn get_file_from_path(&mut self, id: &Id, path: &Path) -> Option<File> {
        match *id {
            Id::ExplorerHostBridge => self.host_bridge.stat(path).ok(),
            Id::ExplorerRemote => self.client.stat(path).ok(),
            Id::ExplorerFind => {
                let found = self.browser.found_tab().unwrap();
                match found {
                    FoundExplorerTab::Local => self.host_bridge.stat(path).ok(),
                    FoundExplorerTab::Remote => self.client.stat(path).ok(),
                }
            }
            _ => None,
        }
    }

    fn browser_by_id(&self, id: &Id) -> &FileExplorer {
        match *id {
            Id::ExplorerHostBridge => self.host_bridge(),
            Id::ExplorerRemote => self.remote(),
            Id::ExplorerFind => self.found().as_ref().unwrap(),
            _ => unreachable!(),
        }
    }

    fn get_selected_file_by_id(&self, id: &Id) -> Option<File> {
        let browser = self.browser_by_id(id);
        // if no transfer queue, return selected files
        match self.get_selected_index(id) {
            SelectedFileIndex::One(idx) => browser.get(idx).cloned(),
            SelectedFileIndex::None => None,
        }
    }
}
