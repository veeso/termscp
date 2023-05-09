//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
// Ext
use remotefs::fs::File;
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::tui::widgets::Clear;
use tuirealm::{Sub, SubClause, SubEventClause};
use unicode_width::UnicodeWidthStr;

use super::browser::{FileExplorerTab, FoundExplorerTab};
use super::{components, Context, FileTransferActivity, Id};
use crate::explorer::FileSorting;
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
        assert!(self
            .app
            .mount(
                Id::FooterBar,
                Box::new(components::FooterBar::new(key_color)),
                vec![]
            )
            .is_ok());
        assert!(self
            .app
            .mount(
                Id::ExplorerLocal,
                Box::new(components::ExplorerLocal::new(
                    "",
                    &[],
                    local_explorer_background,
                    local_explorer_foreground,
                    local_explorer_highlighted
                )),
                vec![]
            )
            .is_ok());
        assert!(self
            .app
            .mount(
                Id::ExplorerRemote,
                Box::new(components::ExplorerRemote::new(
                    "",
                    &[],
                    remote_explorer_background,
                    remote_explorer_foreground,
                    remote_explorer_highlighted
                )),
                vec![]
            )
            .is_ok());
        assert!(self
            .app
            .mount(
                Id::Log,
                Box::new(components::Log::new(vec![], log_panel, log_background)),
                vec![]
            )
            .is_ok());
        // Load status bar
        self.refresh_local_status_bar();
        self.refresh_remote_status_bar();
        // Update components
        self.update_local_filelist();
        self.update_remote_filelist();
        // Global listener
        self.mount_global_listener();
        // Give focus to local explorer
        assert!(self.app.active(&Id::ExplorerLocal).is_ok());
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
                .split(f.size());
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
            // Draw footer
            self.app.view(&Id::FooterBar, f, body[1]);
            // Draw explorers
            // @! Local explorer (Find or default)
            if matches!(self.browser.found_tab(), Some(FoundExplorerTab::Local)) {
                self.app.view(&Id::ExplorerFind, f, tabs_chunks[0]);
            } else {
                self.app.view(&Id::ExplorerLocal, f, tabs_chunks[0]);
            }
            // @! Remote explorer (Find or default)
            if matches!(self.browser.found_tab(), Some(FoundExplorerTab::Remote)) {
                self.app.view(&Id::ExplorerFind, f, tabs_chunks[1]);
            } else {
                self.app.view(&Id::ExplorerRemote, f, tabs_chunks[1]);
            }
            // Draw log box
            self.app.view(&Id::Log, f, bottom_chunks[1]);
            // Draw status bar
            self.app.view(&Id::StatusBarLocal, f, status_bar_chunks[0]);
            self.app.view(&Id::StatusBarRemote, f, status_bar_chunks[1]);
            // @! Draw popups
            if self.app.mounted(&Id::CopyPopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::CopyPopup, f, popup);
            } else if self.app.mounted(&Id::FindPopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::FindPopup, f, popup);
            } else if self.app.mounted(&Id::GotoPopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::GotoPopup, f, popup);
            } else if self.app.mounted(&Id::MkdirPopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::MkdirPopup, f, popup);
            } else if self.app.mounted(&Id::NewfilePopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::NewfilePopup, f, popup);
            } else if self.app.mounted(&Id::OpenWithPopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::OpenWithPopup, f, popup);
            } else if self.app.mounted(&Id::RenamePopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::RenamePopup, f, popup);
            } else if self.app.mounted(&Id::SaveAsPopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::SaveAsPopup, f, popup);
            } else if self.app.mounted(&Id::SymlinkPopup) {
                let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::SymlinkPopup, f, popup);
            } else if self.app.mounted(&Id::ExecPopup) {
                let popup = Popup(Size::Percentage(40), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::ExecPopup, f, popup);
            } else if self.app.mounted(&Id::FileInfoPopup) {
                let popup = Popup(Size::Percentage(50), Size::Percentage(50)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::FileInfoPopup, f, popup);
            } else if self.app.mounted(&Id::ProgressBarPartial) {
                let popup = Popup(Size::Percentage(50), Size::Percentage(20)).draw_in(f.size());
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
                let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::DeletePopup, f, popup);
            } else if self.app.mounted(&Id::ReplacePopup) {
                // NOTE: handle extended / normal modes
                if self.is_radio_replace_extended() {
                    let popup = Popup(Size::Percentage(50), Size::Percentage(50)).draw_in(f.size());
                    f.render_widget(Clear, popup);
                    let popup_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Percentage(85), // List
                                Constraint::Percentage(15), // Radio
                            ]
                            .as_ref(),
                        )
                        .split(popup);
                    self.app
                        .view(&Id::ReplacingFilesListPopup, f, popup_chunks[0]);
                    self.app.view(&Id::ReplacePopup, f, popup_chunks[1]);
                } else {
                    let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.size());
                    f.render_widget(Clear, popup);
                    // make popup
                    self.app.view(&Id::ReplacePopup, f, popup);
                }
            } else if self.app.mounted(&Id::DisconnectPopup) {
                let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::DisconnectPopup, f, popup);
            } else if self.app.mounted(&Id::QuitPopup) {
                let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::QuitPopup, f, popup);
            } else if self.app.mounted(&Id::WatchedPathsList) {
                let popup = Popup(Size::Percentage(60), Size::Percentage(50)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::WatchedPathsList, f, popup);
            } else if self.app.mounted(&Id::WatcherPopup) {
                let popup = Popup(Size::Percentage(60), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::WatcherPopup, f, popup);
            } else if self.app.mounted(&Id::SortingPopup) {
                let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::SortingPopup, f, popup);
            } else if self.app.mounted(&Id::ErrorPopup) {
                let popup = Popup(
                    Size::Percentage(50),
                    self.calc_popup_height(Id::ErrorPopup, f.size().width, f.size().height),
                )
                .draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::ErrorPopup, f, popup);
            } else if self.app.mounted(&Id::FatalPopup) {
                let popup = Popup(
                    Size::Percentage(50),
                    self.calc_popup_height(Id::FatalPopup, f.size().width, f.size().height),
                )
                .draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::FatalPopup, f, popup);
            } else if self.app.mounted(&Id::WaitPopup) {
                let popup = Popup(Size::Percentage(50), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::WaitPopup, f, popup);
            } else if self.app.mounted(&Id::SyncBrowsingMkdirPopup) {
                let popup = Popup(Size::Percentage(60), Size::Unit(3)).draw_in(f.size());
                f.render_widget(Clear, popup);
                // make popup
                self.app.view(&Id::SyncBrowsingMkdirPopup, f, popup);
            } else if self.app.mounted(&Id::KeybindingsPopup) {
                let popup = Popup(Size::Percentage(50), Size::Percentage(80)).draw_in(f.size());
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
        assert!(self
            .app
            .remount(
                Id::ErrorPopup,
                Box::new(components::ErrorPopup::new(text, info_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::ErrorPopup).is_ok());
    }

    /// Mount error box
    pub(super) fn mount_error<S: AsRef<str>>(&mut self, text: S) {
        // Mount
        let error_color = self.theme().misc_error_dialog;
        assert!(self
            .app
            .remount(
                Id::ErrorPopup,
                Box::new(components::ErrorPopup::new(text, error_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::ErrorPopup).is_ok());
    }

    /// Umount error message
    pub(super) fn umount_error(&mut self) {
        let _ = self.app.umount(&Id::ErrorPopup);
    }

    pub(super) fn mount_fatal<S: AsRef<str>>(&mut self, text: S) {
        // Mount
        let error_color = self.theme().misc_error_dialog;
        assert!(self
            .app
            .remount(
                Id::FatalPopup,
                Box::new(components::FatalPopup::new(text, error_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::FatalPopup).is_ok());
    }

    /// Umount fatal error message
    pub(super) fn umount_fatal(&mut self) {
        let _ = self.app.umount(&Id::FatalPopup);
    }

    pub(super) fn mount_wait<S: AsRef<str>>(&mut self, text: S) {
        let color = self.theme().misc_info_dialog;
        assert!(self
            .app
            .remount(
                Id::WaitPopup,
                Box::new(components::WaitPopup::new(text, color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::WaitPopup).is_ok());
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
        assert!(self
            .app
            .remount(
                Id::QuitPopup,
                Box::new(components::QuitPopup::new(quit_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::QuitPopup).is_ok());
    }

    /// Umount quit popup
    pub(super) fn umount_quit(&mut self) {
        let _ = self.app.umount(&Id::QuitPopup);
    }

    /// Mount disconnect popup
    pub(super) fn mount_disconnect(&mut self) {
        // Protocol
        let quit_color = self.theme().misc_quit_dialog;
        assert!(self
            .app
            .remount(
                Id::DisconnectPopup,
                Box::new(components::DisconnectPopup::new(quit_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::DisconnectPopup).is_ok());
    }

    /// Umount disconnect popup
    pub(super) fn umount_disconnect(&mut self) {
        let _ = self.app.umount(&Id::DisconnectPopup);
    }

    pub(super) fn mount_copy(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        assert!(self
            .app
            .remount(
                Id::CopyPopup,
                Box::new(components::CopyPopup::new(input_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::CopyPopup).is_ok());
    }

    pub(super) fn umount_copy(&mut self) {
        let _ = self.app.umount(&Id::CopyPopup);
    }

    pub(super) fn mount_exec(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        assert!(self
            .app
            .remount(
                Id::ExecPopup,
                Box::new(components::ExecPopup::new(input_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::ExecPopup).is_ok());
    }

    pub(super) fn umount_exec(&mut self) {
        let _ = self.app.umount(&Id::ExecPopup);
    }

    pub(super) fn mount_find(&mut self, search: &str) {
        // Get color
        let (bg, fg, hg) = match self.browser.tab() {
            FileExplorerTab::Local | FileExplorerTab::FindLocal => (
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
        assert!(self
            .app
            .remount(
                Id::ExplorerFind,
                Box::new(components::ExplorerFind::new(
                    format!(r#"Search results for "{search}""#),
                    &[],
                    bg,
                    fg,
                    hg
                )),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::ExplorerFind).is_ok());
    }

    pub(super) fn umount_find(&mut self) {
        let _ = self.app.umount(&Id::ExplorerFind);
    }

    pub(super) fn mount_find_input(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        assert!(self
            .app
            .remount(
                Id::FindPopup,
                Box::new(components::FindPopup::new(input_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::FindPopup).is_ok());
    }

    pub(super) fn umount_find_input(&mut self) {
        // Umount input find
        let _ = self.app.umount(&Id::FindPopup);
    }

    pub(super) fn mount_goto(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        assert!(self
            .app
            .remount(
                Id::GotoPopup,
                Box::new(components::GoToPopup::new(input_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::GotoPopup).is_ok());
    }

    pub(super) fn umount_goto(&mut self) {
        let _ = self.app.umount(&Id::GotoPopup);
    }

    pub(super) fn mount_mkdir(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        assert!(self
            .app
            .remount(
                Id::MkdirPopup,
                Box::new(components::MkdirPopup::new(input_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::MkdirPopup).is_ok());
    }

    pub(super) fn umount_mkdir(&mut self) {
        let _ = self.app.umount(&Id::MkdirPopup);
    }

    pub(super) fn mount_newfile(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        assert!(self
            .app
            .remount(
                Id::NewfilePopup,
                Box::new(components::NewfilePopup::new(input_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::NewfilePopup).is_ok());
    }

    pub(super) fn umount_newfile(&mut self) {
        let _ = self.app.umount(&Id::NewfilePopup);
    }

    pub(super) fn mount_openwith(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        assert!(self
            .app
            .remount(
                Id::OpenWithPopup,
                Box::new(components::OpenWithPopup::new(input_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::OpenWithPopup).is_ok());
    }

    pub(super) fn umount_openwith(&mut self) {
        let _ = self.app.umount(&Id::OpenWithPopup);
    }

    pub(super) fn mount_rename(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        assert!(self
            .app
            .remount(
                Id::RenamePopup,
                Box::new(components::RenamePopup::new(input_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::RenamePopup).is_ok());
    }

    pub(super) fn umount_rename(&mut self) {
        let _ = self.app.umount(&Id::RenamePopup);
    }

    pub(super) fn mount_saveas(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        assert!(self
            .app
            .remount(
                Id::SaveAsPopup,
                Box::new(components::SaveAsPopup::new(input_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::SaveAsPopup).is_ok());
    }

    pub(super) fn umount_saveas(&mut self) {
        let _ = self.app.umount(&Id::SaveAsPopup);
    }

    pub(super) fn mount_progress_bar(&mut self, root_name: String) {
        let prog_color_full = self.theme().transfer_progress_bar_full;
        let prog_color_partial = self.theme().transfer_progress_bar_partial;
        assert!(self
            .app
            .remount(
                Id::ProgressBarFull,
                Box::new(components::ProgressBarFull::new(
                    0.0,
                    "",
                    &root_name,
                    prog_color_full
                )),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ProgressBarPartial,
                Box::new(components::ProgressBarPartial::new(
                    0.0,
                    "",
                    "Please wait",
                    prog_color_partial
                )),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::ProgressBarPartial).is_ok());
    }

    pub(super) fn umount_progress_bar(&mut self) {
        let _ = self.app.umount(&Id::ProgressBarPartial);
        let _ = self.app.umount(&Id::ProgressBarFull);
    }

    pub(super) fn mount_file_sorting(&mut self) {
        let sorting_color = self.theme().transfer_status_sorting;
        let sorting: FileSorting = match self.browser.tab() {
            FileExplorerTab::Local => self.local().get_file_sorting(),
            FileExplorerTab::Remote => self.remote().get_file_sorting(),
            _ => return,
        };
        assert!(self
            .app
            .remount(
                Id::SortingPopup,
                Box::new(components::SortingPopup::new(sorting, sorting_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::SortingPopup).is_ok());
    }

    pub(super) fn umount_file_sorting(&mut self) {
        let _ = self.app.umount(&Id::SortingPopup);
    }

    pub(super) fn mount_radio_delete(&mut self) {
        let warn_color = self.theme().misc_warn_dialog;
        assert!(self
            .app
            .remount(
                Id::DeletePopup,
                Box::new(components::DeletePopup::new(warn_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::DeletePopup).is_ok());
    }

    pub(super) fn umount_radio_delete(&mut self) {
        let _ = self.app.umount(&Id::DeletePopup);
    }

    pub(super) fn mount_radio_watch(&mut self, watch: bool, local: &str, remote: &str) {
        let info_color = self.theme().misc_info_dialog;
        assert!(self
            .app
            .remount(
                Id::WatcherPopup,
                Box::new(components::WatcherPopup::new(
                    watch, local, remote, info_color
                )),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::WatcherPopup).is_ok());
    }

    pub(super) fn umount_radio_watcher(&mut self) {
        let _ = self.app.umount(&Id::WatcherPopup);
    }

    pub(super) fn mount_watched_paths_list(&mut self, paths: &[std::path::PathBuf]) {
        let info_color = self.theme().misc_info_dialog;
        assert!(self
            .app
            .remount(
                Id::WatchedPathsList,
                Box::new(components::WatchedPathsList::new(paths, info_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::WatchedPathsList).is_ok());
    }

    pub(super) fn umount_watched_paths_list(&mut self) {
        let _ = self.app.umount(&Id::WatchedPathsList);
    }

    pub(super) fn mount_radio_replace(&mut self, file_name: &str) {
        let warn_color = self.theme().misc_warn_dialog;
        assert!(self
            .app
            .remount(
                Id::ReplacePopup,
                Box::new(components::ReplacePopup::new(Some(file_name), warn_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::ReplacePopup).is_ok());
    }

    pub(super) fn mount_radio_replace_many(&mut self, files: &[String]) {
        let warn_color = self.theme().misc_warn_dialog;
        assert!(self
            .app
            .remount(
                Id::ReplacingFilesListPopup,
                Box::new(components::ReplacingFilesListPopup::new(files, warn_color)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ReplacePopup,
                Box::new(components::ReplacePopup::new(None, warn_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::ReplacePopup).is_ok());
    }

    /// Returns whether radio replace is in "extended" mode (for many files)
    pub(super) fn is_radio_replace_extended(&self) -> bool {
        self.app.mounted(&Id::ReplacingFilesListPopup)
    }

    pub(super) fn umount_radio_replace(&mut self) {
        let _ = self.app.umount(&Id::ReplacePopup);
        let _ = self.app.umount(&Id::ReplacingFilesListPopup); // NOTE: replace anyway
    }

    pub(super) fn mount_file_info(&mut self, file: &File) {
        assert!(self
            .app
            .remount(
                Id::FileInfoPopup,
                Box::new(components::FileInfoPopup::new(file)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::FileInfoPopup).is_ok());
    }

    pub(super) fn umount_file_info(&mut self) {
        let _ = self.app.umount(&Id::FileInfoPopup);
    }

    pub(super) fn refresh_local_status_bar(&mut self) {
        let sorting_color = self.theme().transfer_status_sorting;
        let hidden_color = self.theme().transfer_status_hidden;
        assert!(self
            .app
            .remount(
                Id::StatusBarLocal,
                Box::new(components::StatusBarLocal::new(
                    &self.browser,
                    sorting_color,
                    hidden_color
                )),
                vec![],
            )
            .is_ok());
    }

    pub(super) fn refresh_remote_status_bar(&mut self) {
        let sorting_color = self.theme().transfer_status_sorting;
        let hidden_color = self.theme().transfer_status_hidden;
        let sync_color = self.theme().transfer_status_sync_browsing;
        assert!(self
            .app
            .remount(
                Id::StatusBarRemote,
                Box::new(components::StatusBarRemote::new(
                    &self.browser,
                    sorting_color,
                    hidden_color,
                    sync_color
                )),
                vec![],
            )
            .is_ok());
    }

    pub(super) fn mount_symlink(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        assert!(self
            .app
            .remount(
                Id::SymlinkPopup,
                Box::new(components::SymlinkPopup::new(input_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::SymlinkPopup).is_ok());
    }

    pub(super) fn umount_symlink(&mut self) {
        let _ = self.app.umount(&Id::SymlinkPopup);
    }

    pub(super) fn mount_sync_browsing_mkdir_popup(&mut self, dir_name: &str) {
        let color = self.theme().misc_info_dialog;
        assert!(self
            .app
            .remount(
                Id::SyncBrowsingMkdirPopup,
                Box::new(components::SyncBrowsingMkdirPopup::new(color, dir_name,)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::SyncBrowsingMkdirPopup).is_ok());
    }

    pub(super) fn umount_sync_browsing_mkdir_popup(&mut self) {
        let _ = self.app.umount(&Id::SyncBrowsingMkdirPopup);
    }

    /// Mount help
    pub(super) fn mount_help(&mut self) {
        let key_color = self.theme().misc_keys;
        assert!(self
            .app
            .remount(
                Id::KeybindingsPopup,
                Box::new(components::KeybindingsPopup::new(key_color)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::KeybindingsPopup).is_ok());
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
                    Sub::new(SubEventClause::WindowResize, SubClause::Always)
                ]
            )
            .is_ok());
    }

    /// Returns a sub clause which requires that no popup is mounted in order to be satisfied
    fn no_popup_mounted_clause() -> SubClause<Id> {
        SubClause::And(
            Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                Id::CopyPopup,
            )))),
            Box::new(SubClause::And(
                Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                    Id::DeletePopup,
                )))),
                Box::new(SubClause::And(
                    Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                        Id::DisconnectPopup,
                    )))),
                    Box::new(SubClause::And(
                        Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                            Id::ErrorPopup,
                        )))),
                        Box::new(SubClause::And(
                            Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                Id::ExecPopup,
                            )))),
                            Box::new(SubClause::And(
                                Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                    Id::FatalPopup,
                                )))),
                                Box::new(SubClause::And(
                                    Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                        Id::FileInfoPopup,
                                    )))),
                                    Box::new(SubClause::And(
                                        Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                            Id::GotoPopup,
                                        )))),
                                        Box::new(SubClause::And(
                                            Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                Id::KeybindingsPopup,
                                            )))),
                                            Box::new(SubClause::And(
                                                Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                    Id::MkdirPopup,
                                                )))),
                                                Box::new(SubClause::And(
                                                    Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                        Id::NewfilePopup,
                                                    )))),
                                                    Box::new(SubClause::And(
                                                        Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                            Id::OpenWithPopup,
                                                        )))),
                                                        Box::new(SubClause::And(
                                                            Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                                Id::ProgressBarFull,
                                                            )))),
                                                            Box::new(SubClause::And(
                                                                Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                                    Id::ProgressBarPartial,
                                                                )))),
                                                                Box::new(SubClause::And(
                                                                    Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                                        Id::ExplorerFind,
                                                                    )))),
                                                                    Box::new(SubClause::And(
                                                                        Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                                            Id::QuitPopup,
                                                                        )))),
                                                                        Box::new(SubClause::And(
                                                                            Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                                                Id::RenamePopup,
                                                                            )))),
                                                                            Box::new(SubClause::And(
                                                                                Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                                                    Id::ReplacePopup,
                                                                                )))),
                                                                                Box::new(SubClause::And(
                                                                                    Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                                                        Id::SaveAsPopup,
                                                                                    )))),
                                                                                    Box::new(SubClause::And(
                                                                                        Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                                                            Id::SortingPopup,
                                                                                        )))),
                                                                                        Box::new(SubClause::And(
                                                                                            Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                                                                Id::FindPopup,
                                                                                            )))),
                                                                                            Box::new(SubClause::And(
                                                                                                Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                                                                    Id::SyncBrowsingMkdirPopup,
                                                                                                )))),
                                                                                                Box::new(SubClause::And(
                                                                                                    Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                                                                        Id::SymlinkPopup,
                                                                                                    )))),
                                                                                                    Box::new(SubClause::And(
                                                                                                        Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                                                                            Id::WatcherPopup,
                                                                                                        )))),
                                                                                                        Box::new(SubClause::And(
                                                                                                            Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
                                                                                                                Id::WatchedPathsList,
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
                                                                                )),
                                                                            )),
                                                                        )),
                                                                    )),
                                                                )),
                                                            )),
                                                        )),
                                                    )),
                                                )),
                                            )),
                                        )),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        )
    }
}
