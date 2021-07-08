//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

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
// locals
use super::{browser::FileExplorerTab, Context, FileTransferActivity};
use crate::fs::explorer::FileSorting;
use crate::fs::FsEntry;
use crate::ui::components::{
    file_list::{FileList, FileListPropsBuilder},
    logbox::{LogBox, LogboxPropsBuilder},
    msgbox::{MsgBox, MsgBoxPropsBuilder},
};
use crate::ui::store::Store;
use crate::utils::fmt::fmt_time;
use crate::utils::ui::draw_area_in;
// Ext
use bytesize::ByteSize;
use std::path::PathBuf;
use tuirealm::components::{
    input::{Input, InputPropsBuilder},
    progress_bar::{ProgressBar, ProgressBarPropsBuilder},
    radio::{Radio, RadioPropsBuilder},
    scrolltable::{ScrollTablePropsBuilder, Scrolltable},
    span::{Span, SpanPropsBuilder},
    table::{Table, TablePropsBuilder},
};
use tuirealm::props::{PropsBuilder, TableBuilder, TextSpan, TextSpanBuilder};
use tuirealm::tui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{BorderType, Borders, Clear},
};
#[cfg(target_family = "unix")]
use users::{get_group_by_gid, get_user_by_uid};

impl FileTransferActivity {
    // -- init

    /// ### init
    ///
    /// Initialize file transfer activity's view
    pub(super) fn init(&mut self) {
        // Mount local file explorer
        let local_explorer_background = self.theme().transfer_local_explorer_background;
        let local_explorer_foreground = self.theme().transfer_local_explorer_foreground;
        let local_explorer_highlighted = self.theme().transfer_local_explorer_highlighted;
        let remote_explorer_background = self.theme().transfer_remote_explorer_background;
        let remote_explorer_foreground = self.theme().transfer_remote_explorer_foreground;
        let remote_explorer_highlighted = self.theme().transfer_remote_explorer_highlighted;
        let log_panel = self.theme().transfer_log_window;
        let log_background = self.theme().transfer_log_background;
        self.view.mount(
            super::COMPONENT_EXPLORER_LOCAL,
            Box::new(FileList::new(
                FileListPropsBuilder::default()
                    .with_highlight_color(local_explorer_highlighted)
                    .with_background(local_explorer_background)
                    .with_foreground(local_explorer_foreground)
                    .with_borders(Borders::ALL, BorderType::Plain, local_explorer_highlighted)
                    .build(),
            )),
        );
        // Mount remote file explorer
        self.view.mount(
            super::COMPONENT_EXPLORER_REMOTE,
            Box::new(FileList::new(
                FileListPropsBuilder::default()
                    .with_highlight_color(remote_explorer_highlighted)
                    .with_background(remote_explorer_background)
                    .with_foreground(remote_explorer_foreground)
                    .with_borders(Borders::ALL, BorderType::Plain, remote_explorer_highlighted)
                    .build(),
            )),
        );
        // Mount log box
        self.view.mount(
            super::COMPONENT_LOG_BOX,
            Box::new(LogBox::new(
                LogboxPropsBuilder::default()
                    .with_background(log_background)
                    .with_borders(Borders::ALL, BorderType::Plain, log_panel)
                    .build(),
            )),
        );
        // Mount status bars
        self.view.mount(
            super::COMPONENT_SPAN_STATUS_BAR_LOCAL,
            Box::new(Span::new(SpanPropsBuilder::default().build())),
        );
        self.view.mount(
            super::COMPONENT_SPAN_STATUS_BAR_REMOTE,
            Box::new(Span::new(SpanPropsBuilder::default().build())),
        );
        // Load process bar
        self.refresh_local_status_bar();
        self.refresh_remote_status_bar();
        // Update components
        let _ = self.update_local_filelist();
        let _ = self.update_remote_filelist();
        // Give focus to local explorer
        self.view.active(super::COMPONENT_EXPLORER_LOCAL);
    }

    // -- view

    /// ### view
    ///
    /// View gui
    pub(super) fn view(&mut self) {
        let mut context: Context = self.context.take().unwrap();
        let store: &mut Store = &mut context.store;
        let _ = context.terminal.draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(70), // Explorer
                        Constraint::Percentage(30), // Log
                    ]
                    .as_ref(),
                )
                .split(f.size());
            // Create explorer chunks
            let tabs_chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .split(chunks[0]);
            // Create log box chunks
            let bottom_chunks = Layout::default()
                .constraints([Constraint::Length(1), Constraint::Length(10)].as_ref())
                .direction(Direction::Vertical)
                .split(chunks[1]);
            // Create status bar chunks
            let status_bar_chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .horizontal_margin(1)
                .split(bottom_chunks[0]);
            // If width is unset in the storage, set width
            if !store.isset(super::STORAGE_EXPLORER_WIDTH) {
                store.set_unsigned(super::STORAGE_EXPLORER_WIDTH, tabs_chunks[0].width as usize);
            }
            // Draw explorers
            // @! Local explorer (Find or default)
            match self.browser.tab() {
                FileExplorerTab::FindLocal => {
                    self.view
                        .render(super::COMPONENT_EXPLORER_FIND, f, tabs_chunks[0])
                }
                _ => self
                    .view
                    .render(super::COMPONENT_EXPLORER_LOCAL, f, tabs_chunks[0]),
            }
            // @! Remote explorer (Find or default)
            match self.browser.tab() {
                FileExplorerTab::FindRemote => {
                    self.view
                        .render(super::COMPONENT_EXPLORER_FIND, f, tabs_chunks[1])
                }
                _ => self
                    .view
                    .render(super::COMPONENT_EXPLORER_REMOTE, f, tabs_chunks[1]),
            }
            // Draw log box
            self.view
                .render(super::COMPONENT_LOG_BOX, f, bottom_chunks[1]);
            // Draw status bar
            self.view.render(
                super::COMPONENT_SPAN_STATUS_BAR_LOCAL,
                f,
                status_bar_chunks[0],
            );
            self.view.render(
                super::COMPONENT_SPAN_STATUS_BAR_REMOTE,
                f,
                status_bar_chunks[1],
            );
            // @! Draw popups
            if let Some(props) = self.view.get_props(super::COMPONENT_INPUT_COPY) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 40, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_INPUT_COPY, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_INPUT_FIND) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 40, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_INPUT_FIND, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_INPUT_GOTO) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 40, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_INPUT_GOTO, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_INPUT_MKDIR) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 40, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_INPUT_MKDIR, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_INPUT_NEWFILE) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 40, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_INPUT_NEWFILE, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_INPUT_OPEN_WITH) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 40, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_INPUT_OPEN_WITH, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_INPUT_RENAME) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 40, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_INPUT_RENAME, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_INPUT_SAVEAS) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 40, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_INPUT_SAVEAS, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_INPUT_EXEC) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 40, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_INPUT_EXEC, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_LIST_FILEINFO) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 50, 50);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_LIST_FILEINFO, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_PROGRESS_BAR_PARTIAL) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 50, 20);
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
                    self.view
                        .render(super::COMPONENT_PROGRESS_BAR_FULL, f, popup_chunks[0]);
                    self.view
                        .render(super::COMPONENT_PROGRESS_BAR_PARTIAL, f, popup_chunks[1]);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_RADIO_DELETE) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 30, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_RADIO_DELETE, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_RADIO_DISCONNECT) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 30, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view
                        .render(super::COMPONENT_RADIO_DISCONNECT, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_RADIO_QUIT) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 30, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_RADIO_QUIT, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_RADIO_SORTING) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 50, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_RADIO_SORTING, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_TEXT_ERROR) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 50, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_TEXT_ERROR, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_TEXT_FATAL) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 50, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_TEXT_FATAL, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_TEXT_WAIT) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 50, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_TEXT_WAIT, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_TEXT_HELP) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 50, 80);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_TEXT_HELP, f, popup);
                }
            }
        });
        // Re-give context
        self.context = Some(context);
    }

    // -- partials

    /// ### mount_error
    ///
    /// Mount error box
    pub(super) fn mount_error(&mut self, text: &str) {
        // Mount
        let error_color = self.theme().misc_error_dialog;
        self.view.mount(
            super::COMPONENT_TEXT_ERROR,
            Box::new(MsgBox::new(
                MsgBoxPropsBuilder::default()
                    .with_foreground(error_color)
                    .with_borders(Borders::ALL, BorderType::Rounded, error_color)
                    .bold()
                    .with_texts(None, vec![TextSpan::from(text)])
                    .build(),
            )),
        );
        // Give focus to error
        self.view.active(super::COMPONENT_TEXT_ERROR);
    }

    /// ### umount_error
    ///
    /// Umount error message
    pub(super) fn umount_error(&mut self) {
        self.view.umount(super::COMPONENT_TEXT_ERROR);
    }

    pub(super) fn mount_fatal(&mut self, text: &str) {
        // Mount
        let error_color = self.theme().misc_error_dialog;
        self.view.mount(
            super::COMPONENT_TEXT_FATAL,
            Box::new(MsgBox::new(
                MsgBoxPropsBuilder::default()
                    .with_foreground(error_color)
                    .with_borders(Borders::ALL, BorderType::Rounded, error_color)
                    .bold()
                    .with_texts(None, vec![TextSpan::from(text)])
                    .build(),
            )),
        );
        // Give focus to error
        self.view.active(super::COMPONENT_TEXT_FATAL);
    }

    pub(super) fn mount_wait(&mut self, text: &str) {
        // Mount
        self.view.mount(
            super::COMPONENT_TEXT_WAIT,
            Box::new(MsgBox::new(
                MsgBoxPropsBuilder::default()
                    .with_foreground(Color::White)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::White)
                    .bold()
                    .with_texts(None, vec![TextSpan::from(text)])
                    .build(),
            )),
        );
        // Give focus to info
        self.view.active(super::COMPONENT_TEXT_WAIT);
    }

    pub(super) fn umount_wait(&mut self) {
        self.view.umount(super::COMPONENT_TEXT_WAIT);
    }

    /// ### mount_quit
    ///
    /// Mount quit popup
    pub(super) fn mount_quit(&mut self) {
        // Protocol
        let quit_color = self.theme().misc_quit_dialog;
        self.view.mount(
            super::COMPONENT_RADIO_QUIT,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(quit_color)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, quit_color)
                    .with_options(
                        Some(String::from("Are you sure you want to quit?")),
                        vec![String::from("Yes"), String::from("No")],
                    )
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_RADIO_QUIT);
    }

    /// ### umount_quit
    ///
    /// Umount quit popup
    pub(super) fn umount_quit(&mut self) {
        self.view.umount(super::COMPONENT_RADIO_QUIT);
    }

    /// ### mount_disconnect
    ///
    /// Mount disconnect popup
    pub(super) fn mount_disconnect(&mut self) {
        // Protocol
        let quit_color = self.theme().misc_quit_dialog;
        self.view.mount(
            super::COMPONENT_RADIO_DISCONNECT,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(quit_color)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, quit_color)
                    .with_options(
                        Some(String::from("Are you sure you want to disconnect?")),
                        vec![String::from("Yes"), String::from("No")],
                    )
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_RADIO_DISCONNECT);
    }

    /// ### umount_disconnect
    ///
    /// Umount disconnect popup
    pub(super) fn umount_disconnect(&mut self) {
        self.view.umount(super::COMPONENT_RADIO_DISCONNECT);
    }

    pub(super) fn mount_copy(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        self.view.mount(
            super::COMPONENT_INPUT_COPY,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, input_color)
                    .with_foreground(input_color)
                    .with_label(String::from("Copy file(s) to..."))
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_INPUT_COPY);
    }

    pub(super) fn umount_copy(&mut self) {
        self.view.umount(super::COMPONENT_INPUT_COPY);
    }

    pub(super) fn mount_exec(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        self.view.mount(
            super::COMPONENT_INPUT_EXEC,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, input_color)
                    .with_foreground(input_color)
                    .with_label(String::from("Execute command"))
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_INPUT_EXEC);
    }

    pub(super) fn umount_exec(&mut self) {
        self.view.umount(super::COMPONENT_INPUT_EXEC);
    }

    pub(super) fn mount_find(&mut self, search: &str) {
        // Get color
        let (bg, fg, hg): (Color, Color, Color) = match self.browser.tab() {
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
        self.view.mount(
            super::COMPONENT_EXPLORER_FIND,
            Box::new(FileList::new(
                FileListPropsBuilder::default()
                    .with_files(Some(format!("Search results for \"{}\"", search)), vec![])
                    .with_borders(Borders::ALL, BorderType::Plain, hg)
                    .with_highlight_color(hg)
                    .with_background(bg)
                    .with_foreground(fg)
                    .build(),
            )),
        );
        // Give focus to explorer findd
        self.view.active(super::COMPONENT_EXPLORER_FIND);
    }

    pub(super) fn umount_find(&mut self) {
        self.view.umount(super::COMPONENT_EXPLORER_FIND);
    }

    pub(super) fn mount_find_input(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        self.view.mount(
            super::COMPONENT_INPUT_FIND,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, input_color)
                    .with_foreground(input_color)
                    .with_label(String::from("Search files by name"))
                    .build(),
            )),
        );
        // Give focus to input find
        self.view.active(super::COMPONENT_INPUT_FIND);
    }

    pub(super) fn umount_find_input(&mut self) {
        // Umount input find
        self.view.umount(super::COMPONENT_INPUT_FIND);
    }

    pub(super) fn mount_goto(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        self.view.mount(
            super::COMPONENT_INPUT_GOTO,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, input_color)
                    .with_foreground(input_color)
                    .with_label(String::from("Change working directory"))
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_INPUT_GOTO);
    }

    pub(super) fn umount_goto(&mut self) {
        self.view.umount(super::COMPONENT_INPUT_GOTO);
    }

    pub(super) fn mount_mkdir(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        self.view.mount(
            super::COMPONENT_INPUT_MKDIR,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, input_color)
                    .with_foreground(input_color)
                    .with_label(String::from("Insert directory name"))
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_INPUT_MKDIR);
    }

    pub(super) fn umount_mkdir(&mut self) {
        self.view.umount(super::COMPONENT_INPUT_MKDIR);
    }

    pub(super) fn mount_newfile(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        self.view.mount(
            super::COMPONENT_INPUT_NEWFILE,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, input_color)
                    .with_foreground(input_color)
                    .with_label(String::from("New file name"))
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_INPUT_NEWFILE);
    }

    pub(super) fn umount_newfile(&mut self) {
        self.view.umount(super::COMPONENT_INPUT_NEWFILE);
    }

    pub(super) fn mount_openwith(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        self.view.mount(
            super::COMPONENT_INPUT_OPEN_WITH,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, input_color)
                    .with_foreground(input_color)
                    .with_label(String::from("Open file with..."))
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_INPUT_OPEN_WITH);
    }

    pub(super) fn umount_openwith(&mut self) {
        self.view.umount(super::COMPONENT_INPUT_OPEN_WITH);
    }

    pub(super) fn mount_rename(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        self.view.mount(
            super::COMPONENT_INPUT_RENAME,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, input_color)
                    .with_foreground(input_color)
                    .with_label(String::from("Move file(s) to..."))
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_INPUT_RENAME);
    }

    pub(super) fn umount_rename(&mut self) {
        self.view.umount(super::COMPONENT_INPUT_RENAME);
    }

    pub(super) fn mount_saveas(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        self.view.mount(
            super::COMPONENT_INPUT_SAVEAS,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, input_color)
                    .with_foreground(input_color)
                    .with_label(String::from("Save as..."))
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_INPUT_SAVEAS);
    }

    pub(super) fn umount_saveas(&mut self) {
        self.view.umount(super::COMPONENT_INPUT_SAVEAS);
    }

    pub(super) fn mount_progress_bar(&mut self, root_name: String) {
        let prog_color_full = self.theme().transfer_progress_bar_full;
        let prog_color_partial = self.theme().transfer_progress_bar_partial;
        self.view.mount(
            super::COMPONENT_PROGRESS_BAR_FULL,
            Box::new(ProgressBar::new(
                ProgressBarPropsBuilder::default()
                    .with_progbar_color(prog_color_full)
                    .with_background(Color::Black)
                    .with_borders(
                        Borders::TOP | Borders::RIGHT | Borders::LEFT,
                        BorderType::Rounded,
                        Color::Reset,
                    )
                    .with_texts(Some(root_name), String::new())
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_PROGRESS_BAR_PARTIAL,
            Box::new(ProgressBar::new(
                ProgressBarPropsBuilder::default()
                    .with_progbar_color(prog_color_partial)
                    .with_background(Color::Black)
                    .with_borders(
                        Borders::BOTTOM | Borders::RIGHT | Borders::LEFT,
                        BorderType::Rounded,
                        Color::Reset,
                    )
                    .with_texts(Some(String::from("Please wait")), String::new())
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_PROGRESS_BAR_PARTIAL);
    }

    pub(super) fn umount_progress_bar(&mut self) {
        self.view.umount(super::COMPONENT_PROGRESS_BAR_PARTIAL);
        self.view.umount(super::COMPONENT_PROGRESS_BAR_FULL);
    }

    pub(super) fn mount_file_sorting(&mut self) {
        let sorting_color = self.theme().transfer_status_sorting;
        let sorting: FileSorting = match self.browser.tab() {
            FileExplorerTab::Local => self.local().get_file_sorting(),
            FileExplorerTab::Remote => self.remote().get_file_sorting(),
            _ => panic!("You can't mount file sorting when in found result"),
        };
        let index: usize = match sorting {
            FileSorting::ByCreationTime => 2,
            FileSorting::ByModifyTime => 1,
            FileSorting::ByName => 0,
            FileSorting::BySize => 3,
        };
        self.view.mount(
            super::COMPONENT_RADIO_SORTING,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(sorting_color)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, sorting_color)
                    .with_options(
                        Some(String::from("Sort files by")),
                        vec![
                            String::from("Name"),
                            String::from("Modify time"),
                            String::from("Creation time"),
                            String::from("Size"),
                        ],
                    )
                    .with_value(index)
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_RADIO_SORTING);
    }

    pub(super) fn umount_file_sorting(&mut self) {
        self.view.umount(super::COMPONENT_RADIO_SORTING);
    }

    pub(super) fn mount_radio_delete(&mut self) {
        let warn_color = self.theme().misc_warn_dialog;
        self.view.mount(
            super::COMPONENT_RADIO_DELETE,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(warn_color)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Plain, warn_color)
                    .with_options(
                        Some(String::from("Delete file")),
                        vec![String::from("Yes"), String::from("No")],
                    )
                    .with_value(1)
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_RADIO_DELETE);
    }

    pub(super) fn umount_radio_delete(&mut self) {
        self.view.umount(super::COMPONENT_RADIO_DELETE);
    }

    pub(super) fn mount_file_info(&mut self, file: &FsEntry) {
        let mut texts: TableBuilder = TableBuilder::default();
        // Abs path
        let real_path: Option<PathBuf> = {
            let real_file: FsEntry = file.get_realfile();
            match real_file.get_abs_path() != file.get_abs_path() {
                true => Some(real_file.get_abs_path()),
                false => None,
            }
        };
        let path: String = match real_path {
            Some(symlink) => format!("{} -> {}", file.get_abs_path().display(), symlink.display()),
            None => format!("{}", file.get_abs_path().display()),
        };
        // Make texts
        texts.add_col(TextSpan::from("Path: ")).add_col(
            TextSpanBuilder::new(path.as_str())
                .with_foreground(Color::Yellow)
                .build(),
        );
        if let Some(filetype) = file.get_ftype() {
            texts
                .add_row()
                .add_col(TextSpan::from("File type: "))
                .add_col(
                    TextSpanBuilder::new(filetype.as_str())
                        .with_foreground(Color::LightGreen)
                        .build(),
                );
        }
        let (bsize, size): (ByteSize, usize) = (ByteSize(file.get_size() as u64), file.get_size());
        texts.add_row().add_col(TextSpan::from("Size: ")).add_col(
            TextSpanBuilder::new(format!("{} ({})", bsize, size).as_str())
                .with_foreground(Color::Cyan)
                .build(),
        );
        let ctime: String = fmt_time(file.get_creation_time(), "%b %d %Y %H:%M:%S");
        let atime: String = fmt_time(file.get_last_access_time(), "%b %d %Y %H:%M:%S");
        let mtime: String = fmt_time(file.get_creation_time(), "%b %d %Y %H:%M:%S");
        texts
            .add_row()
            .add_col(TextSpan::from("Creation time: "))
            .add_col(
                TextSpanBuilder::new(ctime.as_str())
                    .with_foreground(Color::LightGreen)
                    .build(),
            );
        texts
            .add_row()
            .add_col(TextSpan::from("Last modified time: "))
            .add_col(
                TextSpanBuilder::new(mtime.as_str())
                    .with_foreground(Color::LightBlue)
                    .build(),
            );
        texts
            .add_row()
            .add_col(TextSpan::from("Last access time: "))
            .add_col(
                TextSpanBuilder::new(atime.as_str())
                    .with_foreground(Color::LightRed)
                    .build(),
            );
        // User
        #[cfg(target_family = "unix")]
        let username: String = match file.get_user() {
            Some(uid) => match get_user_by_uid(uid) {
                Some(user) => user.name().to_string_lossy().to_string(),
                None => uid.to_string(),
            },
            None => String::from("0"),
        };
        #[cfg(target_os = "windows")]
        let username: String = format!("{}", file.get_user().unwrap_or(0));
        // Group
        #[cfg(target_family = "unix")]
        let group: String = match file.get_group() {
            Some(gid) => match get_group_by_gid(gid) {
                Some(group) => group.name().to_string_lossy().to_string(),
                None => gid.to_string(),
            },
            None => String::from("0"),
        };
        #[cfg(target_os = "windows")]
        let group: String = format!("{}", file.get_group().unwrap_or(0));
        texts.add_row().add_col(TextSpan::from("User: ")).add_col(
            TextSpanBuilder::new(username.as_str())
                .with_foreground(Color::LightYellow)
                .build(),
        );
        texts.add_row().add_col(TextSpan::from("Group: ")).add_col(
            TextSpanBuilder::new(group.as_str())
                .with_foreground(Color::Blue)
                .build(),
        );
        self.view.mount(
            super::COMPONENT_LIST_FILEINFO,
            Box::new(Table::new(
                TablePropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::White)
                    .with_table(Some(file.get_name().to_string()), texts.build())
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_LIST_FILEINFO);
    }

    pub(super) fn umount_file_info(&mut self) {
        self.view.umount(super::COMPONENT_LIST_FILEINFO);
    }

    pub(super) fn refresh_local_status_bar(&mut self) {
        let sorting_color = self.theme().transfer_status_sorting;
        let hidden_color = self.theme().transfer_status_hidden;
        let local_bar_spans: Vec<TextSpan> = vec![
            TextSpanBuilder::new("File sorting: ")
                .with_foreground(sorting_color)
                .build(),
            TextSpanBuilder::new(Self::get_file_sorting_str(self.local().get_file_sorting()))
                .with_foreground(sorting_color)
                .reversed()
                .build(),
            TextSpanBuilder::new(" Hidden files: ")
                .with_foreground(hidden_color)
                .build(),
            TextSpanBuilder::new(Self::get_hidden_files_str(
                self.local().hidden_files_visible(),
            ))
            .with_foreground(hidden_color)
            .reversed()
            .build(),
        ];
        if let Some(props) = self.view.get_props(super::COMPONENT_SPAN_STATUS_BAR_LOCAL) {
            self.view.update(
                super::COMPONENT_SPAN_STATUS_BAR_LOCAL,
                SpanPropsBuilder::from(props)
                    .with_spans(local_bar_spans)
                    .build(),
            );
        }
    }

    pub(super) fn refresh_remote_status_bar(&mut self) {
        let sorting_color = self.theme().transfer_status_sorting;
        let hidden_color = self.theme().transfer_status_hidden;
        let sync_color = self.theme().transfer_status_sync_browsing;
        let remote_bar_spans: Vec<TextSpan> = vec![
            TextSpanBuilder::new("File sorting: ")
                .with_foreground(sorting_color)
                .build(),
            TextSpanBuilder::new(Self::get_file_sorting_str(self.remote().get_file_sorting()))
                .with_foreground(sorting_color)
                .reversed()
                .build(),
            TextSpanBuilder::new(" Hidden files: ")
                .with_foreground(hidden_color)
                .build(),
            TextSpanBuilder::new(Self::get_hidden_files_str(
                self.remote().hidden_files_visible(),
            ))
            .with_foreground(hidden_color)
            .reversed()
            .build(),
            TextSpanBuilder::new(" Sync Browsing: ")
                .with_foreground(sync_color)
                .build(),
            TextSpanBuilder::new(match self.browser.sync_browsing {
                true => "ON ",
                false => "OFF",
            })
            .with_foreground(sync_color)
            .reversed()
            .build(),
        ];
        if let Some(props) = self.view.get_props(super::COMPONENT_SPAN_STATUS_BAR_REMOTE) {
            self.view.update(
                super::COMPONENT_SPAN_STATUS_BAR_REMOTE,
                SpanPropsBuilder::from(props)
                    .with_spans(remote_bar_spans)
                    .build(),
            );
        }
    }

    /// ### mount_help
    ///
    /// Mount help
    pub(super) fn mount_help(&mut self) {
        let key_color = self.theme().misc_keys;
        self.view.mount(
            super::COMPONENT_TEXT_HELP,
            Box::new(Scrolltable::new(
                ScrollTablePropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::White)
                    .with_highlighted_str(Some("?"))
                    .with_max_scroll_step(8)
                    .bold()
                    .with_table(
                        Some(String::from("Help")),
                        TableBuilder::default()
                            .add_col(
                                TextSpanBuilder::new("<ESC>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("           Disconnect"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<TAB>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from(
                                "           Switch between explorer and logs",
                            ))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<BACKSPACE>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("     Go to previous directory"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<RIGHT/LEFT>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("    Change explorer tab"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<UP/DOWN>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("       Move up/down in list"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<ENTER>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("         Enter directory"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<SPACE>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("         Upload/Download file"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<A>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Toggle hidden files"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<B>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Change file sorting mode"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<C>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Copy"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<D>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Make directory"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<G>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Go to path"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<H>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Show help"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<I>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Show info about selected file"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<L>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Reload directory content"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<M>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Select file"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<N>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Create new file"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<O>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from(
                                "             Open text file with preferred editor",
                            ))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<Q>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Quit termscp"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<R>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Rename file"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<S>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Save file as"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<U>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Go to parent directory"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<V>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from(
                                "             Open file with default application for file type",
                            ))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<W>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from(
                                "             Open file with specified application",
                            ))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<X>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Execute shell command"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<Y>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("             Toggle synchronized browsing"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<DEL|E>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("         Delete selected file"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<CTRL+A>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("        Select all files"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<CTRL+C>")
                                    .bold()
                                    .with_foreground(key_color)
                                    .build(),
                            )
                            .add_col(TextSpan::from("        Interrupt file transfer"))
                            .build(),
                    )
                    .build(),
            )),
        );
        // Active help
        self.view.active(super::COMPONENT_TEXT_HELP);
    }

    pub(super) fn umount_help(&mut self) {
        self.view.umount(super::COMPONENT_TEXT_HELP);
    }

    fn get_file_sorting_str(mode: FileSorting) -> &'static str {
        match mode {
            FileSorting::ByName => "By name",
            FileSorting::ByCreationTime => "By creation time",
            FileSorting::ByModifyTime => "By modify time",
            FileSorting::BySize => "By size",
        }
    }

    fn get_hidden_files_str(show: bool) -> &'static str {
        match show {
            true => "Show",
            false => "Hide",
        }
    }
}
