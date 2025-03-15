//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

// Locals
// Ext
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};

use super::{Context, Id, IdCommon, IdTheme, SetupActivity, Theme, ViewLayout, components};

impl SetupActivity {
    // -- view

    /// Initialize thene view
    pub(super) fn init_theme(&mut self) {
        // Init view (and mount commons)
        self.new_app(ViewLayout::Theme);
        // Mount titles
        self.load_titles();
        // Load styles
        self.load_styles();
        // Active first field
        assert!(self.app.active(&Id::Theme(IdTheme::AuthProtocol)).is_ok());
    }

    pub(super) fn view_theme(&mut self) {
        let mut ctx: Context = self.context.take().unwrap();
        let _ = ctx.terminal().raw_mut().draw(|f| {
            // Prepare main chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3), // Current tab
                        Constraint::Min(22),   // Main body
                        Constraint::Length(1), // Help footer
                    ]
                    .as_ref(),
                )
                .split(f.area());
            // Render common widget
            self.app.view(&Id::Common(IdCommon::Header), f, chunks[0]);
            self.app.view(&Id::Common(IdCommon::Footer), f, chunks[2]);
            // Make chunks
            let colors_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ]
                    .as_ref(),
                )
                .split(chunks[1]);
            let auth_colors_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(1), // Title
                        Constraint::Length(3), // Protocol
                        Constraint::Length(3), // Addr
                        Constraint::Length(3), // Port
                        Constraint::Length(3), // Username
                        Constraint::Length(3), // Password
                        Constraint::Length(3), // Bookmarks
                        Constraint::Length(3), // Recents
                        Constraint::Length(1), // Prevent overflow
                    ]
                    .as_ref(),
                )
                .split(colors_layout[0]);
            self.app
                .view(&Id::Theme(IdTheme::AuthTitle), f, auth_colors_layout[0]);
            self.app
                .view(&Id::Theme(IdTheme::AuthProtocol), f, auth_colors_layout[1]);
            self.app
                .view(&Id::Theme(IdTheme::AuthAddress), f, auth_colors_layout[2]);
            self.app
                .view(&Id::Theme(IdTheme::AuthPort), f, auth_colors_layout[3]);
            self.app
                .view(&Id::Theme(IdTheme::AuthUsername), f, auth_colors_layout[4]);
            self.app
                .view(&Id::Theme(IdTheme::AuthPassword), f, auth_colors_layout[5]);
            self.app
                .view(&Id::Theme(IdTheme::AuthBookmarks), f, auth_colors_layout[6]);
            self.app.view(
                &Id::Theme(IdTheme::AuthRecentHosts),
                f,
                auth_colors_layout[7],
            );
            let misc_colors_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(1), // Title
                        Constraint::Length(3), // Error
                        Constraint::Length(3), // Info
                        Constraint::Length(3), // Input
                        Constraint::Length(3), // Keys
                        Constraint::Length(3), // Quit
                        Constraint::Length(3), // Save
                        Constraint::Length(3), // Warn
                        Constraint::Length(1), // Prevent overflow
                    ]
                    .as_ref(),
                )
                .split(colors_layout[1]);
            self.app
                .view(&Id::Theme(IdTheme::MiscTitle), f, misc_colors_layout[0]);
            self.app
                .view(&Id::Theme(IdTheme::MiscError), f, misc_colors_layout[1]);
            self.app
                .view(&Id::Theme(IdTheme::MiscInfo), f, misc_colors_layout[2]);
            self.app
                .view(&Id::Theme(IdTheme::MiscInput), f, misc_colors_layout[3]);
            self.app
                .view(&Id::Theme(IdTheme::MiscKeys), f, misc_colors_layout[4]);
            self.app
                .view(&Id::Theme(IdTheme::MiscQuit), f, misc_colors_layout[5]);
            self.app
                .view(&Id::Theme(IdTheme::MiscSave), f, misc_colors_layout[6]);
            self.app
                .view(&Id::Theme(IdTheme::MiscWarn), f, misc_colors_layout[7]);

            let transfer_colors_layout_col1 = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(1), // Title
                        Constraint::Length(3), // local explorer bg
                        Constraint::Length(3), // local explorer fg
                        Constraint::Length(3), // local explorer hg
                        Constraint::Length(3), // remote explorer bg
                        Constraint::Length(3), // remote explorer fg
                        Constraint::Length(3), // remote explorer hg
                        Constraint::Length(1), // Prevent overflow
                    ]
                    .as_ref(),
                )
                .split(colors_layout[2]);
            self.app.view(
                &Id::Theme(IdTheme::TransferTitle),
                f,
                transfer_colors_layout_col1[0],
            );
            self.app.view(
                &Id::Theme(IdTheme::ExplorerLocalBg),
                f,
                transfer_colors_layout_col1[1],
            );
            self.app.view(
                &Id::Theme(IdTheme::ExplorerLocalFg),
                f,
                transfer_colors_layout_col1[2],
            );
            self.app.view(
                &Id::Theme(IdTheme::ExplorerLocalHg),
                f,
                transfer_colors_layout_col1[3],
            );
            self.app.view(
                &Id::Theme(IdTheme::ExplorerRemoteBg),
                f,
                transfer_colors_layout_col1[4],
            );
            self.app.view(
                &Id::Theme(IdTheme::ExplorerRemoteFg),
                f,
                transfer_colors_layout_col1[5],
            );
            self.app.view(
                &Id::Theme(IdTheme::ExplorerRemoteHg),
                f,
                transfer_colors_layout_col1[6],
            );
            let transfer_colors_layout_col2 = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(1), // Title
                        Constraint::Length(3), // Full prog bar
                        Constraint::Length(3), // Partial prog bar
                        Constraint::Length(3), // log bg
                        Constraint::Length(3), // log window
                        Constraint::Length(3), // status sorting
                        Constraint::Length(3), // status hidden
                        Constraint::Length(3), // sync browsing
                        Constraint::Length(1), // Prevent overflow
                    ]
                    .as_ref(),
                )
                .split(colors_layout[3]);
            self.app.view(
                &Id::Theme(IdTheme::TransferTitle2),
                f,
                transfer_colors_layout_col2[0],
            );
            self.app.view(
                &Id::Theme(IdTheme::ProgBarFull),
                f,
                transfer_colors_layout_col2[1],
            );
            self.app.view(
                &Id::Theme(IdTheme::ProgBarPartial),
                f,
                transfer_colors_layout_col2[2],
            );
            self.app.view(
                &Id::Theme(IdTheme::LogBg),
                f,
                transfer_colors_layout_col2[3],
            );
            self.app.view(
                &Id::Theme(IdTheme::LogWindow),
                f,
                transfer_colors_layout_col2[4],
            );
            self.app.view(
                &Id::Theme(IdTheme::StatusSorting),
                f,
                transfer_colors_layout_col2[5],
            );
            self.app.view(
                &Id::Theme(IdTheme::StatusHidden),
                f,
                transfer_colors_layout_col2[6],
            );
            self.app.view(
                &Id::Theme(IdTheme::StatusSync),
                f,
                transfer_colors_layout_col2[7],
            );
            // Popups
            self.view_popups(f);
        });
        // Put context back to context
        self.context = Some(ctx);
    }

    fn load_titles(&mut self) {
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::AuthTitle),
                    Box::<components::AuthTitle>::default(),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::MiscTitle),
                    Box::<components::MiscTitle>::default(),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::TransferTitle),
                    Box::<components::TransferTitle>::default(),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::TransferTitle2),
                    Box::<components::TransferTitle2>::default(),
                    vec![]
                )
                .is_ok()
        );
    }

    /// Load values from theme into input fields
    pub(crate) fn load_styles(&mut self) {
        let theme: Theme = self.theme().clone();
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::AuthAddress),
                    Box::new(components::AuthAddress::new(theme.auth_address)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::AuthBookmarks),
                    Box::new(components::AuthBookmarks::new(theme.auth_bookmarks)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::AuthPassword),
                    Box::new(components::AuthPassword::new(theme.auth_password)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::AuthPort),
                    Box::new(components::AuthPort::new(theme.auth_port)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::AuthProtocol),
                    Box::new(components::AuthProtocol::new(theme.auth_protocol)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::AuthRecentHosts),
                    Box::new(components::AuthRecentHosts::new(theme.auth_recents)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::AuthUsername),
                    Box::new(components::AuthUsername::new(theme.auth_username)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::MiscError),
                    Box::new(components::MiscError::new(theme.misc_error_dialog)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::MiscInfo),
                    Box::new(components::MiscInfo::new(theme.misc_info_dialog)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::MiscInput),
                    Box::new(components::MiscInput::new(theme.misc_input_dialog)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::MiscKeys),
                    Box::new(components::MiscKeys::new(theme.misc_keys)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::MiscQuit),
                    Box::new(components::MiscQuit::new(theme.misc_quit_dialog)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::MiscSave),
                    Box::new(components::MiscSave::new(theme.misc_save_dialog)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::MiscWarn),
                    Box::new(components::MiscWarn::new(theme.misc_warn_dialog)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::ExplorerLocalBg),
                    Box::new(components::ExplorerLocalBg::new(
                        theme.transfer_local_explorer_background
                    )),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::ExplorerLocalFg),
                    Box::new(components::ExplorerLocalFg::new(
                        theme.transfer_local_explorer_foreground
                    )),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::ExplorerLocalHg),
                    Box::new(components::ExplorerLocalHg::new(
                        theme.transfer_local_explorer_highlighted
                    )),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::ExplorerRemoteBg),
                    Box::new(components::ExplorerRemoteBg::new(
                        theme.transfer_remote_explorer_background
                    )),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::ExplorerRemoteFg),
                    Box::new(components::ExplorerRemoteFg::new(
                        theme.transfer_remote_explorer_foreground
                    )),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::ExplorerRemoteHg),
                    Box::new(components::ExplorerRemoteHg::new(
                        theme.transfer_remote_explorer_highlighted
                    )),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::ProgBarFull),
                    Box::new(components::ProgBarFull::new(
                        theme.transfer_progress_bar_full
                    )),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::ProgBarPartial),
                    Box::new(components::ProgBarPartial::new(
                        theme.transfer_progress_bar_partial
                    )),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::LogBg),
                    Box::new(components::LogBg::new(theme.transfer_log_background)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::LogWindow),
                    Box::new(components::LogWindow::new(theme.transfer_log_window)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::StatusSorting),
                    Box::new(components::StatusSorting::new(
                        theme.transfer_status_sorting
                    )),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::StatusHidden),
                    Box::new(components::StatusHidden::new(theme.transfer_status_hidden)),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            self.app
                .remount(
                    Id::Theme(IdTheme::StatusSync),
                    Box::new(components::StatusSync::new(
                        theme.transfer_status_sync_browsing
                    )),
                    vec![]
                )
                .is_ok()
        );
    }
}
