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
    browser::FileExplorerTab, FileTransferActivity, Id, LogLevel, TransferOpts, TransferPayload,
};
pub(self) use remotefs::Entry;
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
pub(crate) mod rename;
pub(crate) mod save;
pub(crate) mod submit;

#[derive(Debug)]
pub(crate) enum SelectedEntry {
    One(Entry),
    Many(Vec<Entry>),
    None,
}

#[derive(Debug)]
enum SelectedEntryIndex {
    One(usize),
    Many(Vec<usize>),
    None,
}

impl From<Option<&Entry>> for SelectedEntry {
    fn from(opt: Option<&Entry>) -> Self {
        match opt {
            Some(e) => SelectedEntry::One(e.clone()),
            None => SelectedEntry::None,
        }
    }
}

impl From<Vec<&Entry>> for SelectedEntry {
    fn from(files: Vec<&Entry>) -> Self {
        SelectedEntry::Many(files.into_iter().cloned().collect())
    }
}

impl FileTransferActivity {
    /// Get local file entry
    pub(crate) fn get_local_selected_entries(&self) -> SelectedEntry {
        match self.get_selected_index(&Id::ExplorerLocal) {
            SelectedEntryIndex::One(idx) => SelectedEntry::from(self.local().get(idx)),
            SelectedEntryIndex::Many(files) => {
                let files: Vec<&Entry> = files
                    .iter()
                    .map(|x| self.local().get(*x)) // Usize to Option<Entry>
                    .flatten()
                    .collect();
                SelectedEntry::from(files)
            }
            SelectedEntryIndex::None => SelectedEntry::None,
        }
    }

    /// Get remote file entry
    pub(crate) fn get_remote_selected_entries(&self) -> SelectedEntry {
        match self.get_selected_index(&Id::ExplorerRemote) {
            SelectedEntryIndex::One(idx) => SelectedEntry::from(self.remote().get(idx)),
            SelectedEntryIndex::Many(files) => {
                let files: Vec<&Entry> = files
                    .iter()
                    .map(|x| self.remote().get(*x)) // Usize to Option<Entry>
                    .flatten()
                    .collect();
                SelectedEntry::from(files)
            }
            SelectedEntryIndex::None => SelectedEntry::None,
        }
    }

    /// Get remote file entry
    pub(crate) fn get_found_selected_entries(&self) -> SelectedEntry {
        match self.get_selected_index(&Id::ExplorerFind) {
            SelectedEntryIndex::One(idx) => {
                SelectedEntry::from(self.found().as_ref().unwrap().get(idx))
            }
            SelectedEntryIndex::Many(files) => {
                let files: Vec<&Entry> = files
                    .iter()
                    .map(|x| self.found().as_ref().unwrap().get(*x)) // Usize to Option<Entry>
                    .flatten()
                    .collect();
                SelectedEntry::from(files)
            }
            SelectedEntryIndex::None => SelectedEntry::None,
        }
    }

    // -- private

    fn get_selected_index(&self, id: &Id) -> SelectedEntryIndex {
        match self.app.state(id) {
            Ok(State::One(StateValue::Usize(idx))) => SelectedEntryIndex::One(idx),
            Ok(State::Vec(files)) => {
                let list: Vec<usize> = files
                    .iter()
                    .map(|x| match x {
                        StateValue::Usize(v) => *v,
                        _ => 0,
                    })
                    .collect();
                SelectedEntryIndex::Many(list)
            }
            _ => SelectedEntryIndex::None,
        }
    }
}
