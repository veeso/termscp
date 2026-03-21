//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

use tuirealm::props::Color;
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::ratatui::widgets::Clear;

use super::{
    AuthActivity, AuthFormId, Context, FileTransferProtocol, FormTab, HostBridgeProtocol, Id,
    InputMask, components,
};
use crate::utils::ui::{Popup, Size};

#[path = "view/mounting.rs"]
mod mounting;
#[path = "view/query.rs"]
mod query;
#[path = "view/visible.rs"]
mod visible;

impl AuthActivity {
    /// Initialize view, mounting all startup components inside the view
    pub(super) fn init(&mut self) {
        let key_color = self.theme().misc_keys;
        let info_color = self.theme().misc_info_dialog;
        if let Err(err) = self
            .app
            .mount(Id::Title, Box::<components::Title>::default(), vec![])
        {
            error!("Failed to mount component: {err}");
        }
        if let Err(err) =
            self.app
                .mount(Id::Subtitle, Box::<components::Subtitle>::default(), vec![])
        {
            error!("Failed to mount component: {err}");
        }
        if let Err(err) = self.app.mount(
            Id::HelpFooter,
            Box::new(components::HelpFooter::new(key_color)),
            vec![],
        ) {
            error!("Failed to mount component: {err}");
        }

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

        let remote_default_protocol = self.context().config().get_default_protocol();
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

        if let Some(version) = self
            .context()
            .store()
            .get_string(super::STORE_KEY_LATEST_VERSION)
        {
            let version = version.to_string();
            if let Err(err) = self.app.mount(
                Id::NewVersionDisclaimer,
                Box::new(components::NewVersionDisclaimer::new(
                    version.as_str(),
                    info_color,
                )),
                vec![],
            ) {
                error!("Failed to mount component: {err}");
            }
        }
        self.view_bookmarks();
        self.view_recent_connections();
        self.init_global_listener();
        if let Err(err) = self.app.active(&Id::Remote(AuthFormId::Protocol)) {
            error!("Failed to activate component: {err}");
        }
    }

    /// Display view on canvas
    pub(super) fn view(&mut self) {
        self.redraw = false;
        let mut ctx: Context = self.context.take().unwrap();
        let _ = ctx.terminal().raw_mut().draw(|f| {
            let height = f.area().height;
            self.check_minimum_window_size(height);
            let body = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(24), Constraint::Length(1)].as_ref())
                .split(f.area());
            self.app.view(&Id::HelpFooter, f, body[1]);
            let auth_form_len = 7 + self.max_input_mask_size();
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(auth_form_len), Constraint::Min(3)].as_ref())
                .split(body[0]);
            let auth_chunks = Layout::default()
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(self.max_input_mask_size()),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .direction(Direction::Vertical)
                .split(main_chunks[0]);
            let bookmark_chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .spacing(2)
                .split(main_chunks[1]);
            self.app.view(&Id::Title, f, auth_chunks[0]);
            self.app.view(&Id::Subtitle, f, auth_chunks[1]);
            self.app.view(&Id::NewVersionDisclaimer, f, auth_chunks[2]);

            let host_bridge_and_remote_chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .spacing(2)
                .direction(Direction::Horizontal)
                .split(auth_chunks[3]);
            self.render_host_bridge_input_mask(f, host_bridge_and_remote_chunks[0]);
            self.render_remote_input_mask(f, host_bridge_and_remote_chunks[1]);
            self.app.view(&Id::BookmarksList, f, bookmark_chunks[0]);
            self.app.view(&Id::RecentsList, f, bookmark_chunks[1]);
            self.render_popup(f);
        });
        self.context = Some(ctx);
    }

    fn render_popup(&mut self, f: &mut tuirealm::ratatui::Frame<'_>) {
        if self.app.mounted(&Id::ErrorPopup) {
            let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.area());
            f.render_widget(Clear, popup);
            self.app.view(&Id::ErrorPopup, f, popup);
        } else if self.app.mounted(&Id::InfoPopup) {
            let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.area());
            f.render_widget(Clear, popup);
            self.app.view(&Id::InfoPopup, f, popup);
        } else if self.app.mounted(&Id::WaitPopup) {
            let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.area());
            f.render_widget(Clear, popup);
            self.app.view(&Id::WaitPopup, f, popup);
        } else if self.app.mounted(&Id::WindowSizeError) {
            let popup = Popup(Size::Percentage(80), Size::Percentage(20)).draw_in(f.area());
            f.render_widget(Clear, popup);
            self.app.view(&Id::WindowSizeError, f, popup);
        } else if self.app.mounted(&Id::QuitPopup) {
            let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.area());
            f.render_widget(Clear, popup);
            self.app.view(&Id::QuitPopup, f, popup);
        } else if self.app.mounted(&Id::DeleteBookmarkPopup) {
            let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.area());
            f.render_widget(Clear, popup);
            self.app.view(&Id::DeleteBookmarkPopup, f, popup);
        } else if self.app.mounted(&Id::DeleteRecentPopup) {
            let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.area());
            f.render_widget(Clear, popup);
            self.app.view(&Id::DeleteRecentPopup, f, popup);
        } else if self.app.mounted(&Id::NewVersionChangelog) {
            let popup = Popup(Size::Percentage(90), Size::Percentage(85)).draw_in(f.area());
            f.render_widget(Clear, popup);
            let popup_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(90), Constraint::Length(3)].as_ref())
                .split(popup);
            self.app.view(&Id::NewVersionChangelog, f, popup_chunks[0]);
            self.app.view(&Id::InstallUpdatePopup, f, popup_chunks[1]);
        } else if self.app.mounted(&Id::Keybindings) {
            let popup = Popup(Size::Percentage(50), Size::Percentage(70)).draw_in(f.area());
            f.render_widget(Clear, popup);
            self.app.view(&Id::Keybindings, f, popup);
        } else if self.app.mounted(&Id::BookmarkSavePassword) {
            let popup = Popup(Size::Percentage(20), Size::Percentage(20)).draw_in(f.area());
            f.render_widget(Clear, popup);
            let popup_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Length(4)].as_ref())
                .split(popup);
            self.app.view(&Id::BookmarkName, f, popup_chunks[0]);
            self.app.view(&Id::BookmarkSavePassword, f, popup_chunks[1]);
        }
    }

    fn render_host_bridge_input_mask(
        &mut self,
        f: &mut tuirealm::ratatui::Frame<'_>,
        area: tuirealm::ratatui::layout::Rect,
    ) {
        let protocol_and_mask_chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Length(12)].as_ref())
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
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .direction(Direction::Vertical)
            .split(protocol_and_mask_chunks[1]);
        match self.host_bridge_input_mask() {
            InputMask::AwsS3 => self.render_view_ids(f, input_mask, self.get_host_bridge_s3_view()),
            InputMask::Generic => {
                self.render_view_ids(f, input_mask, self.get_host_bridge_generic_params_view())
            }
            InputMask::Kube => {
                self.render_view_ids(f, input_mask, self.get_host_bridge_kube_view())
            }
            InputMask::Localhost => {
                let view_ids = self.get_host_bridge_localhost_view();
                self.app.view(&view_ids[0], f, input_mask[0]);
            }
            InputMask::Smb => self.render_view_ids(f, input_mask, self.get_host_bridge_smb_view()),
            InputMask::WebDAV => {
                self.render_view_ids(f, input_mask, self.get_host_bridge_webdav_view())
            }
        }
    }

    fn render_remote_input_mask(
        &mut self,
        f: &mut tuirealm::ratatui::Frame<'_>,
        area: tuirealm::ratatui::layout::Rect,
    ) {
        let protocol_and_mask_chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Length(12)].as_ref())
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
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .direction(Direction::Vertical)
            .split(protocol_and_mask_chunks[1]);
        match self.remote_input_mask() {
            InputMask::AwsS3 => self.render_view_ids(f, input_mask, self.get_remote_s3_view()),
            InputMask::Generic => {
                self.render_view_ids(f, input_mask, self.get_remote_generic_params_view())
            }
            InputMask::Kube => self.render_view_ids(f, input_mask, self.get_remote_kube_view()),
            InputMask::Localhost => unreachable!(),
            InputMask::Smb => self.render_view_ids(f, input_mask, self.get_remote_smb_view()),
            InputMask::WebDAV => self.render_view_ids(f, input_mask, self.get_remote_webdav_view()),
        }
    }

    fn render_view_ids(
        &mut self,
        f: &mut tuirealm::ratatui::Frame<'_>,
        input_mask: std::rc::Rc<[tuirealm::ratatui::layout::Rect]>,
        view_ids: [Id; 4],
    ) {
        self.app.view(&view_ids[0], f, input_mask[0]);
        self.app.view(&view_ids[1], f, input_mask[1]);
        self.app.view(&view_ids[2], f, input_mask[2]);
        self.app.view(&view_ids[3], f, input_mask[3]);
    }
}
