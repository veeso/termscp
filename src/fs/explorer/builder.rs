//! ## Builder
//!
//! `builder` is the module which provides a builder for FileExplorer

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
use super::{ExplorerOpts, FileExplorer, FileSorting, GroupDirs};
// Ext
use std::collections::VecDeque;

/// ## FileExplorerBuilder
///
/// Struct used to create a `FileExplorer`
pub struct FileExplorerBuilder {
    explorer: Option<FileExplorer>,
}

impl FileExplorerBuilder {
    /// ### new
    ///
    /// Build a new `FileExplorerBuilder`
    pub fn new() -> Self {
        FileExplorerBuilder {
            explorer: Some(FileExplorer::default()),
        }
    }

    /// ### build
    ///
    /// Take FileExplorer out of builder
    pub fn build(&mut self) -> FileExplorer {
        self.explorer.take().unwrap()
    }

    /// ### with_hidden_files
    ///
    /// Enable HIDDEN_FILES option
    pub fn with_hidden_files(&mut self) -> &mut FileExplorerBuilder {
        if let Some(e) = self.explorer.as_mut() {
            e.opts.insert(ExplorerOpts::SHOW_HIDDEN_FILES);
        }
        self
    }

    /// ### with_file_sorting
    ///
    /// Set sorting method
    pub fn with_file_sorting(&mut self, sorting: FileSorting) -> &mut FileExplorerBuilder {
        if let Some(e) = self.explorer.as_mut() {
            e.sort_by(sorting);
        }
        self
    }

    /// ### with_dirs_first
    ///
    /// Enable DIRS_FIRST option
    pub fn with_group_dirs(&mut self, group_dirs: Option<GroupDirs>) -> &mut FileExplorerBuilder {
        if let Some(e) = self.explorer.as_mut() {
            e.group_dirs_by(group_dirs);
        }
        self
    }

    /// ### with_stack_size
    ///
    /// Set stack size for FileExplorer
    pub fn with_stack_size(&mut self, sz: usize) -> &mut FileExplorerBuilder {
        if let Some(e) = self.explorer.as_mut() {
            e.stack_size = sz;
            e.dirstack = VecDeque::with_capacity(sz);
        }
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fs_explorer_builder_new_default() {
        let explorer: FileExplorer = FileExplorerBuilder::new().build();
        // Verify
        assert!(!explorer.opts.intersects(ExplorerOpts::SHOW_HIDDEN_FILES));
        assert_eq!(explorer.file_sorting, FileSorting::ByName); // Default
        assert_eq!(explorer.group_dirs, None);
        assert_eq!(explorer.stack_size, 16);
    }

    #[test]
    fn test_fs_explorer_builder_new_all() {
        let explorer: FileExplorer = FileExplorerBuilder::new()
            .with_file_sorting(FileSorting::ByModifyTime)
            .with_group_dirs(Some(GroupDirs::First))
            .with_hidden_files()
            .with_stack_size(24)
            .build();
        // Verify
        assert!(explorer.opts.intersects(ExplorerOpts::SHOW_HIDDEN_FILES));
        assert_eq!(explorer.file_sorting, FileSorting::ByModifyTime); // Default
        assert_eq!(explorer.group_dirs, Some(GroupDirs::First));
        assert_eq!(explorer.stack_size, 24);
    }
}
