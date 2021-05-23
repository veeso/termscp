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
use super::{FileTransferActivity, FsEntry};
use std::path::PathBuf;

impl FileTransferActivity {
    /// ### action_enter_local_dir
    ///
    /// Enter a directory on local host from entry
    /// Return true whether the directory changed
    pub(crate) fn action_enter_local_dir(&mut self, entry: FsEntry, block_sync: bool) -> bool {
        match entry {
            FsEntry::Directory(dir) => {
                self.local_changedir(dir.abs_path.as_path(), true);
                if self.browser.sync_browsing && !block_sync {
                    self.action_change_remote_dir(dir.name, true);
                }
                true
            }
            FsEntry::File(file) => {
                match &file.symlink {
                    Some(symlink_entry) => {
                        // If symlink and is directory, point to symlink
                        match &**symlink_entry {
                            FsEntry::Directory(dir) => {
                                self.local_changedir(dir.abs_path.as_path(), true);
                                // Check whether to sync
                                if self.browser.sync_browsing && !block_sync {
                                    self.action_change_remote_dir(dir.name.clone(), true);
                                }
                                true
                            }
                            _ => false,
                        }
                    }
                    None => false,
                }
            }
        }
    }

    /// ### action_enter_remote_dir
    ///
    /// Enter a directory on local host from entry
    /// Return true whether the directory changed
    pub(crate) fn action_enter_remote_dir(&mut self, entry: FsEntry, block_sync: bool) -> bool {
        match entry {
            FsEntry::Directory(dir) => {
                self.remote_changedir(dir.abs_path.as_path(), true);
                if self.browser.sync_browsing && !block_sync {
                    self.action_change_local_dir(dir.name, true);
                }
                true
            }
            FsEntry::File(file) => {
                match &file.symlink {
                    Some(symlink_entry) => {
                        // If symlink and is directory, point to symlink
                        match &**symlink_entry {
                            FsEntry::Directory(dir) => {
                                self.remote_changedir(dir.abs_path.as_path(), true);
                                // Check whether to sync
                                if self.browser.sync_browsing && !block_sync {
                                    self.action_change_local_dir(dir.name.clone(), true);
                                }
                                true
                            }
                            _ => false,
                        }
                    }
                    None => false,
                }
            }
        }
    }

    /// ### action_change_local_dir
    ///
    /// Change local directory reading value from input
    pub(crate) fn action_change_local_dir(&mut self, input: String, block_sync: bool) {
        let dir_path: PathBuf = self.local_to_abs_path(PathBuf::from(input.as_str()).as_path());
        self.local_changedir(dir_path.as_path(), true);
        // Check whether to sync
        if self.browser.sync_browsing && !block_sync {
            self.action_change_remote_dir(input, true);
        }
    }

    /// ### action_change_remote_dir
    ///
    /// Change remote directory reading value from input
    pub(crate) fn action_change_remote_dir(&mut self, input: String, block_sync: bool) {
        let dir_path: PathBuf = self.remote_to_abs_path(PathBuf::from(input.as_str()).as_path());
        self.remote_changedir(dir_path.as_path(), true);
        // Check whether to sync
        if self.browser.sync_browsing && !block_sync {
            self.action_change_local_dir(input, true);
        }
    }

    /// ### action_go_to_previous_local_dir
    ///
    /// Go to previous directory from localhost
    pub(crate) fn action_go_to_previous_local_dir(&mut self, block_sync: bool) {
        if let Some(d) = self.local_mut().popd() {
            self.local_changedir(d.as_path(), false);
            // Check whether to sync
            if self.browser.sync_browsing && !block_sync {
                self.action_go_to_previous_remote_dir(true);
            }
        }
    }

    /// ### action_go_to_previous_remote_dir
    ///
    /// Go to previous directory from remote host
    pub(crate) fn action_go_to_previous_remote_dir(&mut self, block_sync: bool) {
        if let Some(d) = self.remote_mut().popd() {
            self.remote_changedir(d.as_path(), false);
            // Check whether to sync
            if self.browser.sync_browsing && !block_sync {
                self.action_go_to_previous_local_dir(true);
            }
        }
    }

    /// ### action_go_to_local_upper_dir
    ///
    /// Go to upper directory on local host
    pub(crate) fn action_go_to_local_upper_dir(&mut self, block_sync: bool) {
        // Get pwd
        let path: PathBuf = self.local().wrkdir.clone();
        // Go to parent directory
        if let Some(parent) = path.as_path().parent() {
            self.local_changedir(parent, true);
            // If sync is enabled update remote too
            if self.browser.sync_browsing && !block_sync {
                self.action_go_to_remote_upper_dir(true);
            }
        }
    }

    /// #### action_go_to_remote_upper_dir
    ///
    /// Go to upper directory on remote host
    pub(crate) fn action_go_to_remote_upper_dir(&mut self, block_sync: bool) {
        // Get pwd
        let path: PathBuf = self.remote().wrkdir.clone();
        // Go to parent directory
        if let Some(parent) = path.as_path().parent() {
            self.remote_changedir(parent, true);
            // If sync is enabled update local too
            if self.browser.sync_browsing && !block_sync {
                self.action_go_to_local_upper_dir(true);
            }
        }
    }
}
