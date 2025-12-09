//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

// Locals
use std::path::PathBuf;
use std::str::FromStr;

use tuirealm::props::Color;
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::ratatui::widgets::Clear;
use tuirealm::{State, StateValue, Sub, SubClause, SubEventClause};

use super::{
    AuthActivity, AuthFormId, Context, FileTransferProtocol, FormTab, HostBridgeProtocol, Id,
    InputMask, components,
};
use crate::filetransfer::FileTransferParams;
use crate::filetransfer::params::{
    AwsS3Params, GenericProtocolParams, KubeProtocolParams, ProtocolParams, SmbParams,
    WebDAVProtocolParams,
};
use crate::utils::ui::{Popup, Size};

impl AuthActivity {
    /// Initialize view, mounting all startup components inside the view
    pub(super) fn init(&mut self) {
        let key_color = self.theme().misc_keys;
        let info_color = self.theme().misc_info_dialog;
        // Headers
        assert!(
            self.app
                .mount(Id::Title, Box::<components::Title>::default(), vec![])
                .is_ok()
        );
        assert!(
            self.app
                .mount(Id::Subtitle, Box::<components::Subtitle>::default(), vec![])
                .is_ok()
        );
        // Footer
        assert!(
            self.app
                .mount(
                    Id::HelpFooter,
                    Box::new(components::HelpFooter::new(key_color)),
                    vec![]
                )
                .is_ok()
        );

        // Host bridge auth form
        self.mount_host_bridge_protocol(HostBridgeProtocol::Localhost);
        self.mount_remote_directory(FormTab::HostBridge, "");
        self.mount_local_directory(FormTab::HostBridge, "");
        self.mount_address(FormTab::HostBridge, "");
        self.mount_port(FormTab::HostBridge, 22);
        self.mount_username(FormTab::HostBridge, "");
        self.mount_password(FormTab::HostBridge, "");
        self.mount_s3_bucket(FormTab::HostBridge, "");
        self.mount_s3_profile(FormTab::HostBridge, "");
        self.mount_s3_region(FormTab::HostBridge, "");
        self.mount_s3_endpoint(FormTab::HostBridge, "");
        self.mount_s3_access_key(FormTab::HostBridge, "");
        self.mount_s3_secret_access_key(FormTab::HostBridge, "");
        self.mount_s3_security_token(FormTab::HostBridge, "");
        self.mount_s3_session_token(FormTab::HostBridge, "");
        self.mount_s3_new_path_style(FormTab::HostBridge, false);
        self.mount_kube_client_cert(FormTab::HostBridge, "");
        self.mount_kube_client_key(FormTab::HostBridge, "");
        self.mount_kube_cluster_url(FormTab::HostBridge, "");
        self.mount_kube_namespace(FormTab::HostBridge, "");
        self.mount_kube_username(FormTab::HostBridge, "");
        self.mount_smb_share(FormTab::HostBridge, "");
        #[cfg(posix)]
        self.mount_smb_workgroup(FormTab::HostBridge, "");
        self.mount_webdav_uri(FormTab::HostBridge, "");

        // Remote Auth form
        // Get default protocol
        let remote_default_protocol: FileTransferProtocol =
            self.context().config().get_default_protocol();
        self.mount_remote_protocol(remote_default_protocol);
        self.mount_remote_directory(FormTab::Remote, "");
        self.mount_local_directory(FormTab::Remote, "");
        self.mount_address(FormTab::Remote, "");
        self.mount_port(
            FormTab::Remote,
            Self::get_default_port_for_protocol(remote_default_protocol),
        );
        self.mount_username(FormTab::Remote, "");
        self.mount_password(FormTab::Remote, "");
        self.mount_s3_bucket(FormTab::Remote, "");
        self.mount_s3_profile(FormTab::Remote, "");
        self.mount_s3_region(FormTab::Remote, "");
        self.mount_s3_endpoint(FormTab::Remote, "");
        self.mount_s3_access_key(FormTab::Remote, "");
        self.mount_s3_secret_access_key(FormTab::Remote, "");
        self.mount_s3_security_token(FormTab::Remote, "");
        self.mount_s3_session_token(FormTab::Remote, "");
        self.mount_s3_new_path_style(FormTab::Remote, false);
        self.mount_kube_client_cert(FormTab::Remote, "");
        self.mount_kube_client_key(FormTab::Remote, "");
        self.mount_kube_cluster_url(FormTab::Remote, "");
        self.mount_kube_namespace(FormTab::Remote, "");
        self.mount_kube_username(FormTab::Remote, "");
        self.mount_smb_share(FormTab::Remote, "");
        #[cfg(posix)]
        self.mount_smb_workgroup(FormTab::Remote, "");
        self.mount_webdav_uri(FormTab::Remote, "");

        // Version notice
        if let Some(version) = self
            .context()
            .store()
            .get_string(super::STORE_KEY_LATEST_VERSION)
        {
            let version: String = version.to_string();
            assert!(
                self.app
                    .mount(
                        Id::NewVersionDisclaimer,
                        Box::new(components::NewVersionDisclaimer::new(
                            version.as_str(),
                            info_color
                        )),
                        vec![]
                    )
                    .is_ok()
            );
        }
        // Load bookmarks
        self.view_bookmarks();
        self.view_recent_connections();
        // Global listener
        self.init_global_listener();
        // Active protocol
        assert!(self.app.active(&Id::Remote(AuthFormId::Protocol)).is_ok());
    }

    /// Display view on canvas
    pub(super) fn view(&mut self) {
        self.redraw = false;
        let mut ctx: Context = self.context.take().unwrap();
        let _ = ctx.terminal().raw_mut().draw(|f| {
            // Check window size
            let height: u16 = f.area().height;
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
                .split(f.area());
            // Footer
            self.app.view(&Id::HelpFooter, f, body[1]);
            let auth_form_len = 7 + self.max_input_mask_size();
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
                        Constraint::Length(1),                          // h1
                        Constraint::Length(1),                          // h2
                        Constraint::Length(1),                          // Version
                        Constraint::Length(self.max_input_mask_size()), // Input mask
                        Constraint::Length(1), // Prevents last field to overflow
                    ]
                    .as_ref(),
                )
                .direction(Direction::Vertical)
                .split(main_chunks[0]);

            // Create bookmark chunks
            let bookmark_chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .spacing(2)
                .split(main_chunks[1]);
            // Render
            // Auth chunks
            self.app.view(&Id::Title, f, auth_chunks[0]);
            self.app.view(&Id::Subtitle, f, auth_chunks[1]);
            self.app.view(&Id::NewVersionDisclaimer, f, auth_chunks[2]);

            // Render the host bridge and remote forms
            let host_bridge_and_remote_chunks = Layout::default()
                .constraints(
                    [
                        Constraint::Percentage(50), // Host bridge
                        Constraint::Percentage(50), // Remote
                    ]
                    .as_ref(),
                )
                .spacing(2)
                .direction(Direction::Horizontal)
                .split(auth_chunks[3]);
            // Input mask
            self.render_host_bridge_input_mask(f, host_bridge_and_remote_chunks[0]);
            self.render_remote_input_mask(f, host_bridge_and_remote_chunks[1]);
            // Bookmark chunks
            self.app.view(&Id::BookmarksList, f, bookmark_chunks[0]);
            self.app.view(&Id::RecentsList, f, bookmark_chunks[1]);
            // Popups
            if self.app.mounted(&Id::ErrorPopup) {
                let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::ErrorPopup, f, popup);
            } else if self.app.mounted(&Id::InfoPopup) {
                let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::InfoPopup, f, popup);
            } else if self.app.mounted(&Id::WaitPopup) {
                let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::WaitPopup, f, popup);
            } else if self.app.mounted(&Id::WindowSizeError) {
                let popup = Popup(Size::Percentage(80), Size::Percentage(20)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::WindowSizeError, f, popup);
            } else if self.app.mounted(&Id::QuitPopup) {
                // make popup
                let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                self.app.view(&Id::QuitPopup, f, popup);
            } else if self.app.mounted(&Id::DeleteBookmarkPopup) {
                // make popup
                let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                self.app.view(&Id::DeleteBookmarkPopup, f, popup);
            } else if self.app.mounted(&Id::DeleteRecentPopup) {
                // make popup
                let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                self.app.view(&Id::DeleteRecentPopup, f, popup);
            } else if self.app.mounted(&Id::NewVersionChangelog) {
                // make popup
                let popup = Popup(Size::Percentage(90), Size::Percentage(85)).draw_in(f.area());
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
                let popup = Popup(Size::Percentage(50), Size::Percentage(70)).draw_in(f.area());
                f.render_widget(Clear, popup);
                self.app.view(&Id::Keybindings, f, popup);
            } else if self.app.mounted(&Id::BookmarkSavePassword) {
                // make popup
                let popup = Popup(Size::Percentage(20), Size::Percentage(20)).draw_in(f.area());
                f.render_widget(Clear, popup);
                let popup_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(3), // Input form
                            Constraint::Length(4), // Yes/No
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

    fn render_host_bridge_input_mask(
        &mut self,
        f: &mut tuirealm::ratatui::Frame<'_>,
        area: tuirealm::ratatui::layout::Rect,
    ) {
        let protocol_and_mask_chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(3),  // protocol
                    Constraint::Length(12), // Input mask
                ]
                .as_ref(),
            )
            .direction(Direction::Vertical)
            .split(area);

        self.app.view(
            &Id::HostBridge(AuthFormId::Protocol),
            f,
            protocol_and_mask_chunks[0],
        );

        let input_mask = Layout::default()
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
            .split(protocol_and_mask_chunks[1]);
        // Render input mask
        match self.host_bridge_input_mask() {
            InputMask::AwsS3 => {
                let view_ids = self.get_host_bridge_s3_view();
                self.app.view(&view_ids[0], f, input_mask[0]);
                self.app.view(&view_ids[1], f, input_mask[1]);
                self.app.view(&view_ids[2], f, input_mask[2]);
                self.app.view(&view_ids[3], f, input_mask[3]);
            }
            InputMask::Generic => {
                let view_ids = self.get_host_bridge_generic_params_view();
                self.app.view(&view_ids[0], f, input_mask[0]);
                self.app.view(&view_ids[1], f, input_mask[1]);
                self.app.view(&view_ids[2], f, input_mask[2]);
                self.app.view(&view_ids[3], f, input_mask[3]);
            }
            InputMask::Kube => {
                let view_ids = self.get_host_bridge_kube_view();
                self.app.view(&view_ids[0], f, input_mask[0]);
                self.app.view(&view_ids[1], f, input_mask[1]);
                self.app.view(&view_ids[2], f, input_mask[2]);
                self.app.view(&view_ids[3], f, input_mask[3]);
            }
            InputMask::Localhost => {
                let view_ids = self.get_host_bridge_localhost_view();
                self.app.view(&view_ids[0], f, input_mask[0]);
            }
            InputMask::Smb => {
                let view_ids = self.get_host_bridge_smb_view();
                self.app.view(&view_ids[0], f, input_mask[0]);
                self.app.view(&view_ids[1], f, input_mask[1]);
                self.app.view(&view_ids[2], f, input_mask[2]);
                self.app.view(&view_ids[3], f, input_mask[3]);
            }
            InputMask::WebDAV => {
                let view_ids = self.get_host_bridge_webdav_view();
                self.app.view(&view_ids[0], f, input_mask[0]);
                self.app.view(&view_ids[1], f, input_mask[1]);
                self.app.view(&view_ids[2], f, input_mask[2]);
                self.app.view(&view_ids[3], f, input_mask[3]);
            }
        }
    }

    fn render_remote_input_mask(
        &mut self,
        f: &mut tuirealm::ratatui::Frame<'_>,
        area: tuirealm::ratatui::layout::Rect,
    ) {
        let protocol_and_mask_chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(3),  // protocol
                    Constraint::Length(12), // Input mask
                ]
                .as_ref(),
            )
            .direction(Direction::Vertical)
            .split(area);

        self.app.view(
            &Id::Remote(AuthFormId::Protocol),
            f,
            protocol_and_mask_chunks[0],
        );

        let input_mask = Layout::default()
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
            .split(protocol_and_mask_chunks[1]);
        // Render input mask
        match self.remote_input_mask() {
            InputMask::AwsS3 => {
                let view_ids = self.get_remote_s3_view();
                self.app.view(&view_ids[0], f, input_mask[0]);
                self.app.view(&view_ids[1], f, input_mask[1]);
                self.app.view(&view_ids[2], f, input_mask[2]);
                self.app.view(&view_ids[3], f, input_mask[3]);
            }
            InputMask::Generic => {
                let view_ids = self.get_remote_generic_params_view();
                self.app.view(&view_ids[0], f, input_mask[0]);
                self.app.view(&view_ids[1], f, input_mask[1]);
                self.app.view(&view_ids[2], f, input_mask[2]);
                self.app.view(&view_ids[3], f, input_mask[3]);
            }
            InputMask::Kube => {
                let view_ids = self.get_remote_kube_view();
                self.app.view(&view_ids[0], f, input_mask[0]);
                self.app.view(&view_ids[1], f, input_mask[1]);
                self.app.view(&view_ids[2], f, input_mask[2]);
                self.app.view(&view_ids[3], f, input_mask[3]);
            }
            InputMask::Localhost => unreachable!(),
            InputMask::Smb => {
                let view_ids = self.get_remote_smb_view();
                self.app.view(&view_ids[0], f, input_mask[0]);
                self.app.view(&view_ids[1], f, input_mask[1]);
                self.app.view(&view_ids[2], f, input_mask[2]);
                self.app.view(&view_ids[3], f, input_mask[3]);
            }
            InputMask::WebDAV => {
                let view_ids = self.get_remote_webdav_view();
                self.app.view(&view_ids[0], f, input_mask[0]);
                self.app.view(&view_ids[1], f, input_mask[1]);
                self.app.view(&view_ids[2], f, input_mask[2]);
                self.app.view(&view_ids[3], f, input_mask[3]);
            }
        }
    }

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

        let key_bindings = self.context().key_bindings();
        assert!(
            self.app
                .remount(
                    Id::BookmarksList,
                    Box::new(components::BookmarksList::new(
                        &bookmarks,
                        bookmarks_color,
                        key_bindings
                    )),
                    vec![]
                )
                .is_ok()
        );
    }

    /// View recent connections
    pub(super) fn view_recent_connections(&mut self) {
        let bookmarks: Vec<String> = self
            .recents_list
            .iter()
            .map(|x| Self::fmt_recent(self.bookmarks_client().unwrap().get_recent(x).unwrap()))
            .collect();
        let recents_color = self.theme().auth_recents;
        let key_bindings = self.context().key_bindings();
        assert!(
            self.app
                .remount(
                    Id::RecentsList,
                    Box::new(components::RecentsList::new(
                        &bookmarks,
                        recents_color,
                        key_bindings
                    )),
                    vec![]
                )
                .is_ok()
        );
    }

    // -- mount

    /// Mount error box
    pub(super) fn mount_error<S: AsRef<str>>(&mut self, text: S) {
        let err_color = self.theme().misc_error_dialog;
        assert!(
            self.app
                .remount(
                    Id::ErrorPopup,
                    Box::new(components::ErrorPopup::new(text, err_color)),
                    vec![]
                )
                .is_ok()
        );
        assert!(self.app.active(&Id::ErrorPopup).is_ok());
    }

    /// Umount error message
    pub(super) fn umount_error(&mut self) {
        let _ = self.app.umount(&Id::ErrorPopup);
    }

    /// Mount info box
    pub(super) fn mount_info<S: AsRef<str>>(&mut self, text: S) {
        let color = self.theme().misc_info_dialog;
        assert!(
            self.app
                .remount(
                    Id::InfoPopup,
                    Box::new(components::InfoPopup::new(text, color)),
                    vec![]
                )
                .is_ok()
        );
        assert!(self.app.active(&Id::InfoPopup).is_ok());
    }

    /// Umount info message
    pub(super) fn umount_info(&mut self) {
        let _ = self.app.umount(&Id::InfoPopup);
    }

    /// Mount wait box
    pub(super) fn mount_wait(&mut self, text: &str) {
        let wait_color = self.theme().misc_info_dialog;
        assert!(
            self.app
                .remount(
                    Id::WaitPopup,
                    Box::new(components::WaitPopup::new(text, wait_color)),
                    vec![]
                )
                .is_ok()
        );
        assert!(self.app.active(&Id::WaitPopup).is_ok());
    }

    /// Umount wait message
    pub(super) fn umount_wait(&mut self) {
        let _ = self.app.umount(&Id::WaitPopup);
    }

    /// Mount size error
    pub(super) fn mount_size_err(&mut self) {
        // Mount
        assert!(
            self.app
                .remount(
                    Id::WindowSizeError,
                    Box::new(components::WindowSizeError::new(Color::Red)),
                    vec![]
                )
                .is_ok()
        );
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
        assert!(
            self.app
                .remount(
                    Id::QuitPopup,
                    Box::new(components::QuitPopup::new(quit_color)),
                    vec![]
                )
                .is_ok()
        );
        assert!(self.app.active(&Id::QuitPopup).is_ok());
    }

    /// Umount quit popup
    pub(super) fn umount_quit(&mut self) {
        let _ = self.app.umount(&Id::QuitPopup);
    }

    /// Mount bookmark delete dialog
    pub(super) fn mount_bookmark_del_dialog(&mut self) {
        let warn_color = self.theme().misc_warn_dialog;
        assert!(
            self.app
                .remount(
                    Id::DeleteBookmarkPopup,
                    Box::new(components::DeleteBookmarkPopup::new(warn_color)),
                    vec![]
                )
                .is_ok()
        );
        assert!(self.app.active(&Id::DeleteBookmarkPopup).is_ok());
    }

    /// umount delete bookmark dialog
    pub(super) fn umount_bookmark_del_dialog(&mut self) {
        let _ = self.app.umount(&Id::DeleteBookmarkPopup);
    }

    /// Mount recent delete dialog
    pub(super) fn mount_recent_del_dialog(&mut self) {
        let warn_color = self.theme().misc_warn_dialog;
        assert!(
            self.app
                .remount(
                    Id::DeleteRecentPopup,
                    Box::new(components::DeleteRecentPopup::new(warn_color)),
                    vec![]
                )
                .is_ok()
        );
        assert!(self.app.active(&Id::DeleteRecentPopup).is_ok());
    }

    /// umount delete recent dialog
    pub(super) fn umount_recent_del_dialog(&mut self) {
        let _ = self.app.umount(&Id::DeleteRecentPopup);
    }

    /// Mount bookmark save dialog
    pub(super) fn mount_bookmark_save_dialog(&mut self, form_tab: FormTab) {
        let save_color = self.theme().misc_save_dialog;
        let warn_color = self.theme().misc_warn_dialog;
        assert!(
            self.app
                .remount(
                    Id::BookmarkName,
                    Box::new(components::BookmarkName::new(form_tab, save_color)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::BookmarkSavePassword,
                    Box::new(components::BookmarkSavePassword::new(form_tab, warn_color)),
                    vec![]
                )
                .is_ok()
        );
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
        assert!(
            self.app
                .remount(
                    Id::Keybindings,
                    Box::new(components::Keybindings::new(key_color)),
                    vec![]
                )
                .is_ok()
        );
        // Active help
        assert!(self.app.active(&Id::Keybindings).is_ok());
    }

    /// Umount help
    pub(super) fn umount_help(&mut self) {
        let _ = self.app.umount(&Id::Keybindings);
    }

    /// mount release notes text area
    pub(super) fn mount_release_notes(&mut self) {
        if let Some(ctx) = self.context.as_ref()
            && let Some(release_notes) = ctx.store().get_string(super::STORE_KEY_RELEASE_NOTES)
        {
            // make spans
            let info_color = self.theme().misc_info_dialog;
            assert!(
                self.app
                    .remount(
                        Id::NewVersionChangelog,
                        Box::new(components::ReleaseNotes::new(release_notes, info_color)),
                        vec![]
                    )
                    .is_ok()
            );
            assert!(
                self.app
                    .remount(
                        Id::InstallUpdatePopup,
                        Box::new(components::InstallUpdatePopup::new(info_color)),
                        vec![]
                    )
                    .is_ok()
            );
            assert!(self.app.active(&Id::InstallUpdatePopup).is_ok());
        }
    }

    /// Umount release notes text area
    pub(super) fn umount_release_notes(&mut self) {
        let _ = self.app.umount(&Id::NewVersionChangelog);
        let _ = self.app.umount(&Id::InstallUpdatePopup);
    }

    pub(super) fn mount_host_bridge_protocol(&mut self, protocol: HostBridgeProtocol) {
        let protocol_color = self.theme().auth_protocol;
        assert!(
            self.app
                .remount(
                    Id::HostBridge(AuthFormId::Protocol),
                    Box::new(components::HostBridgeProtocolRadio::new(
                        protocol,
                        protocol_color
                    )),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_remote_protocol(&mut self, protocol: FileTransferProtocol) {
        let protocol_color = self.theme().auth_protocol;
        assert!(
            self.app
                .remount(
                    Id::Remote(AuthFormId::Protocol),
                    Box::new(components::RemoteProtocolRadio::new(
                        protocol,
                        protocol_color
                    )),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_remote_directory<S: AsRef<str>>(
        &mut self,
        form_tab: FormTab,
        remote_path: S,
    ) {
        let id = Self::form_tab_id(form_tab, AuthFormId::RemoteDirectory);
        let protocol_color = self.theme().auth_protocol;
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputRemoteDirectory::new(
                        remote_path.as_ref(),
                        form_tab,
                        protocol_color
                    )),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_local_directory<S: AsRef<str>>(
        &mut self,
        form_tab: FormTab,
        local_path: S,
    ) {
        let id = Self::form_tab_id(form_tab, AuthFormId::LocalDirectory);
        let color = self.theme().auth_username;
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputLocalDirectory::new(
                        local_path.as_ref(),
                        form_tab,
                        color
                    )),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_address(&mut self, form_tab: FormTab, address: &str) {
        let addr_color = self.theme().auth_address;
        let id = Self::form_tab_id(form_tab, AuthFormId::Address);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputAddress::new(address, form_tab, addr_color)),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_port(&mut self, form_tab: FormTab, port: u16) {
        let port_color = self.theme().auth_port;
        let id = Self::form_tab_id(form_tab, AuthFormId::Port);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputPort::new(port, form_tab, port_color)),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_username(&mut self, form_tab: FormTab, username: &str) {
        let username_color = self.theme().auth_username;
        let id = Self::form_tab_id(form_tab, AuthFormId::Username);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputUsername::new(
                        username,
                        form_tab,
                        username_color
                    )),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_password(&mut self, form_tab: FormTab, password: &str) {
        let password_color = self.theme().auth_password;
        let id = Self::form_tab_id(form_tab, AuthFormId::Password);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputPassword::new(
                        password,
                        form_tab,
                        password_color
                    )),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_s3_bucket(&mut self, form_tab: FormTab, bucket: &str) {
        let addr_color = self.theme().auth_address;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3Bucket);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputS3Bucket::new(bucket, form_tab, addr_color)),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_s3_region(&mut self, form_tab: FormTab, region: &str) {
        let port_color = self.theme().auth_port;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3Region);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputS3Region::new(region, form_tab, port_color)),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_s3_endpoint(&mut self, form_tab: FormTab, endpoint: &str) {
        let username_color = self.theme().auth_username;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3Endpoint);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputS3Endpoint::new(
                        endpoint,
                        form_tab,
                        username_color
                    )),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_s3_profile(&mut self, form_tab: FormTab, profile: &str) {
        let color = self.theme().auth_password;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3Profile);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputS3Profile::new(profile, form_tab, color)),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_s3_access_key(&mut self, form_tab: FormTab, key: &str) {
        let color = self.theme().auth_address;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3AccessKey);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputS3AccessKey::new(key, form_tab, color)),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_s3_secret_access_key(&mut self, form_tab: FormTab, key: &str) {
        let color = self.theme().auth_port;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3SecretAccessKey);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputS3SecretAccessKey::new(
                        key, form_tab, color
                    )),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_s3_security_token(&mut self, form_tab: FormTab, token: &str) {
        let color = self.theme().auth_username;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3SecurityToken);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputS3SecurityToken::new(
                        token, form_tab, color
                    )),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_s3_session_token(&mut self, form_tab: FormTab, token: &str) {
        let color = self.theme().auth_password;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3SessionToken);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputS3SessionToken::new(token, form_tab, color)),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_s3_new_path_style(&mut self, form_tab: FormTab, new_path_style: bool) {
        let color = self.theme().auth_address;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3NewPathStyle);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::RadioS3NewPathStyle::new(
                        new_path_style,
                        form_tab,
                        color
                    )),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_kube_namespace(&mut self, form_tab: FormTab, value: &str) {
        let color = self.theme().auth_port;
        let id = Self::form_tab_id(form_tab, AuthFormId::KubeNamespace);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputKubeNamespace::new(value, form_tab, color)),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_kube_cluster_url(&mut self, form_tab: FormTab, value: &str) {
        let color = self.theme().auth_username;
        let id = Self::form_tab_id(form_tab, AuthFormId::KubeClusterUrl);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputKubeClusterUrl::new(value, form_tab, color)),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_kube_username(&mut self, form_tab: FormTab, value: &str) {
        let color = self.theme().auth_password;
        let id = Self::form_tab_id(form_tab, AuthFormId::KubeUsername);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputKubeUsername::new(value, form_tab, color)),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_kube_client_cert(&mut self, form_tab: FormTab, value: &str) {
        let color = self.theme().auth_address;
        let id = Self::form_tab_id(form_tab, AuthFormId::KubeClientCert);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputKubeClientCert::new(value, form_tab, color)),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_kube_client_key(&mut self, form_tab: FormTab, value: &str) {
        let color = self.theme().auth_port;
        let id = Self::form_tab_id(form_tab, AuthFormId::KubeClientKey);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputKubeClientKey::new(value, form_tab, color)),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_smb_share(&mut self, form_tab: FormTab, share: &str) {
        let color = self.theme().auth_password;
        let id = Self::form_tab_id(form_tab, AuthFormId::SmbShare);

        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputSmbShare::new(share, form_tab, color)),
                    vec![]
                )
                .is_ok()
        );
    }

    #[cfg(posix)]
    pub(super) fn mount_smb_workgroup(&mut self, form_tab: FormTab, workgroup: &str) {
        let color = self.theme().auth_address;
        let id = Self::form_tab_id(form_tab, AuthFormId::SmbWorkgroup);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputSmbWorkgroup::new(
                        workgroup, form_tab, color
                    )),
                    vec![]
                )
                .is_ok()
        );
    }

    pub(super) fn mount_webdav_uri(&mut self, form_tab: FormTab, uri: &str) {
        let addr_color = self.theme().auth_address;
        let id = Self::form_tab_id(form_tab, AuthFormId::WebDAVUri);
        assert!(
            self.app
                .remount(
                    id,
                    Box::new(components::InputWebDAVUri::new(uri, form_tab, addr_color)),
                    vec![]
                )
                .is_ok()
        );
    }

    fn form_tab_id(form_tab: FormTab, id: AuthFormId) -> Id {
        match form_tab {
            FormTab::HostBridge => Id::HostBridge(id),
            FormTab::Remote => Id::Remote(id),
        }
    }

    // -- query

    /// Collect input values from view
    pub(super) fn get_generic_params_input(&self, form_tab: FormTab) -> GenericProtocolParams {
        let addr: String = self.get_input_addr(form_tab);
        let port: u16 = self.get_input_port(form_tab);
        let username = self.get_input_username(form_tab);
        let password = self.get_input_password(form_tab);
        GenericProtocolParams::default()
            .address(addr)
            .port(port)
            .username(username)
            .password(password)
    }

    /// Collect s3 input values from view
    pub(super) fn get_s3_params_input(&self, form_tab: FormTab) -> AwsS3Params {
        let bucket: String = self.get_input_s3_bucket(form_tab);
        let region: Option<String> = self.get_input_s3_region(form_tab);
        let endpoint = self.get_input_s3_endpoint(form_tab);
        let profile: Option<String> = self.get_input_s3_profile(form_tab);
        let access_key = self.get_input_s3_access_key(form_tab);
        let secret_access_key = self.get_input_s3_secret_access_key(form_tab);
        let security_token = self.get_input_s3_security_token(form_tab);
        let session_token = self.get_input_s3_session_token(form_tab);
        let new_path_style = self.get_input_s3_new_path_style(form_tab);
        AwsS3Params::new(bucket, region, profile)
            .endpoint(endpoint)
            .access_key(access_key)
            .secret_access_key(secret_access_key)
            .security_token(security_token)
            .session_token(session_token)
            .new_path_style(new_path_style)
    }

    /// Collect s3 input values from view
    pub(super) fn get_kube_params_input(&self, form_tab: FormTab) -> KubeProtocolParams {
        let namespace = self.get_input_kube_namespace(form_tab);
        let cluster_url = self.get_input_kube_cluster_url(form_tab);
        let username = self.get_input_kube_username(form_tab);
        let client_cert = self.get_input_kube_client_cert(form_tab);
        let client_key = self.get_input_kube_client_key(form_tab);
        KubeProtocolParams {
            namespace,
            cluster_url,
            username,
            client_cert,
            client_key,
        }
    }

    /// Collect s3 input values from view
    #[cfg(posix)]
    pub(super) fn get_smb_params_input(&self, form_tab: FormTab) -> SmbParams {
        let share: String = self.get_input_smb_share(form_tab);
        let workgroup: Option<String> = self.get_input_smb_workgroup(form_tab);

        let address: String = self.get_input_addr(form_tab);
        let port: u16 = self.get_input_port(form_tab);
        let username = self.get_input_username(form_tab);
        let password = self.get_input_password(form_tab);

        SmbParams::new(address, share)
            .port(port)
            .username(username)
            .password(password)
            .workgroup(workgroup)
    }

    #[cfg(win)]
    pub(super) fn get_smb_params_input(&self, form_tab: FormTab) -> SmbParams {
        let share: String = self.get_input_smb_share(form_tab);

        let address: String = self.get_input_addr(form_tab);
        let username = self.get_input_username(form_tab);
        let password = self.get_input_password(form_tab);

        SmbParams::new(address, share)
            .username(username)
            .password(password)
    }

    pub(super) fn get_webdav_params_input(&self, form_tab: FormTab) -> WebDAVProtocolParams {
        let uri: String = self.get_webdav_uri(form_tab);
        let username = self.get_input_username(form_tab).unwrap_or_default();
        let password = self.get_input_password(form_tab).unwrap_or_default();

        WebDAVProtocolParams {
            uri,
            username,
            password,
        }
    }

    pub(super) fn get_input_remote_directory(&self, form_tab: FormTab) -> Option<PathBuf> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::RemoteDirectory))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => {
                Some(PathBuf::from(x.as_str()))
            }
            _ => None,
        }
    }

    pub(super) fn get_input_local_directory(&self, form_tab: FormTab) -> Option<PathBuf> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::LocalDirectory))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => {
                Some(PathBuf::from(x.as_str()))
            }
            _ => None,
        }
    }

    pub(super) fn get_webdav_uri(&self, form_tab: FormTab) -> String {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::WebDAVUri))
        {
            Ok(State::One(StateValue::String(x))) => x,
            _ => String::new(),
        }
    }

    pub(super) fn get_input_addr(&self, form_tab: FormTab) -> String {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::Address))
        {
            Ok(State::One(StateValue::String(x))) => x,
            _ => String::new(),
        }
    }

    pub(super) fn get_input_port(&self, form_tab: FormTab) -> u16 {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::Port))
        {
            Ok(State::One(StateValue::String(x))) => u16::from_str(x.as_str()).unwrap_or_default(),
            _ => 0,
        }
    }

    pub(super) fn get_input_username(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::Username))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_password(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::Password))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_bucket(&self, form_tab: FormTab) -> String {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3Bucket))
        {
            Ok(State::One(StateValue::String(x))) => x,
            _ => String::new(),
        }
    }

    pub(super) fn get_input_s3_region(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3Region))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_endpoint(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3Endpoint))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_profile(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3Profile))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_access_key(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3AccessKey))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_secret_access_key(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3SecretAccessKey))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_security_token(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3SecurityToken))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_session_token(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3SessionToken))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_s3_new_path_style(&self, form_tab: FormTab) -> bool {
        matches!(
            self.app
                .state(&Self::form_tab_id(form_tab, AuthFormId::S3NewPathStyle)),
            Ok(State::One(StateValue::Usize(0)))
        )
    }

    pub(super) fn get_input_kube_namespace(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::KubeNamespace))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_kube_cluster_url(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::KubeClusterUrl))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_kube_username(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::KubeUsername))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_kube_client_cert(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::KubeClientCert))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_kube_client_key(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::KubeClientKey))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(super) fn get_input_smb_share(&self, form_tab: FormTab) -> String {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::SmbShare))
        {
            Ok(State::One(StateValue::String(x))) => x,
            _ => String::new(),
        }
    }

    #[cfg(posix)]
    pub(super) fn get_input_smb_workgroup(&self, form_tab: FormTab) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::SmbWorkgroup))
        {
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

    /// Returns the max input mask size based on current input mask
    fn max_input_mask_size(&self) -> u16 {
        Self::input_mask_size(self.host_bridge_input_mask())
            .max(Self::input_mask_size(self.remote_input_mask()))
            + 3 // +3 because of protocol
    }

    /// Get the input mask size based on input mask
    fn input_mask_size(input_mask: InputMask) -> u16 {
        match input_mask {
            InputMask::AwsS3 => 12,
            InputMask::Generic => 12,
            InputMask::Kube => 12,
            InputMask::Localhost => 3,
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
                    "{}://{}{}",
                    protocol,
                    params
                        .namespace
                        .as_deref()
                        .map(|x| format!("/{x}"))
                        .unwrap_or_else(|| String::from("default")),
                    params
                        .cluster_url
                        .as_deref()
                        .map(|x| format!("@{x}"))
                        .unwrap_or_default()
                )
            }
            #[cfg(posix)]
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
            #[cfg(win)]
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
    fn get_host_bridge_generic_params_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::HostBridge(AuthFormId::RemoteDirectory)) => [
                Id::HostBridge(AuthFormId::Port),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::HostBridge(AuthFormId::LocalDirectory)) => [
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::RemoteDirectory),
                Id::HostBridge(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::HostBridge(AuthFormId::Address),
                Id::HostBridge(AuthFormId::Port),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
            ],
        }
    }

    /// Get the visible element in the generic params form, based on current focus
    fn get_remote_generic_params_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::Remote(AuthFormId::RemoteDirectory)) => [
                Id::Remote(AuthFormId::Port),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::Remote(AuthFormId::LocalDirectory)) => [
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::RemoteDirectory),
                Id::Remote(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::Remote(AuthFormId::Address),
                Id::Remote(AuthFormId::Port),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
            ],
        }
    }

    fn get_host_bridge_localhost_view(&self) -> [Id; 1] {
        [Id::HostBridge(AuthFormId::LocalDirectory)]
    }

    /// Get the visible element in the aws-s3 form, based on current focus
    fn get_host_bridge_s3_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::HostBridge(AuthFormId::S3AccessKey)) => [
                Id::HostBridge(AuthFormId::S3Region),
                Id::HostBridge(AuthFormId::S3Endpoint),
                Id::HostBridge(AuthFormId::S3Profile),
                Id::HostBridge(AuthFormId::S3AccessKey),
            ],
            Some(&Id::HostBridge(AuthFormId::S3SecretAccessKey)) => [
                Id::HostBridge(AuthFormId::S3Endpoint),
                Id::HostBridge(AuthFormId::S3Profile),
                Id::HostBridge(AuthFormId::S3AccessKey),
                Id::HostBridge(AuthFormId::S3SecretAccessKey),
            ],
            Some(&Id::HostBridge(AuthFormId::S3SecurityToken)) => [
                Id::HostBridge(AuthFormId::S3Profile),
                Id::HostBridge(AuthFormId::S3AccessKey),
                Id::HostBridge(AuthFormId::S3SecretAccessKey),
                Id::HostBridge(AuthFormId::S3SecurityToken),
            ],
            Some(&Id::HostBridge(AuthFormId::S3SessionToken)) => [
                Id::HostBridge(AuthFormId::S3AccessKey),
                Id::HostBridge(AuthFormId::S3SecretAccessKey),
                Id::HostBridge(AuthFormId::S3SecurityToken),
                Id::HostBridge(AuthFormId::S3SessionToken),
            ],
            Some(&Id::HostBridge(AuthFormId::S3NewPathStyle)) => [
                Id::HostBridge(AuthFormId::S3SecretAccessKey),
                Id::HostBridge(AuthFormId::S3SecurityToken),
                Id::HostBridge(AuthFormId::S3SessionToken),
                Id::HostBridge(AuthFormId::S3NewPathStyle),
            ],
            Some(&Id::HostBridge(AuthFormId::RemoteDirectory)) => [
                Id::HostBridge(AuthFormId::S3SecurityToken),
                Id::HostBridge(AuthFormId::S3SessionToken),
                Id::HostBridge(AuthFormId::S3NewPathStyle),
                Id::HostBridge(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::HostBridge(AuthFormId::LocalDirectory)) => [
                Id::HostBridge(AuthFormId::S3SessionToken),
                Id::HostBridge(AuthFormId::S3NewPathStyle),
                Id::HostBridge(AuthFormId::RemoteDirectory),
                Id::HostBridge(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::HostBridge(AuthFormId::S3Bucket),
                Id::HostBridge(AuthFormId::S3Region),
                Id::HostBridge(AuthFormId::S3Endpoint),
                Id::HostBridge(AuthFormId::S3Profile),
            ],
        }
    }

    /// Get the visible element in the aws-s3 form, based on current focus
    fn get_remote_s3_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::Remote(AuthFormId::S3AccessKey)) => [
                Id::Remote(AuthFormId::S3Region),
                Id::Remote(AuthFormId::S3Endpoint),
                Id::Remote(AuthFormId::S3Profile),
                Id::Remote(AuthFormId::S3AccessKey),
            ],
            Some(&Id::Remote(AuthFormId::S3SecretAccessKey)) => [
                Id::Remote(AuthFormId::S3Endpoint),
                Id::Remote(AuthFormId::S3Profile),
                Id::Remote(AuthFormId::S3AccessKey),
                Id::Remote(AuthFormId::S3SecretAccessKey),
            ],
            Some(&Id::Remote(AuthFormId::S3SecurityToken)) => [
                Id::Remote(AuthFormId::S3Profile),
                Id::Remote(AuthFormId::S3AccessKey),
                Id::Remote(AuthFormId::S3SecretAccessKey),
                Id::Remote(AuthFormId::S3SecurityToken),
            ],
            Some(&Id::Remote(AuthFormId::S3SessionToken)) => [
                Id::Remote(AuthFormId::S3AccessKey),
                Id::Remote(AuthFormId::S3SecretAccessKey),
                Id::Remote(AuthFormId::S3SecurityToken),
                Id::Remote(AuthFormId::S3SessionToken),
            ],
            Some(&Id::Remote(AuthFormId::S3NewPathStyle)) => [
                Id::Remote(AuthFormId::S3SecretAccessKey),
                Id::Remote(AuthFormId::S3SecurityToken),
                Id::Remote(AuthFormId::S3SessionToken),
                Id::Remote(AuthFormId::S3NewPathStyle),
            ],
            Some(&Id::Remote(AuthFormId::RemoteDirectory)) => [
                Id::Remote(AuthFormId::S3SecurityToken),
                Id::Remote(AuthFormId::S3SessionToken),
                Id::Remote(AuthFormId::S3NewPathStyle),
                Id::Remote(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::Remote(AuthFormId::LocalDirectory)) => [
                Id::Remote(AuthFormId::S3SessionToken),
                Id::Remote(AuthFormId::S3NewPathStyle),
                Id::Remote(AuthFormId::RemoteDirectory),
                Id::Remote(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::Remote(AuthFormId::S3Bucket),
                Id::Remote(AuthFormId::S3Region),
                Id::Remote(AuthFormId::S3Endpoint),
                Id::Remote(AuthFormId::S3Profile),
            ],
        }
    }

    /// Get the visible element in the kube form, based on current focus
    fn get_host_bridge_kube_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::HostBridge(AuthFormId::KubeClientCert)) => [
                Id::HostBridge(AuthFormId::KubeNamespace),
                Id::HostBridge(AuthFormId::KubeClusterUrl),
                Id::HostBridge(AuthFormId::KubeUsername),
                Id::HostBridge(AuthFormId::KubeClientCert),
            ],
            Some(&Id::HostBridge(AuthFormId::KubeClientKey)) => [
                Id::HostBridge(AuthFormId::KubeClusterUrl),
                Id::HostBridge(AuthFormId::KubeUsername),
                Id::HostBridge(AuthFormId::KubeClientCert),
                Id::HostBridge(AuthFormId::KubeClientKey),
            ],
            Some(&Id::HostBridge(AuthFormId::RemoteDirectory)) => [
                Id::HostBridge(AuthFormId::KubeUsername),
                Id::HostBridge(AuthFormId::KubeClientCert),
                Id::HostBridge(AuthFormId::KubeClientKey),
                Id::HostBridge(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::HostBridge(AuthFormId::LocalDirectory)) => [
                Id::HostBridge(AuthFormId::KubeClientCert),
                Id::HostBridge(AuthFormId::KubeClientKey),
                Id::HostBridge(AuthFormId::RemoteDirectory),
                Id::HostBridge(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::HostBridge(AuthFormId::KubeNamespace),
                Id::HostBridge(AuthFormId::KubeClusterUrl),
                Id::HostBridge(AuthFormId::KubeUsername),
                Id::HostBridge(AuthFormId::KubeClientCert),
            ],
        }
    }

    /// Get the visible element in the kube form, based on current focus
    fn get_remote_kube_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::Remote(AuthFormId::KubeClientCert)) => [
                Id::Remote(AuthFormId::KubeNamespace),
                Id::Remote(AuthFormId::KubeClusterUrl),
                Id::Remote(AuthFormId::KubeUsername),
                Id::Remote(AuthFormId::KubeClientCert),
            ],
            Some(&Id::Remote(AuthFormId::KubeClientKey)) => [
                Id::Remote(AuthFormId::KubeClusterUrl),
                Id::Remote(AuthFormId::KubeUsername),
                Id::Remote(AuthFormId::KubeClientCert),
                Id::Remote(AuthFormId::KubeClientKey),
            ],
            Some(&Id::Remote(AuthFormId::RemoteDirectory)) => [
                Id::Remote(AuthFormId::KubeUsername),
                Id::Remote(AuthFormId::KubeClientCert),
                Id::Remote(AuthFormId::KubeClientKey),
                Id::Remote(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::Remote(AuthFormId::LocalDirectory)) => [
                Id::Remote(AuthFormId::KubeClientCert),
                Id::Remote(AuthFormId::KubeClientKey),
                Id::Remote(AuthFormId::RemoteDirectory),
                Id::Remote(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::Remote(AuthFormId::KubeNamespace),
                Id::Remote(AuthFormId::KubeClusterUrl),
                Id::Remote(AuthFormId::KubeUsername),
                Id::Remote(AuthFormId::KubeClientCert),
            ],
        }
    }

    #[cfg(posix)]
    fn get_host_bridge_smb_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(
                &Id::HostBridge(AuthFormId::Address)
                | &Id::HostBridge(AuthFormId::Port)
                | &Id::HostBridge(AuthFormId::SmbShare)
                | &Id::HostBridge(AuthFormId::Username),
            ) => [
                Id::HostBridge(AuthFormId::Address),
                Id::HostBridge(AuthFormId::Port),
                Id::HostBridge(AuthFormId::SmbShare),
                Id::HostBridge(AuthFormId::Username),
            ],
            Some(&Id::HostBridge(AuthFormId::Password)) => [
                Id::HostBridge(AuthFormId::Port),
                Id::HostBridge(AuthFormId::SmbShare),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
            ],
            Some(&Id::HostBridge(AuthFormId::SmbWorkgroup)) => [
                Id::HostBridge(AuthFormId::SmbShare),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::SmbWorkgroup),
            ],
            Some(&Id::HostBridge(AuthFormId::RemoteDirectory)) => [
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::SmbWorkgroup),
                Id::HostBridge(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::HostBridge(AuthFormId::LocalDirectory)) => [
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::SmbWorkgroup),
                Id::HostBridge(AuthFormId::RemoteDirectory),
                Id::HostBridge(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::HostBridge(AuthFormId::Address),
                Id::HostBridge(AuthFormId::Port),
                Id::HostBridge(AuthFormId::SmbShare),
                Id::HostBridge(AuthFormId::Username),
            ],
        }
    }

    #[cfg(posix)]
    fn get_remote_smb_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(
                &Id::Remote(AuthFormId::Address)
                | &Id::Remote(AuthFormId::Port)
                | &Id::Remote(AuthFormId::SmbShare)
                | &Id::Remote(AuthFormId::Username),
            ) => [
                Id::Remote(AuthFormId::Address),
                Id::Remote(AuthFormId::Port),
                Id::Remote(AuthFormId::SmbShare),
                Id::Remote(AuthFormId::Username),
            ],
            Some(&Id::Remote(AuthFormId::Password)) => [
                Id::Remote(AuthFormId::Port),
                Id::Remote(AuthFormId::SmbShare),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
            ],
            Some(&Id::Remote(AuthFormId::SmbWorkgroup)) => [
                Id::Remote(AuthFormId::SmbShare),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::SmbWorkgroup),
            ],
            Some(&Id::Remote(AuthFormId::RemoteDirectory)) => [
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::SmbWorkgroup),
                Id::Remote(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::Remote(AuthFormId::LocalDirectory)) => [
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::SmbWorkgroup),
                Id::Remote(AuthFormId::RemoteDirectory),
                Id::Remote(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::Remote(AuthFormId::Address),
                Id::Remote(AuthFormId::Port),
                Id::Remote(AuthFormId::SmbShare),
                Id::Remote(AuthFormId::Username),
            ],
        }
    }

    #[cfg(win)]
    fn get_host_bridge_smb_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(
                &Id::HostBridge(AuthFormId::Address)
                | &Id::HostBridge(AuthFormId::Password)
                | &Id::HostBridge(AuthFormId::SmbShare)
                | &Id::HostBridge(AuthFormId::Username),
            ) => [
                Id::HostBridge(AuthFormId::Address),
                Id::HostBridge(AuthFormId::SmbShare),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
            ],
            Some(&Id::HostBridge(AuthFormId::RemoteDirectory)) => [
                Id::HostBridge(AuthFormId::SmbShare),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::HostBridge(AuthFormId::LocalDirectory)) => [
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::RemoteDirectory),
                Id::HostBridge(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::HostBridge(AuthFormId::Address),
                Id::HostBridge(AuthFormId::SmbShare),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
            ],
        }
    }

    #[cfg(win)]
    fn get_remote_smb_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(
                &Id::Remote(AuthFormId::Address)
                | &Id::Remote(AuthFormId::Password)
                | &Id::Remote(AuthFormId::SmbShare)
                | &Id::Remote(AuthFormId::Username),
            ) => [
                Id::Remote(AuthFormId::Address),
                Id::Remote(AuthFormId::SmbShare),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
            ],
            Some(&Id::Remote(AuthFormId::RemoteDirectory)) => [
                Id::Remote(AuthFormId::SmbShare),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::Remote(AuthFormId::LocalDirectory)) => [
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::RemoteDirectory),
                Id::Remote(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::Remote(AuthFormId::Address),
                Id::Remote(AuthFormId::SmbShare),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
            ],
        }
    }

    fn get_host_bridge_webdav_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::HostBridge(AuthFormId::LocalDirectory)) => [
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::RemoteDirectory),
                Id::HostBridge(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::HostBridge(AuthFormId::WebDAVUri),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::RemoteDirectory),
            ],
        }
    }

    fn get_remote_webdav_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::Remote(AuthFormId::LocalDirectory)) => [
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::RemoteDirectory),
                Id::Remote(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::Remote(AuthFormId::WebDAVUri),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::RemoteDirectory),
            ],
        }
    }

    fn init_global_listener(&mut self) {
        use tuirealm::event::{Key, KeyEvent, KeyModifiers};
        assert!(
            self.app
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
                .is_ok()
        );
    }

    pub(super) fn get_current_form_tab(&self) -> FormTab {
        match self.app.focus() {
            Some(&Id::HostBridge(_)) => FormTab::HostBridge,
            _ => FormTab::Remote,
        }
    }

    /// Returns a sub clause which requires that no popup is mounted in order to be satisfied
    fn no_popup_mounted_clause() -> SubClause<Id> {
        tuirealm::subclause_and_not!(
            Id::ErrorPopup,
            Id::InfoPopup,
            Id::Keybindings,
            Id::DeleteBookmarkPopup,
            Id::DeleteRecentPopup,
            Id::InstallUpdatePopup,
            Id::BookmarkSavePassword,
            Id::WaitPopup
        )
    }
}
