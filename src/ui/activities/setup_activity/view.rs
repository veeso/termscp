//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

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
// Locals
use super::{Context, SetupActivity, ViewLayout};
use crate::filetransfer::FileTransferProtocol;
use crate::fs::explorer::GroupDirs;
use crate::ui::components::{
    bookmark_list::{BookmarkList, BookmarkListPropsBuilder},
    msgbox::{MsgBox, MsgBoxPropsBuilder},
};
use crate::utils::ui::draw_area_in;
// Ext
use std::path::PathBuf;
use tuirealm::components::{
    input::{Input, InputPropsBuilder},
    radio::{Radio, RadioPropsBuilder},
    span::{Span, SpanPropsBuilder},
    table::{Table, TablePropsBuilder},
};
use tuirealm::tui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{BorderType, Borders, Clear},
};
use tuirealm::{
    props::{PropsBuilder, TableBuilder, TextSpan, TextSpanBuilder},
    Payload, View,
};

impl SetupActivity {
    // -- view

    /// ### init_setup
    ///
    /// Initialize setup view
    pub(super) fn init_setup(&mut self) {
        // Init view
        self.view = View::init();
        // Common stuff
        // Radio tab
        self.view.mount(
            super::COMPONENT_RADIO_TAB,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::LightYellow)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::BOTTOM, BorderType::Thick, Color::White)
                    .with_options(
                        None,
                        vec![String::from("User Interface"), String::from("SSH Keys")],
                    )
                    .with_value(0)
                    .build(),
            )),
        );
        // Footer
        self.view.mount(
            super::COMPONENT_TEXT_FOOTER,
            Box::new(Span::new(
                SpanPropsBuilder::default()
                    .with_spans(vec![
                        TextSpanBuilder::new("Press ").bold().build(),
                        TextSpanBuilder::new("<CTRL+H>")
                            .bold()
                            .with_foreground(Color::Cyan)
                            .build(),
                        TextSpanBuilder::new(" to show keybindings").bold().build(),
                    ])
                    .build(),
            )),
        );
        // Input fields
        self.view.mount(
            super::COMPONENT_INPUT_TEXT_EDITOR,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_foreground(Color::LightGreen)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightGreen)
                    .with_label(String::from("Text editor"))
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_INPUT_TEXT_EDITOR); // <-- Focus
        self.view.mount(
            super::COMPONENT_RADIO_DEFAULT_PROTOCOL,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::LightCyan)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightCyan)
                    .with_options(
                        Some(String::from("Default file transfer protocol")),
                        vec![
                            String::from("SFTP"),
                            String::from("SCP"),
                            String::from("FTP"),
                            String::from("FTPS"),
                        ],
                    )
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_RADIO_HIDDEN_FILES,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::LightRed)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightRed)
                    .with_options(
                        Some(String::from("Show hidden files (by default)")),
                        vec![String::from("Yes"), String::from("No")],
                    )
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_RADIO_UPDATES,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::LightYellow)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightYellow)
                    .with_options(
                        Some(String::from("Check for updates?")),
                        vec![String::from("Yes"), String::from("No")],
                    )
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_RADIO_GROUP_DIRS,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::LightMagenta)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightMagenta)
                    .with_options(
                        Some(String::from("Group directories")),
                        vec![
                            String::from("Display first"),
                            String::from("Display Last"),
                            String::from("No"),
                        ],
                    )
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_INPUT_LOCAL_FILE_FMT,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_foreground(Color::LightBlue)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightBlue)
                    .with_label(String::from("File formatter syntax (local)"))
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_INPUT_REMOTE_FILE_FMT,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_foreground(Color::LightGreen)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightGreen)
                    .with_label(String::from("File formatter syntax (remote)"))
                    .build(),
            )),
        );
        // Load values
        self.load_input_values();
        // Set view
        self.layout = ViewLayout::SetupForm;
    }

    /// ### init_ssh_keys
    ///
    /// Initialize ssh keys view
    pub(super) fn init_ssh_keys(&mut self) {
        // Init view
        self.view = View::init();
        // Common stuff
        // Radio tab
        self.view.mount(
            super::COMPONENT_RADIO_TAB,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::LightYellow)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::BOTTOM, BorderType::Thick, Color::LightYellow)
                    .with_options(
                        None,
                        vec![String::from("User Interface"), String::from("SSH Keys")],
                    )
                    .with_value(1)
                    .build(),
            )),
        );
        // Footer
        self.view.mount(
            super::COMPONENT_TEXT_FOOTER,
            Box::new(Span::new(
                SpanPropsBuilder::default()
                    .with_spans(vec![
                        TextSpanBuilder::new("Press ").bold().build(),
                        TextSpanBuilder::new("<CTRL+H>")
                            .bold()
                            .with_foreground(Color::Cyan)
                            .build(),
                        TextSpanBuilder::new(" to show keybindings").bold().build(),
                    ])
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_LIST_SSH_KEYS,
            Box::new(BookmarkList::new(
                BookmarkListPropsBuilder::default()
                    .with_bookmarks(Some(String::from("SSH Keys")), vec![])
                    .with_borders(Borders::ALL, BorderType::Plain, Color::LightGreen)
                    .with_background(Color::LightGreen)
                    .with_foreground(Color::Black)
                    .build(),
            )),
        );
        // Give focus
        self.view.active(super::COMPONENT_LIST_SSH_KEYS);
        // Load keys
        self.reload_ssh_keys();
        // Set view
        self.layout = ViewLayout::SshKeys;
    }

    /// ### view
    ///
    /// View gui
    pub(super) fn view(&mut self) {
        let mut ctx: Context = self.context.take().unwrap();
        let _ = ctx.terminal.draw(|f| {
            // Prepare main chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),      // Current tab
                        Constraint::Percentage(90), // Main body
                        Constraint::Length(3),      // Help footer
                    ]
                    .as_ref(),
                )
                .split(f.size());
            // Render common widget
            self.view.render(super::COMPONENT_RADIO_TAB, f, chunks[0]);
            self.view.render(super::COMPONENT_TEXT_FOOTER, f, chunks[2]);
            match self.layout {
                ViewLayout::SetupForm => {
                    // Make chunks
                    let ui_cfg_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Length(3), // Text editor
                                Constraint::Length(3), // Protocol tab
                                Constraint::Length(3), // Hidden files
                                Constraint::Length(3), // Updates tab
                                Constraint::Length(3), // Group dirs
                                Constraint::Length(3), // Local Format input
                                Constraint::Length(3), // Remote Format input
                                Constraint::Length(1), // Empty ?
                            ]
                            .as_ref(),
                        )
                        .split(chunks[1]);
                    self.view
                        .render(super::COMPONENT_INPUT_TEXT_EDITOR, f, ui_cfg_chunks[0]);
                    self.view
                        .render(super::COMPONENT_RADIO_DEFAULT_PROTOCOL, f, ui_cfg_chunks[1]);
                    self.view
                        .render(super::COMPONENT_RADIO_HIDDEN_FILES, f, ui_cfg_chunks[2]);
                    self.view
                        .render(super::COMPONENT_RADIO_UPDATES, f, ui_cfg_chunks[3]);
                    self.view
                        .render(super::COMPONENT_RADIO_GROUP_DIRS, f, ui_cfg_chunks[4]);
                    self.view
                        .render(super::COMPONENT_INPUT_LOCAL_FILE_FMT, f, ui_cfg_chunks[5]);
                    self.view
                        .render(super::COMPONENT_INPUT_REMOTE_FILE_FMT, f, ui_cfg_chunks[6]);
                }
                ViewLayout::SshKeys => {
                    let sshcfg_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Percentage(100)].as_ref())
                        .split(chunks[1]);
                    self.view
                        .render(super::COMPONENT_LIST_SSH_KEYS, f, sshcfg_chunks[0]);
                }
            }
            // Popups
            if let Some(props) = self.view.get_props(super::COMPONENT_TEXT_ERROR) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 50, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_TEXT_ERROR, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_RADIO_QUIT) {
                if props.visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 40, 10);
                    f.render_widget(Clear, popup);
                    self.view.render(super::COMPONENT_RADIO_QUIT, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_TEXT_HELP) {
                if props.visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 50, 70);
                    f.render_widget(Clear, popup);
                    self.view.render(super::COMPONENT_TEXT_HELP, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_RADIO_SAVE) {
                if props.visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 30, 10);
                    f.render_widget(Clear, popup);
                    self.view.render(super::COMPONENT_RADIO_SAVE, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_RADIO_DEL_SSH_KEY) {
                if props.visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 30, 10);
                    f.render_widget(Clear, popup);
                    self.view
                        .render(super::COMPONENT_RADIO_DEL_SSH_KEY, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_INPUT_SSH_HOST) {
                if props.visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 50, 20);
                    f.render_widget(Clear, popup);
                    let popup_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Length(3), // Host
                                Constraint::Length(3), // Username
                            ]
                            .as_ref(),
                        )
                        .split(popup);
                    self.view
                        .render(super::COMPONENT_INPUT_SSH_HOST, f, popup_chunks[0]);
                    self.view
                        .render(super::COMPONENT_INPUT_SSH_USERNAME, f, popup_chunks[1]);
                }
            }
        });
        // Put context back to context
        self.context = Some(ctx);
    }

    // -- mount

    /// ### mount_error
    ///
    /// Mount error box
    pub(super) fn mount_error(&mut self, text: &str) {
        // Mount
        self.view.mount(
            super::COMPONENT_TEXT_ERROR,
            Box::new(MsgBox::new(
                MsgBoxPropsBuilder::default()
                    .with_foreground(Color::Red)
                    .bold()
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::Red)
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

    /// ### mount_del_ssh_key
    ///
    /// Mount delete ssh key component
    pub(super) fn mount_del_ssh_key(&mut self) {
        self.view.mount(
            super::COMPONENT_RADIO_DEL_SSH_KEY,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::LightRed)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightRed)
                    .with_options(
                        Some(String::from("Delete key?")),
                        vec![String::from("Yes"), String::from("No")],
                    )
                    .with_value(1) // Default: No
                    .build(),
            )),
        );
        // Active
        self.view.active(super::COMPONENT_RADIO_DEL_SSH_KEY);
    }

    /// ### umount_del_ssh_key
    ///
    /// Umount delete ssh key
    pub(super) fn umount_del_ssh_key(&mut self) {
        self.view.umount(super::COMPONENT_RADIO_DEL_SSH_KEY);
    }

    /// ### mount_new_ssh_key
    ///
    /// Mount new ssh key prompt
    pub(super) fn mount_new_ssh_key(&mut self) {
        self.view.mount(
            super::COMPONENT_INPUT_SSH_HOST,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_label(String::from("Hostname or address"))
                    .with_borders(
                        Borders::TOP | Borders::RIGHT | Borders::LEFT,
                        BorderType::Plain,
                        Color::Reset,
                    )
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_INPUT_SSH_USERNAME,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_label(String::from("Username"))
                    .with_borders(
                        Borders::ALL | Borders::RIGHT | Borders::LEFT,
                        BorderType::Plain,
                        Color::Reset,
                    )
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_INPUT_SSH_HOST);
    }

    /// ### umount_new_ssh_key
    ///
    /// Umount new ssh key prompt
    pub(super) fn umount_new_ssh_key(&mut self) {
        self.view.umount(super::COMPONENT_INPUT_SSH_HOST);
        self.view.umount(super::COMPONENT_INPUT_SSH_USERNAME);
    }

    /// ### mount_quit
    ///
    /// Mount quit popup
    pub(super) fn mount_quit(&mut self) {
        self.view.mount(
            super::COMPONENT_RADIO_QUIT,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::LightRed)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightRed)
                    .with_options(
                        Some(String::from("Exit setup?")),
                        vec![
                            String::from("Save"),
                            String::from("Don't save"),
                            String::from("Cancel"),
                        ],
                    )
                    .build(),
            )),
        );
        // Active
        self.view.active(super::COMPONENT_RADIO_QUIT);
    }

    /// ### umount_quit
    ///
    /// Umount quit
    pub(super) fn umount_quit(&mut self) {
        self.view.umount(super::COMPONENT_RADIO_QUIT);
    }

    /// ### mount_save_popup
    ///
    /// Mount save popup
    pub(super) fn mount_save_popup(&mut self) {
        self.view.mount(
            super::COMPONENT_RADIO_SAVE,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::LightYellow)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightYellow)
                    .with_options(
                        Some(String::from("Save changes?")),
                        vec![String::from("Yes"), String::from("No")],
                    )
                    .build(),
            )),
        );
        // Active
        self.view.active(super::COMPONENT_RADIO_SAVE);
    }

    /// ### umount_quit
    ///
    /// Umount quit
    pub(super) fn umount_save_popup(&mut self) {
        self.view.umount(super::COMPONENT_RADIO_SAVE);
    }

    /// ### mount_help
    ///
    /// Mount help
    pub(super) fn mount_help(&mut self) {
        self.view.mount(
            super::COMPONENT_TEXT_HELP,
            Box::new(Table::new(
                TablePropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::White)
                    .with_table(
                        Some(String::from("Help")),
                        TableBuilder::default()
                            .add_col(
                                TextSpanBuilder::new("<ESC>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("           Exit setup"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<TAB>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("           Change setup page"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<RIGHT/LEFT>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("    Change cursor"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<UP/DOWN>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("       Change input field"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<ENTER>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("         Select / Dismiss popup"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<DEL|E>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("         Delete SSH key"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<CTRL+N>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("        New SSH key"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<CTRL+R>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("        Revert changes"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<CTRL+S>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("        Save configuration"))
                            .build(),
                    )
                    .build(),
            )),
        );
        // Active help
        self.view.active(super::COMPONENT_TEXT_HELP);
    }

    /// ### umount_help
    ///
    /// Umount help
    pub(super) fn umount_help(&mut self) {
        self.view.umount(super::COMPONENT_TEXT_HELP);
    }

    /// ### load_input_values
    ///
    /// Load values from configuration into input fields
    pub(super) fn load_input_values(&mut self) {
        if let Some(cli) = self.context.as_mut().unwrap().config_client.as_mut() {
            // Text editor
            if let Some(props) = self.view.get_props(super::COMPONENT_INPUT_TEXT_EDITOR) {
                let text_editor: String =
                    String::from(cli.get_text_editor().as_path().to_string_lossy());
                let props = InputPropsBuilder::from(props)
                    .with_value(text_editor)
                    .build();
                let _ = self.view.update(super::COMPONENT_INPUT_TEXT_EDITOR, props);
            }
            // Protocol
            if let Some(props) = self.view.get_props(super::COMPONENT_RADIO_DEFAULT_PROTOCOL) {
                let protocol: usize = match cli.get_default_protocol() {
                    FileTransferProtocol::Sftp => 0,
                    FileTransferProtocol::Scp => 1,
                    FileTransferProtocol::Ftp(false) => 2,
                    FileTransferProtocol::Ftp(true) => 3,
                };
                let props = RadioPropsBuilder::from(props).with_value(protocol).build();
                let _ = self
                    .view
                    .update(super::COMPONENT_RADIO_DEFAULT_PROTOCOL, props);
            }
            // Hidden files
            if let Some(props) = self.view.get_props(super::COMPONENT_RADIO_HIDDEN_FILES) {
                let hidden: usize = match cli.get_show_hidden_files() {
                    true => 0,
                    false => 1,
                };
                let props = RadioPropsBuilder::from(props).with_value(hidden).build();
                let _ = self.view.update(super::COMPONENT_RADIO_HIDDEN_FILES, props);
            }
            // Updates
            if let Some(props) = self.view.get_props(super::COMPONENT_RADIO_UPDATES) {
                let updates: usize = match cli.get_check_for_updates() {
                    true => 0,
                    false => 1,
                };
                let props = RadioPropsBuilder::from(props).with_value(updates).build();
                let _ = self.view.update(super::COMPONENT_RADIO_UPDATES, props);
            }
            // Group dirs
            if let Some(props) = self.view.get_props(super::COMPONENT_RADIO_GROUP_DIRS) {
                let dirs: usize = match cli.get_group_dirs() {
                    Some(GroupDirs::First) => 0,
                    Some(GroupDirs::Last) => 1,
                    None => 2,
                };
                let props = RadioPropsBuilder::from(props).with_value(dirs).build();
                let _ = self.view.update(super::COMPONENT_RADIO_GROUP_DIRS, props);
            }
            // Local File Fmt
            if let Some(props) = self.view.get_props(super::COMPONENT_INPUT_LOCAL_FILE_FMT) {
                let file_fmt: String = cli.get_local_file_fmt().unwrap_or_default();
                let props = InputPropsBuilder::from(props).with_value(file_fmt).build();
                let _ = self
                    .view
                    .update(super::COMPONENT_INPUT_LOCAL_FILE_FMT, props);
            }
            // Remote File Fmt
            if let Some(props) = self.view.get_props(super::COMPONENT_INPUT_REMOTE_FILE_FMT) {
                let file_fmt: String = cli.get_remote_file_fmt().unwrap_or_default();
                let props = InputPropsBuilder::from(props).with_value(file_fmt).build();
                let _ = self
                    .view
                    .update(super::COMPONENT_INPUT_REMOTE_FILE_FMT, props);
            }
        }
    }

    /// ### collect_input_values
    ///
    /// Collect values from input and put them into the configuration
    pub(super) fn collect_input_values(&mut self) {
        if let Some(cli) = self.context.as_mut().unwrap().config_client.as_mut() {
            if let Some(Payload::Text(editor)) =
                self.view.get_state(super::COMPONENT_INPUT_TEXT_EDITOR)
            {
                cli.set_text_editor(PathBuf::from(editor.as_str()));
            }
            if let Some(Payload::Unsigned(protocol)) =
                self.view.get_state(super::COMPONENT_RADIO_DEFAULT_PROTOCOL)
            {
                let protocol: FileTransferProtocol = match protocol {
                    1 => FileTransferProtocol::Scp,
                    2 => FileTransferProtocol::Ftp(false),
                    3 => FileTransferProtocol::Ftp(true),
                    _ => FileTransferProtocol::Sftp,
                };
                cli.set_default_protocol(protocol);
            }
            if let Some(Payload::Unsigned(opt)) =
                self.view.get_state(super::COMPONENT_RADIO_HIDDEN_FILES)
            {
                let show: bool = matches!(opt, 0);
                cli.set_show_hidden_files(show);
            }
            if let Some(Payload::Unsigned(opt)) =
                self.view.get_state(super::COMPONENT_RADIO_UPDATES)
            {
                let check: bool = matches!(opt, 0);
                cli.set_check_for_updates(check);
            }
            if let Some(Payload::Text(fmt)) =
                self.view.get_state(super::COMPONENT_INPUT_LOCAL_FILE_FMT)
            {
                cli.set_local_file_fmt(fmt);
            }
            if let Some(Payload::Text(fmt)) =
                self.view.get_state(super::COMPONENT_INPUT_REMOTE_FILE_FMT)
            {
                cli.set_remote_file_fmt(fmt);
            }
            if let Some(Payload::Unsigned(opt)) =
                self.view.get_state(super::COMPONENT_RADIO_GROUP_DIRS)
            {
                let dirs: Option<GroupDirs> = match opt {
                    0 => Some(GroupDirs::First),
                    1 => Some(GroupDirs::Last),
                    _ => None,
                };
                cli.set_group_dirs(dirs);
            }
        }
    }

    /// ### reload_ssh_keys
    ///
    /// Reload ssh keys
    pub(super) fn reload_ssh_keys(&mut self) {
        if let Some(cli) = self.context.as_ref().unwrap().config_client.as_ref() {
            // get props
            if let Some(props) = self.view.get_props(super::COMPONENT_LIST_SSH_KEYS) {
                // Create texts
                let keys: Vec<String> = cli
                    .iter_ssh_keys()
                    .map(|x| {
                        let (addr, username, _) = cli.get_ssh_key(x).ok().unwrap().unwrap();
                        format!("{} at {}", addr, username)
                    })
                    .collect();
                let props = BookmarkListPropsBuilder::from(props)
                    .with_bookmarks(Some(String::from("SSH Keys")), keys)
                    .build();
                self.view.update(super::COMPONENT_LIST_SSH_KEYS, props);
            }
        }
    }
}
