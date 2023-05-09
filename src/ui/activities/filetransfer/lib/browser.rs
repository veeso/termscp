//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

use std::path::Path;

use remotefs::File;

use crate::explorer::builder::FileExplorerBuilder;
use crate::explorer::{FileExplorer, FileSorting, GroupDirs};
use crate::system::config_client::ConfigClient;

/// File explorer tab
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FileExplorerTab {
    Local,
    Remote,
    FindLocal,  // Find result tab
    FindRemote, // Find result tab
}

/// Describes the explorer tab type
#[derive(Copy, Clone, Debug)]
pub enum FoundExplorerTab {
    Local,
    Remote,
}

/// Browser contains the browser options
pub struct Browser {
    local: FileExplorer,                             // Local File explorer state
    remote: FileExplorer,                            // Remote File explorer state
    found: Option<(FoundExplorerTab, FileExplorer)>, // File explorer for find result
    tab: FileExplorerTab,                            // Current selected tab
    pub sync_browsing: bool,
}

impl Browser {
    /// Build a new `Browser` struct
    pub fn new(cli: &ConfigClient) -> Self {
        Self {
            local: Self::build_local_explorer(cli),
            remote: Self::build_remote_explorer(cli),
            found: None,
            tab: FileExplorerTab::Local,
            sync_browsing: false,
        }
    }

    pub fn local(&self) -> &FileExplorer {
        &self.local
    }

    pub fn local_mut(&mut self) -> &mut FileExplorer {
        &mut self.local
    }

    pub fn remote(&self) -> &FileExplorer {
        &self.remote
    }

    pub fn remote_mut(&mut self) -> &mut FileExplorer {
        &mut self.remote
    }

    pub fn found(&self) -> Option<&FileExplorer> {
        self.found.as_ref().map(|x| &x.1)
    }

    pub fn found_mut(&mut self) -> Option<&mut FileExplorer> {
        self.found.as_mut().map(|x| &mut x.1)
    }

    pub fn set_found(&mut self, tab: FoundExplorerTab, files: Vec<File>, wrkdir: &Path) {
        let mut explorer = Self::build_found_explorer(wrkdir);
        explorer.set_files(files);
        self.found = Some((tab, explorer));
    }

    pub fn del_found(&mut self) {
        self.found = None;
    }

    /// Returns found tab if any
    pub fn found_tab(&self) -> Option<FoundExplorerTab> {
        self.found.as_ref().map(|x| x.0)
    }

    pub fn tab(&self) -> FileExplorerTab {
        self.tab
    }

    /// Update tab value
    pub fn change_tab(&mut self, tab: FileExplorerTab) {
        self.tab = tab;
    }

    /// Invert the current state for the sync browsing
    pub fn toggle_sync_browsing(&mut self) {
        self.sync_browsing = !self.sync_browsing;
    }

    /// Build a file explorer with local host setup
    pub fn build_local_explorer(cli: &ConfigClient) -> FileExplorer {
        let mut builder = Self::build_explorer(cli);
        builder.with_formatter(cli.get_local_file_fmt().as_deref());
        builder.build()
    }

    /// Build a file explorer with remote host setup
    pub fn build_remote_explorer(cli: &ConfigClient) -> FileExplorer {
        let mut builder = Self::build_explorer(cli);
        builder.with_formatter(cli.get_remote_file_fmt().as_deref());
        builder.build()
    }

    /// Build explorer reading configuration from `ConfigClient`
    fn build_explorer(cli: &ConfigClient) -> FileExplorerBuilder {
        let mut builder: FileExplorerBuilder = FileExplorerBuilder::new();
        // Set common keys
        builder
            .with_file_sorting(FileSorting::Name)
            .with_stack_size(16)
            .with_group_dirs(cli.get_group_dirs())
            .with_hidden_files(cli.get_show_hidden_files());
        builder
    }

    /// Build explorer reading from `ConfigClient`, for found result (has some differences)
    fn build_found_explorer(wrkdir: &Path) -> FileExplorer {
        FileExplorerBuilder::new()
            .with_file_sorting(FileSorting::Name)
            .with_group_dirs(Some(GroupDirs::First))
            .with_hidden_files(true)
            .with_stack_size(0)
            .with_formatter(Some(
                format!("{{PATH:36:{}}} {{SYMLINK}}", wrkdir.display()).as_str(),
            ))
            .build()
    }
}
