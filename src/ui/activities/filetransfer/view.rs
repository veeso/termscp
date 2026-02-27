//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
// Ext
use remotefs::fs::{File, UnixPex};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{PropPayload, PropValue, TextSpan};
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::ratatui::widgets::Clear;
use tuirealm::{AttrValue, Attribute, Sub, SubClause, SubEventClause};
use unicode_width::UnicodeWidthStr;

use super::browser::{FileExplorerTab, FoundExplorerTab};
use super::components::ATTR_FILES;
use super::{Context, FileTransferActivity, Id, components, ui_result};
use crate::explorer::FileSorting;
use crate::ui::activities::filetransfer::MarkQueue;
use crate::utils::ui::{Popup, Size};

impl FileTransferActivity {
    // -- init

    /// Initialize file transfer activity's view
    pub(super) fn init(&mut self) {
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
    pub(super) fn view(&mut self) {
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

    // -- partials

    /// Mount info box
    pub(super) fn mount_info<S: AsRef<str>>(&mut self, text: S) {
        // Mount
        let info_color = self.theme().misc_info_dialog;
        ui_result(self.app.remount(
            Id::ErrorPopup,
            Box::new(components::ErrorPopup::new(text, info_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::ErrorPopup));
    }

    /// Mount error box
    pub(super) fn mount_error<S: AsRef<str>>(&mut self, text: S) {
        // Mount
        let error_color = self.theme().misc_error_dialog;
        ui_result(self.app.remount(
            Id::ErrorPopup,
            Box::new(components::ErrorPopup::new(text, error_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::ErrorPopup));
    }

    /// Umount error message
    pub(super) fn umount_error(&mut self) {
        let _ = self.app.umount(&Id::ErrorPopup);
    }

    pub(super) fn mount_fatal<S: AsRef<str>>(&mut self, text: S) {
        self.umount_wait();
        // Mount
        let error_color = self.theme().misc_error_dialog;
        ui_result(self.app.remount(
            Id::FatalPopup,
            Box::new(components::FatalPopup::new(text, error_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::FatalPopup));
    }

    /// Umount fatal error message
    pub(super) fn umount_fatal(&mut self) {
        let _ = self.app.umount(&Id::FatalPopup);
    }

    pub(super) fn mount_wait<S: AsRef<str>>(&mut self, text: S) {
        let color = self.theme().misc_info_dialog;
        ui_result(self.app.remount(
            Id::WaitPopup,
            Box::new(components::WaitPopup::new(text, color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::WaitPopup));
    }

    pub(super) fn mount_walkdir_wait(&mut self) {
        let color = self.theme().misc_info_dialog;
        ui_result(self.app.remount(
            Id::WaitPopup,
            Box::new(components::WalkdirWaitPopup::new(
                "Scanning current directory…",
                color,
            )),
            vec![],
        ));
        ui_result(self.app.active(&Id::WaitPopup));

        self.view();
    }

    pub(super) fn update_walkdir_entries(&mut self, entries: usize) {
        let text = format!("Scanning current directory… ({entries} items found)",);
        let _ = self.app.attr(
            &Id::WaitPopup,
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(vec![
                PropValue::TextSpan(TextSpan::from(text)),
                PropValue::TextSpan(TextSpan::from("Press 'CTRL+C' to abort")),
            ])),
        );

        self.view();
    }

    pub(super) fn mount_blocking_wait<S: AsRef<str>>(&mut self, text: S) {
        self.mount_wait(text);
        self.view();
    }

    pub(super) fn umount_wait(&mut self) {
        let _ = self.app.umount(&Id::WaitPopup);
    }

    /// Mount quit popup
    pub(super) fn mount_quit(&mut self) {
        // Protocol
        let quit_color = self.theme().misc_quit_dialog;
        ui_result(self.app.remount(
            Id::QuitPopup,
            Box::new(components::QuitPopup::new(quit_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::QuitPopup));
    }

    /// Umount quit popup
    pub(super) fn umount_quit(&mut self) {
        let _ = self.app.umount(&Id::QuitPopup);
    }

    /// Mount disconnect popup
    pub(super) fn mount_disconnect(&mut self) {
        // Protocol
        let quit_color = self.theme().misc_quit_dialog;
        ui_result(self.app.remount(
            Id::DisconnectPopup,
            Box::new(components::DisconnectPopup::new(quit_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::DisconnectPopup));
    }

    /// Umount disconnect popup
    pub(super) fn umount_disconnect(&mut self) {
        let _ = self.app.umount(&Id::DisconnectPopup);
    }

    pub(super) fn mount_chmod(&mut self, mode: UnixPex, title: String) {
        // Mount
        let color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::ChmodPopup,
            Box::new(components::ChmodPopup::new(mode, color, title)),
            vec![],
        ));
        ui_result(self.app.active(&Id::ChmodPopup));
    }

    pub(super) fn umount_chmod(&mut self) {
        let _ = self.app.umount(&Id::ChmodPopup);
    }

    pub(super) fn umount_filter(&mut self) {
        let _ = self.app.umount(&Id::FilterPopup);
    }

    pub(super) fn mount_filter(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::FilterPopup,
            Box::new(components::FilterPopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::FilterPopup));
    }

    pub(super) fn mount_copy(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::CopyPopup,
            Box::new(components::CopyPopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::CopyPopup));
    }

    pub(super) fn umount_copy(&mut self) {
        let _ = self.app.umount(&Id::CopyPopup);
    }

    pub(super) fn mount_exec(&mut self) {
        let tab = self.browser.tab();
        let id = match tab {
            FileExplorerTab::HostBridge => Id::TerminalHostBridge,
            FileExplorerTab::Remote => Id::TerminalRemote,
            _ => {
                error!("Cannot mount terminal on this tab");
                return;
            }
        };

        let border = match tab {
            FileExplorerTab::HostBridge => self.theme().transfer_local_explorer_highlighted,
            FileExplorerTab::Remote => self.theme().transfer_remote_explorer_highlighted,
            _ => {
                error!("Cannot mount terminal on this tab");
                return;
            }
        };

        let input_color = self.theme().misc_input_dialog;
        ui_result(
            self.app.remount(
                id.clone(),
                Box::new(
                    components::Terminal::default()
                        .foreground(input_color)
                        .prompt(self.terminal_prompt())
                        .title(format!("Terminal - {}", self.get_tab_hostname()))
                        .border_color(border),
                ),
                vec![],
            ),
        );
        ui_result(self.app.active(&id));
    }

    /// Update the terminal prompt based on the current directory
    pub(super) fn update_terminal_prompt(&mut self) {
        let prompt = self.terminal_prompt();
        let id = match self.browser.tab() {
            FileExplorerTab::HostBridge => Id::TerminalHostBridge,
            FileExplorerTab::Remote => Id::TerminalRemote,
            _ => {
                error!("Cannot update terminal prompt on this tab");
                return;
            }
        };
        let _ = self
            .app
            .attr(&id, Attribute::Content, AttrValue::String(prompt));
    }

    /// Print output to terminal
    pub(super) fn print_terminal(&mut self, text: String) {
        // get id
        let focus = self.app.focus().unwrap().clone();

        // replace all \n with \r\n
        let mut text = text.replace('\n', "\r\n");
        if !text.ends_with("\r\n") && !text.is_empty() {
            text.push_str("\r\n");
        }
        let _ = self
            .app
            .attr(&focus, Attribute::Text, AttrValue::String(text));
    }

    pub(super) fn umount_exec(&mut self) {
        let focus = self.app.focus().unwrap().clone();
        let _ = self.app.umount(&focus);
    }

    pub(super) fn mount_find(&mut self, msg: impl ToString, fuzzy_search: bool) {
        // Get color
        let (bg, fg, hg) = match self.browser.tab() {
            FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => (
                self.theme().transfer_local_explorer_background,
                self.theme().transfer_local_explorer_foreground,
                self.theme().transfer_local_explorer_highlighted,
            ),
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => (
                self.theme().transfer_remote_explorer_background,
                self.theme().transfer_remote_explorer_foreground,
                self.theme().transfer_remote_explorer_highlighted,
            ),
        };

        // Mount component
        ui_result(self.app.remount(
            Id::ExplorerFind,
            if fuzzy_search {
                Box::new(components::ExplorerFuzzy::new(
                    msg.to_string(),
                    &[],
                    bg,
                    fg,
                    hg,
                ))
            } else {
                Box::new(components::ExplorerFind::new(
                    msg.to_string(),
                    &[],
                    bg,
                    fg,
                    hg,
                ))
            },
            vec![],
        ));
        ui_result(self.app.active(&Id::ExplorerFind));
    }

    pub(super) fn umount_find(&mut self) {
        let _ = self.app.umount(&Id::ExplorerFind);
    }

    pub(super) fn mount_goto(&mut self) {
        // get files
        let files = self
            .browser
            .explorer()
            .iter_files()
            .filter(|f| f.is_dir() || f.is_symlink())
            .map(|f| f.path().to_string_lossy().to_string())
            .collect::<Vec<String>>();

        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::GotoPopup,
            Box::new(components::GotoPopup::new(input_color, files)),
            vec![],
        ));
        ui_result(self.app.active(&Id::GotoPopup));
    }

    pub(super) fn update_goto(&mut self, files: Vec<String>) {
        let payload = files
            .into_iter()
            .map(PropValue::Str)
            .collect::<Vec<PropValue>>();

        let _ = self.app.attr(
            &Id::GotoPopup,
            Attribute::Custom(ATTR_FILES),
            AttrValue::Payload(PropPayload::Vec(payload)),
        );
    }

    pub(super) fn umount_goto(&mut self) {
        let _ = self.app.umount(&Id::GotoPopup);
    }

    pub(super) fn mount_mkdir(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::MkdirPopup,
            Box::new(components::MkdirPopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::MkdirPopup));
    }

    pub(super) fn umount_mkdir(&mut self) {
        let _ = self.app.umount(&Id::MkdirPopup);
    }

    pub(super) fn mount_newfile(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::NewfilePopup,
            Box::new(components::NewfilePopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::NewfilePopup));
    }

    pub(super) fn umount_newfile(&mut self) {
        let _ = self.app.umount(&Id::NewfilePopup);
    }

    pub(super) fn mount_openwith(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::OpenWithPopup,
            Box::new(components::OpenWithPopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::OpenWithPopup));
    }

    pub(super) fn umount_openwith(&mut self) {
        let _ = self.app.umount(&Id::OpenWithPopup);
    }

    pub(super) fn mount_rename(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::RenamePopup,
            Box::new(components::RenamePopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::RenamePopup));
    }

    pub(super) fn umount_rename(&mut self) {
        let _ = self.app.umount(&Id::RenamePopup);
    }

    pub(super) fn mount_saveas(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::SaveAsPopup,
            Box::new(components::SaveAsPopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::SaveAsPopup));
    }

    pub(super) fn umount_saveas(&mut self) {
        let _ = self.app.umount(&Id::SaveAsPopup);
    }

    pub(super) fn mount_progress_bar(&mut self, root_name: String) {
        let prog_color_full = self.theme().transfer_progress_bar_full;
        let prog_color_partial = self.theme().transfer_progress_bar_partial;
        ui_result(self.app.remount(
            Id::ProgressBarFull,
            Box::new(components::ProgressBarFull::new(
                0.0,
                "",
                &root_name,
                prog_color_full,
            )),
            vec![],
        ));
        ui_result(self.app.remount(
            Id::ProgressBarPartial,
            Box::new(components::ProgressBarPartial::new(
                0.0,
                "",
                "Please wait",
                prog_color_partial,
            )),
            vec![],
        ));
        ui_result(self.app.active(&Id::ProgressBarPartial));
    }

    pub(super) fn umount_progress_bar(&mut self) {
        let _ = self.app.umount(&Id::ProgressBarPartial);
        let _ = self.app.umount(&Id::ProgressBarFull);
    }

    pub(super) fn mount_file_sorting(&mut self) {
        let sorting_color = self.theme().transfer_status_sorting;
        let sorting: FileSorting = match self.browser.tab() {
            FileExplorerTab::HostBridge => self.host_bridge().get_file_sorting(),
            FileExplorerTab::Remote => self.remote().get_file_sorting(),
            _ => return,
        };
        ui_result(self.app.remount(
            Id::SortingPopup,
            Box::new(components::SortingPopup::new(sorting, sorting_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::SortingPopup));
    }

    pub(super) fn umount_file_sorting(&mut self) {
        let _ = self.app.umount(&Id::SortingPopup);
    }

    pub(super) fn mount_radio_delete(&mut self) {
        let warn_color = self.theme().misc_warn_dialog;
        ui_result(self.app.remount(
            Id::DeletePopup,
            Box::new(components::DeletePopup::new(warn_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::DeletePopup));
    }

    pub(super) fn umount_radio_delete(&mut self) {
        let _ = self.app.umount(&Id::DeletePopup);
    }

    pub(super) fn mount_radio_watch(&mut self, watch: bool, local: &str, remote: &str) {
        let info_color = self.theme().misc_info_dialog;
        ui_result(self.app.remount(
            Id::WatcherPopup,
            Box::new(components::WatcherPopup::new(
                watch, local, remote, info_color,
            )),
            vec![],
        ));
        ui_result(self.app.active(&Id::WatcherPopup));
    }

    pub(super) fn umount_radio_watcher(&mut self) {
        let _ = self.app.umount(&Id::WatcherPopup);
    }

    pub(super) fn mount_watched_paths_list(&mut self, paths: &[std::path::PathBuf]) {
        let info_color = self.theme().misc_info_dialog;
        ui_result(self.app.remount(
            Id::WatchedPathsList,
            Box::new(components::WatchedPathsList::new(paths, info_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::WatchedPathsList));
    }

    pub(super) fn umount_watched_paths_list(&mut self) {
        let _ = self.app.umount(&Id::WatchedPathsList);
    }

    pub(super) fn mount_radio_replace(&mut self, file_name: &str) {
        let warn_color = self.theme().misc_warn_dialog;
        ui_result(self.app.remount(
            Id::ReplacePopup,
            Box::new(components::ReplacePopup::new(Some(file_name), warn_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::ReplacePopup));
    }

    pub(super) fn umount_radio_replace(&mut self) {
        let _ = self.app.umount(&Id::ReplacePopup);
    }

    pub(super) fn mount_file_info(&mut self, file: &File) {
        ui_result(self.app.remount(
            Id::FileInfoPopup,
            Box::new(components::FileInfoPopup::new(file)),
            vec![],
        ));
        ui_result(self.app.active(&Id::FileInfoPopup));
    }

    pub(super) fn umount_file_info(&mut self) {
        let _ = self.app.umount(&Id::FileInfoPopup);
    }

    pub(super) fn refresh_host_bridge_transfer_queue(&mut self) {
        let enqueued = self
            .host_bridge()
            .enqueued()
            .iter()
            .map(|(src, dest)| (src.clone(), dest.clone()))
            .collect::<Vec<_>>();
        let log_panel = self.theme().transfer_log_window;

        ui_result(self.app.remount(
            Id::TransferQueueHostBridge,
            Box::new(components::SelectedFilesList::new(
                &enqueued,
                MarkQueue::Local,
                log_panel,
                "Host Bridge selected files",
            )),
            vec![],
        ));
    }

    pub(super) fn refresh_remote_transfer_queue(&mut self) {
        let enqueued = self
            .remote()
            .enqueued()
            .iter()
            .map(|(src, dest)| (src.clone(), dest.clone()))
            .collect::<Vec<_>>();
        let log_panel = self.theme().transfer_log_window;

        ui_result(self.app.remount(
            Id::TransferQueueRemote,
            Box::new(components::SelectedFilesList::new(
                &enqueued,
                MarkQueue::Remote,
                log_panel,
                "Remote transfer selected files",
            )),
            vec![],
        ));
    }

    pub(super) fn refresh_local_status_bar(&mut self) {
        let sorting_color = self.theme().transfer_status_sorting;
        let hidden_color = self.theme().transfer_status_hidden;
        ui_result(self.app.remount(
            Id::StatusBarHostBridge,
            Box::new(components::StatusBarLocal::new(
                &self.browser,
                sorting_color,
                hidden_color,
            )),
            vec![],
        ));
    }

    pub(super) fn refresh_remote_status_bar(&mut self) {
        let sorting_color = self.theme().transfer_status_sorting;
        let hidden_color = self.theme().transfer_status_hidden;
        let sync_color = self.theme().transfer_status_sync_browsing;
        ui_result(self.app.remount(
            Id::StatusBarRemote,
            Box::new(components::StatusBarRemote::new(
                &self.browser,
                sorting_color,
                hidden_color,
                sync_color,
            )),
            vec![],
        ));
    }

    pub(super) fn mount_symlink(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::SymlinkPopup,
            Box::new(components::SymlinkPopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::SymlinkPopup));
    }

    pub(super) fn umount_symlink(&mut self) {
        let _ = self.app.umount(&Id::SymlinkPopup);
    }

    pub(super) fn mount_sync_browsing_mkdir_popup(&mut self, dir_name: &str) {
        let color = self.theme().misc_info_dialog;
        ui_result(self.app.remount(
            Id::SyncBrowsingMkdirPopup,
            Box::new(components::SyncBrowsingMkdirPopup::new(color, dir_name)),
            vec![],
        ));
        ui_result(self.app.active(&Id::SyncBrowsingMkdirPopup));
    }

    pub(super) fn umount_sync_browsing_mkdir_popup(&mut self) {
        let _ = self.app.umount(&Id::SyncBrowsingMkdirPopup);
    }

    /// Mount help
    pub(super) fn mount_help(&mut self) {
        let key_color = self.theme().misc_keys;
        ui_result(self.app.remount(
            Id::KeybindingsPopup,
            Box::new(components::KeybindingsPopup::new(key_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::KeybindingsPopup));
    }

    pub(super) fn umount_help(&mut self) {
        let _ = self.app.umount(&Id::KeybindingsPopup);
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
