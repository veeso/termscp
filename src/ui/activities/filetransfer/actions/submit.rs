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
use super::{Entry, FileTransferActivity};

use remotefs::fs::{File, Metadata};

enum SubmitAction {
    ChangeDir,
    None,
}

impl FileTransferActivity {
    /// Decides which action to perform on submit for local explorer
    /// Return true whether the directory changed
    pub(crate) fn action_submit_local(&mut self, entry: Entry) {
        let (action, entry) = match &entry {
            Entry::Directory(_) => (SubmitAction::ChangeDir, entry),
            Entry::File(File {
                path,
                metadata:
                    Metadata {
                        symlink: Some(symlink),
                        ..
                    },
                ..
            }) => {
                // Stat file
                let stat_file = match self.host.stat(symlink.as_path()) {
                    Ok(e) => e,
                    Err(err) => {
                        warn!(
                            "Could not stat file pointed by {} ({}): {}",
                            path.display(),
                            symlink.display(),
                            err
                        );
                        entry
                    }
                };
                (SubmitAction::ChangeDir, stat_file)
            }
            Entry::File(_) => (SubmitAction::None, entry),
        };
        if let (SubmitAction::ChangeDir, Entry::Directory(dir)) = (action, entry) {
            self.action_enter_local_dir(dir)
        }
    }

    /// Decides which action to perform on submit for remote explorer
    /// Return true whether the directory changed
    pub(crate) fn action_submit_remote(&mut self, entry: Entry) {
        let (action, entry) = match &entry {
            Entry::Directory(_) => (SubmitAction::ChangeDir, entry),
            Entry::File(File {
                path,
                metadata:
                    Metadata {
                        symlink: Some(symlink),
                        ..
                    },
                ..
            }) => {
                // Stat file
                let stat_file = match self.client.stat(symlink.as_path()) {
                    Ok(e) => e,
                    Err(err) => {
                        warn!(
                            "Could not stat file pointed by {} ({}): {}",
                            path.display(),
                            symlink.display(),
                            err
                        );
                        entry
                    }
                };
                (SubmitAction::ChangeDir, stat_file)
            }
            Entry::File(_) => (SubmitAction::None, entry),
        };
        if let (SubmitAction::ChangeDir, Entry::Directory(dir)) = (action, entry) {
            self.action_enter_remote_dir(dir)
        }
    }
}
