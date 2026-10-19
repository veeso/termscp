//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

use std::path::{Path, PathBuf};

use remotefs::File;
use remotefs::fs::UnixPex;
use tuirealm::state::{State, StateValue};

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

/// Points each entry at `dest_dir/<name>`.
///
/// Queued targets point to the folder shown when the files were marked. With a typed
/// destination folder each entry goes to `dest_dir` under its own name instead.
pub(crate) fn retarget_queue(
    entries: Vec<(File, PathBuf)>,
    dest_dir: &Path,
) -> Vec<(File, PathBuf)> {
    entries
        .into_iter()
        .map(|(entry, _)| {
            let target = dest_dir.join(entry.name());
            (entry, target)
        })
        .collect()
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

    /// Moves each queued entry into `dest_dir` under its own name and runs `op` on it,
    /// then clears the queue and refreshes both panels.
    pub(crate) fn apply_to_queue<F>(
        &mut self,
        entries: Vec<(File, PathBuf)>,
        dest_dir: &Path,
        mut op: F,
    ) where
        F: FnMut(&mut Self, File, &Path),
    {
        for (entry, target) in retarget_queue(entries, dest_dir) {
            op(self, entry, &target);
        }
        self.clear_queue();
    }

    // -- private

    fn get_selected_index(&self, id: &Id) -> SelectedFileIndex {
        match self.app.state(id) {
            Ok(State::Single(StateValue::Usize(idx))) => SelectedFileIndex::One(idx),
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
            Id::ExplorerHostBridge => self.browser.local_pane_mut().fs.stat(path).ok(),
            Id::ExplorerRemote => self.browser.remote_pane_mut().fs.stat(path).ok(),
            Id::ExplorerFind => {
                let found = self.browser.found_tab().unwrap();
                match found {
                    FoundExplorerTab::Local => self.browser.local_pane_mut().fs.stat(path).ok(),
                    FoundExplorerTab::Remote => self.browser.remote_pane_mut().fs.stat(path).ok(),
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
            _ => {
                error!(
                    "browser_by_id called with unexpected id: {id:?}; falling back to local explorer"
                );
                self.host_bridge()
            }
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

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use tempfile::TempDir;

    use super::*;
    use crate::host::{HostBridge, Localhost};
    use crate::utils::test_helpers::{make_dir_at, make_file_at, make_fsentry};

    #[test]
    fn should_retarget_queued_entries_to_dest_dir() {
        let file = make_fsentry("/src/a.txt", false);
        let dir = make_fsentry("/src/dir", true);
        // queued targets point to the folder shown when the files were marked
        let queued = Path::new("/queued");
        let entries = vec![
            (file.clone(), queued.join("a.txt")),
            (dir.clone(), queued.join("dir")),
        ];

        for dest_dir in [PathBuf::from("backup"), Path::new("/").join("typed")] {
            assert_eq!(
                retarget_queue(entries.clone(), &dest_dir),
                vec![
                    (file.clone(), dest_dir.join("a.txt")),
                    (dir.clone(), dest_dir.join("dir")),
                ]
            );
        }
    }

    #[test]
    fn should_copy_queued_entries_into_dest_dir() {
        let src = TempDir::new().unwrap();
        let queued = TempDir::new().unwrap();
        let typed = TempDir::new().unwrap();
        make_file_at(src.path(), "a.txt").unwrap();
        make_dir_at(src.path(), "dir").unwrap();
        make_file_at(&src.path().join("dir"), "b.txt").unwrap();
        let mut fs = Localhost::new(src.path().to_path_buf()).unwrap();
        let file = fs.stat(&src.path().join("a.txt")).unwrap();
        let dir = fs.stat(&src.path().join("dir")).unwrap();

        // queued targets are full paths in the folder shown when the files were marked
        let entries = vec![
            (file, queued.path().join("a.txt")),
            (dir, queued.path().join("dir")),
        ];
        for (entry, target) in retarget_queue(entries, typed.path()) {
            fs.copy(&entry, &target).unwrap();
        }

        assert!(typed.path().join("a.txt").is_file());
        assert!(typed.path().join("dir").join("b.txt").is_file());
        assert!(!typed.path().join("dir").join("dir").exists());
        assert!(!queued.path().join("a.txt").exists());
        assert!(!queued.path().join("dir").exists());
    }
}
