//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

use std::path::Path;

use nucleo::Utf32String;
use remotefs::File;

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
    host_bridge: FileExplorer, // Local File explorer state
    remote: FileExplorer,      // Remote File explorer state
    found: Option<Found>,      // File explorer for find result
    tab: FileExplorerTab,      // Current selected tab
    pub sync_browsing: bool,
}

impl Browser {
    /// Build a new `Browser` struct
    pub fn new(cli: &ConfigClient) -> Self {
        Self {
            host_bridge: Self::build_local_explorer(cli),
            remote: Self::build_remote_explorer(cli),
            found: None,
            tab: FileExplorerTab::HostBridge,
            sync_browsing: false,
        }
    }

    pub fn explorer(&self) -> &FileExplorer {
        match self.tab {
            FileExplorerTab::HostBridge => &self.host_bridge,
            FileExplorerTab::Remote => &self.remote,
            FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => {
                self.found.as_ref().map(|x| &x.explorer).unwrap()
            }
        }
    }

    pub fn other_explorer_no_found(&self) -> &FileExplorer {
        match self.tab {
            FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => &self.remote,
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => &self.host_bridge,
        }
    }

    pub fn explorer_mut(&mut self) -> &mut FileExplorer {
        match self.tab {
            FileExplorerTab::HostBridge => &mut self.host_bridge,
            FileExplorerTab::Remote => &mut self.remote,
            FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => {
                self.found.as_mut().map(|x| &mut x.explorer).unwrap()
            }
        }
    }

    pub fn host_bridge(&self) -> &FileExplorer {
        &self.host_bridge
    }

    pub fn host_bridge_mut(&mut self) -> &mut FileExplorer {
        &mut self.host_bridge
    }

    pub fn remote(&self) -> &FileExplorer {
        &self.remote
    }

    pub fn remote_mut(&mut self) -> &mut FileExplorer {
        &mut self.remote
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
