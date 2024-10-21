//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

// Sub modules
mod bookmarks;
mod components;
mod misc;
mod update;
mod view;

// locals
// Includes
use std::time::Duration;

use tuirealm::application::PollStrategy;
use tuirealm::listener::EventListenerCfg;
use tuirealm::{Application, NoUserEvent, Update};

use super::{Activity, Context, ExitReason, CROSSTERM_MAX_POLL};
use crate::config::themes::Theme;
use crate::filetransfer::{FileTransferParams, FileTransferProtocol};
use crate::system::bookmarks_client::BookmarksClient;
use crate::system::config_client::ConfigClient;

// host bridge protocol radio
const HOST_BRIDGE_RADIO_PROTOCOL_LOCALHOST: usize = 0;
const HOST_BRIDGE_RADIO_PROTOCOL_SFTP: usize = 1;
const HOST_BRIDGE_RADIO_PROTOCOL_SCP: usize = 2;
const HOST_BRIDGE_RADIO_PROTOCOL_FTP: usize = 3;
const HOST_BRIDGE_RADIO_PROTOCOL_FTPS: usize = 4;
const HOST_BRIDGE_RADIO_PROTOCOL_S3: usize = 5;
const HOST_BRIDGE_RADIO_PROTOCOL_KUBE: usize = 6;
const HOST_BRIDGE_RADIO_PROTOCOL_WEBDAV: usize = 7;
const HOST_BRIDGE_RADIO_PROTOCOL_SMB: usize = 8; // Keep as last

// remote protocol radio
const REMOTE_RADIO_PROTOCOL_SFTP: usize = 0;
const REMOTE_RADIO_PROTOCOL_SCP: usize = 1;
const REMOTE_RADIO_PROTOCOL_FTP: usize = 2;
const REMOTE_RADIO_PROTOCOL_FTPS: usize = 3;
const REMOTE_RADIO_PROTOCOL_S3: usize = 4;
const REMOTE_RADIO_PROTOCOL_KUBE: usize = 5;
const REMOTE_RADIO_PROTOCOL_WEBDAV: usize = 6;
const REMOTE_RADIO_PROTOCOL_SMB: usize = 7; // Keep as last

// -- components
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    BookmarkName,
    BookmarkSavePassword,
    BookmarksList,
    DeleteBookmarkPopup,
    DeleteRecentPopup,
    ErrorPopup,
    GlobalListener,
    HelpFooter,
    HostBridge(AuthFormId),
    InfoPopup,
    InstallUpdatePopup,
    Keybindings,
    NewVersionChangelog,
    NewVersionDisclaimer,
    QuitPopup,
    RecentsList,
    Remote(AuthFormId),
    Subtitle,
    Title,
    WaitPopup,
    WindowSizeError,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum AuthFormId {
    Address,
    KubeNamespace,
    KubeClusterUrl,
    KubeUsername,
    KubeClientCert,
    KubeClientKey,
    LocalDirectory,
    Password,
    Port,
    Protocol,
    RemoteDirectory,
    S3AccessKey,
    S3Bucket,
    S3Endpoint,
    S3NewPathStyle,
    S3Profile,
    S3Region,
    S3SecretAccessKey,
    S3SecurityToken,
    S3SessionToken,
    SmbShare,
    #[cfg(posix)]
    SmbWorkgroup,
    Username,
    WebDAVUri,
}

#[derive(Debug, Eq, PartialEq)]
enum Msg {
    Form(FormMsg),
    Ui(UiMsg),
    None,
}

#[derive(Debug, PartialEq, Eq)]
enum FormMsg {
    Connect,
    DeleteBookmark,
    DeleteRecent,
    EnterSetup,
    InstallUpdate,
    LoadBookmark(usize),
    LoadRecent(usize),
    HostBridgeProtocolChanged(HostBridgeProtocol),
    RemoteProtocolChanged(FileTransferProtocol),
    Quit,
    SaveBookmark(FormTab),
}

#[derive(Debug, PartialEq, Eq)]
pub enum UiMsg {
    BookmarksListBlur,
    BookmarksTabBlur,
    CloseDeleteBookmark,
    CloseDeleteRecent,
    CloseErrorPopup,
    CloseInfoPopup,
    CloseInstallUpdatePopup,
    CloseKeybindingsPopup,
    CloseQuitPopup,
    CloseSaveBookmark,
    HostBridge(UiAuthFormMsg),
    RececentsListBlur,
    Remote(UiAuthFormMsg),
    BookmarkNameBlur,
    SaveBookmarkPasswordBlur,
    ShowDeleteBookmarkPopup,
    ShowDeleteRecentPopup,
    ShowKeybindingsPopup,
    ShowQuitPopup,
    ShowReleaseNotes,
    ShowSaveBookmarkPopup,
    WindowResized,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UiAuthFormMsg {
    AddressBlurDown,
    AddressBlurUp,
    ChangeFormTab,
    KubeNamespaceBlurDown,
    KubeNamespaceBlurUp,
    KubeClusterUrlBlurDown,
    KubeClusterUrlBlurUp,
    KubeUsernameBlurDown,
    KubeUsernameBlurUp,
    KubeClientCertBlurDown,
    KubeClientCertBlurUp,
    KubeClientKeyBlurDown,
    KubeClientKeyBlurUp,
    LocalDirectoryBlurDown,
    LocalDirectoryBlurUp,
    ParamsFormBlur,
    PasswordBlurDown,
    PasswordBlurUp,
    PortBlurDown,
    PortBlurUp,
    ProtocolBlurDown,
    ProtocolBlurUp,
    RemoteDirectoryBlurDown,
    RemoteDirectoryBlurUp,
    S3AccessKeyBlurDown,
    S3AccessKeyBlurUp,
    S3BucketBlurDown,
    S3BucketBlurUp,
    S3EndpointBlurDown,
    S3EndpointBlurUp,
    S3NewPathStyleBlurDown,
    S3NewPathStyleBlurUp,
    S3ProfileBlurDown,
    S3ProfileBlurUp,
    S3RegionBlurDown,
    S3RegionBlurUp,
    S3SecretAccessKeyBlurDown,
    S3SecretAccessKeyBlurUp,
    S3SecurityTokenBlurDown,
    S3SecurityTokenBlurUp,
    S3SessionTokenBlurDown,
    S3SessionTokenBlurUp,
    SmbShareBlurDown,
    SmbShareBlurUp,
    #[cfg(posix)]
    SmbWorkgroupDown,
    #[cfg(posix)]
    SmbWorkgroupUp,
    UsernameBlurDown,
    UsernameBlurUp,
    WebDAVUriBlurDown,
    WebDAVUriBlurUp,
}

/// Auth form input mask
#[derive(Eq, PartialEq)]
enum InputMask {
    Generic,
    AwsS3,
    Kube,
    Localhost,
    Smb,
    WebDAV,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum HostBridgeProtocol {
    Localhost,
    Remote(FileTransferProtocol),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FormTab {
    HostBridge,
    Remote,
}

// Store keys
const STORE_KEY_LATEST_VERSION: &str = "AUTH_LATEST_VERSION";
const STORE_KEY_RELEASE_NOTES: &str = "AUTH_RELEASE_NOTES";

/// AuthActivity is the data holder for the authentication activity
pub struct AuthActivity {
    app: Application<Id, Msg, NoUserEvent>,
    /// List of bookmarks
    bookmarks_list: Vec<String>,
    /// List of recent hosts
    recents_list: Vec<String>,
    /// Exit reason
    exit_reason: Option<ExitReason>,
    /// Should redraw ui
    redraw: bool,
    /// Host bridge protocol
    host_bridge_protocol: HostBridgeProtocol,
    last_form_tab: FormTab,
    /// Remote file transfer protocol
    remote_protocol: FileTransferProtocol,
    context: Option<Context>,
}

impl AuthActivity {
    /// Instantiates a new AuthActivity
    pub fn new(ticks: Duration) -> AuthActivity {
        AuthActivity {
            app: Application::init(
                EventListenerCfg::default()
                    .crossterm_input_listener(ticks, CROSSTERM_MAX_POLL)
                    .poll_timeout(ticks),
            ),
            context: None,
            bookmarks_list: Vec::new(),
            exit_reason: None,
            last_form_tab: FormTab::Remote,
            recents_list: Vec::new(),
            redraw: true,
            host_bridge_protocol: HostBridgeProtocol::Localhost,
            remote_protocol: FileTransferProtocol::Sftp,
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

    /// Returns config client reference
    fn config(&self) -> &ConfigClient {
        self.context().config()
    }

    fn bookmarks_client(&self) -> Option<&BookmarksClient> {
        self.context().bookmarks_client()
    }

    fn bookmarks_client_mut(&mut self) -> Option<&mut BookmarksClient> {
        self.context_mut().bookmarks_client_mut()
    }

    /// Returns a reference to theme
    fn theme(&self) -> &Theme {
        self.context().theme_provider().theme()
    }

    /// Get current input mask to show
    fn remote_input_mask(&self) -> InputMask {
        Self::file_transfer_protocol_input_mask(self.remote_protocol)
    }

    /// Get current input mask to show
    fn host_bridge_input_mask(&self) -> InputMask {
        match self.host_bridge_protocol {
            HostBridgeProtocol::Localhost => InputMask::Localhost,
            HostBridgeProtocol::Remote(protocol) => {
                Self::file_transfer_protocol_input_mask(protocol)
            }
        }
    }

    /// Get input mask for protocol
    fn file_transfer_protocol_input_mask(protocol: FileTransferProtocol) -> InputMask {
        match protocol {
            FileTransferProtocol::AwsS3 => InputMask::AwsS3,
            FileTransferProtocol::Ftp(_)
            | FileTransferProtocol::Scp
            | FileTransferProtocol::Sftp => InputMask::Generic,
            FileTransferProtocol::Kube => InputMask::Kube,
            FileTransferProtocol::Smb => InputMask::Smb,
            FileTransferProtocol::WebDAV => InputMask::WebDAV,
        }
    }
}

impl Activity for AuthActivity {
    /// `on_create` is the function which must be called to initialize the activity.
    /// `on_create` must initialize all the data structures used by the activity
    /// Context is taken from activity manager and will be released only when activity is destroyed
    fn on_create(&mut self, mut context: Context) {
        debug!("Initializing activity");
        // Initialize file transfer params
        context.set_remote_params(FileTransferParams::default());
        // Set context
        self.context = Some(context);
        // Clear terminal
        if let Err(err) = self.context_mut().terminal().clear_screen() {
            error!("Failed to clear screen: {}", err);
        }
        // Put raw mode on enabled
        if let Err(err) = self.context_mut().terminal().enable_raw_mode() {
            error!("Failed to enter raw mode: {}", err);
        }
        // If check for updates is enabled, check for updates
        self.check_for_updates();
        // Initialize view
        self.init();
        // Init bookmarks client
        if self.bookmarks_client().is_some() {
            self.init_bookmarks_client();
            self.view_bookmarks();
            self.view_recent_connections();
        }
        // Verify error state from context
        if let Some(err) = self.context_mut().error() {
            self.mount_error(err.as_str());
        }
        info!("Activity initialized");
    }

    /// `on_draw` is the function which draws the graphical interface.
    /// This function must be called at each tick to refresh the interface
    fn on_draw(&mut self) {
        // Context must be something
        if self.context.is_none() {
            return;
        }
        // Tick
        match self.app.tick(PollStrategy::UpTo(3)) {
            Ok(messages) => {
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = self.update(msg);
                    }
                }
            }
            Err(err) => {
                self.mount_error(format!("Application error: {err}"));
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
