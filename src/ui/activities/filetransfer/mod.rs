//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// This module is split into files, cause it's just too big
mod actions;
mod components;
mod fswatcher;
mod lib;
mod misc;
mod session;
mod update;
mod view;

// locals
use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::Duration;

// Includes
use chrono::{DateTime, Local};
use lib::browser;
use lib::browser::Browser;
use lib::transfer::{TransferOpts, TransferStates};
use lib::walkdir::WalkdirStates;
use remotefs::RemoteFs;
use session::TransferPayload;
use tempfile::TempDir;
use tuirealm::{Application, EventListenerCfg, NoUserEvent};

use super::{Activity, CROSSTERM_MAX_POLL, Context, ExitReason};
use crate::config::themes::Theme;
use crate::explorer::{FileExplorer, FileSorting};
use crate::filetransfer::{
    FileTransferParams, HostBridgeBuilder, HostBridgeParams, RemoteFsBuilder,
};
use crate::host::HostBridge;
use crate::system::config_client::ConfigClient;
use crate::system::watcher::FsWatcher;

// -- components

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Id {
    ChmodPopup,
    CopyPopup,
    DeletePopup,
    DisconnectPopup,
    ErrorPopup,
    ExecPopup,
    ExplorerFind,
    ExplorerHostBridge,
    ExplorerRemote,
    FatalPopup,
    FileInfoPopup,
    FilterPopup,
    FooterBar,
    GlobalListener,
    GotoPopup,
    KeybindingsPopup,
    Log,
    MkdirPopup,
    NewfilePopup,
    OpenWithPopup,
    ProgressBarFull,
    ProgressBarPartial,
    QuitPopup,
    RenamePopup,
    ReplacePopup,
    ReplacingFilesListPopup,
    SaveAsPopup,
    SortingPopup,
    StatusBarHostBridge,
    StatusBarRemote,
    SymlinkPopup,
    SyncBrowsingMkdirPopup,
    WaitPopup,
    WatchedPathsList,
    WatcherPopup,
}

#[derive(Debug, PartialEq)]
enum Msg {
    PendingAction(PendingActionMsg),
    Transfer(TransferMsg),
    Ui(UiMsg),
    None,
}

#[derive(Debug, PartialEq)]
enum PendingActionMsg {
    CloseReplacePopups,
    CloseSyncBrowsingMkdirPopup,
    MakePendingDirectory,
    TransferPendingFile,
}

#[derive(Debug, PartialEq)]
enum TransferMsg {
    AbortWalkdir,
    AbortTransfer,
    Chmod(remotefs::fs::UnixPex),
    CopyFileTo(String),
    CreateSymlink(String),
    DeleteFile,
    EnterDirectory,
    ExecuteCmd(String),
    GoTo(String),
    GoToParentDirectory,
    GoToPreviousDirectory,
    InitFuzzySearch,
    Mkdir(String),
    NewFile(String),
    OpenFile,
    OpenFileWith(String),
    OpenTextFile,
    ReloadDir,
    RenameFile(String),
    RescanGotoFiles(PathBuf),
    SaveFileAs(String),
    ToggleWatch,
    ToggleWatchFor(usize),
    TransferFile,
}

#[derive(Debug, PartialEq)]
enum UiMsg {
    ChangeFileSorting(FileSorting),
    ChangeTransferWindow,
    CloseChmodPopup,
    CloseCopyPopup,
    CloseDeletePopup,
    CloseDisconnectPopup,
    CloseErrorPopup,
    CloseExecPopup,
    CloseFatalPopup,
    CloseFileInfoPopup,
    CloseFileSortingPopup,
    CloseFilterPopup,
    CloseFindExplorer,
    CloseGotoPopup,
    CloseKeybindingsPopup,
    CloseMkdirPopup,
    CloseNewFilePopup,
    CloseOpenWithPopup,
    CloseQuitPopup,
    CloseRenamePopup,
    CloseSaveAsPopup,
    CloseSymlinkPopup,
    CloseWatchedPathsList,
    CloseWatcherPopup,
    Disconnect,
    FilterFiles(String),
    FuzzySearch(String),
    LogBackTabbed,
    Quit,
    ReplacePopupTabbed,
    ShowChmodPopup,
    ShowCopyPopup,
    ShowDeletePopup,
    ShowDisconnectPopup,
    ShowExecPopup,
    ShowFileInfoPopup,
    ShowFileSortingPopup,
    ShowFilterPopup,
    ShowGotoPopup,
    ShowKeybindingsPopup,
    ShowLogPanel,
    ShowMkdirPopup,
    ShowNewFilePopup,
    ShowOpenWithPopup,
    ShowQuitPopup,
    ShowRenamePopup,
    ShowSaveAsPopup,
    ShowSymlinkPopup,
    ShowWatchedPathsList,
    ShowWatcherPopup,
    ToggleHiddenFiles,
    ToggleSyncBrowsing,
    WindowResized,
}

/// Log level type
enum LogLevel {
    Error,
    Warn,
    Info,
}

/// Log record entry
struct LogRecord {
    pub time: DateTime<Local>,
    pub level: LogLevel,
    pub msg: String,
}

impl LogRecord {
    /// Instantiates a new LogRecord
    pub fn new(level: LogLevel, msg: String) -> LogRecord {
        LogRecord {
            time: Local::now(),
            level,
            msg,
        }
    }
}

/// FileTransferActivity is the data holder for the file transfer activity
pub struct FileTransferActivity {
    /// Exit reason
    exit_reason: Option<ExitReason>,
    /// Context holder
    context: Option<Context>,
    /// Tui-realm application
    app: Application<Id, Msg, NoUserEvent>,
    /// Whether should redraw UI
    redraw: bool,
    /// Host bridge
    host_bridge: Box<dyn HostBridge>,
    /// Remote host client
    client: Box<dyn RemoteFs>,
    /// Browser
    browser: Browser,
    /// Current log lines
    log_records: VecDeque<LogRecord>,
    /// Fuzzy search states
    walkdir: WalkdirStates,
    /// Transfer states
    transfer: TransferStates,
    /// Temporary directory where to store temporary stuff
    cache: Option<TempDir>,
    /// Fs watcher
    fswatcher: Option<FsWatcher>,
    /// host bridge connected
    host_bridge_connected: bool,
    /// remote connected once
    remote_connected: bool,
}

impl FileTransferActivity {
    /// Instantiates a new FileTransferActivity
    pub fn new(
        host_bridge_params: HostBridgeParams,
        remote_params: &FileTransferParams,
        ticks: Duration,
    ) -> Result<Self, String> {
        // Get config client
        let config_client: ConfigClient = Self::init_config_client();
        // init host bridge
        let host_bridge = HostBridgeBuilder::build(host_bridge_params, &config_client)?;
        let host_bridge_connected = host_bridge.is_localhost();
        let enable_fs_watcher = host_bridge.is_localhost();
        Ok(Self {
            exit_reason: None,
            context: None,
            app: Application::init(
                EventListenerCfg::default()
                    .poll_timeout(ticks)
                    .crossterm_input_listener(ticks, CROSSTERM_MAX_POLL),
            ),
            redraw: true,
            host_bridge,
            client: RemoteFsBuilder::build(
                remote_params.protocol,
                remote_params.params.clone(),
                &config_client,
            )?,
            browser: Browser::new(&config_client),
            log_records: VecDeque::with_capacity(256), // 256 events is enough I guess
            walkdir: WalkdirStates::default(),
            transfer: TransferStates::default(),
            cache: match TempDir::new() {
                Ok(d) => Some(d),
                Err(_) => None,
            },
            fswatcher: if enable_fs_watcher {
                FsWatcher::init(Duration::from_secs(5)).ok()
            } else {
                None
            },
            host_bridge_connected,
            remote_connected: false,
        })
    }

    fn host_bridge(&self) -> &FileExplorer {
        self.browser.host_bridge()
    }

    fn host_bridge_mut(&mut self) -> &mut FileExplorer {
        self.browser.host_bridge_mut()
    }

    fn remote(&self) -> &FileExplorer {
        self.browser.remote()
    }

    fn remote_mut(&mut self) -> &mut FileExplorer {
        self.browser.remote_mut()
    }

    fn found(&self) -> Option<&FileExplorer> {
        self.browser.found()
    }

    fn found_mut(&mut self) -> Option<&mut FileExplorer> {
        self.browser.found_mut()
    }

    /// Get file name for a file in cache
    fn get_cache_tmp_name(&self, name: &str, file_type: Option<&str>) -> Option<String> {
        self.cache.as_ref().map(|_| {
            let base: String = format!(
                "{}-{}",
                name,
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            );
            match file_type {
                None => base,
                Some(file_type) => format!("{base}.{file_type}"),
            }
        })
    }

    /// Returns a reference to context
    fn context(&self) -> &Context {
        self.context.as_ref().unwrap()
    }

    /// Returns a mutable reference to context
    fn context_mut(&mut self) -> &mut Context {
        self.context.as_mut().unwrap()
    }

    /// Returns config client reference
    fn config(&self) -> &ConfigClient {
        self.context().config()
    }

    /// Get a reference to `Theme`
    fn theme(&self) -> &Theme {
        self.context().theme_provider().theme()
    }

    /// Map a function to fs watcher if any
    fn map_on_fswatcher<F, T>(&mut self, mapper: F) -> Option<T>
    where
        F: FnOnce(&mut FsWatcher) -> T,
    {
        self.fswatcher.as_mut().map(mapper)
    }
}

/**
 * Activity Trait
 * Keep it clean :)
 * Use methods instead!
 */
impl Activity for FileTransferActivity {
    /// `on_create` is the function which must be called to initialize the activity.
    /// `on_create` must initialize all the data structures used by the activity
    fn on_create(&mut self, context: Context) {
        debug!("Initializing activity...");
        // Set context
        self.context = Some(context);
        // Clear terminal
        if let Err(err) = self.context.as_mut().unwrap().terminal().clear_screen() {
            error!("Failed to clear screen: {}", err);
        }
        // Put raw mode on enabled
        if let Err(err) = self.context_mut().terminal().enable_raw_mode() {
            error!("Failed to enter raw mode: {}", err);
        }
        // Get files at current pwd
        if self.host_bridge.is_localhost() {
            debug!("Reloading host bridge directory");
            self.reload_host_bridge_dir();
        }
        debug!("Read working directory");
        // Configure text editor
        self.setup_text_editor();
        debug!("Setup text editor");
        // init view
        self.init();
        debug!("Initialized view");
        // Verify error state from context
        if let Some(err) = self.context.as_mut().unwrap().error() {
            error!("Fatal error on create: {}", err);
            self.mount_fatal(&err);
        }
        info!("Created FileTransferActivity");
    }

    /// `on_draw` is the function which draws the graphical interface.
    /// This function must be called at each tick to refresh the interface
    fn on_draw(&mut self) {
        // Context must be something
        if self.context.is_none() {
            return;
        }
        // Check if connected to host bridge (popup must be None, otherwise would try reconnecting in loop in case of error)
        if (!self.host_bridge.is_connected() || !self.host_bridge_connected)
            && !self.app.mounted(&Id::FatalPopup)
            && !self.host_bridge.is_localhost()
        {
            let host_bridge_params = self.context().host_bridge_params().unwrap();
            let ft_params = host_bridge_params.unwrap_protocol_params();
            // print params
            let msg: String = Self::get_connection_msg(ft_params);
            // Set init state to connecting popup
            self.mount_blocking_wait(msg.as_str());
            // Connect to remote
            self.connect_to_host_bridge();
            // Redraw
            self.redraw = true;
        }
        // Check if connected to remote (popup must be None, otherwise would try reconnecting in loop in case of error)
        if (!self.client.is_connected() || !self.remote_connected)
            && !self.app.mounted(&Id::FatalPopup)
            && self.host_bridge.is_connected()
        {
            let ftparams = self.context().remote_params().unwrap();
            // print params
            let msg: String = Self::get_connection_msg(&ftparams.params);
            // Set init state to connecting popup
            self.mount_blocking_wait(msg.as_str());
            // Connect to remote
            self.connect_to_remote();
            // Redraw
            self.redraw = true;
        }
        self.tick();
        // poll
        self.poll_watcher();
        // View
        if self.redraw {
            self.view();
        }
    }

    /// `will_umount` is the method which must be able to report to the activity manager, whether
    /// the activity should be terminated or not.
    /// If not, the call will return `None`, otherwise return`Some(ExitReason)`
    fn will_umount(&self) -> Option<&ExitReason> {
        self.exit_reason.as_ref()
    }

    /// `on_destroy` is the function which cleans up runtime variables and data before terminating the activity.
    /// This function must be called once before terminating the activity.
    fn on_destroy(&mut self) -> Option<Context> {
        // Destroy cache
        if let Some(cache) = self.cache.take() {
            if let Err(err) = cache.close() {
                error!("Failed to delete cache: {}", err);
            }
        }
        // Disable raw mode
        if let Err(err) = self.context_mut().terminal().disable_raw_mode() {
            error!("Failed to disable raw mode: {}", err);
        }
        if let Err(err) = self.context_mut().terminal().clear_screen() {
            error!("Failed to clear screen: {}", err);
        }
        // Disconnect client
        if self.client.is_connected() {
            let _ = self.client.disconnect();
        }
        // disconnect host bridge
        if self.host_bridge.is_connected() {
            let _ = self.host_bridge.disconnect();
        }
        self.context.take()
    }
}
