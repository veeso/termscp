//! ## Builder
//!
//! `builder` is the module which provides a builder for FileExplorer

// Locals
use super::formatter::Formatter;
use super::{ExplorerOpts, FileExplorer, FileSorting, GroupDirs};
// Ext
use std::collections::VecDeque;

/// Struct used to create a `FileExplorer`
pub struct FileExplorerBuilder {
    explorer: Option<FileExplorer>,
}

impl FileExplorerBuilder {
    /// Build a new `FileExplorerBuilder`
    pub fn new() -> Self {
        FileExplorerBuilder {
            explorer: Some(FileExplorer::default()),
        }
    }

    /// Take FileExplorer out of builder
    pub fn build(&mut self) -> FileExplorer {
        self.explorer.take().unwrap()
    }

    /// Enable HIDDEN_FILES option
    pub fn with_hidden_files(&mut self, val: bool) -> &mut FileExplorerBuilder {
        if let Some(e) = self.explorer.as_mut() {
            match val {
                true => e.opts.insert(ExplorerOpts::SHOW_HIDDEN_FILES),
                false => e.opts.remove(ExplorerOpts::SHOW_HIDDEN_FILES),
            }
        }
        self
    }

    /// Set sorting method
    pub fn with_file_sorting(&mut self, sorting: FileSorting) -> &mut FileExplorerBuilder {
        if let Some(e) = self.explorer.as_mut() {
            e.sort_by(sorting);
        }
        self
    }

    /// Enable DIRS_FIRST option
    pub fn with_group_dirs(&mut self, group_dirs: Option<GroupDirs>) -> &mut FileExplorerBuilder {
        if let Some(e) = self.explorer.as_mut() {
            e.group_dirs_by(group_dirs);
        }
        self
    }

    /// Set stack size for FileExplorer
    pub fn with_stack_size(&mut self, sz: usize) -> &mut FileExplorerBuilder {
        if let Some(e) = self.explorer.as_mut() {
            e.stack_size = sz;
            e.dirstack = VecDeque::with_capacity(sz);
        }
        self
    }

    /// Set formatter for FileExplorer
    pub fn with_formatter(&mut self, fmt_str: Option<&str>) -> &mut FileExplorerBuilder {
        if let Some(e) = self.explorer.as_mut() {
            if let Some(fmt_str) = fmt_str {
                e.fmt = Formatter::new(fmt_str);
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_fs_explorer_builder_new_default() {
        let explorer: FileExplorer = FileExplorerBuilder::new().build();
        // Verify
        assert!(!explorer.opts.intersects(ExplorerOpts::SHOW_HIDDEN_FILES));
        assert_eq!(explorer.file_sorting, FileSorting::Name); // Default
        assert_eq!(explorer.group_dirs, None);
        assert_eq!(explorer.stack_size, 16);
    }

    #[test]
    fn test_fs_explorer_builder_new_all() {
        let explorer: FileExplorer = FileExplorerBuilder::new()
            .with_file_sorting(FileSorting::ModifyTime)
            .with_group_dirs(Some(GroupDirs::First))
            .with_hidden_files(true)
            .with_stack_size(24)
            .with_formatter(Some("{NAME}"))
            .build();
        // Verify
        assert!(explorer.opts.intersects(ExplorerOpts::SHOW_HIDDEN_FILES));
        assert_eq!(explorer.file_sorting, FileSorting::ModifyTime); // Default
        assert_eq!(explorer.group_dirs, Some(GroupDirs::First));
        assert_eq!(explorer.stack_size, 24);
    }
}
