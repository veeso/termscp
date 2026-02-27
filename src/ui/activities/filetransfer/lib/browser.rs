//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

use std::path::Path;

use nucleo::Utf32String;
use remotefs::File;

use super::pane::Pane;
use crate::explorer::builder::FileExplorerBuilder;
use crate::explorer::{FileExplorer, FileSorting};
use crate::system::config_client::ConfigClient;

const FUZZY_SEARCH_THRESHOLD: u16 = 50;

/// File explorer tab
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FileExplorerTab {
    HostBridge,
    Remote,
    FindHostBridge, // Find result tab
    FindRemote,     // Find result tab
}

/// Describes the explorer tab type
#[derive(Copy, Clone, Debug)]
pub enum FoundExplorerTab {
    Local,
    Remote,
}

/// Browser contains the browser options
pub struct Browser {
    local: Pane,
    remote: Pane,
    found: Option<Found>, // File explorer for find result
    tab: FileExplorerTab, // Current selected tab
    pub sync_browsing: bool,
}

impl Browser {
    /// Build a new `Browser` struct
    pub fn new(local: Pane, remote: Pane) -> Self {
        Self {
            local,
            remote,
            found: None,
            tab: FileExplorerTab::HostBridge,
            sync_browsing: false,
        }
    }

    pub fn explorer(&self) -> &FileExplorer {
        match self.tab {
            FileExplorerTab::HostBridge => &self.local.explorer,
            FileExplorerTab::Remote => &self.remote.explorer,
            FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => {
                self.found.as_ref().map(|x| &x.explorer).unwrap()
            }
        }
    }

    pub fn other_explorer_no_found(&self) -> &FileExplorer {
        match self.tab {
            FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => &self.remote.explorer,
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => &self.local.explorer,
        }
    }

    pub fn explorer_mut(&mut self) -> &mut FileExplorer {
        match self.tab {
            FileExplorerTab::HostBridge => &mut self.local.explorer,
            FileExplorerTab::Remote => &mut self.remote.explorer,
            FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => {
                self.found.as_mut().map(|x| &mut x.explorer).unwrap()
            }
        }
    }

    pub fn host_bridge(&self) -> &FileExplorer {
        &self.local.explorer
    }

    pub fn host_bridge_mut(&mut self) -> &mut FileExplorer {
        &mut self.local.explorer
    }

    pub fn remote(&self) -> &FileExplorer {
        &self.remote.explorer
    }

    pub fn remote_mut(&mut self) -> &mut FileExplorer {
        &mut self.remote.explorer
    }

    pub fn found(&self) -> Option<&FileExplorer> {
        self.found.as_ref().map(|x| &x.explorer)
    }

    pub fn found_mut(&mut self) -> Option<&mut FileExplorer> {
        self.found.as_mut().map(|x| &mut x.explorer)
    }

    /// Perform fuzzy search on found tab
    pub fn fuzzy_search(&mut self, needle: &str) {
        if let Some(x) = self.found.as_mut() {
            x.fuzzy_search(needle)
        }
    }

    /// Initialize fuzzy search
    pub fn init_fuzzy_search(&mut self) {
        if let Some(explorer) = self.found_mut() {
            explorer.set_files(vec![]);
        }
    }

    pub fn set_found(&mut self, tab: FoundExplorerTab, files: Vec<File>, wrkdir: &Path) {
        let mut explorer = Self::build_found_explorer(wrkdir);
        explorer.set_files(files.clone());
        self.found = Some(Found {
            tab,
            explorer,
            search_results: files,
        });
    }

    pub fn del_found(&mut self) {
        self.found = None;
    }

    /// Returns found tab if any
    pub fn found_tab(&self) -> Option<FoundExplorerTab> {
        self.found.as_ref().map(|x| x.tab)
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

    /// Toggle terminal for the current tab
    pub fn toggle_terminal(&mut self, terminal: bool) {
        if self.tab == FileExplorerTab::HostBridge {
            self.local.explorer.toggle_terminal(terminal);
        } else if self.tab == FileExplorerTab::Remote {
            self.remote.explorer.toggle_terminal(terminal);
        }
    }

    /// Check if terminal is open for the host bridge tab
    pub fn is_terminal_open_host_bridge(&self) -> bool {
        self.tab == FileExplorerTab::HostBridge && self.local.explorer.terminal_open()
    }

    /// Check if terminal is open for the remote tab
    pub fn is_terminal_open_remote(&self) -> bool {
        self.tab == FileExplorerTab::Remote && self.remote.explorer.terminal_open()
    }

    // -- Pane accessors --

    /// The pane whose filesystem is targeted by the current tab.
    pub fn fs_pane(&self) -> &Pane {
        match self.tab {
            FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => &self.local,
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => &self.remote,
        }
    }

    /// The pane whose filesystem is targeted by the current tab (mutable).
    pub fn fs_pane_mut(&mut self) -> &mut Pane {
        match self.tab {
            FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => &mut self.local,
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => &mut self.remote,
        }
    }

    /// The opposite pane (transfer destination).
    pub fn opposite_pane(&self) -> &Pane {
        match self.tab {
            FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => &self.remote,
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => &self.local,
        }
    }

    /// Direct access to local pane
    pub fn local_pane(&self) -> &Pane {
        &self.local
    }

    /// Direct access to local pane (mutable)
    pub fn local_pane_mut(&mut self) -> &mut Pane {
        &mut self.local
    }

    /// Direct access to remote pane
    pub fn remote_pane(&self) -> &Pane {
        &self.remote
    }

    /// Direct access to remote pane (mutable)
    pub fn remote_pane_mut(&mut self) -> &mut Pane {
        &mut self.remote
    }

    // -- Explorer builders (static helpers) --

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
            .with_file_sorting(FileSorting::None)
            .with_group_dirs(None)
            .with_hidden_files(true)
            .with_stack_size(0)
            .with_formatter(Some(
                format!("{{PATH:36:{}}} {{SYMLINK}}", wrkdir.display()).as_str(),
            ))
            .build()
    }
}

/// Found state
struct Found {
    explorer: FileExplorer,
    /// Search results; original copy of files
    search_results: Vec<File>,
    tab: FoundExplorerTab,
}

impl Found {
    /// Fuzzy search from `search_results` and update `explorer.files` with the results.
    pub fn fuzzy_search(&mut self, needle: &str) {
        let search = Utf32String::from(needle);
        let mut nucleo = nucleo::Matcher::new(nucleo::Config::DEFAULT.match_paths());

        // get scores
        let mut fuzzy_results_with_score = self
            .search_results
            .iter()
            .map(|f| {
                (
                    Utf32String::from(f.path().to_string_lossy().into_owned()),
                    f,
                )
            })
            .filter_map(|(path, file)| {
                nucleo
                    .fuzzy_match(path.slice(..), search.slice(..))
                    .map(|score| (path, file, score))
            })
            .filter(|(_, _, score)| *score >= FUZZY_SEARCH_THRESHOLD)
            .collect::<Vec<_>>();

        // sort by score; highest first
        fuzzy_results_with_score.sort_by(|(_, _, a), (_, _, b)| b.cmp(a));

        // update files
        self.explorer.set_files(
            fuzzy_results_with_score
                .into_iter()
                .map(|(_, file, _)| file.clone())
                .collect(),
        );
    }
}
