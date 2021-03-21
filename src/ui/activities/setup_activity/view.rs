//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

/*
*
*   Copyright (C) 2020-2021 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "TermSCP"
*
*   TermSCP is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   TermSCP is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with TermSCP.  If not, see <http://www.gnu.org/licenses/>.
*
*/

// Locals
use super::{Context, SetupActivity, ViewLayout};
use crate::filetransfer::FileTransferProtocol;
use crate::fs::explorer::GroupDirs;
use crate::ui::layout::components::{
    bookmark_list::BookmarkList, msgbox::MsgBox, input::Input, radio_group::RadioGroup, table::Table,
    text::Text,
};
use crate::ui::layout::props::{
    PropValue, PropsBuilder, TableBuilder, TextParts, TextSpan, TextSpanBuilder,
};
use crate::ui::layout::utils::draw_area_in;
use crate::ui::layout::view::View;
use crate::ui::layout::Payload;
// Ext
use std::path::PathBuf;
use tui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{Borders, Clear},
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
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightYellow)
                    .with_background(Color::Black)
                    .with_borders(Borders::BOTTOM)
                    .with_texts(TextParts::new(
                        None,
                        Some(vec![
                            TextSpan::from("User Interface"),
                            TextSpan::from("SSH Keys"),
                        ]),
                    ))
                    .with_value(PropValue::Unsigned(0))
                    .build(),
            )),
        );
        // Footer
        self.view.mount(
            super::COMPONENT_TEXT_FOOTER,
            Box::new(Text::new(
                PropsBuilder::default()
                    .with_texts(TextParts::new(
                        None,
                        Some(vec![
                            TextSpanBuilder::new("Press ").bold().build(),
                            TextSpanBuilder::new("<CTRL+H>")
                                .bold()
                                .with_foreground(Color::Cyan)
                                .build(),
                            TextSpanBuilder::new(" to show keybindings").bold().build(),
                        ]),
                    ))
                    .build(),
            )),
        );
        // Input fields
        self.view.mount(
            super::COMPONENT_INPUT_TEXT_EDITOR,
            Box::new(Input::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightGreen)
                    .with_texts(TextParts::new(Some(String::from("Text editor")), None))
                    .build(),
            )),
        );
        self.view.active(super::COMPONENT_INPUT_TEXT_EDITOR); // <-- Focus
        self.view.mount(
            super::COMPONENT_RADIO_DEFAULT_PROTOCOL,
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightCyan)
                    .with_background(Color::Black)
                    .with_texts(TextParts::new(
                        Some(String::from("Default file transfer protocol")),
                        Some(vec![
                            TextSpan::from("SFTP"),
                            TextSpan::from("SCP"),
                            TextSpan::from("FTP"),
                            TextSpan::from("FTPS"),
                        ]),
                    ))
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_RADIO_HIDDEN_FILES,
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightRed)
                    .with_background(Color::Black)
                    .with_texts(TextParts::new(
                        Some(String::from("Show hidden files (by default)")),
                        Some(vec![TextSpan::from("Yes"), TextSpan::from("No")]),
                    ))
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_RADIO_UPDATES,
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightYellow)
                    .with_background(Color::Black)
                    .with_texts(TextParts::new(
                        Some(String::from("Check for updates?")),
                        Some(vec![TextSpan::from("Yes"), TextSpan::from("No")]),
                    ))
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_RADIO_GROUP_DIRS,
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightMagenta)
                    .with_background(Color::Black)
                    .with_texts(TextParts::new(
                        Some(String::from("Group directories")),
                        Some(vec![
                            TextSpan::from("Display first"),
                            TextSpan::from("Display Last"),
                            TextSpan::from("No"),
                        ]),
                    ))
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_INPUT_FILE_FMT,
            Box::new(Input::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightBlue)
                    .with_texts(TextParts::new(
                        Some(String::from("File formatter syntax")),
                        None,
                    ))
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
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightYellow)
                    .with_background(Color::Black)
                    .with_borders(Borders::BOTTOM)
                    .with_texts(TextParts::new(
                        None,
                        Some(vec![
                            TextSpan::from("User Interface"),
                            TextSpan::from("SSH Keys"),
                        ]),
                    ))
                    .with_value(PropValue::Unsigned(1))
                    .build(),
            )),
        );
        // Footer
        self.view.mount(
            super::COMPONENT_TEXT_FOOTER,
            Box::new(Text::new(
                PropsBuilder::default()
                    .with_texts(TextParts::new(
                        None,
                        Some(vec![
                            TextSpanBuilder::new("Press ").bold().build(),
                            TextSpanBuilder::new("<CTRL+H>")
                                .bold()
                                .with_foreground(Color::Cyan)
                                .build(),
                            TextSpanBuilder::new(" to show keybindings").bold().build(),
                        ]),
                    ))
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_LIST_SSH_KEYS,
            Box::new(BookmarkList::new(
                PropsBuilder::default()
                    .with_texts(TextParts::new(Some(String::from("SSH Keys")), Some(vec![])))
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
                                Constraint::Length(3), // Format input
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
                        .render(super::COMPONENT_INPUT_FILE_FMT, f, ui_cfg_chunks[5]);
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
            if let Some(mut props) = self.view.get_props(super::COMPONENT_TEXT_ERROR) {
                if props.build().visible {
                    let popup = draw_area_in(f.size(), 50, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_TEXT_ERROR, f, popup);
                }
            }
            if let Some(mut props) = self.view.get_props(super::COMPONENT_RADIO_QUIT) {
                if props.build().visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 40, 10);
                    f.render_widget(Clear, popup);
                    self.view.render(super::COMPONENT_RADIO_QUIT, f, popup);
                }
            }
            if let Some(mut props) = self.view.get_props(super::COMPONENT_TEXT_HELP) {
                if props.build().visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 50, 70);
                    f.render_widget(Clear, popup);
                    self.view.render(super::COMPONENT_TEXT_HELP, f, popup);
                }
            }
            if let Some(mut props) = self.view.get_props(super::COMPONENT_RADIO_SAVE) {
                if props.build().visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 30, 10);
                    f.render_widget(Clear, popup);
                    self.view.render(super::COMPONENT_RADIO_SAVE, f, popup);
                }
            }
            if let Some(mut props) = self.view.get_props(super::COMPONENT_RADIO_DEL_SSH_KEY) {
                if props.build().visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 30, 10);
                    f.render_widget(Clear, popup);
                    self.view
                        .render(super::COMPONENT_RADIO_DEL_SSH_KEY, f, popup);
                }
            }
            if let Some(mut props) = self.view.get_props(super::COMPONENT_INPUT_SSH_HOST) {
                if props.build().visible {
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
                PropsBuilder::default()
                    .with_foreground(Color::Red)
                    .bold()
                    .with_texts(TextParts::new(None, Some(vec![TextSpan::from(text)])))
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
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightRed)
                    .bold()
                    .with_texts(TextParts::new(
                        Some(String::from("Delete key?")),
                        Some(vec![TextSpan::from("Yes"), TextSpan::from("No")]),
                    ))
                    .with_value(PropValue::Unsigned(1)) // Default: No
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
                PropsBuilder::default()
                    .with_texts(TextParts::new(
                        Some(String::from("Hostname or address")),
                        None,
                    ))
                    .with_borders(Borders::TOP | Borders::RIGHT | Borders::LEFT)
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_INPUT_SSH_USERNAME,
            Box::new(Input::new(
                PropsBuilder::default()
                    .with_texts(TextParts::new(Some(String::from("Username")), None))
                    .with_borders(Borders::BOTTOM | Borders::RIGHT | Borders::LEFT)
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
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightRed)
                    .bold()
                    .with_texts(TextParts::new(
                        Some(String::from("Exit setup?")),
                        Some(vec![
                            TextSpan::from("Save"),
                            TextSpan::from("Don't save"),
                            TextSpan::from("Cancel"),
                        ]),
                    ))
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
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightYellow)
                    .bold()
                    .with_texts(TextParts::new(
                        Some(String::from("Save changes?")),
                        Some(vec![TextSpan::from("Yes"), TextSpan::from("No")]),
                    ))
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
                PropsBuilder::default()
                    .with_texts(TextParts::table(
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
                    ))
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
            if let Some(props) = self
                .view
                .get_props(super::COMPONENT_INPUT_TEXT_EDITOR)
                .as_mut()
            {
                let text_editor: String =
                    String::from(cli.get_text_editor().as_path().to_string_lossy());
                let props = props.with_value(PropValue::Str(text_editor)).build();
                let _ = self.view.update(super::COMPONENT_INPUT_TEXT_EDITOR, props);
            }
            // Protocol
            if let Some(props) = self
                .view
                .get_props(super::COMPONENT_RADIO_DEFAULT_PROTOCOL)
                .as_mut()
            {
                let protocol: usize = match cli.get_default_protocol() {
                    FileTransferProtocol::Sftp => 0,
                    FileTransferProtocol::Scp => 1,
                    FileTransferProtocol::Ftp(false) => 2,
                    FileTransferProtocol::Ftp(true) => 3,
                };
                let props = props.with_value(PropValue::Unsigned(protocol)).build();
                let _ = self
                    .view
                    .update(super::COMPONENT_RADIO_DEFAULT_PROTOCOL, props);
            }
            // Hidden files
            if let Some(props) = self
                .view
                .get_props(super::COMPONENT_RADIO_HIDDEN_FILES)
                .as_mut()
            {
                let hidden: usize = match cli.get_show_hidden_files() {
                    true => 0,
                    false => 1,
                };
                let props = props.with_value(PropValue::Unsigned(hidden)).build();
                let _ = self.view.update(super::COMPONENT_RADIO_HIDDEN_FILES, props);
            }
            // Updates
            if let Some(props) = self.view.get_props(super::COMPONENT_RADIO_UPDATES).as_mut() {
                let updates: usize = match cli.get_check_for_updates() {
                    true => 0,
                    false => 1,
                };
                let props = props.with_value(PropValue::Unsigned(updates)).build();
                let _ = self.view.update(super::COMPONENT_RADIO_UPDATES, props);
            }
            // Group dirs
            if let Some(props) = self
                .view
                .get_props(super::COMPONENT_RADIO_GROUP_DIRS)
                .as_mut()
            {
                let dirs: usize = match cli.get_group_dirs() {
                    Some(GroupDirs::First) => 0,
                    Some(GroupDirs::Last) => 1,
                    None => 2,
                };
                let props = props.with_value(PropValue::Unsigned(dirs)).build();
                let _ = self.view.update(super::COMPONENT_RADIO_GROUP_DIRS, props);
            }
            // File Fmt
            if let Some(props) = self
                .view
                .get_props(super::COMPONENT_INPUT_FILE_FMT)
                .as_mut()
            {
                let file_fmt: String = cli.get_file_fmt().unwrap_or(String::new());
                let props = props.with_value(PropValue::Str(file_fmt)).build();
                let _ = self.view.update(super::COMPONENT_INPUT_FILE_FMT, props);
            }
        }
    }

    /// ### collect_input_values
    ///
    /// Collect values from input and put them into the configuration
    pub(super) fn collect_input_values(&mut self) {
        if let Some(cli) = self.context.as_mut().unwrap().config_client.as_mut() {
            if let Some(Payload::Text(editor)) =
                self.view.get_value(super::COMPONENT_INPUT_TEXT_EDITOR)
            {
                cli.set_text_editor(PathBuf::from(editor.as_str()));
            }
            if let Some(Payload::Unsigned(protocol)) =
                self.view.get_value(super::COMPONENT_RADIO_DEFAULT_PROTOCOL)
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
                self.view.get_value(super::COMPONENT_RADIO_HIDDEN_FILES)
            {
                let show: bool = match opt {
                    0 => true,
                    _ => false,
                };
                cli.set_show_hidden_files(show);
            }
            if let Some(Payload::Unsigned(opt)) =
                self.view.get_value(super::COMPONENT_RADIO_UPDATES)
            {
                let check: bool = match opt {
                    0 => true,
                    _ => false,
                };
                cli.set_check_for_updates(check);
            }
            if let Some(Payload::Text(fmt)) = self.view.get_value(super::COMPONENT_INPUT_FILE_FMT) {
                cli.set_file_fmt(fmt);
            }
            if let Some(Payload::Unsigned(opt)) =
                self.view.get_value(super::COMPONENT_RADIO_GROUP_DIRS)
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
            if let Some(props) = self.view.get_props(super::COMPONENT_LIST_SSH_KEYS).as_mut() {
                // Create texts
                let keys: Vec<TextSpan> = cli
                    .iter_ssh_keys()
                    .map(|x| {
                        let (addr, username, _) = cli.get_ssh_key(x).ok().unwrap().unwrap();
                        TextSpan::from(format!("{} at {}", addr, username).as_str())
                    })
                    .collect();
                let props = props
                    .with_texts(TextParts::new(Some(String::from("SSH Keys")), Some(keys)))
                    .build();
                self.view.update(super::COMPONENT_LIST_SSH_KEYS, props);
            }
        }
    }
}
