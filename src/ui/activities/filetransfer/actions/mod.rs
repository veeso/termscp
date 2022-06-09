//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
pub(self) use super::{
    browser::FileExplorerTab, FileTransferActivity, Id, LogLevel, Msg, PendingActionMsg,
    TransferMsg, TransferOpts, TransferPayload, UiMsg,
};
pub(self) use remotefs::File;
use tuirealm::{State, StateValue};

// actions
pub(crate) mod change_dir;
pub(crate) mod copy;
pub(crate) mod delete;
pub(crate) mod edit;
pub(crate) mod exec;
pub(crate) mod find;
pub(crate) mod mkdir;
pub(crate) mod newfile;
pub(crate) mod open;
mod pending;
pub(crate) mod rename;
pub(crate) mod save;
pub(crate) mod submit;
pub(crate) mod symlink;
pub(crate) mod watcher;

#[derive(Debug)]
pub(crate) enum SelectedFile {
    One(File),
    Many(Vec<File>),
    None,
}

#[derive(Debug)]
enum SelectedFileIndex {
    One(usize),
    Many(Vec<usize>),
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

impl From<Vec<&File>> for SelectedFile {
    fn from(files: Vec<&File>) -> Self {
        SelectedFile::Many(files.into_iter().cloned().collect())
    }
}

impl FileTransferActivity {
    /// Get local file entry
    pub(crate) fn get_local_selected_entries(&self) -> SelectedFile {
        match self.get_selected_index(&Id::ExplorerLocal) {
            SelectedFileIndex::One(idx) => SelectedFile::from(self.local().get(idx)),
            SelectedFileIndex::Many(files) => {
                let files: Vec<&File> = files
                    .iter()
                    .filter_map(|x| self.local().get(*x)) // Usize to Option<File>
                    .collect();
                SelectedFile::from(files)
            }
            SelectedFileIndex::None => SelectedFile::None,
        }
    }

    /// Get remote file entry
    pub(crate) fn get_remote_selected_entries(&self) -> SelectedFile {
        match self.get_selected_index(&Id::ExplorerRemote) {
            SelectedFileIndex::One(idx) => SelectedFile::from(self.remote().get(idx)),
            SelectedFileIndex::Many(files) => {
                let files: Vec<&File> = files
                    .iter()
                    .filter_map(|x| self.remote().get(*x)) // Usize to Option<File>
                    .collect();
                SelectedFile::from(files)
            }
            SelectedFileIndex::None => SelectedFile::None,
        }
    }

    /// Returns whether only one entry is selected on local host
    pub(crate) fn is_local_selected_one(&self) -> bool {
        matches!(self.get_local_selected_entries(), SelectedFile::One(_))
    }

    /// Returns whether only one entry is selected on remote host
    pub(crate) fn is_remote_selected_one(&self) -> bool {
        matches!(self.get_remote_selected_entries(), SelectedFile::One(_))
    }

    /// Get remote file entry
    pub(crate) fn get_found_selected_entries(&self) -> SelectedFile {
        match self.get_selected_index(&Id::ExplorerFind) {
            SelectedFileIndex::One(idx) => {
                SelectedFile::from(self.found().as_ref().unwrap().get(idx))
            }
            SelectedFileIndex::Many(files) => {
                let files: Vec<&File> = files
                    .iter()
                    .filter_map(|x| self.found().as_ref().unwrap().get(*x)) // Usize to Option<File>
                    .collect();
                SelectedFile::from(files)
            }
            SelectedFileIndex::None => SelectedFile::None,
        }
    }

    // -- private

    fn get_selected_index(&self, id: &Id) -> SelectedFileIndex {
        match self.app.state(id) {
            Ok(State::One(StateValue::Usize(idx))) => SelectedFileIndex::One(idx),
            Ok(State::Vec(files)) => {
                let list: Vec<usize> = files
                    .iter()
                    .map(|x| match x {
                        StateValue::Usize(v) => *v,
                        _ => 0,
                    })
                    .collect();
                SelectedFileIndex::Many(list)
            }
            _ => SelectedFileIndex::None,
        }
    }
}
