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
pub(self) use super::{FileTransferActivity, FsEntry, LogLevel};
use tuirealm::{Payload, Value};

// actions
pub(crate) mod change_dir;
pub(crate) mod copy;
pub(crate) mod delete;
pub(crate) mod edit;
pub(crate) mod exec;
pub(crate) mod find;
pub(crate) mod mkdir;
pub(crate) mod newfile;
pub(crate) mod rename;
pub(crate) mod save;

pub(crate) enum SelectedEntry {
    One(FsEntry),
    Multi(Vec<FsEntry>),
    None,
}

enum SelectedEntryIndex {
    One(usize),
    Multi(Vec<usize>),
    None,
}

impl From<Option<&FsEntry>> for SelectedEntry {
    fn from(opt: Option<&FsEntry>) -> Self {
        match opt {
            Some(e) => SelectedEntry::One(e.clone()),
            None => SelectedEntry::None,
        }
    }
}

impl From<Vec<&FsEntry>> for SelectedEntry {
    fn from(files: Vec<&FsEntry>) -> Self {
        SelectedEntry::Multi(files.into_iter().cloned().collect())
    }
}

impl FileTransferActivity {
    /// ### get_local_selected_entries
    ///
    /// Get local file entry
    pub(crate) fn get_local_selected_entries(&self) -> SelectedEntry {
        match self.get_selected_index(super::COMPONENT_EXPLORER_LOCAL) {
            SelectedEntryIndex::One(idx) => SelectedEntry::from(self.local().get(idx)),
            SelectedEntryIndex::Multi(files) => {
                let files: Vec<&FsEntry> = files
                    .iter()
                    .map(|x| self.local().get(*x)) // Usize to Option<FsEntry>
                    .filter(|x| x.is_some()) // Get only some values
                    .map(|x| x.unwrap()) // Option to FsEntry
                    .collect();
                SelectedEntry::from(files)
            }
            SelectedEntryIndex::None => SelectedEntry::None,
        }
    }

    /// ### get_remote_selected_entries
    ///
    /// Get remote file entry
    pub(crate) fn get_remote_selected_entries(&self) -> SelectedEntry {
        match self.get_selected_index(super::COMPONENT_EXPLORER_REMOTE) {
            SelectedEntryIndex::One(idx) => SelectedEntry::from(self.remote().get(idx)),
            SelectedEntryIndex::Multi(files) => {
                let files: Vec<&FsEntry> = files
                    .iter()
                    .map(|x| self.remote().get(*x)) // Usize to Option<FsEntry>
                    .filter(|x| x.is_some()) // Get only some values
                    .map(|x| x.unwrap()) // Option to FsEntry
                    .collect();
                SelectedEntry::from(files)
            }
            SelectedEntryIndex::None => SelectedEntry::None,
        }
    }

    /// ### get_remote_selected_entries
    ///
    /// Get remote file entry
    pub(crate) fn get_found_selected_entries(&self) -> SelectedEntry {
        match self.get_selected_index(super::COMPONENT_EXPLORER_FIND) {
            SelectedEntryIndex::One(idx) => {
                SelectedEntry::from(self.found().as_ref().unwrap().get(idx))
            }
            SelectedEntryIndex::Multi(files) => {
                let files: Vec<&FsEntry> = files
                    .iter()
                    .map(|x| self.found().as_ref().unwrap().get(*x)) // Usize to Option<FsEntry>
                    .filter(|x| x.is_some()) // Get only some values
                    .map(|x| x.unwrap()) // Option to FsEntry
                    .collect();
                SelectedEntry::from(files)
            }
            SelectedEntryIndex::None => SelectedEntry::None,
        }
    }

    // -- private

    fn get_selected_index(&self, component: &str) -> SelectedEntryIndex {
        match self.view.get_state(component) {
            Some(Payload::One(Value::Usize(idx))) => SelectedEntryIndex::One(idx),
            Some(Payload::Vec(files)) => {
                let list: Vec<usize> = files
                    .iter()
                    .map(|x| match x {
                        Value::Usize(v) => *v,
                        _ => 0,
                    })
                    .collect();
                SelectedEntryIndex::Multi(list)
            }
            _ => SelectedEntryIndex::None,
        }
    }
}
