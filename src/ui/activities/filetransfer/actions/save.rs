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
// locals
use super::{FileTransferActivity, SelectedEntry};
use std::path::PathBuf;

impl FileTransferActivity {
    pub(crate) fn action_local_saveas(&mut self, input: String) {
        self.action_local_send_file(Some(input));
    }

    pub(crate) fn action_remote_saveas(&mut self, input: String) {
        self.action_remote_recv_file(Some(input));
    }

    pub(crate) fn action_local_send(&mut self) {
        self.action_local_send_file(None);
    }

    pub(crate) fn action_remote_recv(&mut self) {
        self.action_remote_recv_file(None);
    }

    fn action_local_send_file(&mut self, save_as: Option<String>) {
        let wrkdir: PathBuf = self.remote().wrkdir.clone();
        match self.get_local_selected_entries() {
            SelectedEntry::One(entry) => {
                self.filetransfer_send(&entry.get_realfile(), wrkdir.as_path(), save_as);
            }
            SelectedEntry::Many(entries) => {
                // In case of selection: save multiple files in wrkdir/input
                let mut dest_path: PathBuf = wrkdir;
                if let Some(save_as) = save_as {
                    dest_path.push(save_as);
                }
                // Iter files
                for entry in entries.iter() {
                    self.filetransfer_send(&entry.get_realfile(), dest_path.as_path(), None);
                }
            }
            SelectedEntry::None => {}
        }
    }

    fn action_remote_recv_file(&mut self, save_as: Option<String>) {
        let wrkdir: PathBuf = self.local().wrkdir.clone();
        match self.get_remote_selected_entries() {
            SelectedEntry::One(entry) => {
                self.filetransfer_recv(&entry.get_realfile(), wrkdir.as_path(), save_as);
            }
            SelectedEntry::Many(entries) => {
                // In case of selection: save multiple files in wrkdir/input
                let mut dest_path: PathBuf = wrkdir;
                if let Some(save_as) = save_as {
                    dest_path.push(save_as);
                }
                // Iter files
                for entry in entries.iter() {
                    self.filetransfer_recv(&entry.get_realfile(), dest_path.as_path(), None);
                }
            }
            SelectedEntry::None => {}
        }
    }
}
