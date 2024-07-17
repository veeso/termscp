//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

// Locals
use std::path::PathBuf;
use std::str::FromStr;

use tuirealm::props::Color;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::tui::widgets::Clear;
use tuirealm::{State, StateValue, Sub, SubClause, SubEventClause};

use super::{components, AuthActivity, Context, FileTransferProtocol, Id, InputMask};
use crate::filetransfer::params::{
    AwsS3Params, GenericProtocolParams, KubeProtocolParams, ProtocolParams, SmbParams,
    WebDAVProtocolParams,
};
use crate::filetransfer::FileTransferParams;
use crate::utils::ui::{Popup, Size};

impl AuthActivity {
    /// Initialize view, mounting all startup components inside the view
    pub(super) fn init(&mut self) {
        let key_color = self.theme().misc_keys;
        let info_color = self.theme().misc_info_dialog;
        // Headers
        assert!(self
            .app
            .mount(Id::Title, Box::<components::Title>::default(), vec![])
            .is_ok());
        assert!(self
            .app
            .mount(Id::Subtitle, Box::<components::Subtitle>::default(), vec![])
            .is_ok());
        // Footer
        assert!(self
            .app
            .mount(
                Id::HelpFooter,
                Box::new(components::HelpFooter::new(key_color)),
                vec![]
            )
            .is_ok());
        // Get default protocol
        let default_protocol: FileTransferProtocol = self.context().config().get_default_protocol();
        // Auth form
        self.mount_protocol(default_protocol);
        self.mount_remote_directory("");
        self.mount_local_directory("");
        self.mount_address("");
        self.mount_port(Self::get_default_port_for_protocol(default_protocol));
        self.mount_username("");
        self.mount_password("");
        self.mount_s3_bucket("");
        self.mount_s3_profile("");
        self.mount_s3_region("");
        self.mount_s3_endpoint("");
        self.mount_s3_access_key("");
        self.mount_s3_secret_access_key("");
        self.mount_s3_security_token("");
        self.mount_s3_session_token("");
        self.mount_s3_new_path_style(false);
        self.mount_kube_client_cert("");
        self.mount_kube_client_key("");
        self.mount_kube_cluster_url("");
        self.mount_kube_container("");
        self.mount_kube_namespace("");
        self.mount_kube_pod_name("");
        self.mount_kube_username("");
        self.mount_smb_share("");
        #[cfg(unix)]
        self.mount_smb_workgroup("");
        self.mount_webdav_uri("");
        // Version notice
        if let Some(version) = self
            .context()
            .store()
            .get_string(super::STORE_KEY_LATEST_VERSION)
        {
            let version: String = version.to_string();
            assert!(self
                .app
                .mount(
                    Id::NewVersionDisclaimer,
                    Box::new(components::NewVersionDisclaimer::new(
                        version.as_str(),
                        info_color
                    )),
                    vec![]
                )
                .is_ok());
        }
        // Load bookmarks
        self.view_bookmarks();
        self.view_recent_connections();
        // Global listener
        self.init_global_listener();
        // Active protocol
        assert!(self.app.active(&Id::Protocol).is_ok());
    }

    /// Display view on canvas
    pub(super) fn view(&mut self) {
        self.redraw = false;
        let mut ctx: Context = self.context.take().unwrap();
        let _ = ctx.terminal().raw_mut().draw(|f| {
            // Check window size
            let height: u16 = f.size().height;
            self.check_minimum_window_size(height);
            // Prepare chunks
            let body = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Min(24),   // Body
                        Constraint::Length(1), // Footer
                    ]
                    .as_ref(),
                )
                .split(f.size());
            // Footer
            self.app.view(&Id::HelpFooter, f, body[1]);
            let auth_form_len = 7 + self.input_mask_size();
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(auth_form_len), // Auth Form
                        Constraint::Min(3),                // Bookmarks
                    ]
                    .as_ref(),
                )
                .split(body[0]);
            // Create explorer chunks
            let auth_chunks = Layout::default()
                .constraints(
                    [
                        Constraint::Length(1),                      // h1
                        Constraint::Length(1),                      // h2
                        Constraint::Length(1),                      // Version
                        Constraint::Length(3),                      // protocol
                        Constraint::Length(self.input_mask_size()), // Input mask
                        Constraint::Length(1), // Prevents last field to overflow
                    ]
                    .as_ref(),
                )
                .direction(Direction::Vertical)
                .split(main_chunks[0]);
            // Input mask chunks
            let input_mask = match self.input_mask() {
                InputMask::AwsS3 => Layout::default()
                    .constraints(
                        [
                            Constraint::Length(3), // bucket
                            Constraint::Length(3), // region
                            Constraint::Length(3), // profile
                            Constraint::Length(3), // access_key
                            Constraint::Length(3), // remote directory
                        ]
                        .as_ref(),
                    )
                    .direction(Direction::Vertical)
                    .split(auth_chunks[4]),
                InputMask::Kube => Layout::default()
                    .constraints([
                        Constraint::Length(3), // ...
                        Constraint::Length(3), // ...
                        Constraint::Length(3), // ...
                        Constraint::Length(3), // ...
                        Constraint::Length(3), // remote directory
                    ])
                    .direction(Direction::Vertical)
                    .split(auth_chunks[4]),
                InputMask::Generic => Layout::default()
                    .constraints(
                        [
                            Constraint::Length(3), // address
                            Constraint::Length(3), // port
                            Constraint::Length(3), // username
                            Constraint::Length(3), // password
                            Constraint::Length(3), // remote directory
                        ]
                        .as_ref(),
                    )
                    .direction(Direction::Vertical)
                    .split(auth_chunks[4]),
                #[cfg(unix)]
                InputMask::Smb => Layout::default()
                    .constraints(
                        [
                            Constraint::Length(3), // address
                            Constraint::Length(3), // port
                            Constraint::Length(3), // share
                            Constraint::Length(3), // username
                            Constraint::Length(3), // password
                            Constraint::Length(3), // workgroup
                            Constraint::Length(3), // remote directory
                        ]
                        .as_ref(),
                    )
                    .direction(Direction::Vertical)
                    .split(auth_chunks[4]),
                #[cfg(windows)]
                InputMask::Smb => Layout::default()
                    .constraints(
                        [
                            Constraint::Length(3), // address
                            Constraint::Length(3), // share
                            Constraint::Length(3), // username
                            Constraint::Length(3), // password
                            Constraint::Length(3), // remote directory
                        ]
                        .as_ref(),
                    )
                    .direction(Direction::Vertical)
                    .split(auth_chunks[4]),
                InputMask::WebDAV => Layout::default()
                    .constraints(
                        [
                            Constraint::Length(3), // uri
                            Constraint::Length(3), // username
                            Constraint::Length(3), // password
                            Constraint::Length(3), // dir
                        ]
                        .as_ref(),
                    )
                    .direction(Direction::Vertical)
                    .split(auth_chunks[4]),
            };
            // Create bookmark chunks
            let bookmark_chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .split(main_chunks[1]);
            // Render
            // Auth chunks
            self.app.view(&Id::Title, f, auth_chunks[0]);
            self.app.view(&Id::Subtitle, f, auth_chunks[1]);
            self.app.view(&Id::NewVersionDisclaimer, f, auth_chunks[2]);
            self.app.view(&Id::Protocol, f, auth_chunks[3]);
            // Render input mask
            match self.input_mask() {
                InputMask::AwsS3 => {
                    let view_ids = self.get_s3_view();
                    self.app.view(&view_ids[0], f, input_mask[0]);
                    self.app.view(&view_ids[1], f, input_mask[1]);
                    self.app.view(&view_ids[2], f, input_mask[2]);
                    self.app.view(&view_ids[3], f, input_mask[3]);
                }
                InputMask::Generic => {
                    let view_ids = self.get_generic_params_view();
                    self.app.view(&view_ids[0], f, input_mask[0]);
                    self.app.view(&view_ids[1], f, input_mask[1]);
                    self.app.view(&view_ids[2], f, input_mask[2]);
                    self.app.view(&view_ids[3], f, input_mask[3]);
                }
                InputMask::Kube => {
                    let view_ids = self.get_kube_view();
                    self.app.view(&view_ids[0], f, input_mask[0]);
                    self.app.view(&view_ids[1], f, input_mask[1]);
                    self.app.view(&view_ids[2], f, input_mask[2]);
                    self.app.view(&view_ids[3], f, input_mask[3]);
                }
                InputMask::Smb => {
                    let view_ids = self.get_smb_view();
                    self.app.view(&view_ids[0], f, input_mask[0]);
                    self.app.view(&view_ids[1], f, input_mask[1]);
                    self.app.view(&view_ids[2], f, input_mask[2]);
                    self.app.view(&view_ids[3], f, input_mask[3]);
                }
                InputMask::WebDAV => {
                    let view_ids = self.get_webdav_view();
                    self.app.view(&view_ids[0], f, input_mask[0]);
                    self.app.view(&view_ids[1], f, input_mask[1]);
                    self.app.view(&view_ids[2], f, input_mask[2]);
                    self.app.view(&view_ids[3], f, input_mask[3]);
                }
            }
            // Bookmark chunks
            self.app.view(&Id::BookmarksList, f, bookmark_chunks[0]);
            self.app.view(&Id::RecentsList, f, bookmark_chunks[1]);
            // Popups
            if self.app.mounted(&Id::ErrorPopup) {
                let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::ErrorPopup, f, popup);
            } else if self.app.mounted(&Id::InfoPopup) {
                let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::InfoPopup, f, popup);
            } else if self.app.mounted(&Id::WaitPopup) {
                let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::WaitPopup, f, popup);
            } else if self.app.mounted(&Id::WindowSizeError) {
                let popup = Popup(Size::Percentage(80), Size::Percentage(20)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::WindowSizeError, f, popup);
            } else if self.app.mounted(&Id::QuitPopup) {
                // make popup
                let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                self.app.view(&Id::QuitPopup, f, popup);
            } else if self.app.mounted(&Id::DeleteBookmarkPopup) {
                // make popup
                let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                self.app.view(&Id::DeleteBookmarkPopup, f, popup);
            } else if self.app.mounted(&Id::DeleteRecentPopup) {
                // make popup
                let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                self.app.view(&Id::DeleteRecentPopup, f, popup);
            } else if self.app.mounted(&Id::NewVersionChangelog) {
                // make popup
                let popup = Popup(Size::Percentage(90), Size::Percentage(85)).draw_in(f.size());
                f.render_widget(Clear, popup);
                let popup_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(90), // Notes
                            Constraint::Length(3),      // Install radio
                        ]
                        .as_ref(),
                    )
                    .split(popup);
                self.app.view(&Id::NewVersionChangelog, f, popup_chunks[0]);
                self.app.view(&Id::InstallUpdatePopup, f, popup_chunks[1]);
            } else if self.app.mounted(&Id::Keybindings) {
                // make popup
                let popup = Popup(Size::Percentage(50), Size::Percentage(70)).draw_in(f.size());
                f.render_widget(Clear, popup);
                self.app.view(&Id::Keybindings, f, popup);
            } else if self.app.mounted(&Id::BookmarkSavePassword) {
                // make popup
                let popup = Popup(Size::Percentage(20), Size::Percentage(20)).draw_in(f.size());
                f.render_widget(Clear, popup);
                let popup_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(3), // Input form
                            Constraint::Length(2), // Yes/No
                        ]
                        .as_ref(),
                    )
                    .split(popup);
                self.app.view(&Id::BookmarkName, f, popup_chunks[0]);
                self.app.view(&Id::BookmarkSavePassword, f, popup_chunks[1]);
            }
        });
        self.context = Some(ctx);
    }

    // -- partials

    /// Make text span from bookmarks
    pub(super) fn view_bookmarks(&mut self) {
        let bookmarks: Vec<String> = self
            .bookmarks_list
            .iter()
            .map(|x| {
                Self::fmt_bookmark(x, self.bookmarks_client().unwrap().get_bookmark(x).unwrap())
            })
            .collect();
        let bookmarks_color = self.theme().auth_bookmarks;
        assert!(self
            .app
            .remount(
                Id::BookmarksList,
                Box::new(components::BookmarksList::new(&bookmarks, bookmarks_color)),
                vec![]
            )
            .is_ok());
    }

    /// View recent connections
    pub(super) fn view_recent_connections(&mut self) {
        let bookmarks: Vec<String> = self
            .recents_list
            .iter()
            .map(|x| Self::fmt_recent(self.bookmarks_client().unwrap().get_recent(x).unwrap()))
            .collect();
        let recents_color = self.theme().auth_recents;
        assert!(self
            .app
            .remount(
                Id::RecentsList,
                Box::new(components::RecentsList::new(&bookmarks, recents_color)),
                vec![]
            )
            .is_ok());
    }

    // -- mount

    /// Mount error box
    pub(super) fn mount_error<S: AsRef<str>>(&mut self, text: S) {
        let err_color = self.theme().misc_error_dialog;
        assert!(self
            .app
            .remount(
                Id::ErrorPopup,
                Box::new(components::ErrorPopup::new(text, err_color)),
                vec![]
            )
            .is_ok());
        assert!(self.app.active(&Id::ErrorPopup).is_ok());
    }

    /// Umount error message
    pub(super) fn umount_error(&mut self) {
        let _ = self.app.umount(&Id::ErrorPopup);
    }

    /// Mount info box
    pub(super) fn mount_info<S: AsRef<str>>(&mut self, text: S) {
        let color = self.theme().misc_info_dialog;
        assert!(self
            .app
            .remount(
                Id::InfoPopup,
                Box::new(components::InfoPopup::new(text, color)),
                vec![]
            )
            .is_ok());
        assert!(self.app.active(&Id::InfoPopup).is_ok());
    }

    /// Umount info message
    pub(super) fn umount_info(&mut self) {
        let _ = self.app.umount(&Id::InfoPopup);
    }

    /// Mount wait box
    pub(super) fn mount_wait(&mut self, text: &str) {
        let wait_color = self.theme().misc_info_dialog;
        assert!(self
            .app
            .remount(
                Id::WaitPopup,
                Box::new(components::WaitPopup::new(text, wait_color)),
                vec![]
            )
            .is_ok());
        assert!(self.app.active(&Id::WaitPopup).is_ok());
    }

    /// Umount wait message
    pub(super) fn umount_wait(&mut self) {
        let _ = self.app.umount(&Id::WaitPopup);
    }

    /// Mount size error
    pub(super) fn mount_size_err(&mut self) {
        // Mount
        assert!(self
            .app
            .remount(
                Id::WindowSizeError,
                Box::new(components::WindowSizeError::new(Color::Red)),
                vec![]
            )
            .is_ok());
        assert!(self.app.active(&Id::WindowSizeError).is_ok());
    }

    /// Umount error size error
    pub(super) fn umount_size_err(&mut self) {
        let _ = self.app.umount(&Id::WindowSizeError);
    }

    /// Mount quit popup
    pub(super) fn mount_quit(&mut self) {
        // Protocol
        let quit_color = self.theme().misc_quit_dialog;
        assert!(self
            .app
            .remount(
                Id::QuitPopup,
                Box::new(components::QuitPopup::new(quit_color)),
                vec![]
            )
            .is_ok());
        assert!(self.app.active(&Id::QuitPopup).is_ok());
    }

    /// Umount quit popup
    pub(super) fn umount_quit(&mut self) {
        let _ = self.app.umount(&Id::QuitPopup);
    }

    /// Mount bookmark delete dialog
    pub(super) fn mount_bookmark_del_dialog(&mut self) {
        let warn_color = self.theme().misc_warn_dialog;
        assert!(self
            .app
            .remount(
                Id::DeleteBookmarkPopup,
                Box::new(components::DeleteBookmarkPopup::new(warn_color)),
                vec![]
            )
            .is_ok());
        assert!(self.app.active(&Id::DeleteBookmarkPopup).is_ok());
    }

    /// umount delete bookmark dialog
    pub(super) fn umount_bookmark_del_dialog(&mut self) {
        let _ = self.app.umount(&Id::DeleteBookmarkPopup);
    }

    /// Mount recent delete dialog
    pub(super) fn mount_recent_del_dialog(&mut self) {
        let warn_color = self.theme().misc_warn_dialog;
        assert!(self
            .app
            .remount(
                Id::DeleteRecentPopup,
                Box::new(components::DeleteRecentPopup::new(warn_color)),
                vec![]
            )
            .is_ok());
        assert!(self.app.active(&Id::DeleteRecentPopup).is_ok());
    }

    /// umount delete recent dialog
    pub(super) fn umount_recent_del_dialog(&mut self) {
        let _ = self.app.umount(&Id::DeleteRecentPopup);
    }

    /// Mount bookmark save dialog
    pub(super) fn mount_bookmark_save_dialog(&mut self) {
        let save_color = self.theme().misc_save_dialog;
        let warn_color = self.theme().misc_warn_dialog;
        assert!(self
            .app
            .remount(
                Id::BookmarkName,
                Box::new(components::BookmarkName::new(save_color)),
                vec![]
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::BookmarkSavePassword,
                Box::new(components::BookmarkSavePassword::new(warn_color)),
                vec![]
            )
            .is_ok());
        // Give focus to input bookmark name
        assert!(self.app.active(&Id::BookmarkName).is_ok());
    }

    /// Umount bookmark save dialog
    pub(super) fn umount_bookmark_save_dialog(&mut self) {
        let _ = self.app.umount(&Id::BookmarkName);
        let _ = self.app.umount(&Id::BookmarkSavePassword);
    }

    /// Mount keybindings
    pub(super) fn mount_keybindings(&mut self) {
        let key_color = self.theme().misc_keys;
        assert!(self
            .app
            .remount(
                Id::Keybindings,
                Box::new(components::Keybindings::new(key_color)),
                vec![]
            )
            .is_ok());
        // Active help
        assert!(self.app.active(&Id::Keybindings).is_ok());
    }

    /// Umount help
    pub(super) fn umount_help(&mut self) {
        let _ = self.app.umount(&Id::Keybindings);
    }

    /// mount release notes text area
    pub(super) fn mount_release_notes(&mut self) {
        if let Some(ctx) = self.context.as_ref() {
            if let Some(release_notes) = ctx.store().get_string(super::STORE_KEY_RELEASE_NOTES) {
                // make spans
                let info_color = self.theme().misc_info_dialog;
                assert!(self
                    .app
                    .remount(
                        Id::NewVersionChangelog,
                        Box::new(components::ReleaseNotes::new(release_notes, info_color)),
                        vec![]
                    )
                    .is_ok());
                assert!(self
                    .app
                    .remount(
                        Id::InstallUpdatePopup,
                        Box::new(components::InstallUpdatePopup::new(info_color)),
                        vec![]
                    )
                    .is_ok());
                assert!(self.app.active(&Id::InstallUpdatePopup).is_ok());
            }
        }
    }

    /// Umount release notes text area
    pub(super) fn umount_release_notes(&mut self) {
        let _ = self.app.umount(&Id::NewVersionChangelog);
        let _ = self.app.umount(&Id::InstallUpdatePopup);
    }

    pub(super) fn mount_protocol(&mut self, protocol: FileTransferProtocol) {
        let protocol_color = self.theme().auth_protocol;
        assert!(self
            .app
            .remount(
                Id::Protocol,
                Box::new(components::ProtocolRadio::new(protocol, protocol_color)),
                vec![]
            )
            .is_ok());
    }

    pub(super) fn mount_remote_directory<S: AsRef<str>>(&mut self, remote_path: S) {
        let protocol_color = self.theme().auth_protocol;
        assert!(self
            .app
            .remount(
                Id::RemoteDirectory,
                Box::new(components::InputRemoteDirectory::new(
                    remote_path.as_ref(),
                    protocol_color
                )),
                vec![]
            )
            .is_ok());
    }

    pub(super) fn mount_local_directory<S: AsRef<str>>(&mut self, local_path: S) {
        let color = self.theme().auth_username;
        assert!(self
            .app
            .remount(
                Id::LocalDirectory,
                Box::new(components::InputLocalDirectory::new(
                    local_path.as_ref(),
                    color
                )),
                vec![]
            )
            .is_ok());
    }

    pub(super) fn mount_address(&mut self, address: &str) {
        let addr_color = self.theme().auth_address;
        assert!(self
            .app
            .remount(
                Id::Address,
                Box::new(components::InputAddress::new(address, addr_color)),
                vec![]
            )
            .is_ok());
    }

    pub(super) fn mount_port(&mut self, port: u16) {
        let port_color = self.theme().auth_port;
        assert!(self
            .app
            .remount(
                Id::Port,
                Box::new(components::InputPort::new(port, port_color)),
                vec![]
            )
            .is_ok());
    }

    pub(crate) fn mount_username(&mut self, username: &str) {
        let username_color = self.theme().auth_username;
        assert!(self
            .app
            .remount(
                Id::Username,
                Box::new(components::InputUsername::new(username, username_color)),
                vec![]
            )
            .is_ok());
    }

    pub(crate) fn mount_password(&mut self, password: &str) {
        let password_color = self.theme().auth_password;
        assert!(self
            .app
            .remount(
                Id::Password,
                Box::new(components::InputPassword::new(password, password_color)),
                vec![]
            )
            .is_ok());
    }

    pub(super) fn mount_s3_bucket(&mut self, bucket: &str) {
        let addr_color = self.theme().auth_address;
        assert!(self
            .app
            .remount(
                Id::S3Bucket,
                Box::new(components::InputS3Bucket::new(bucket, addr_color)),
                vec![]
            )
            .is_ok());
    }

    pub(super) fn mount_s3_region(&mut self, region: &str) {
        let port_color = self.theme().auth_port;
        assert!(self
            .app
            .remount(
                Id::S3Region,
                Box::new(components::InputS3Region::new(region, port_color)),
                vec![]
            )
            .is_ok());
    }

    pub(crate) fn mount_s3_endpoint(&mut self, endpoint: &str) {
        let username_color = self.theme().auth_username;
        assert!(self
            .app
            .remount(
                Id::S3Endpoint,
                Box::new(components::InputS3Endpoint::new(endpoint, username_color)),
                vec![]
            )
            .is_ok());
    }

    pub(crate) fn mount_s3_profile(&mut self, profile: &str) {
        let color = self.theme().auth_password;
        assert!(self
            .app
            .remount(
                Id::S3Profile,
                Box::new(components::InputS3Profile::new(profile, color)),
                vec![]
            )
            .is_ok());
    }

    pub(crate) fn mount_s3_access_key(&mut self, key: &str) {
        let color = self.theme().auth_address;
        assert!(self
            .app
            .remount(
                Id::S3AccessKey,
                Box::new(components::InputS3AccessKey::new(key, color)),
                vec![]
            )
            .is_ok());
    }

    pub(crate) fn mount_s3_secret_access_key(&mut self, key: &str) {
        let color = self.theme().auth_port;
        assert!(self
            .app
            .remount(
                Id::S3SecretAccessKey,
                Box::new(components::InputS3SecretAccessKey::new(key, color)),
                vec![]
            )
            .is_ok());
    }

    pub(crate) fn mount_s3_security_token(&mut self, token: &str) {
        let color = self.theme().auth_username;
        assert!(self
            .app
            .remount(
                Id::S3SecurityToken,
                Box::new(components::InputS3SecurityToken::new(token, color)),
                vec![]
            )
            .is_ok());
    }

    pub(crate) fn mount_s3_session_token(&mut self, token: &str) {
        let color = self.theme().auth_password;
        assert!(self
            .app
            .remount(
                Id::S3SessionToken,
                Box::new(components::InputS3SessionToken::new(token, color)),
                vec![]
            )
            .is_ok());
    }

    pub(crate) fn mount_s3_new_path_style(&mut self, new_path_style: bool) {
        let color = self.theme().auth_address;
        assert!(self
            .app
            .remount(
                Id::S3NewPathStyle,
                Box::new(components::RadioS3NewPathStyle::new(new_path_style, color)),
                vec![]
            )
            .is_ok());
    }

    pub(super) fn mount_kube_pod_name(&mut self, value: &str) {
        let color = self.theme().auth_address;
        assert!(self
            .app
            .remount(
                Id::KubePodName,
                Box::new(components::InputKubePodName::new(value, color)),
                vec![]
            )
            .is_ok());
    }

    pub(super) fn mount_kube_container(&mut self, value: &str) {
        let color = self.theme().auth_password;
        assert!(self
            .app
            .remount(
                Id::KubeContainer,
                Box::new(components::InputKubeContainer::new(value, color)),
                vec![]
            )
            .is_ok());
    }

    pub(super) fn mount_kube_namespace(&mut self, value: &str) {
        let color = self.theme().auth_port;
        assert!(self
            .app
            .remount(
                Id::KubeNamespace,
                Box::new(components::InputKubeNamespace::new(value, color)),
                vec![]
            )
            .is_ok());
    }

    pub(super) fn mount_kube_cluster_url(&mut self, value: &str) {
        let color = self.theme().auth_username;
        assert!(self
            .app
            .remount(
                Id::KubeClusterUrl,
                Box::new(components::InputKubeClusterUrl::new(value, color)),
                vec![]
            )
            .is_ok());
    }

    pub(super) fn mount_kube_username(&mut self, value: &str) {
        let color = self.theme().auth_password;
        assert!(self
            .app
            .remount(
                Id::KubeUsername,
                Box::new(components::InputKubeUsername::new(value, color)),
                vec![]
            )
            .is_ok());
    }

    pub(super) fn mount_kube_client_cert(&mut self, value: &str) {
        let color = self.theme().auth_address;
        assert!(self
            .app
            .remount(
                Id::KubeClientCert,
                Box::new(components::InputKubeClientCert::new(value, color)),
                vec![]
            )
            .is_ok());
    }

    pub(super) fn mount_kube_client_key(&mut self, value: &str) {
        let color = self.theme().auth_port;
        assert!(self
            .app
            .remount(
                Id::KubeClientKey,
                Box::new(components::InputKubeClientKey::new(value, color)),
                vec![]
            )
            .is_ok());
    }

    pub(crate) fn mount_smb_share(&mut self, share: &str) {
        let color = self.theme().auth_password;
        assert!(self
            .app
            .remount(
                Id::SmbShare,
                Box::new(components::InputSmbShare::new(share, color)),
                vec![]
            )
            .is_ok());
    }

    #[cfg(unix)]
    pub(crate) fn mount_smb_workgroup(&mut self, workgroup: &str) {
        let color = self.theme().auth_address;
        assert!(self
            .app
            .remount(
                Id::SmbWorkgroup,
                Box::new(components::InputSmbWorkgroup::new(workgroup, color)),
                vec![]
            )
            .is_ok());
    }

    pub(super) fn mount_webdav_uri(&mut self, uri: &str) {
        let addr_color = self.theme().auth_address;
        assert!(self
            .app
            .remount(
                Id::WebDAVUri,
                Box::new(components::InputWebDAVUri::new(uri, addr_color)),
                vec![]
            )
            .is_ok());
    }

    // -- query

    /// Collect input values from view
    pub(super) fn get_generic_params_input(&self) -> GenericProtocolParams {
        let addr: String = self.get_input_addr();
        let port: u16 = self.get_input_port();
        let username = self.get_input_username();
        let password = self.get_input_password();
        GenericProtocolParams::default()
            .address(addr)
            .port(port)
            .username(username)
            .password(password)
    }

    /// Collect s3 input values from view
    pub(super) fn get_s3_params_input(&self) -> AwsS3Params {
        let bucket: String = self.get_input_s3_bucket();
        let region: Option<String> = self.get_input_s3_region();
        let endpoint = self.get_input_s3_endpoint();
        let profile: Option<String> = self.get_input_s3_profile();
        let access_key = self.get_input_s3_access_key();
        let secret_access_key = self.get_input_s3_secret_access_key();
        let security_token = self.get_input_s3_security_token();
        let session_token = self.get_input_s3_session_token();
        let new_path_style = self.get_input_s3_new_path_style();
        AwsS3Params::new(bucket, region, profile)
            .endpoint(endpoint)
            .access_key(access_key)
            .secret_access_key(secret_access_key)
            .security_token(security_token)
            .session_token(session_token)
            .new_path_style(new_path_style)
    }

    /// Collect s3 input values from view
    pub(super) fn get_kube_params_input(&self) -> KubeProtocolParams {
        let pod = self.get_input_kube_pod_name();
        let container = self.get_input_kube_container();
        let namespace = self.get_input_kube_namespace();
        let cluster_url = self.get_input_kube_cluster_url();
        let username = self.get_input_kube_username();
        let client_cert = self.get_input_kube_client_cert();
        let client_key = self.get_input_kube_client_key();
        KubeProtocolParams {
            pod,
            container,
            namespace,
            cluster_url,
            username,
            client_cert,
            client_key,
        }
    }

    /// Collect s3 input values from view
    #[cfg(unix)]
    pub(super) fn get_smb_params_input(&self) -> SmbParams {
        let share: String = self.get_input_smb_share();
        let workgroup: Option<String> = self.get_input_smb_workgroup();

        let address: String = self.get_input_addr();
        let port: u16 = self.get_input_port();
        let username = self.get_input_username();
        let password = self.get_input_password();

        SmbParams::new(address, share)
            .port(port)
            .username(username)
            .password(password)
            .workgroup(workgroup)
    }

    #[cfg(windows)]
    pub(super) fn get_smb_params_input(&self) -> SmbParams {
        let share: String = self.get_input_smb_share();

        let address: String = self.get_input_addr();
        let username = self.get_input_username();
        let password = self.get_input_password();

        SmbParams::new(address, share)
            .username(username)
            .password(password)
    }

    pub(super) fn get_webdav_params_input(&self) -> WebDAVProtocolParams {
        let uri: String = self.get_webdav_uri();
        let username = self.get_input_username().unwrap_or_default();
        let password = self.get_input_password().unwrap_or_default();

        WebDAVProtocolParams {
            uri,
            username,
            password,
        }
    }

    pub(super) fn get_input_remote_directory(&self) -> Option<PathBuf> {
        match self.app.state(&Id::RemoteDirectory) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => {
                Some(PathBuf::from(x.as_str()))
            }
            _ => None,
        }
    }

    pub(super) fn get_input_local_directory(&self) -> Option<PathBuf> {
        match self.app.state(&Id::LocalDirectory) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => {
                Some(PathBuf::from(x.as_str()))
            }
            _ => None,
        }
    }

    pub(super) fn get_webdav_uri(&self) -> String {
        match self.app.state(&Id::WebDAVUri) {
            Ok(State::One(StateValue::String(x))) => x,
            _ => String::new(),
        }
    }

    pub(super) fn get_input_addr(&self) -> String {
        match self.app.state(&Id::Address) {
            Ok(State::One(StateValue::String(x))) => x,
            _ => String::new(),
        }
    }

    pub(super) fn get_input_port(&self) -> u16 {
        match self.app.state(&Id::Port) {
            Ok(State::One(StateValue::String(x))) => match u16::from_str(x.as_str()) {
                Ok(v) => v,
                _ => 0,
            },
            _ => 0,
        }
    }

    pub(super) fn get_input_username(&self) -> Option<String> {
        match self.app.state(&Id::Username) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_password(&self) -> Option<String> {
        match self.app.state(&Id::Password) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_bucket(&self) -> String {
        match self.app.state(&Id::S3Bucket) {
            Ok(State::One(StateValue::String(x))) => x,
            _ => String::new(),
        }
    }

    pub(super) fn get_input_s3_region(&self) -> Option<String> {
        match self.app.state(&Id::S3Region) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_endpoint(&self) -> Option<String> {
        match self.app.state(&Id::S3Endpoint) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_profile(&self) -> Option<String> {
        match self.app.state(&Id::S3Profile) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_access_key(&self) -> Option<String> {
        match self.app.state(&Id::S3AccessKey) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_secret_access_key(&self) -> Option<String> {
        match self.app.state(&Id::S3SecretAccessKey) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_security_token(&self) -> Option<String> {
        match self.app.state(&Id::S3SecurityToken) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_session_token(&self) -> Option<String> {
        match self.app.state(&Id::S3SessionToken) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_new_path_style(&self) -> bool {
        matches!(
            self.app.state(&Id::S3NewPathStyle),
            Ok(State::One(StateValue::Usize(0)))
        )
    }

    pub(super) fn get_input_kube_pod_name(&self) -> String {
        match self.app.state(&Id::KubePodName) {
            Ok(State::One(StateValue::String(x))) => x,
            _ => String::new(),
        }
    }

    pub(super) fn get_input_kube_container(&self) -> String {
        match self.app.state(&Id::KubeContainer) {
            Ok(State::One(StateValue::String(x))) => x,
            _ => String::new(),
        }
    }

    pub(super) fn get_input_kube_namespace(&self) -> Option<String> {
        match self.app.state(&Id::KubeNamespace) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_kube_cluster_url(&self) -> Option<String> {
        match self.app.state(&Id::KubeClusterUrl) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_kube_username(&self) -> Option<String> {
        match self.app.state(&Id::KubeUsername) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_kube_client_cert(&self) -> Option<String> {
        match self.app.state(&Id::KubeClientCert) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_kube_client_key(&self) -> Option<String> {
        match self.app.state(&Id::KubeClientKey) {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_smb_share(&self) -> String {
        match self.app.state(&Id::SmbShare) {
            Ok(State::One(StateValue::String(x))) => x,
            _ => String::new(),
        }
    }

    #[cfg(unix)]
    pub(super) fn get_input_smb_workgroup(&self) -> Option<String> {
        match self.app.state(&Id::SmbWorkgroup) {
            Ok(State::One(StateValue::String(x))) => Some(x),
            _ => None,
        }
    }

    /// Get new bookmark params
    pub(super) fn get_new_bookmark(&self) -> (String, bool) {
        let name = match self.app.state(&Id::BookmarkName) {
            Ok(State::One(StateValue::String(name))) => name,
            _ => String::default(),
        };
        if matches!(
            self.app.state(&Id::BookmarkSavePassword),
            Ok(State::One(StateValue::Usize(0)))
        ) {
            (name, true)
        } else {
            (name, false)
        }
    }

    // -- len

    /// Returns the input mask size based on current input mask
    pub(super) fn input_mask_size(&self) -> u16 {
        match self.input_mask() {
            InputMask::AwsS3 => 12,
            InputMask::Generic => 12,
            InputMask::Kube => 12,
            InputMask::Smb => 12,
            InputMask::WebDAV => 12,
        }
    }

    // -- fmt

    /// Format bookmark to display on ui
    fn fmt_bookmark(name: &str, b: FileTransferParams) -> String {
        let addr: String = Self::fmt_recent(b);
        format!("{name} ({addr})")
    }

    /// Format recent connection to display on ui
    fn fmt_recent(b: FileTransferParams) -> String {
        let protocol: String = b.protocol.to_string().to_lowercase();
        match b.params {
            ProtocolParams::AwsS3(s3) => {
                let profile: String = match s3.profile {
                    Some(p) => format!("[{p}]"),
                    None => String::default(),
                };
                format!(
                    "{}://{}{} ({}) {}",
                    protocol,
                    s3.endpoint.unwrap_or_default(),
                    s3.bucket_name,
                    s3.region.as_deref().unwrap_or("custom"),
                    profile
                )
            }
            ProtocolParams::Generic(params) => {
                let username: String = match params.username {
                    None => String::default(),
                    Some(u) => format!("{u}@"),
                };
                format!(
                    "{}://{}{}:{}",
                    protocol, username, params.address, params.port
                )
            }
            ProtocolParams::Kube(params) => {
                format!(
                    "{}://{}@{}{}{}",
                    protocol,
                    params.container,
                    params.pod,
                    params
                        .namespace
                        .as_deref()
                        .map(|x| format!("/{x}"))
                        .unwrap_or_default(),
                    params
                        .cluster_url
                        .as_deref()
                        .map(|x| format!("@{x}"))
                        .unwrap_or_default()
                )
            }
            #[cfg(unix)]
            ProtocolParams::Smb(params) => {
                let username: String = match params.username {
                    None => String::default(),
                    Some(u) => format!("{u}@"),
                };
                format!(
                    "\\\\{username}{}:{}\\{}",
                    params.address, params.port, params.share
                )
            }
            #[cfg(windows)]
            ProtocolParams::Smb(params) => {
                let username: String = match params.username {
                    None => String::default(),
                    Some(u) => format!("{u}@"),
                };
                format!("\\\\{username}{}\\{}", params.address, params.share)
            }
            ProtocolParams::WebDAV(params) => params.uri,
        }
    }

    /// Get the visible element in the generic params form, based on current focus
    fn get_generic_params_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::RemoteDirectory) => {
                [Id::Port, Id::Username, Id::Password, Id::RemoteDirectory]
            }
            Some(&Id::LocalDirectory) => [
                Id::Username,
                Id::Password,
                Id::RemoteDirectory,
                Id::LocalDirectory,
            ],
            _ => [Id::Address, Id::Port, Id::Username, Id::Password],
        }
    }

    /// Get the visible element in the aws-s3 form, based on current focus
    fn get_s3_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::S3AccessKey) => {
                [Id::S3Region, Id::S3Endpoint, Id::S3Profile, Id::S3AccessKey]
            }
            Some(&Id::S3SecretAccessKey) => [
                Id::S3Endpoint,
                Id::S3Profile,
                Id::S3AccessKey,
                Id::S3SecretAccessKey,
            ],
            Some(&Id::S3SecurityToken) => [
                Id::S3Profile,
                Id::S3AccessKey,
                Id::S3SecretAccessKey,
                Id::S3SecurityToken,
            ],
            Some(&Id::S3SessionToken) => [
                Id::S3AccessKey,
                Id::S3SecretAccessKey,
                Id::S3SecurityToken,
                Id::S3SessionToken,
            ],
            Some(&Id::S3NewPathStyle) => [
                Id::S3SecretAccessKey,
                Id::S3SecurityToken,
                Id::S3SessionToken,
                Id::S3NewPathStyle,
            ],
            Some(&Id::RemoteDirectory) => [
                Id::S3SecurityToken,
                Id::S3SessionToken,
                Id::S3NewPathStyle,
                Id::RemoteDirectory,
            ],
            Some(&Id::LocalDirectory) => [
                Id::S3SessionToken,
                Id::S3NewPathStyle,
                Id::RemoteDirectory,
                Id::LocalDirectory,
            ],
            _ => [Id::S3Bucket, Id::S3Region, Id::S3Endpoint, Id::S3Profile],
        }
    }

    /// Get the visible element in the kube form, based on current focus
    fn get_kube_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::KubePodName) => [
                Id::KubePodName,
                Id::KubeContainer,
                Id::KubeNamespace,
                Id::KubeClusterUrl,
            ],
            Some(&Id::KubeUsername) => [
                Id::KubeContainer,
                Id::KubeNamespace,
                Id::KubeClusterUrl,
                Id::KubeUsername,
            ],
            Some(&Id::KubeClientCert) => [
                Id::KubeNamespace,
                Id::KubeClusterUrl,
                Id::KubeUsername,
                Id::KubeClientCert,
            ],
            Some(&Id::KubeClientKey) => [
                Id::KubeClusterUrl,
                Id::KubeUsername,
                Id::KubeClientCert,
                Id::KubeClientKey,
            ],
            Some(&Id::RemoteDirectory) => [
                Id::KubeUsername,
                Id::KubeClientCert,
                Id::KubeClientKey,
                Id::RemoteDirectory,
            ],
            Some(&Id::LocalDirectory) => [
                Id::KubeClientCert,
                Id::KubeClientKey,
                Id::RemoteDirectory,
                Id::LocalDirectory,
            ],
            _ => [
                Id::KubePodName,
                Id::KubeContainer,
                Id::KubeNamespace,
                Id::KubeClusterUrl,
            ],
        }
    }

    #[cfg(unix)]
    fn get_smb_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::Address | &Id::Port | &Id::SmbShare | &Id::Username) => {
                [Id::Address, Id::Port, Id::SmbShare, Id::Username]
            }
            Some(&Id::Password) => [Id::Port, Id::SmbShare, Id::Username, Id::Password],
            Some(&Id::SmbWorkgroup) => [Id::SmbShare, Id::Username, Id::Password, Id::SmbWorkgroup],
            Some(&Id::RemoteDirectory) => [
                Id::Username,
                Id::Password,
                Id::SmbWorkgroup,
                Id::RemoteDirectory,
            ],
            Some(&Id::LocalDirectory) => [
                Id::Password,
                Id::SmbWorkgroup,
                Id::RemoteDirectory,
                Id::LocalDirectory,
            ],
            _ => [Id::Address, Id::Port, Id::SmbShare, Id::Username],
        }
    }

    #[cfg(windows)]
    fn get_smb_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::Address | &Id::Password | &Id::SmbShare | &Id::Username) => {
                [Id::Address, Id::SmbShare, Id::Username, Id::Password]
            }
            Some(&Id::RemoteDirectory) => [
                Id::SmbShare,
                Id::Username,
                Id::Password,
                Id::RemoteDirectory,
            ],
            Some(&Id::LocalDirectory) => [
                Id::Username,
                Id::Password,
                Id::RemoteDirectory,
                Id::LocalDirectory,
            ],
            _ => [Id::Address, Id::SmbShare, Id::Username, Id::Password],
        }
    }

    fn get_webdav_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::LocalDirectory) => [
                Id::Username,
                Id::Password,
                Id::RemoteDirectory,
                Id::LocalDirectory,
            ],
            _ => [
                Id::WebDAVUri,
                Id::Username,
                Id::Password,
                Id::RemoteDirectory,
            ],
        }
    }

    fn init_global_listener(&mut self) {
        use tuirealm::event::{Key, KeyEvent, KeyModifiers};
        assert!(self
            .app
            .mount(
                Id::GlobalListener,
                Box::<components::GlobalListener>::default(),
                vec![
                    Sub::new(
                        SubEventClause::Keyboard(KeyEvent {
                            code: Key::Esc,
                            modifiers: KeyModifiers::NONE,
                        }),
                        Self::no_popup_mounted_clause(),
                    ),
                    Sub::new(
                        SubEventClause::Keyboard(KeyEvent {
                            code: Key::Function(10),
                            modifiers: KeyModifiers::NONE,
                        }),
                        Self::no_popup_mounted_clause(),
                    ),
                    Sub::new(
                        SubEventClause::Keyboard(KeyEvent {
                            code: Key::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                        }),
                        Self::no_popup_mounted_clause(),
                    ),
                    Sub::new(
                        SubEventClause::Keyboard(KeyEvent {
                            code: Key::Char('h'),
                            modifiers: KeyModifiers::CONTROL,
                        }),
                        Self::no_popup_mounted_clause(),
                    ),
                    Sub::new(
                        SubEventClause::Keyboard(KeyEvent {
                            code: Key::Function(1),
                            modifiers: KeyModifiers::NONE,
                        }),
                        Self::no_popup_mounted_clause(),
                    ),
                    Sub::new(
                        SubEventClause::Keyboard(KeyEvent {
                            code: Key::Char('r'),
                            modifiers: KeyModifiers::CONTROL,
                        }),
                        Self::no_popup_mounted_clause(),
                    ),
                    Sub::new(
                        SubEventClause::Keyboard(KeyEvent {
                            code: Key::Char('s'),
                            modifiers: KeyModifiers::CONTROL,
                        }),
                        Self::no_popup_mounted_clause(),
                    ),
                    Sub::new(SubEventClause::WindowResize, SubClause::Always)
                ]
            )
            .is_ok());
    }

    /// Returns a sub clause which requires that no popup is mounted in order to be satisfied
    fn no_popup_mounted_clause() -> SubClause<Id> {
        SubClause::And(
            Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                Id::ErrorPopup,
            )))),
            Box::new(SubClause::And(
                Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                    Id::InfoPopup,
                )))),
                Box::new(SubClause::And(
                    Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                        Id::Keybindings,
                    )))),
                    Box::new(SubClause::And(
                        Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                            Id::DeleteBookmarkPopup,
                        )))),
                        Box::new(SubClause::And(
                            Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                Id::DeleteRecentPopup,
                            )))),
                            Box::new(SubClause::And(
                                Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                    Id::InstallUpdatePopup,
                                )))),
                                Box::new(SubClause::And(
                                    Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                        Id::BookmarkSavePassword,
                                    )))),
                                    Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                        Id::WaitPopup,
                                    )))),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        )
    }
}
