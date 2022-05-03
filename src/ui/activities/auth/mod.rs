//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// Sub modules
mod bookmarks;
mod components;
mod misc;
mod update;
mod view;

// locals
use super::{Activity, Context, ExitReason};
use crate::config::themes::Theme;
use crate::filetransfer::{FileTransferParams, FileTransferProtocol};
use crate::system::bookmarks_client::BookmarksClient;
use crate::system::config_client::ConfigClient;

// Includes
use std::time::Duration;
use tuirealm::listener::EventListenerCfg;
use tuirealm::{application::PollStrategy, Application, NoUserEvent, Update};

// -- components
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Address,
    BookmarkName,
    BookmarkSavePassword,
    BookmarksList,
    DeleteBookmarkPopup,
    DeleteRecentPopup,
    ErrorPopup,
    GlobalListener,
    HelpFooter,
    InfoPopup,
    InstallUpdatePopup,
    Keybindings,
    NewVersionChangelog,
    NewVersionDisclaimer,
    Password,
    Port,
    Protocol,
    QuitPopup,
    RecentsList,
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
    Subtitle,
    Title,
    Username,
    WaitPopup,
    WindowSizeError,
}

#[derive(Debug, PartialEq)]
pub enum Msg {
    Form(FormMsg),
    Ui(UiMsg),
    None,
}

#[derive(Debug, PartialEq)]
pub enum FormMsg {
    Connect,
    DeleteBookmark,
    DeleteRecent,
    EnterSetup,
    InstallUpdate,
    LoadBookmark(usize),
    LoadRecent(usize),
    ProtocolChanged(FileTransferProtocol),
    Quit,
    SaveBookmark,
}

#[derive(Debug, PartialEq)]
pub enum UiMsg {
    AddressBlurDown,
    AddressBlurUp,
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
    ParamsFormBlur,
    PasswordBlurDown,
    PasswordBlurUp,
    PortBlurDown,
    PortBlurUp,
    ProtocolBlurDown,
    ProtocolBlurUp,
    RececentsListBlur,
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
    BookmarkNameBlur,
    SaveBookmarkPasswordBlur,
    ShowDeleteBookmarkPopup,
    ShowDeleteRecentPopup,
    ShowKeybindingsPopup,
    ShowQuitPopup,
    ShowReleaseNotes,
    ShowSaveBookmarkPopup,
    UsernameBlurDown,
    UsernameBlurUp,
    WindowResized,
}

/// Auth form input mask
#[derive(Eq, PartialEq)]
enum InputMask {
    Generic,
    AwsS3,
}

// Store keys
const STORE_KEY_LATEST_VERSION: &str = "AUTH_LATEST_VERSION";
const STORE_KEY_RELEASE_NOTES: &str = "AUTH_RELEASE_NOTES";

/// ### AuthActivity
///
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
    /// Protocol
    protocol: FileTransferProtocol,
    context: Option<Context>,
}

impl AuthActivity {
    /// Instantiates a new AuthActivity
    pub fn new(ticks: Duration) -> AuthActivity {
        AuthActivity {
            app: Application::init(
                EventListenerCfg::default()
                    .default_input_listener(ticks)
                    .poll_timeout(ticks),
            ),
            context: None,
            bookmarks_list: Vec::new(),
            exit_reason: None,
            recents_list: Vec::new(),
            redraw: true,
            protocol: FileTransferProtocol::Sftp,
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
    fn input_mask(&self) -> InputMask {
        match self.protocol {
            FileTransferProtocol::AwsS3 => InputMask::AwsS3,
            FileTransferProtocol::Ftp(_)
            | FileTransferProtocol::Scp
            | FileTransferProtocol::Sftp => InputMask::Generic,
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
        context.set_ftparams(FileTransferParams::default());
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
