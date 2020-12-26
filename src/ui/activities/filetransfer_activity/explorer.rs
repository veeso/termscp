//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "TermSCP"
*
*   TermSCP is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   TermSCP is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with TermSCP.  If not, see <http://www.gnu.org/licenses/>.
*
*/

// Locals
use super::FsEntry;
// Ext
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

/// ## FileExplorer
///
/// File explorer states
pub struct FileExplorer {
    pub wrkdir: PathBuf,         // Current directory
    index: usize,                // Selected file
    files: Vec<FsEntry>,         // Files in directory
    dirstack: VecDeque<PathBuf>, // Stack of visited directory (max 16)
    stack_size: usize,           // Directory stack size
    hidden_files: bool,          // Should hidden files be shown or not; hidden if false
}

impl FileExplorer {
    /// ### new
    ///
    /// Instantiates a new FileExplorer
    pub fn new(stack_size: usize) -> FileExplorer {
        FileExplorer {
            wrkdir: PathBuf::from("/"),
            index: 0,
            files: Vec::new(),
            dirstack: VecDeque::with_capacity(stack_size),
            stack_size,
            hidden_files: false, // Default: don't show hidden files
        }
    }

    /// ### pushd
    ///
    /// push directory to stack
    pub fn pushd(&mut self, dir: &Path) {
        // Check if stack would overflow the size
        while self.dirstack.len() >= self.stack_size {
            self.dirstack.pop_front(); // Start cleaning events from back
        }
        // Eventually push front the new record
        self.dirstack.push_back(PathBuf::from(dir));
    }

    /// ### popd
    ///
    /// Pop directory from the stack and return the directory
    pub fn popd(&mut self) -> Option<PathBuf> {
        self.dirstack.pop_back()
    }

    /// ### set_files
    ///
    /// Set Explorer files
    /// Index is then moved to first valid `FsEntry` for current setup
    pub fn set_files(&mut self, files: Vec<FsEntry>) {
        self.files = files;
        // Set index to first valid entry
        self.index_at_first();
    }

    /// ### count
    ///
    /// Return amount of files
    pub fn count(&self) -> usize {
        self.files.len()
    }

    /// ### iter_files
    ///
    /// Iterate over files
    /// Filters are applied based on current options (e.g. hidden files not returned)
    pub fn iter_files(&self) -> Box<dyn Iterator<Item = &FsEntry> + '_> {
        // Match options
        match self.hidden_files {
            false => Box::new(self.files.iter().filter(|x| !x.is_hidden())), // Show only visible files
            true => self.iter_files_all(),                                   // Show all
        }
    }

    /// ### iter_files_all
    ///
    /// Iterate all files; doesn't care about options
    pub fn iter_files_all(&self) -> Box<dyn Iterator<Item = &FsEntry> + '_> {
        Box::new(self.files.iter())
    }

    /// ### get_current_file
    ///
    /// Get file at index
    pub fn get_current_file(&self) -> Option<&FsEntry> {
        self.files.get(self.index)
    }

    /// ### sort_files_by_name
    ///
    /// Sort explorer files by their name. All names are converted to lowercase
    pub fn sort_files_by_name(&mut self) {
        self.files.sort_by_key(|x: &FsEntry| match x {
            FsEntry::Directory(dir) => dir.name.as_str().to_lowercase(),
            FsEntry::File(file) => file.name.as_str().to_lowercase(),
        });
        // Reset index
        self.index_at_first();
    }

    /// ### incr_index
    ///
    /// Increment index to the first visible FsEntry.
    /// If index goes to `files.len() - 1`, the value will be seto to the minimum acceptable value
    pub fn incr_index(&mut self) {
        let sz: usize = self.files.len();
        // Increment or wrap
        if self.index + 1 >= sz {
            self.index = 0; // Wrap
        } else {
            self.index += 1; // Increment
        }
        // Validate
        match self.files.get(self.index) {
            Some(assoc_entry) => {
                if !self.hidden_files {
                    // Check if file is hidden, otherwise increment
                    if assoc_entry.is_hidden() {
                        // Check if all files are hidden (NOTE: PREVENT STACK OVERFLOWS)
                        let hidden_files: usize =
                            self.files.iter().filter(|x| x.is_hidden()).count();
                        // Only if there are more files, than hidden files keep incrementing
                        if sz > hidden_files {
                            self.incr_index();
                        }
                    }
                }
            }
            None => self.index = 0, // Reset to 0, for safety reasons
        }
    }

    /// ### incr_index_by
    ///
    /// Increment index by up to n
    /// If index goes to `files.len() - 1`, the value will be seto to the minimum acceptable value
    pub fn incr_index_by(&mut self, n: usize) {
        for _ in 0..n {
            let prev_idx: usize = self.index;
            // Increment
            self.incr_index();
            // If prev index is > index and break
            if prev_idx > self.index {
                self.index = prev_idx;
                break;
            }
        }
    }

    /// ### decr_index
    ///
    /// Decrement index to the first visible FsEntry.
    /// If index is 0, its value will be set to the maximum acceptable value
    pub fn decr_index(&mut self) {
        let sz: usize = self.files.len();
        // Increment or wrap
        if self.index > 0 {
            self.index -= 1; // Decrement
        } else {
            self.index = sz - 1; // Wrap
        }
        // Validate index
        match self.files.get(self.index) {
            Some(assoc_entry) => {
                if !self.hidden_files {
                    // Check if file is hidden, otherwise increment
                    if assoc_entry.is_hidden() {
                        // Check if all files are hidden (NOTE: PREVENT STACK OVERFLOWS)
                        let hidden_files: usize =
                            self.files.iter().filter(|x| x.is_hidden()).count();
                        // Only if there are more files, than hidden files keep decrementing
                        if sz > hidden_files {
                            self.decr_index();
                        }
                    }
                }
            }
            None => self.index = 0, // Reset to 0, for safety reasons
        }
    }

    /// ### decr_index_by
    ///
    /// Decrement index by up to n
    pub fn decr_index_by(&mut self, n: usize) {
        for _ in 0..n {
            let prev_idx: usize = self.index;
            // Increment
            self.decr_index();
            // If prev index is < index and break
            if prev_idx < self.index {
                self.index = prev_idx;
                break;
            }
        }
    }

    /// ### index_at_first
    ///
    /// Move index to first "visible" fs entry
    pub fn index_at_first(&mut self) {
        self.index = self.get_first_valid_index();
    }

    /// ### get_first_valid_index
    ///
    /// Return first valid index
    fn get_first_valid_index(&self) -> usize {
        match self.hidden_files {
            true => 0,
            false => {
                // Look for first "non-hidden" entry
                for (i, f) in self.files.iter().enumerate() {
                    if !f.is_hidden() {
                        return i;
                    }
                }
                // If all files are hidden, return 0
                0
            }
        }
    }

    /// ### get_index
    ///
    /// Return index
    pub fn get_index(&self) -> usize {
        self.index
    }

    /// ### get_relative_index
    ///
    /// Get relative index based on current options
    pub fn get_relative_index(&self) -> usize {
        match self.files.get(self.index) {
            Some(abs_entry) => {
                // Search abs entry in relative iterator
                for (i, f) in self.iter_files().enumerate() {
                    if abs_entry.get_name() == f.get_name() {
                        // If abs entry is f, return index
                        return i;
                    }
                }
                // Return 0 if not found
                0
            }
            None => 0, // Absolute entry doesn't exist
        }
    }

    /// ### set_index
    ///
    /// Set index to idx.
    /// If index exceeds size, is set to count() - 1; or 0
    pub fn set_index(&mut self, idx: usize) {
        let visible_sz: usize = self.iter_files().count();
        match idx >= visible_sz {
            true => match visible_sz {
                0 => self.index_at_first(),
                _ => self.index = visible_sz - 1,
            },
            false => match self.get_first_valid_index() > idx {
                true => self.index_at_first(),
                false => self.index = idx,
            },
        }
    }

    /// ### toggle_hidden_files
    ///
    /// Enable/disable hidden files
    pub fn toggle_hidden_files(&mut self) {
        self.hidden_files = !self.hidden_files;
        // Adjust index
        if self.index < self.get_first_valid_index() {
            self.index_at_first();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::fs::{FsDirectory, FsFile};

    use std::time::SystemTime;

    #[test]
    fn test_ui_filetransfer_activity_explorer_new() {
        let explorer: FileExplorer = FileExplorer::new(16);
        // Verify
        assert_eq!(explorer.dirstack.len(), 0);
        assert_eq!(explorer.files.len(), 0);
        assert_eq!(explorer.hidden_files, false);
        assert_eq!(explorer.wrkdir, PathBuf::from("/"));
        assert_eq!(explorer.stack_size, 16);
        assert_eq!(explorer.index, 0);
    }

    #[test]
    fn test_ui_filetransfer_activity_explorer_stack() {
        let mut explorer: FileExplorer = FileExplorer::new(2);
        // Push dir
        explorer.pushd(&Path::new("/tmp"));
        explorer.pushd(&Path::new("/home/omar"));
        // Pop
        assert_eq!(explorer.popd().unwrap(), PathBuf::from("/home/omar"));
        assert_eq!(explorer.dirstack.len(), 1);
        assert_eq!(explorer.popd().unwrap(), PathBuf::from("/tmp"));
        assert_eq!(explorer.dirstack.len(), 0);
        // Dirstack is empty now
        assert!(explorer.popd().is_none());
        // Exceed limit
        explorer.pushd(&Path::new("/tmp"));
        explorer.pushd(&Path::new("/home/omar"));
        explorer.pushd(&Path::new("/dev"));
        assert_eq!(explorer.dirstack.len(), 2);
        assert_eq!(*explorer.dirstack.get(1).unwrap(), PathBuf::from("/dev"));
        assert_eq!(
            *explorer.dirstack.get(0).unwrap(),
            PathBuf::from("/home/omar")
        );
    }

    #[test]
    fn test_ui_filetransfer_activity_explorer_files() {
        let mut explorer: FileExplorer = FileExplorer::new(16);
        explorer.hidden_files = false;
        // Create files
        explorer.set_files(vec![
            make_fs_entry("README.md", false),
            make_fs_entry("src/", true),
            make_fs_entry(".git/", true),
            make_fs_entry("CONTRIBUTING.md", false),
            make_fs_entry("codecov.yml", false),
            make_fs_entry(".gitignore", false),
        ]);
        assert_eq!(explorer.count(), 6);
        // Verify
        assert_eq!(
            explorer.files.get(0).unwrap().get_name(),
            String::from("README.md")
        );
        // Sort files
        explorer.sort_files_by_name();
        // Verify
        assert_eq!(
            explorer.files.get(0).unwrap().get_name(),
            String::from(".git/")
        );
        // Iter files (all)
        assert_eq!(explorer.iter_files_all().count(), 6);
        // Iter files (hidden excluded) (.git, .gitignore are hidden)
        assert_eq!(explorer.iter_files().count(), 4);
        // Toggle hidden
        explorer.toggle_hidden_files();
        assert_eq!(explorer.iter_files().count(), 6); // All files are returned now
    }

    #[test]
    fn test_ui_filetransfer_activity_explorer_index() {
        let mut explorer: FileExplorer = FileExplorer::new(16);
        explorer.hidden_files = false;
        // Create files
        explorer.set_files(vec![
            make_fs_entry("README.md", false),
            make_fs_entry("src/", true),
            make_fs_entry(".git/", true),
            make_fs_entry("CONTRIBUTING.md", false),
            make_fs_entry("CODE_OF_CONDUCT.md", false),
            make_fs_entry("CHANGELOG.md", false),
            make_fs_entry("LICENSE", false),
            make_fs_entry("Cargo.toml", false),
            make_fs_entry("Cargo.lock", false),
            make_fs_entry("codecov.yml", false),
            make_fs_entry(".gitignore", false),
        ]);
        let sz: usize = explorer.count();
        // Sort by name
        explorer.sort_files_by_name();
        // Get first index
        assert_eq!(explorer.get_first_valid_index(), 2);
        // Index should be 2 now; files hidden; this happens because `index_at_first` is called after loading files
        assert_eq!(explorer.get_index(), 2);
        assert_eq!(explorer.get_relative_index(), 0); // Relative index should be 0
        assert_eq!(explorer.hidden_files, false);
        // Increment index
        explorer.incr_index();
        // Index should now be 3 (was 0, + 2 + 1); first 2 files are hidden (.git, .gitignore)
        assert_eq!(explorer.get_index(), 3);
        // Relative index should be 1 instead
        assert_eq!(explorer.get_relative_index(), 1);
        // Increment by 2
        explorer.incr_index_by(2);
        // Index should now be 5, 3
        assert_eq!(explorer.get_index(), 5);
        assert_eq!(explorer.get_relative_index(), 3);
        // Increment by (exceed size)
        explorer.incr_index_by(20);
        // Index should be at last element
        assert_eq!(explorer.get_index(), sz - 1);
        assert_eq!(explorer.get_relative_index(), sz - 3);
        // Increment; should go to 2
        explorer.incr_index();
        assert_eq!(explorer.get_index(), 2);
        assert_eq!(explorer.get_relative_index(), 0);
        // Increment and then decrement
        explorer.incr_index();
        explorer.decr_index();
        assert_eq!(explorer.get_index(), 2);
        assert_eq!(explorer.get_relative_index(), 0);
        // Decrement (and wrap)
        explorer.decr_index();
        // Index should be at last element
        assert_eq!(explorer.get_index(), sz - 1);
        assert_eq!(explorer.get_relative_index(), sz - 3);
        // Set index to 5
        explorer.set_index(5);
        assert_eq!(explorer.get_index(), 5);
        assert_eq!(explorer.get_relative_index(), 3);
        // Decr by 2
        explorer.decr_index_by(2);
        assert_eq!(explorer.get_index(), 3);
        assert_eq!(explorer.get_relative_index(), 1);
        // Decr by 2
        explorer.decr_index_by(2);
        // Should decrement actually by 1 (since first two files are hidden)
        assert_eq!(explorer.get_index(), 2);
        assert_eq!(explorer.get_relative_index(), 0);
        // Toggle hidden files
        explorer.toggle_hidden_files();
        assert_eq!(explorer.hidden_files, true);
        // Move index to 0
        explorer.set_index(0);
        assert_eq!(explorer.get_index(), 0);
        // Toggle hidden files
        explorer.toggle_hidden_files();
        // Index should now have been moved to 2
        assert_eq!(explorer.get_index(), 2);
        // Show hidden files
        explorer.toggle_hidden_files();
        // Set index to 5
        explorer.set_index(5);
        // Verify index
        assert_eq!(explorer.get_index(), 5);
        assert_eq!(explorer.get_relative_index(), 5); // Now relative matches
                                                      // Decrement by 6, goes to 0
        explorer.decr_index_by(6);
        assert_eq!(explorer.get_index(), 0);
        assert_eq!(explorer.get_relative_index(), 0); // Now relative matches
                                                      // Toggle; move at first
        explorer.toggle_hidden_files();
        assert_eq!(explorer.hidden_files, false);
        explorer.index_at_first();
        assert_eq!(explorer.get_index(), 2);
        assert_eq!(explorer.get_relative_index(), 0);
        // Verify set index if exceeds
        let sz: usize = explorer.iter_files().count();
        explorer.set_index(sz);
        assert_eq!(explorer.get_index(), sz - 1); // Should be at last element
                                                  // Empty files
        explorer.files.clear();
        explorer.index_at_first();
        assert_eq!(explorer.get_index(), 0);
        assert_eq!(explorer.get_relative_index(), 0);
    }

    fn make_fs_entry(name: &str, is_dir: bool) -> FsEntry {
        let t_now: SystemTime = SystemTime::now();
        match is_dir {
            false => FsEntry::File(FsFile {
                name: name.to_string(),
                abs_path: PathBuf::from(name),
                last_change_time: t_now,
                last_access_time: t_now,
                creation_time: t_now,
                size: 64,
                ftype: None, // File type
                readonly: false,
                symlink: None,             // UNIX only
                user: Some(0),             // UNIX only
                group: Some(0),            // UNIX only
                unix_pex: Some((6, 4, 4)), // UNIX only
            }),
            true => FsEntry::Directory(FsDirectory {
                name: name.to_string(),
                abs_path: PathBuf::from(name),
                last_change_time: t_now,
                last_access_time: t_now,
                creation_time: t_now,
                readonly: false,
                symlink: None,             // UNIX only
                user: Some(0),             // UNIX only
                group: Some(0),            // UNIX only
                unix_pex: Some((7, 5, 5)), // UNIX only
            }),
        }
    }
}
