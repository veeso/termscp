//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

// Submodules
mod actions;
mod components;
mod config;
mod update;
mod view;

// Locals
use super::{Activity, Context, ExitReason};
use crate::config::themes::Theme;
use crate::system::config_client::ConfigClient;
use crate::system::theme_provider::ThemeProvider;
// Ext
use std::time::Duration;
use tuirealm::listener::EventListenerCfg;
use tuirealm::props::Color;
use tuirealm::{application::PollStrategy, Application, NoUserEvent, Update};

// -- components
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Id {
    Common(IdCommon),
    Config(IdConfig),
    Ssh(IdSsh),
    Theme(IdTheme),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum IdCommon {
    ErrorPopup,
    Footer,
    GlobalListener,
    Header,
    Keybindings,
    QuitPopup,
    SavePopup,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum IdConfig {
    CheckUpdates,
    DefaultProtocol,
    GroupDirs,
    HiddenFiles,
    LocalFileFmt,
    NotificationsEnabled,
    NotificationsThreshold,
    PromptOnFileReplace,
    RemoteFileFmt,
    SshConfig,
    TextEditor,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum IdSsh {
    DelSshKeyPopup,
    SshHost,
    SshKeys,
    SshUsername,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum IdTheme {
    AuthAddress,
    AuthBookmarks,
    AuthPassword,
    AuthPort,
    AuthProtocol,
    AuthRecentHosts,
    AuthTitle,
    AuthUsername,
    ExplorerLocalBg,
    ExplorerLocalFg,
    ExplorerLocalHg,
    ExplorerRemoteBg,
    ExplorerRemoteFg,
    ExplorerRemoteHg,
    LogBg,
    LogWindow,
    MiscError,
    MiscInfo,
    MiscInput,
    MiscKeys,
    MiscQuit,
    MiscSave,
    MiscTitle,
    MiscWarn,
    ProgBarFull,
    ProgBarPartial,
    StatusHidden,
    StatusSorting,
    StatusSync,
    TransferTitle,
    TransferTitle2,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    Common(CommonMsg),
    Config(ConfigMsg),
    Ssh(SshMsg),
    Theme(ThemeMsg),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommonMsg {
    ChangeLayout,
    CloseErrorPopup,
    CloseKeybindingsPopup,
    CloseQuitPopup,
    CloseSavePopup,
    Quit,
    RevertChanges,
    SaveAndQuit,
    SaveConfig,
    ShowKeybindings,
    ShowQuitPopup,
    ShowSavePopup,
    WindowResized,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigMsg {
    CheckUpdatesBlurDown,
    CheckUpdatesBlurUp,
    ConfigChanged,
    DefaultProtocolBlurDown,
    DefaultProtocolBlurUp,
    GroupDirsBlurDown,
    GroupDirsBlurUp,
    HiddenFilesBlurDown,
    HiddenFilesBlurUp,
    LocalFileFmtBlurDown,
    LocalFileFmtBlurUp,
    NotificationsEnabledBlurDown,
    NotificationsEnabledBlurUp,
    NotificationsThresholdBlurDown,
    NotificationsThresholdBlurUp,
    PromptOnFileReplaceBlurDown,
    PromptOnFileReplaceBlurUp,
    RemoteFileFmtBlurDown,
    RemoteFileFmtBlurUp,
    SshConfigBlurDown,
    SshConfigBlurUp,
    TextEditorBlurDown,
    TextEditorBlurUp,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SshMsg {
    CloseDelSshKeyPopup,
    CloseNewSshKeyPopup,
    DeleteSshKey,
    EditSshKey(usize),
    SaveSshKey,
    ShowDelSshKeyPopup,
    ShowNewSshKeyPopup,
    SshHostBlur,
    SshUsernameBlur,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThemeMsg {
    AuthAddressBlurDown,
    AuthAddressBlurUp,
    AuthBookmarksBlurDown,
    AuthBookmarksBlurUp,
    AuthPasswordBlurDown,
    AuthPasswordBlurUp,
    AuthPortBlurDown,
    AuthPortBlurUp,
    AuthProtocolBlurDown,
    AuthProtocolBlurUp,
    AuthRecentHostsBlurDown,
    AuthRecentHostsBlurUp,
    AuthUsernameBlurDown,
    AuthUsernameBlurUp,
    ColorChanged(IdTheme, Color),
    ExplorerLocalBgBlurDown,
    ExplorerLocalBgBlurUp,
    ExplorerLocalFgBlurDown,
    ExplorerLocalFgBlurUp,
    ExplorerLocalHgBlurDown,
    ExplorerLocalHgBlurUp,
    ExplorerRemoteBgBlurDown,
    ExplorerRemoteBgBlurUp,
    ExplorerRemoteFgBlurDown,
    ExplorerRemoteFgBlurUp,
    ExplorerRemoteHgBlurDown,
    ExplorerRemoteHgBlurUp,
    LogBgBlurDown,
    LogBgBlurUp,
    LogWindowBlurDown,
    LogWindowBlurUp,
    MiscErrorBlurDown,
    MiscErrorBlurUp,
    MiscInfoBlurDown,
    MiscInfoBlurUp,
    MiscInputBlurDown,
    MiscInputBlurUp,
    MiscKeysBlurDown,
    MiscKeysBlurUp,
    MiscQuitBlurDown,
    MiscQuitBlurUp,
    MiscSaveBlurDown,
    MiscSaveBlurUp,
    MiscWarnBlurDown,
    MiscWarnBlurUp,
    ProgBarFullBlurDown,
    ProgBarFullBlurUp,
    ProgBarPartialBlurDown,
    ProgBarPartialBlurUp,
    StatusHiddenBlurDown,
    StatusHiddenBlurUp,
    StatusSortingBlurDown,
    StatusSortingBlurUp,
    StatusSyncBlurDown,
    StatusSyncBlurUp,
}

// -- store
const STORE_CONFIG_CHANGED: &str = "SETUP_CONFIG_CHANGED";

/// ### ViewLayout
///
/// Current view layout
#[derive(PartialEq)]
pub enum ViewLayout {
    SetupForm,
    SshKeys,
    Theme,
}

/// Setup activity states holder
pub struct SetupActivity {
    app: Application<Id, Msg, NoUserEvent>,
    exit_reason: Option<ExitReason>,
    context: Option<Context>, // Context holder
    layout: ViewLayout,       // View layout
    redraw: bool,
}

impl SetupActivity {
    pub fn new(ticks: Duration) -> Self {
        Self {
            app: Application::init(
                EventListenerCfg::default()
                    .default_input_listener(ticks)
                    .poll_timeout(ticks),
            ),
            exit_reason: None,
            context: None,
            layout: ViewLayout::SetupForm,
            redraw: true, // Draw at first `on_draw`
        }
    }

    /// Returns a reference to context
    fn context(&self) -> &Context {
        self.context.as_ref().unwrap()
    }

    /// Returns a mutable reference to context
    fn context_mut(&mut self) -> &mut Context {
        self.context.as_mut().unwrap()
    }

    fn config(&self) -> &ConfigClient {
        self.context().config()
    }

    fn config_mut(&mut self) -> &mut ConfigClient {
        self.context_mut().config_mut()
    }

    fn theme(&self) -> &Theme {
        self.context().theme_provider().theme()
    }

    fn theme_mut(&mut self) -> &mut Theme {
        self.context_mut().theme_provider_mut().theme_mut()
    }

    fn theme_provider(&mut self) -> &mut ThemeProvider {
        self.context_mut().theme_provider_mut()
    }

    /// Returns whether config has changed
    fn config_changed(&self) -> bool {
        self.context()
            .store()
            .get_boolean(STORE_CONFIG_CHANGED)
            .unwrap_or(false)
    }

    /// Set value for config changed key into the store
    fn set_config_changed(&mut self, changed: bool) {
        self.context_mut()
            .store_mut()
            .set_boolean(STORE_CONFIG_CHANGED, changed);
    }
}

impl Activity for SetupActivity {
    /// `on_create` is the function which must be called to initialize the activity.
    /// `on_create` must initialize all the data structures used by the activity
    /// Context is taken from activity manager and will be released only when activity is destroyed
    fn on_create(&mut self, context: Context) {
        // Set context
        self.context = Some(context);
        // Clear terminal
        if let Err(err) = self.context.as_mut().unwrap().terminal().clear_screen() {
            error!("Failed to clear screen: {}", err);
        }
        // Set config changed to false
        self.set_config_changed(false);
        // Put raw mode on enabled
        if let Err(err) = self.context_mut().terminal().enable_raw_mode() {
            error!("Failed to enter raw mode: {}", err);
        }
        // Init view
        self.init(ViewLayout::SetupForm);
        // Verify error state from context
        if let Some(err) = self.context.as_mut().unwrap().error() {
            self.mount_error(err.as_str());
        }
    }

    /// `on_draw` is the function which draws the graphical interface.
    /// This function must be called at each tick to refresh the interface
    fn on_draw(&mut self) {
        // Context must be something
        if self.context.is_none() {
            return;
        }
        match self.app.tick(PollStrategy::UpTo(3)) {
            Ok(messages) => {
                if !messages.is_empty() {
                    self.redraw = true;
                }
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = self.update(msg);
                    }
                }
            }
            Err(err) => {
                self.mount_error(format!("Application error: {}", err));
            }
        }
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
    /// This function finally releases the context
    fn on_destroy(&mut self) -> Option<Context> {
        // Disable raw mode
        if let Err(err) = self.context_mut().terminal().disable_raw_mode() {
            error!("Failed to disable raw mode: {}", err);
        }
        if let Err(err) = self.context_mut().terminal().clear_screen() {
            error!("Failed to clear screen: {}", err);
        }
        self.context.take()
    }
}
