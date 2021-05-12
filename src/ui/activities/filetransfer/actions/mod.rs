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

impl FileTransferActivity {
    /// ### get_local_file_entry
    ///
    /// Get local file entry
    pub(crate) fn get_local_file_entry(&self) -> Option<&FsEntry> {
        match self.get_local_file_state() {
            Some(Payload::One(Value::Usize(idx))) => self.local().get(idx),
            _ => None,
        }
    }

    /// ### get_remote_file_entry
    ///
    /// Get remote file entry
    pub(crate) fn get_remote_file_entry(&self) -> Option<&FsEntry> {
        match self.get_remote_file_state() {
            Some(Payload::One(Value::Usize(idx))) => self.remote().get(idx),
            _ => None,
        }
    }

    // -- private

    /// ### get_local_file_state
    ///
    /// Get index of selected file in the local tab
    fn get_local_file_state(&self) -> Option<Payload> {
        self.view.get_state(super::COMPONENT_EXPLORER_LOCAL)
    }

    /// ### get_remote_file_state
    ///
    /// Get index of selected file in the remote file
    fn get_remote_file_state(&self) -> Option<Payload> {
        self.view.get_state(super::COMPONENT_EXPLORER_REMOTE)
    }
}
