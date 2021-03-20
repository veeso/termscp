//! ## Explorer
//!
//! `explorer` is the module which provides an Helper in handling Directory status through

/*
*
*   Copyright (C) 2020-2021 Christian Visintin - christian.visintin1997@gmail.com
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

// Mods
pub(crate) mod builder;
mod formatter;
// Deps
extern crate bitflags;
// Locals
use super::FsEntry;
use formatter::Formatter;
// Ext
use std::cmp::Reverse;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::string::ToString;

bitflags! {
    /// ## ExplorerOpts
    ///
    /// ExplorerOpts are bit options which provides different behaviours to `FileExplorer`
    pub(crate) struct ExplorerOpts: u32 {
        const SHOW_HIDDEN_FILES = 0b00000001;
    }
}

/// ## FileSorting
///
/// FileSorting defines the criteria for sorting files
#[derive(Copy, Clone, PartialEq, std::fmt::Debug)]
pub enum FileSorting {
    ByName,
    ByModifyTime,
    ByCreationTime,
    BySize,
}

/// ## GroupDirs
///
/// GroupDirs defines how directories should be grouped in sorting files
#[derive(PartialEq, std::fmt::Debug)]
pub enum GroupDirs {
    First,
    Last,
}

/// ## FileExplorer
///
/// File explorer states
pub struct FileExplorer {
    pub wrkdir: PathBuf,                      // Current directory
    pub(crate) dirstack: VecDeque<PathBuf>,   // Stack of visited directory (max 16)
    pub(crate) stack_size: usize,             // Directory stack size
    pub(crate) file_sorting: FileSorting,     // File sorting criteria
    pub(crate) group_dirs: Option<GroupDirs>, // If Some, defines how to group directories
    pub(crate) opts: ExplorerOpts,            // Explorer options
    pub(crate) fmt: Formatter,                // FsEntry formatter
    index: usize,                             // Selected file
    files: Vec<FsEntry>,                      // Files in directory
}

impl Default for FileExplorer {
    fn default() -> Self {
        FileExplorer {
            wrkdir: PathBuf::from("/"),
            dirstack: VecDeque::with_capacity(16),
            stack_size: 16,
            file_sorting: FileSorting::ByName,
            group_dirs: None,
            opts: ExplorerOpts::empty(),
            fmt: Formatter::default(),
            index: 0,
            files: Vec::new(),
        }
    }
}

impl FileExplorer {
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
    /// This method will also sort entries based on current options
    /// Once all sorting have been performed, index is moved to first valid entry.
    pub fn set_files(&mut self, files: Vec<FsEntry>) {
        self.files = files;
        // Sort
        self.sort();
        // Reset index
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
    pub fn iter_files(&self) -> impl Iterator<Item = &FsEntry> + '_ {
        // Filter
        let opts: ExplorerOpts = self.opts;
        Box::new(self.files.iter().filter(move |x| {
            // If true, element IS NOT filtered
            let mut pass: bool = true;
            // If hidden files SHOULDN'T be shown, AND pass with not hidden
            if !opts.intersects(ExplorerOpts::SHOW_HIDDEN_FILES) {
                pass &= !x.is_hidden();
            }
            pass
        }))
    }

    /// ### iter_files_all
    ///
    /// Iterate all files; doesn't care about options
    pub fn iter_files_all(&self) -> impl Iterator<Item = &FsEntry> + '_ {
        Box::new(self.files.iter())
    }

    /// ### get_current_file
    ///
    /// Get file at index
    pub fn get_current_file(&self) -> Option<&FsEntry> {
        self.files.get(self.index)
    }

    /// ### get
    /// 
    /// Get file at index
    pub fn get(&self, idx: usize) -> Option<&FsEntry> {
        self.files.get(idx)
    }

    // Formatting

    /// ### fmt_file
    ///
    /// Format a file entry
    pub fn fmt_file(&self, entry: &FsEntry) -> String {
        self.fmt.fmt(entry)
    }

    // Sorting

    /// ### sort_by
    ///
    /// Choose sorting method; then sort files
    pub fn sort_by(&mut self, sorting: FileSorting) {
        // If method HAS ACTUALLY CHANGED, sort (performance!)
        if self.file_sorting != sorting {
            self.file_sorting = sorting;
            self.sort();
        }
    }

    /// ### get_file_sorting
    ///
    /// Get current file sorting method
    pub fn get_file_sorting(&self) -> FileSorting {
        self.file_sorting
    }

    /// ### group_dirs_by
    ///
    /// Choose group dirs method; then sort files
    pub fn group_dirs_by(&mut self, group_dirs: Option<GroupDirs>) {
        // If method HAS ACTUALLY CHANGED, sort (performance!)
        if self.group_dirs != group_dirs {
            self.group_dirs = group_dirs;
            self.sort();
        }
    }

    /// ### sort
    ///
    /// Sort files based on Explorer options.
    fn sort(&mut self) {
        // Choose sorting method
        match &self.file_sorting {
            FileSorting::ByName => self.sort_files_by_name(),
            FileSorting::ByCreationTime => self.sort_files_by_creation_time(),
            FileSorting::ByModifyTime => self.sort_files_by_mtime(),
            FileSorting::BySize => self.sort_files_by_size(),
        }
        // Directories first (NOTE: MUST COME AFTER OTHER SORTING)
        // Group directories if necessary
        if let Some(group_dirs) = &self.group_dirs {
            match group_dirs {
                GroupDirs::First => self.sort_files_directories_first(),
                GroupDirs::Last => self.sort_files_directories_last(),
            }
        }
    }

    /// ### sort_files_by_name
    ///
    /// Sort explorer files by their name. All names are converted to lowercase
    fn sort_files_by_name(&mut self) {
        self.files
            .sort_by_key(|x: &FsEntry| x.get_name().to_lowercase());
    }

    /// ### sort_files_by_mtime
    ///
    /// Sort files by mtime; the newest comes first
    fn sort_files_by_mtime(&mut self) {
        self.files.sort_by(|a: &FsEntry, b: &FsEntry| {
            b.get_last_change_time().cmp(&a.get_last_change_time())
        });
    }

    /// ### sort_files_by_creation_time
    ///
    /// Sort files by creation time; the newest comes first
    fn sort_files_by_creation_time(&mut self) {
        self.files
            .sort_by_key(|b: &FsEntry| Reverse(b.get_creation_time()));
    }

    /// ### sort_files_by_size
    ///
    /// Sort files by size
    fn sort_files_by_size(&mut self) {
        self.files.sort_by_key(|b: &FsEntry| Reverse(b.get_size()));
    }

    /// ### sort_files_directories_first
    ///
    /// Sort files; directories come first
    fn sort_files_directories_first(&mut self) {
        self.files.sort_by_key(|x: &FsEntry| x.is_file());
    }

    /// ### sort_files_directories_last
    ///
    /// Sort files; directories come last
    fn sort_files_directories_last(&mut self) {
        self.files.sort_by_key(|x: &FsEntry| x.is_dir());
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
                if !self.opts.intersects(ExplorerOpts::SHOW_HIDDEN_FILES) {
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
                if !self.opts.intersects(ExplorerOpts::SHOW_HIDDEN_FILES) {
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
        match self.opts.intersects(ExplorerOpts::SHOW_HIDDEN_FILES) {
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

    /// ### set_abs_index
    ///
    /// Set absolute index
    pub fn set_abs_index(&mut self, idx: usize) {
        self.index = match idx >= self.files.len() {
            true => match self.files.len() {
                0 => 0,
                _ => self.files.len() - 1,
            },
            false => idx,
        };
    }

    /// ### toggle_hidden_files
    ///
    /// Enable/disable hidden files
    pub fn toggle_hidden_files(&mut self) {
        self.opts.toggle(ExplorerOpts::SHOW_HIDDEN_FILES);
        // Adjust index
        if self.index < self.get_first_valid_index() {
            self.index_at_first();
        }
    }
}

// Traits

impl ToString for FileSorting {
    fn to_string(&self) -> String {
        String::from(match self {
            FileSorting::ByCreationTime => "by_creation_time",
            FileSorting::ByModifyTime => "by_mtime",
            FileSorting::ByName => "by_name",
            FileSorting::BySize => "by_size",
        })
    }
}

impl FromStr for FileSorting {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "by_creation_time" => Ok(FileSorting::ByCreationTime),
            "by_mtime" => Ok(FileSorting::ByModifyTime),
            "by_name" => Ok(FileSorting::ByName),
            "by_size" => Ok(FileSorting::BySize),
            _ => Err(()),
        }
    }
}

impl ToString for GroupDirs {
    fn to_string(&self) -> String {
        String::from(match self {
            GroupDirs::First => "first",
            GroupDirs::Last => "last",
        })
    }
}

impl FromStr for GroupDirs {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "first" => Ok(GroupDirs::First),
            "last" => Ok(GroupDirs::Last),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::fs::{FsDirectory, FsFile};
    use crate::utils::fmt::fmt_time;

    use std::thread::sleep;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_fs_explorer_new() {
        let explorer: FileExplorer = FileExplorer::default();
        // Verify
        assert_eq!(explorer.dirstack.len(), 0);
        assert_eq!(explorer.files.len(), 0);
        assert_eq!(explorer.opts, ExplorerOpts::empty());
        assert_eq!(explorer.wrkdir, PathBuf::from("/"));
        assert_eq!(explorer.stack_size, 16);
        assert_eq!(explorer.index, 0);
        assert_eq!(explorer.group_dirs, None);
        assert_eq!(explorer.file_sorting, FileSorting::ByName);
        assert_eq!(explorer.get_file_sorting(), FileSorting::ByName);
    }

    #[test]
    fn test_fs_explorer_stack() {
        let mut explorer: FileExplorer = FileExplorer::default();
        explorer.stack_size = 2;
        explorer.dirstack = VecDeque::with_capacity(2);
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
    fn test_fs_explorer_files() {
        let mut explorer: FileExplorer = FileExplorer::default();
        // Don't show hidden files
        explorer.opts.remove(ExplorerOpts::SHOW_HIDDEN_FILES);
        // Create files
        explorer.set_files(vec![
            make_fs_entry("README.md", false),
            make_fs_entry("src/", true),
            make_fs_entry(".git/", true),
            make_fs_entry("CONTRIBUTING.md", false),
            make_fs_entry("codecov.yml", false),
            make_fs_entry(".gitignore", false),
        ]);
        assert!(explorer.get_current_file().is_some());
        assert!(explorer.get(0).is_some());
        assert!(explorer.get(100).is_none());
        assert_eq!(explorer.count(), 6);
        // Verify (files are sorted by name)
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
    fn test_fs_explorer_index() {
        let mut explorer: FileExplorer = FileExplorer::default();
        explorer.opts.remove(ExplorerOpts::SHOW_HIDDEN_FILES);
        // Create files (files are then sorted by name DEFAULT)
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
        // Get first index
        assert_eq!(explorer.get_first_valid_index(), 2);
        // Index should be 2 now; files hidden; this happens because `index_at_first` is called after loading files
        assert_eq!(explorer.get_index(), 2);
        assert_eq!(explorer.get_relative_index(), 0); // Relative index should be 0
        assert_eq!(
            explorer.opts.intersects(ExplorerOpts::SHOW_HIDDEN_FILES),
            false
        );
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
        assert_eq!(
            explorer.opts.intersects(ExplorerOpts::SHOW_HIDDEN_FILES),
            true
        );
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
        assert_eq!(
            explorer.opts.intersects(ExplorerOpts::SHOW_HIDDEN_FILES),
            false
        );
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

    #[test]
    fn test_fs_explorer_sort_by_name() {
        let mut explorer: FileExplorer = FileExplorer::default();
        // Create files (files are then sorted by name)
        explorer.set_files(vec![
            make_fs_entry("README.md", false),
            make_fs_entry("src/", true),
            make_fs_entry("CONTRIBUTING.md", false),
            make_fs_entry("CODE_OF_CONDUCT.md", false),
            make_fs_entry("CHANGELOG.md", false),
            make_fs_entry("LICENSE", false),
            make_fs_entry("Cargo.toml", false),
            make_fs_entry("Cargo.lock", false),
            make_fs_entry("codecov.yml", false),
        ]);
        explorer.sort_by(FileSorting::ByName);
        // First entry should be "Cargo.lock"
        assert_eq!(explorer.files.get(0).unwrap().get_name(), "Cargo.lock");
        // Last should be "src/"
        assert_eq!(explorer.files.get(8).unwrap().get_name(), "src/");
    }

    #[test]
    fn test_fs_explorer_sort_by_mtime() {
        let mut explorer: FileExplorer = FileExplorer::default();
        let entry1: FsEntry = make_fs_entry("README.md", false);
        // Wait 1 sec
        sleep(Duration::from_secs(1));
        let entry2: FsEntry = make_fs_entry("CODE_OF_CONDUCT.md", false);
        // Create files (files are then sorted by name)
        explorer.set_files(vec![entry1, entry2]);
        explorer.sort_by(FileSorting::ByModifyTime);
        // First entry should be "CODE_OF_CONDUCT.md"
        assert_eq!(
            explorer.files.get(0).unwrap().get_name(),
            "CODE_OF_CONDUCT.md"
        );
        // Last should be "src/"
        assert_eq!(explorer.files.get(1).unwrap().get_name(), "README.md");
    }

    #[test]
    fn test_fs_explorer_set_abs_index() {
        let mut explorer: FileExplorer = FileExplorer::default();
        explorer.opts.remove(ExplorerOpts::SHOW_HIDDEN_FILES);
        // Create files (files are then sorted by name DEFAULT)
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
        explorer.set_abs_index(3);
        assert_eq!(explorer.get_index(), 3);
        explorer.set_abs_index(12);
        assert_eq!(explorer.get_index(), 10);
        explorer.set_files(vec![]);
        explorer.set_abs_index(12);
        assert_eq!(explorer.get_index(), 0);
    }

    #[test]
    fn test_fs_explorer_sort_by_creation_time() {
        let mut explorer: FileExplorer = FileExplorer::default();
        let entry1: FsEntry = make_fs_entry("README.md", false);
        // Wait 1 sec
        sleep(Duration::from_secs(1));
        let entry2: FsEntry = make_fs_entry("CODE_OF_CONDUCT.md", false);
        // Create files (files are then sorted by name)
        explorer.set_files(vec![entry1, entry2]);
        explorer.sort_by(FileSorting::ByCreationTime);
        // First entry should be "CODE_OF_CONDUCT.md"
        assert_eq!(
            explorer.files.get(0).unwrap().get_name(),
            "CODE_OF_CONDUCT.md"
        );
        // Last should be "src/"
        assert_eq!(explorer.files.get(1).unwrap().get_name(), "README.md");
    }

    #[test]
    fn test_fs_explorer_sort_by_size() {
        let mut explorer: FileExplorer = FileExplorer::default();
        // Create files (files are then sorted by name)
        explorer.set_files(vec![
            make_fs_entry_with_size("README.md", false, 1024),
            make_fs_entry("src/", true),
            make_fs_entry_with_size("CONTRIBUTING.md", false, 256),
        ]);
        explorer.sort_by(FileSorting::BySize);
        // Directory has size 4096
        assert_eq!(explorer.files.get(0).unwrap().get_name(), "src/");
        assert_eq!(explorer.files.get(1).unwrap().get_name(), "README.md");
        assert_eq!(explorer.files.get(2).unwrap().get_name(), "CONTRIBUTING.md");
    }

    #[test]
    fn test_fs_explorer_sort_by_name_and_dirs_first() {
        let mut explorer: FileExplorer = FileExplorer::default();
        // Create files (files are then sorted by name)
        explorer.set_files(vec![
            make_fs_entry("README.md", false),
            make_fs_entry("src/", true),
            make_fs_entry("docs/", true),
            make_fs_entry("CONTRIBUTING.md", false),
            make_fs_entry("CODE_OF_CONDUCT.md", false),
            make_fs_entry("CHANGELOG.md", false),
            make_fs_entry("LICENSE", false),
            make_fs_entry("Cargo.toml", false),
            make_fs_entry("Cargo.lock", false),
            make_fs_entry("codecov.yml", false),
        ]);
        explorer.sort_by(FileSorting::ByName);
        explorer.group_dirs_by(Some(GroupDirs::First));
        // First entry should be "docs"
        assert_eq!(explorer.files.get(0).unwrap().get_name(), "docs/");
        assert_eq!(explorer.files.get(1).unwrap().get_name(), "src/");
        // 3rd is file first for alphabetical order
        assert_eq!(explorer.files.get(2).unwrap().get_name(), "Cargo.lock");
        // Last should be "README.md" (last file for alphabetical ordening)
        assert_eq!(explorer.files.get(9).unwrap().get_name(), "README.md");
    }

    #[test]
    fn test_fs_explorer_sort_by_name_and_dirs_last() {
        let mut explorer: FileExplorer = FileExplorer::default();
        // Create files (files are then sorted by name)
        explorer.set_files(vec![
            make_fs_entry("README.md", false),
            make_fs_entry("src/", true),
            make_fs_entry("docs/", true),
            make_fs_entry("CONTRIBUTING.md", false),
            make_fs_entry("CODE_OF_CONDUCT.md", false),
            make_fs_entry("CHANGELOG.md", false),
            make_fs_entry("LICENSE", false),
            make_fs_entry("Cargo.toml", false),
            make_fs_entry("Cargo.lock", false),
            make_fs_entry("codecov.yml", false),
        ]);
        explorer.sort_by(FileSorting::ByName);
        explorer.group_dirs_by(Some(GroupDirs::Last));
        // Last entry should be "src"
        assert_eq!(explorer.files.get(8).unwrap().get_name(), "docs/");
        assert_eq!(explorer.files.get(9).unwrap().get_name(), "src/");
        // first is file for alphabetical order
        assert_eq!(explorer.files.get(0).unwrap().get_name(), "Cargo.lock");
        // Last in files should be "README.md" (last file for alphabetical ordening)
        assert_eq!(explorer.files.get(7).unwrap().get_name(), "README.md");
    }

    #[test]
    fn test_fs_explorer_fmt() {
        let explorer: FileExplorer = FileExplorer::default();
        // Create fs entry
        let t: SystemTime = SystemTime::now();
        let entry: FsEntry = FsEntry::File(FsFile {
            name: String::from("bar.txt"),
            abs_path: PathBuf::from("/bar.txt"),
            last_change_time: t,
            last_access_time: t,
            creation_time: t,
            size: 8192,
            readonly: false,
            ftype: Some(String::from("txt")),
            symlink: None,             // UNIX only
            user: Some(0),             // UNIX only
            group: Some(0),            // UNIX only
            unix_pex: Some((6, 4, 4)), // UNIX only
        });
        #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
        assert_eq!(
            explorer.fmt_file(&entry),
            format!(
                "bar.txt                  -rw-r--r-- root         8.2 KB     {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
        #[cfg(target_os = "windows")]
        assert_eq!(
            explorer.fmt_file(&entry),
            format!(
                "bar.txt                  -rw-r--r-- 0            8.2 KB     {}",
                fmt_time(t, "%b %d %Y %H:%M")
            )
        );
    }

    #[test]
    fn test_fs_explorer_to_string_from_str_traits() {
        // File Sorting
        assert_eq!(FileSorting::ByCreationTime.to_string(), "by_creation_time");
        assert_eq!(FileSorting::ByModifyTime.to_string(), "by_mtime");
        assert_eq!(FileSorting::ByName.to_string(), "by_name");
        assert_eq!(FileSorting::BySize.to_string(), "by_size");
        assert_eq!(
            FileSorting::from_str("by_creation_time").ok().unwrap(),
            FileSorting::ByCreationTime
        );
        assert_eq!(
            FileSorting::from_str("by_mtime").ok().unwrap(),
            FileSorting::ByModifyTime
        );
        assert_eq!(
            FileSorting::from_str("by_name").ok().unwrap(),
            FileSorting::ByName
        );
        assert_eq!(
            FileSorting::from_str("by_size").ok().unwrap(),
            FileSorting::BySize
        );
        assert!(FileSorting::from_str("omar").is_err());
        // Group dirs
        assert_eq!(GroupDirs::First.to_string(), "first");
        assert_eq!(GroupDirs::Last.to_string(), "last");
        assert_eq!(GroupDirs::from_str("first").ok().unwrap(), GroupDirs::First);
        assert_eq!(GroupDirs::from_str("last").ok().unwrap(), GroupDirs::Last);
        assert!(GroupDirs::from_str("omar").is_err());
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

    fn make_fs_entry_with_size(name: &str, is_dir: bool, size: usize) -> FsEntry {
        let t_now: SystemTime = SystemTime::now();
        match is_dir {
            false => FsEntry::File(FsFile {
                name: name.to_string(),
                abs_path: PathBuf::from(name),
                last_change_time: t_now,
                last_access_time: t_now,
                creation_time: t_now,
                size: size,
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
