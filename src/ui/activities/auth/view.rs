//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

use tuirealm::props::Color;
use tuirealm::ratatui::layout::{Constraint, Direction, Layout, Rect};
use tuirealm::ratatui::widgets::Clear;
use tuirealm::terminal::TerminalAdapter;
use unicode_width::UnicodeWidthStr;

use super::{
    AuthActivity, AuthFormId, Context, FileTransferProtocol, FormTab, HostBridgeProtocol, Id,
    InputMask, components,
};
use crate::filetransfer::params::DEFAULT_GCS_ENDPOINT;
#[cfg(posix)]
use crate::filetransfer::params::SmbDialect;
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
        self.mount_gcs_bucket(FormTab::HostBridge, "");
        self.mount_gcs_endpoint(FormTab::HostBridge, DEFAULT_GCS_ENDPOINT);
        self.mount_gcs_service_account_key(FormTab::HostBridge, "");
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
        #[cfg(posix)]
        self.mount_smb_dialect(FormTab::HostBridge, SmbDialect::default());
        #[cfg(posix)]
        self.mount_smb_dialect_warning(FormTab::HostBridge);
        self.mount_webdav_uri(FormTab::HostBridge, "");

        let remote_default_protocol = self.context().config().get_default_protocol();
        self.set_remote_protocol(remote_default_protocol);
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
        self.mount_gcs_bucket(FormTab::Remote, "");
        self.mount_gcs_endpoint(FormTab::Remote, DEFAULT_GCS_ENDPOINT);
        self.mount_gcs_service_account_key(FormTab::Remote, "");
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
        #[cfg(posix)]
        self.mount_smb_dialect(FormTab::Remote, SmbDialect::default());
        #[cfg(posix)]
        self.mount_smb_dialect_warning(FormTab::Remote);
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
            let height = error_popup_height(
                &self.error_popup_text,
                popup.width.saturating_sub(2),
                f.area().height,
            );
            let popup = Popup(Size::Percentage(50), Size::Unit(height)).draw_in(f.area());
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
        let input_mask_size = Self::input_mask_size(self.host_bridge_input_mask());
        let input_mask = self.host_bridge_input_mask();
        let protocol_and_mask_chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Length(input_mask_size)].as_ref())
            .direction(Direction::Vertical)
            .split(area);

        self.app.view(
            &Id::HostBridge(AuthFormId::Protocol),
            f,
            protocol_and_mask_chunks[0],
        );

        let view_ids = match input_mask {
            InputMask::AwsS3 => self.get_host_bridge_s3_view(),
            InputMask::Gcs => self.get_host_bridge_gcs_view(),
            InputMask::Generic => self.get_host_bridge_generic_params_view(),
            InputMask::Kube => self.get_host_bridge_kube_view(),
            InputMask::Localhost => {
                let view_ids = self.get_host_bridge_localhost_view();
                self.app.view(&view_ids[0], f, protocol_and_mask_chunks[1]);
                return;
            }
            InputMask::Smb => self.get_host_bridge_smb_view(),
            InputMask::WebDAV => self.get_host_bridge_webdav_view(),
        };
        self.render_form_rows(
            f,
            protocol_and_mask_chunks[1],
            FormTab::HostBridge,
            view_ids,
        );
    }

    fn render_remote_input_mask(
        &mut self,
        f: &mut tuirealm::ratatui::Frame<'_>,
        area: tuirealm::ratatui::layout::Rect,
    ) {
        let input_mask_size = Self::input_mask_size(self.remote_input_mask());
        let input_mask = self.remote_input_mask();
        let protocol_and_mask_chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Length(input_mask_size)].as_ref())
            .direction(Direction::Vertical)
            .split(area);

        self.app.view(
            &Id::Remote(AuthFormId::Protocol),
            f,
            protocol_and_mask_chunks[0],
        );

        let view_ids = match input_mask {
            InputMask::AwsS3 => self.get_remote_s3_view(),
            InputMask::Gcs => self.get_remote_gcs_view(),
            InputMask::Generic => self.get_remote_generic_params_view(),
            InputMask::Kube => self.get_remote_kube_view(),
            InputMask::Localhost => unreachable!(),
            InputMask::Smb => self.get_remote_smb_view(),
            InputMask::WebDAV => self.get_remote_webdav_view(),
        };
        self.render_form_rows(f, protocol_and_mask_chunks[1], FormTab::Remote, view_ids);
    }

    /// Splits `area` into four 3-line form rows. When `warning_row` is
    /// `Some(index)`, a 1-line row is inserted directly above row `index` and
    /// returned as the second tuple element.
    fn split_input_mask(area: Rect, warning_row: Option<usize>) -> ([Rect; 4], Option<Rect>) {
        let mut constraints = Vec::with_capacity(6);
        for row in 0..4 {
            if warning_row == Some(row) {
                constraints.push(Constraint::Length(1));
            }
            constraints.push(Constraint::Length(3));
        }
        constraints.push(Constraint::Min(0));
        let chunks = Layout::default()
            .constraints(constraints)
            .direction(Direction::Vertical)
            .split(area);

        let mut rows = [Rect::default(); 4];
        let mut warning = None;
        let mut chunk = 0;
        for (row, slot) in rows.iter_mut().enumerate() {
            if warning_row == Some(row) {
                warning = Some(chunks[chunk]);
                chunk += 1;
            }
            *slot = chunks[chunk];
            chunk += 1;
        }
        (rows, warning)
    }

    /// Returns the visible row index of the SMB dialect radio when the form
    /// shows SMB and SMB1 is selected; `None` otherwise.
    #[cfg(posix)]
    fn smb_dialect_warning_row(&self, form_tab: FormTab, view_ids: &[Id; 4]) -> Option<usize> {
        let input_mask = match form_tab {
            FormTab::HostBridge => self.host_bridge_input_mask(),
            FormTab::Remote => self.remote_input_mask(),
        };
        if input_mask != InputMask::Smb || self.get_input_smb_dialect(form_tab) != SmbDialect::Smb1
        {
            return None;
        }
        let dialect_id = Self::form_tab_id(form_tab, AuthFormId::SmbDialect);
        view_ids.iter().position(|id| *id == dialect_id)
    }

    #[cfg(win)]
    fn smb_dialect_warning_row(&self, _form_tab: FormTab, _view_ids: &[Id; 4]) -> Option<usize> {
        None
    }

    fn render_form_rows(
        &mut self,
        f: &mut tuirealm::ratatui::Frame<'_>,
        area: Rect,
        form_tab: FormTab,
        view_ids: [Id; 4],
    ) {
        let warning_row = self.smb_dialect_warning_row(form_tab, &view_ids);
        let (rows, warning) = Self::split_input_mask(area, warning_row);
        #[cfg(posix)]
        if let Some(rect) = warning {
            let id = Self::form_tab_id(form_tab, AuthFormId::SmbDialectWarning);
            self.app.view(&id, f, rect);
        }
        #[cfg(win)]
        let _ = warning;
        for (id, rect) in view_ids.iter().zip(rows) {
            self.app.view(id, f, rect);
        }
    }
}

/// Collapses every run of whitespace, line breaks included, into a single space.
///
/// The popup renders its text as one span, which drops line breaks and glues the
/// fragments together. Normalizing the text first keeps the rendered text and the
/// height estimate in step.
fn normalize_error_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Returns the height of the error popup needed to show `text` in full.
///
/// `inner_width` is the width available for text, without borders. Text is wrapped on word
/// boundaries like the popup paragraph does. The result includes the top and bottom border
/// and never exceeds `max_height`.
fn error_popup_height(text: &str, inner_width: u16, max_height: u16) -> u16 {
    const MIN_HEIGHT: u16 = 3;
    if inner_width == 0 {
        return MIN_HEIGHT.min(max_height);
    }
    let width = usize::from(inner_width);
    // Number of text lines, the first one included
    let mut lines: usize = 1;
    // Width used on the current line, zero while the line is empty
    let mut used: usize = 0;
    for word in text.split_whitespace() {
        let word_width = UnicodeWidthStr::width(word);
        if word_width > width {
            // A word wider than the line takes lines of its own
            if used > 0 {
                lines += 1;
            }
            lines += word_width.div_ceil(width) - 1;
            // The next word starts on a new line
            used = width;
        } else if used == 0 {
            used = word_width;
        } else if used + 1 + word_width <= width {
            used += 1 + word_width;
        } else {
            lines += 1;
            used = word_width;
        }
    }
    let height = u16::try_from(lines.saturating_add(2)).unwrap_or(u16::MAX);
    height.min(max_height)
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use tuirealm::ratatui::layout::Rect;

    use super::{AuthActivity, error_popup_height, normalize_error_text};

    #[test]
    fn should_split_input_mask_without_warning() {
        let area = Rect::new(0, 0, 40, 13);

        let (rows, warning) = AuthActivity::split_input_mask(area, None);

        assert_eq!(warning, None);
        assert_eq!(rows[0], Rect::new(0, 0, 40, 3));
        assert_eq!(rows[1], Rect::new(0, 3, 40, 3));
        assert_eq!(rows[2], Rect::new(0, 6, 40, 3));
        assert_eq!(rows[3], Rect::new(0, 9, 40, 3));
    }

    #[test]
    fn should_split_input_mask_with_middle_warning() {
        let area = Rect::new(0, 0, 40, 13);

        let (rows, warning) = AuthActivity::split_input_mask(area, Some(2));

        assert_eq!(warning, Some(Rect::new(0, 6, 40, 1)));
        assert_eq!(rows[0], Rect::new(0, 0, 40, 3));
        assert_eq!(rows[1], Rect::new(0, 3, 40, 3));
        assert_eq!(rows[2], Rect::new(0, 7, 40, 3));
        assert_eq!(rows[3], Rect::new(0, 10, 40, 3));
    }

    #[test]
    fn should_split_input_mask_with_first_row_warning() {
        let area = Rect::new(0, 0, 40, 13);

        let (rows, warning) = AuthActivity::split_input_mask(area, Some(0));

        assert_eq!(warning, Some(Rect::new(0, 0, 40, 1)));
        assert_eq!(rows[0], Rect::new(0, 1, 40, 3));
        assert_eq!(rows[1], Rect::new(0, 4, 40, 3));
        assert_eq!(rows[2], Rect::new(0, 7, 40, 3));
        assert_eq!(rows[3], Rect::new(0, 10, 40, 3));
    }

    #[test]
    fn test_error_popup_height_should_fit_short_text_in_one_line() {
        assert_eq!(error_popup_height("bad port", 40, 30), 3);
        assert_eq!(error_popup_height("", 40, 30), 3);
    }

    #[test]
    fn test_error_popup_height_should_wrap_on_word_boundaries() {
        // "aaaa bbbb" is 9 wide: fits in 9, wraps in 8.
        assert_eq!(error_popup_height("aaaa bbbb", 9, 30), 3);
        assert_eq!(error_popup_height("aaaa bbbb", 8, 30), 4);
        assert_eq!(error_popup_height("aaaa bbbb cccc", 4, 30), 5);
    }

    #[test]
    fn test_error_popup_height_should_break_words_longer_than_width() {
        // 25 wide word in 10 columns takes 3 lines.
        assert_eq!(error_popup_height(&"x".repeat(25), 10, 30), 5);
    }

    #[test]
    fn test_error_popup_height_should_clamp_to_max_height() {
        assert_eq!(error_popup_height(&"word ".repeat(200), 10, 12), 12);
    }

    #[test]
    fn test_error_popup_height_should_handle_zero_width() {
        assert_eq!(error_popup_height("anything", 0, 30), 3);
        assert_eq!(error_popup_height("anything", 0, 2), 2);
    }

    /// Regression test for #344: the bookmarks error did not fit in one row.
    #[test]
    fn test_error_popup_height_should_grow_for_bookmarks_error() {
        let text = "Could not initialize bookmarks (at \"/home/user/.config/termscp/bookmarks.toml\", \"/home/user/.config/termscp\"): IO error (Could not write key to storage: Provider service error)";
        assert!(error_popup_height(text, 48, 30) > 3);
    }

    #[test]
    fn test_normalize_error_text_should_collapse_whitespace_and_newlines() {
        assert_eq!(
            normalize_error_text("unclosed\ninline  table\n\texpected `}`"),
            "unclosed inline table expected `}`"
        );
    }
}
