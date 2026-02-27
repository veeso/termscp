//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::ratatui::widgets::Clear;
use tuirealm::{Attribute, Sub, SubClause, SubEventClause};
use unicode_width::UnicodeWidthStr;

use crate::ui::activities::filetransfer::browser::FoundExplorerTab;
use crate::ui::activities::filetransfer::components;
use crate::ui::activities::filetransfer::{Context, FileTransferActivity, Id, ui_result};
use crate::utils::ui::{Popup, Size};

impl FileTransferActivity {
    // -- init

    /// Initialize file transfer activity's view
    pub(in crate::ui::activities::filetransfer) fn init(&mut self) {
        // Mount local file explorer
        let local_explorer_background = self.theme().transfer_local_explorer_background;
        let local_explorer_foreground = self.theme().transfer_local_explorer_foreground;
        let local_explorer_highlighted = self.theme().transfer_local_explorer_highlighted;
        let remote_explorer_background = self.theme().transfer_remote_explorer_background;
        let remote_explorer_foreground = self.theme().transfer_remote_explorer_foreground;
        let remote_explorer_highlighted = self.theme().transfer_remote_explorer_highlighted;
        let key_color = self.theme().misc_keys;
        let log_panel = self.theme().transfer_log_window;
        let log_background = self.theme().transfer_log_background;
        ui_result(self.app.mount(
            Id::FooterBar,
            Box::new(components::FooterBar::new(key_color)),
            vec![],
        ));
        ui_result(self.app.mount(
            Id::ExplorerHostBridge,
            Box::new(components::ExplorerLocal::new(
                "",
                &[],
                local_explorer_background,
                local_explorer_foreground,
                local_explorer_highlighted,
            )),
            vec![],
        ));
        ui_result(self.app.mount(
            Id::ExplorerRemote,
            Box::new(components::ExplorerRemote::new(
                "",
                &[],
                remote_explorer_background,
                remote_explorer_foreground,
                remote_explorer_highlighted,
            )),
            vec![],
        ));
        ui_result(self.app.mount(
            Id::Log,
            Box::new(components::Log::new(vec![], log_panel, log_background)),
            vec![],
        ));
        self.refresh_host_bridge_transfer_queue();
        self.refresh_remote_transfer_queue();
        // Load status bar
        self.refresh_local_status_bar();
        self.refresh_remote_status_bar();
        // Update components
        self.update_host_bridge_filelist();
        // self.update_remote_filelist();
        // Global listener
        self.mount_global_listener();
        // Give focus to local explorer
        ui_result(self.app.active(&Id::ExplorerHostBridge));
    }

    // -- view

    /// View gui
    pub(in crate::ui::activities::filetransfer) fn view(&mut self) {
        self.redraw = false;
        let mut context: Context = self.context.take().unwrap();
        let _ = context.terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let body = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Min(7),    // Body
                        Constraint::Length(1), // Footer
                    ]
                    .as_ref(),
                )
                .split(f.area());
            // main chunks
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(70), // Explorer
                        Constraint::Percentage(30), // Log
                    ]
                    .as_ref(),
                )
                .split(body[0]);
            // Create explorer chunks
            let tabs_chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .split(main_chunks[0]);
            // Create log box chunks
            let bottom_chunks = Layout::default()
                .constraints([Constraint::Length(1), Constraint::Length(10)].as_ref())
                .direction(Direction::Vertical)
                .split(main_chunks[1]);
            // Create status bar chunks
            let status_bar_chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .horizontal_margin(1)
                .split(bottom_chunks[0]);
            let bottom_components = Layout::default()
                .constraints(
                    [
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .direction(Direction::Horizontal)
                .split(bottom_chunks[1]);
            // Draw footer
            self.app.view(&Id::FooterBar, f, body[1]);
            // Draw explorers
            // @! Local explorer (Find or default)
            if matches!(self.browser.found_tab(), Some(FoundExplorerTab::Local)) {
                self.app.view(&Id::ExplorerFind, f, tabs_chunks[0]);
            } else if self.browser.is_terminal_open_host_bridge() {
                self.app.view(&Id::TerminalHostBridge, f, tabs_chunks[0]);
            } else {
                self.app.view(&Id::ExplorerHostBridge, f, tabs_chunks[0]);
            }
            // @! Remote explorer (Find or default)
            if matches!(self.browser.found_tab(), Some(FoundExplorerTab::Remote)) {
                self.app.view(&Id::ExplorerFind, f, tabs_chunks[1]);
            } else if self.browser.is_terminal_open_remote() {
                self.app.view(&Id::TerminalRemote, f, tabs_chunks[1]);
            } else {
                self.app.view(&Id::ExplorerRemote, f, tabs_chunks[1]);
            }
            // draw transfer queues
            self.app
                .view(&Id::TransferQueueHostBridge, f, bottom_components[0]);
            self.app
                .view(&Id::TransferQueueRemote, f, bottom_components[1]);
            // Draw log box
            self.app.view(&Id::Log, f, bottom_components[2]);
            // Draw status bar
            self.app
                .view(&Id::StatusBarHostBridge, f, status_bar_chunks[0]);
            self.app.view(&Id::StatusBarRemote, f, status_bar_chunks[1]);
            // @! Draw popups
            if self.app.mounted(&Id::FatalPopup) {
                let popup = Popup(
                    Size::Percentage(50),
                    self.calc_popup_height(Id::FatalPopup, f.area().width, f.area().height),
                )
                .draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::FatalPopup, f, popup);
            } else if self.app.mounted(&Id::CopyPopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::CopyPopup, f, popup);
            } else if self.app.mounted(&Id::ChmodPopup) {
                let popup = Popup(Size::Percentage(50), Size::Unit(12)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::ChmodPopup, f, popup);
            } else if self.app.mounted(&Id::FilterPopup) {
                let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::FilterPopup, f, popup);
            } else if self.app.mounted(&Id::GotoPopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::GotoPopup, f, popup);
            } else if self.app.mounted(&Id::MkdirPopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::MkdirPopup, f, popup);
            } else if self.app.mounted(&Id::NewfilePopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::NewfilePopup, f, popup);
            } else if self.app.mounted(&Id::OpenWithPopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::OpenWithPopup, f, popup);
            } else if self.app.mounted(&Id::RenamePopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::RenamePopup, f, popup);
            } else if self.app.mounted(&Id::SaveAsPopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::SaveAsPopup, f, popup);
            } else if self.app.mounted(&Id::SymlinkPopup) {
                let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::SymlinkPopup, f, popup);
            } else if self.app.mounted(&Id::FileInfoPopup) {
                let popup = Popup(Size::Percentage(80), Size::Percentage(50)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::FileInfoPopup, f, popup);
            } else if self.app.mounted(&Id::ProgressBarPartial) {
                let popup = Popup(Size::Percentage(50), Size::Percentage(20)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                let popup_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(50), // Full
                            Constraint::Percentage(50), // Partial
                        ]
                        .as_ref(),
                    )
                    .split(popup);
                self.app.view(&Id::ProgressBarFull, f, popup_chunks[0]);
                self.app.view(&Id::ProgressBarPartial, f, popup_chunks[1]);
            } else if self.app.mounted(&Id::DeletePopup) {
                let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::DeletePopup, f, popup);
            } else if self.app.mounted(&Id::ReplacePopup) {
                let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::ReplacePopup, f, popup);
            } else if self.app.mounted(&Id::DisconnectPopup) {
                let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::DisconnectPopup, f, popup);
            } else if self.app.mounted(&Id::QuitPopup) {
                let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::QuitPopup, f, popup);
            } else if self.app.mounted(&Id::WatchedPathsList) {
                let popup = Popup(Size::Percentage(60), Size::Percentage(50)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::WatchedPathsList, f, popup);
            } else if self.app.mounted(&Id::WatcherPopup) {
                let popup = Popup(Size::Percentage(60), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::WatcherPopup, f, popup);
            } else if self.app.mounted(&Id::SortingPopup) {
                let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::SortingPopup, f, popup);
            } else if self.app.mounted(&Id::ErrorPopup) {
                let popup = Popup(
                    Size::Percentage(50),
                    self.calc_popup_height(Id::ErrorPopup, f.area().width, f.area().height),
                )
                .draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::ErrorPopup, f, popup);
            } else if self.app.mounted(&Id::WaitPopup) {
                let wait_popup_lines = self
                    .app
                    .query(&Id::WaitPopup, Attribute::Text)
                    .map(|x| x.map(|x| x.unwrap_payload().unwrap_vec().len()))
                    .unwrap_or_default()
                    .unwrap_or(1) as u16;

                let popup =
                    Popup(Size::Percentage(50), Size::Unit(2 + wait_popup_lines)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::WaitPopup, f, popup);
            } else if self.app.mounted(&Id::SyncBrowsingMkdirPopup) {
                let popup = Popup(Size::Percentage(60), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::SyncBrowsingMkdirPopup, f, popup);
            } else if self.app.mounted(&Id::KeybindingsPopup) {
                let popup = Popup(Size::Percentage(50), Size::Percentage(80)).draw_in(f.area());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::KeybindingsPopup, f, popup);
            }
        });
        // Re-give context
        self.context = Some(context);
    }

    // -- dynamic size

    /// Given the id of the component to display and the width and height of the total area,
    /// returns the height in percentage to the entire area height, that the popup should have
    fn calc_popup_height(&self, id: Id, width: u16, height: u16) -> Size {
        // Get current text width
        let text_width = self
            .app
            .query(&id, tuirealm::Attribute::Text)
            .ok()
            .flatten()
            .map(|x| {
                if x.as_payload().is_none() {
                    return 0;
                }

                x.unwrap_payload()
                    .unwrap_vec()
                    .into_iter()
                    .map(|x| x.unwrap_text_span().content)
                    .collect::<Vec<String>>()
                    .join("")
                    .width() as u16
            })
            .unwrap_or(0);
        // Calc real width of a row in the popup
        let row_width = (width / 2).saturating_sub(2);
        // Calc row height in percentage (1 : height = x : 100)
        let row_height_p = (100.0 / (height as f64)).ceil() as u16;
        // Get amount of required rows NOTE: + 2 because of margins
        let display_rows = ((text_width as f64) / (row_width as f64)).ceil() as u16 + 2;
        // Return height (row_height_p * display_rows)
        Size::Percentage(display_rows * row_height_p)
    }

    // -- global listener

    fn mount_global_listener(&mut self) {
        ui_result(self.app.mount(
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
                        code: Key::Char('h'),
                        modifiers: KeyModifiers::NONE,
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
                        code: Key::Function(10),
                        modifiers: KeyModifiers::NONE,
                    }),
                    Self::no_popup_mounted_clause(),
                ),
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Char('q'),
                        modifiers: KeyModifiers::NONE,
                    }),
                    Self::no_popup_mounted_clause(),
                ),
                Sub::new(SubEventClause::WindowResize, SubClause::Always),
            ],
        ));
    }

    /// Returns a sub clause which requires that no popup is mounted in order to be satisfied
    fn no_popup_mounted_clause() -> SubClause<Id> {
        tuirealm::subclause_and_not!(
            Id::CopyPopup,
            Id::DeletePopup,
            Id::DisconnectPopup,
            Id::ErrorPopup,
            Id::TerminalHostBridge,
            Id::TerminalRemote,
            Id::FatalPopup,
            Id::FileInfoPopup,
            Id::GotoPopup,
            Id::KeybindingsPopup,
            Id::MkdirPopup,
            Id::NewfilePopup,
            Id::OpenWithPopup,
            Id::ProgressBarFull,
            Id::ProgressBarPartial,
            Id::ExplorerFind,
            Id::QuitPopup,
            Id::RenamePopup,
            Id::ReplacePopup,
            Id::SaveAsPopup,
            Id::SortingPopup,
            Id::SyncBrowsingMkdirPopup,
            Id::SymlinkPopup,
            Id::WatcherPopup,
            Id::WatchedPathsList,
            Id::ChmodPopup,
            Id::WaitPopup,
            Id::FilterPopup
        )
    }
}
