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

use super::{
    Context, Popup, QuitDialogOption, SetupActivity, SetupTab, UserInterfaceInputField,
    YesNoDialogOption,
};
use crate::filetransfer::FileTransferProtocol;
use crate::fs::explorer::GroupDirs;
use crate::utils::fmt::align_text_center;
// Ext
use tui::{
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs},
};
use unicode_width::UnicodeWidthStr;

impl SetupActivity {
    /// ### draw
    ///
    /// Draw UI
    pub(super) fn draw(&mut self) {
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
            // Prepare selected tab
            f.render_widget(self.draw_selected_tab(), chunks[0]);
            // Draw main layout
            match &self.tab {
                SetupTab::SshConfig => {
                    // Draw ssh config
                    // Create explorer chunks
                    let sshcfg_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Percentage(100)].as_ref())
                        .split(chunks[1]);
                    if let Some(ssh_key_tab) = self.draw_ssh_keys_list() {
                        // Create ssh list state
                        let mut ssh_key_state: ListState = ListState::default();
                        ssh_key_state.select(Some(self.ssh_key_idx));
                        // Render ssh keys
                        f.render_stateful_widget(ssh_key_tab, sshcfg_chunks[0], &mut ssh_key_state);
                    }
                }
                SetupTab::UserInterface(form_field) => {
                    // Create chunks
                    let ui_cfg_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Length(3),
                                Constraint::Length(3),
                                Constraint::Length(3),
                                Constraint::Length(3),
                                Constraint::Length(3),
                                Constraint::Length(1),
                            ]
                            .as_ref(),
                        )
                        .split(chunks[1]);
                    // Render input forms
                    if let Some(field) = self.draw_text_editor_input() {
                        f.render_widget(field, ui_cfg_chunks[0]);
                    }
                    if let Some(tab) = self.draw_default_protocol_tab() {
                        f.render_widget(tab, ui_cfg_chunks[1]);
                    }
                    if let Some(tab) = self.draw_hidden_files_tab() {
                        f.render_widget(tab, ui_cfg_chunks[2]);
                    }
                    if let Some(tab) = self.draw_default_group_dirs_tab() {
                        f.render_widget(tab, ui_cfg_chunks[3]);
                    }
                    if let Some(tab) = self.draw_file_fmt_input() {
                        f.render_widget(tab, ui_cfg_chunks[4]);
                    }
                    // Set cursor
                    if let Some(cli) = &self.config_cli {
                        match form_field {
                            UserInterfaceInputField::TextEditor => {
                                let editor_text: String =
                                    String::from(cli.get_text_editor().as_path().to_string_lossy());
                                f.set_cursor(
                                    ui_cfg_chunks[0].x + editor_text.width() as u16 + 1,
                                    ui_cfg_chunks[0].y + 1,
                                );
                            }
                            UserInterfaceInputField::FileFmt => {
                                let file_fmt: String = cli.get_file_fmt().unwrap_or_default();
                                f.set_cursor(
                                    ui_cfg_chunks[4].x + file_fmt.width() as u16 + 1,
                                    ui_cfg_chunks[4].y + 1,
                                );
                            }
                            _ => { /* Not a text field */ }
                        }
                    }
                }
            }
            // Draw footer
            f.render_widget(self.draw_footer(), chunks[2]);
            // Draw popup
            if let Some(popup) = &self.popup {
                // Calculate popup size
                let (width, height): (u16, u16) = match popup {
                    Popup::Alert(_, _) | Popup::Fatal(_) => (50, 10),
                    Popup::Help => (50, 70),
                    Popup::NewSshKey => (50, 20),
                    Popup::Quit => (40, 10),
                    Popup::YesNo(_, _, _) => (30, 10),
                };
                let popup_area: Rect = self.draw_popup_area(f.size(), width, height);
                f.render_widget(Clear, popup_area); //this clears out the background
                match popup {
                    Popup::Alert(color, txt) => f.render_widget(
                        self.draw_popup_alert(*color, txt.clone(), popup_area.width),
                        popup_area,
                    ),
                    Popup::Fatal(txt) => f.render_widget(
                        self.draw_popup_fatal(txt.clone(), popup_area.width),
                        popup_area,
                    ),
                    Popup::Help => f.render_widget(self.draw_popup_help(), popup_area),
                    Popup::NewSshKey => {
                        let popup_chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints(
                                [
                                    Constraint::Length(3), // Address form
                                    Constraint::Length(3), // Username form
                                ]
                                .as_ref(),
                            )
                            .split(popup_area);
                        let (address_form, username_form): (Paragraph, Paragraph) =
                            self.draw_popup_new_ssh_key();
                        // Render parts
                        f.render_widget(address_form, popup_chunks[0]);
                        f.render_widget(username_form, popup_chunks[1]);
                        // Set cursor to popup form
                        if self.user_input_ptr < 2 {
                            if let Some(selected_text) = self.user_input.get(self.user_input_ptr) {
                                // Set cursor
                                f.set_cursor(
                                    popup_chunks[self.user_input_ptr].x
                                        + selected_text.width() as u16
                                        + 1,
                                    popup_chunks[self.user_input_ptr].y + 1,
                                )
                            }
                        }
                    }
                    Popup::Quit => f.render_widget(self.draw_popup_quit(), popup_area),
                    Popup::YesNo(txt, _, _) => {
                        f.render_widget(self.draw_popup_yesno(txt.clone()), popup_area)
                    }
                }
            }
        });
        self.context = Some(ctx);
    }

    /// ### draw_selecte_tab
    ///
    /// Draw selected tab tab
    fn draw_selected_tab(&self) -> Tabs {
        let choices: Vec<Spans> = vec![Spans::from("User Interface"), Spans::from("SSH Keys")];
        let index: usize = match self.tab {
            SetupTab::UserInterface(_) => 0,
            SetupTab::SshConfig => 1,
        };
        Tabs::new(choices)
            .block(Block::default().borders(Borders::BOTTOM).title("Setup"))
            .select(index)
            .style(Style::default())
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            )
    }

    /// ### draw_footer
    ///
    /// Draw authentication page footer
    fn draw_footer(&self) -> Paragraph {
        // Write header
        let (footer, h_style) = (
            vec![
                Span::raw("Press "),
                Span::styled(
                    "<CTRL+H>",
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Cyan),
                ),
                Span::raw(" to show keybindings"),
            ],
            Style::default().add_modifier(Modifier::BOLD),
        );
        let mut footer_text = Text::from(Spans::from(footer));
        footer_text.patch_style(h_style);
        Paragraph::new(footer_text)
    }

    /// ### draw_text_editor_input
    ///
    /// Draw input text field for text editor parameter
    fn draw_text_editor_input(&self) -> Option<Paragraph> {
        match &self.config_cli {
            Some(cli) => Some(
                Paragraph::new(String::from(
                    cli.get_text_editor().as_path().to_string_lossy(),
                ))
                .style(Style::default().fg(match &self.tab {
                    SetupTab::SshConfig => Color::White,
                    SetupTab::UserInterface(field) => match field {
                        UserInterfaceInputField::TextEditor => Color::LightGreen,
                        _ => Color::White,
                    },
                }))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title("Text Editor"),
                ),
            ),
            None => None,
        }
    }

    /// ### draw_default_protocol_tab
    ///
    /// Draw default protocol input tab
    fn draw_default_protocol_tab(&self) -> Option<Tabs> {
        // Check if config client is some
        match &self.config_cli {
            Some(cli) => {
                let choices: Vec<Spans> = vec![
                    Spans::from("SFTP"),
                    Spans::from("SCP"),
                    Spans::from("FTP"),
                    Spans::from("FTPS"),
                ];
                let index: usize = match cli.get_default_protocol() {
                    FileTransferProtocol::Sftp => 0,
                    FileTransferProtocol::Scp => 1,
                    FileTransferProtocol::Ftp(secure) => match secure {
                        false => 2,
                        true => 3,
                    },
                };
                let (bg, fg, block_fg): (Color, Color, Color) = match &self.tab {
                    SetupTab::UserInterface(field) => match field {
                        UserInterfaceInputField::DefaultProtocol => {
                            (Color::Cyan, Color::Black, Color::Cyan)
                        }
                        _ => (Color::Reset, Color::Cyan, Color::Reset),
                    },
                    _ => (Color::Reset, Color::Reset, Color::Reset),
                };
                Some(
                    Tabs::new(choices)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded)
                                .style(Style::default().fg(block_fg))
                                .title("Default File Transfer Protocol"),
                        )
                        .select(index)
                        .style(Style::default())
                        .highlight_style(
                            Style::default().add_modifier(Modifier::BOLD).fg(fg).bg(bg),
                        ),
                )
            }
            None => None,
        }
    }

    /// ### draw_default_protocol_tab
    ///
    /// Draw default protocol input tab
    fn draw_hidden_files_tab(&self) -> Option<Tabs> {
        // Check if config client is some
        match &self.config_cli {
            Some(cli) => {
                let choices: Vec<Spans> = vec![Spans::from("Yes"), Spans::from("No")];
                let index: usize = match cli.get_show_hidden_files() {
                    true => 0,
                    false => 1,
                };
                let (bg, fg, block_fg): (Color, Color, Color) = match &self.tab {
                    SetupTab::UserInterface(field) => match field {
                        UserInterfaceInputField::ShowHiddenFiles => {
                            (Color::LightRed, Color::Black, Color::LightRed)
                        }
                        _ => (Color::Reset, Color::LightRed, Color::Reset),
                    },
                    _ => (Color::Reset, Color::Reset, Color::Reset),
                };
                Some(
                    Tabs::new(choices)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded)
                                .style(Style::default().fg(block_fg))
                                .title("Show hidden files (by default)"),
                        )
                        .select(index)
                        .style(Style::default())
                        .highlight_style(
                            Style::default().add_modifier(Modifier::BOLD).fg(fg).bg(bg),
                        ),
                )
            }
            None => None,
        }
    }

    /// ### draw_default_group_dirs_tab
    ///
    /// Draw group dirs input tab
    fn draw_default_group_dirs_tab(&self) -> Option<Tabs> {
        // Check if config client is some
        match &self.config_cli {
            Some(cli) => {
                let choices: Vec<Spans> = vec![
                    Spans::from("Display First"),
                    Spans::from("Display Last"),
                    Spans::from("No"),
                ];
                let index: usize = match cli.get_group_dirs() {
                    None => 2,
                    Some(val) => match val {
                        GroupDirs::First => 0,
                        GroupDirs::Last => 1,
                    },
                };
                let (bg, fg, block_fg): (Color, Color, Color) = match &self.tab {
                    SetupTab::UserInterface(field) => match field {
                        UserInterfaceInputField::GroupDirs => {
                            (Color::LightMagenta, Color::Black, Color::LightMagenta)
                        }
                        _ => (Color::Reset, Color::LightMagenta, Color::Reset),
                    },
                    _ => (Color::Reset, Color::Reset, Color::Reset),
                };
                Some(
                    Tabs::new(choices)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded)
                                .style(Style::default().fg(block_fg))
                                .title("Group directories"),
                        )
                        .select(index)
                        .style(Style::default())
                        .highlight_style(
                            Style::default().add_modifier(Modifier::BOLD).fg(fg).bg(bg),
                        ),
                )
            }
            None => None,
        }
    }

    /// ### draw_file_fmt_input
    ///
    /// Draw input text field for file fmt
    fn draw_file_fmt_input(&self) -> Option<Paragraph> {
        match &self.config_cli {
            Some(cli) => Some(
                Paragraph::new(cli.get_file_fmt().unwrap_or_default())
                    .style(Style::default().fg(match &self.tab {
                        SetupTab::SshConfig => Color::White,
                        SetupTab::UserInterface(field) => match field {
                            UserInterfaceInputField::FileFmt => Color::LightCyan,
                            _ => Color::White,
                        },
                    }))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .title("File formatter syntax"),
                    ),
            ),
            None => None,
        }
    }

    /// ### draw_ssh_keys_list
    ///
    /// Draw ssh keys list
    fn draw_ssh_keys_list(&self) -> Option<List> {
        // Check if config client is some
        match &self.config_cli {
            Some(cli) => {
                // Iterate over ssh keys
                let mut ssh_keys: Vec<ListItem> = Vec::with_capacity(cli.iter_ssh_keys().count());
                for key in cli.iter_ssh_keys() {
                    if let Ok(host) = cli.get_ssh_key(key) {
                        if let Some((addr, username, _)) = host {
                            ssh_keys.push(ListItem::new(Span::from(format!(
                                "{} at {}",
                                username, addr,
                            ))));
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                // Return list
                Some(
                    List::new(ssh_keys)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(Color::LightGreen))
                                .title("SSH Keys"),
                        )
                        .start_corner(Corner::TopLeft)
                        .highlight_style(
                            Style::default()
                                .fg(Color::Black)
                                .bg(Color::LightGreen)
                                .add_modifier(Modifier::BOLD),
                        ),
                )
            }
            None => None,
        }
    }

    /// ### draw_popup_area
    ///
    /// Draw popup area
    fn draw_popup_area(&self, area: Rect, width: u16, height: u16) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - height) / 2),
                    Constraint::Percentage(height),
                    Constraint::Percentage((100 - height) / 2),
                ]
                .as_ref(),
            )
            .split(area);
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - width) / 2),
                    Constraint::Percentage(width),
                    Constraint::Percentage((100 - width) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }

    /// ### draw_popup_alert
    ///
    /// Draw alert popup
    fn draw_popup_alert(&self, color: Color, text: String, width: u16) -> List {
        // Wraps texts
        let message_rows = textwrap::wrap(text.as_str(), width as usize);
        let mut lines: Vec<ListItem> = Vec::new();
        for msg in message_rows.iter() {
            lines.push(ListItem::new(Spans::from(align_text_center(msg, width))));
        }
        List::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(color))
                    .border_type(BorderType::Rounded)
                    .title("Alert"),
            )
            .start_corner(Corner::TopLeft)
            .style(Style::default().fg(color))
    }

    /// ### draw_popup_fatal
    ///
    /// Draw fatal error popup
    fn draw_popup_fatal(&self, text: String, width: u16) -> List {
        self.draw_popup_alert(Color::Red, text, width)
    }

    /// ### draw_popup_new_ssh_key
    ///
    /// Draw new ssh key form popup
    fn draw_popup_new_ssh_key(&self) -> (Paragraph, Paragraph) {
        let address: Paragraph = Paragraph::new(self.user_input.get(0).unwrap().as_str())
            .style(Style::default().fg(match self.user_input_ptr {
                0 => Color::LightCyan,
                _ => Color::White,
            }))
            .block(
                Block::default()
                    .borders(Borders::TOP | Borders::RIGHT | Borders::LEFT)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Color::White))
                    .title("Host name or address"),
            );
        let username: Paragraph = Paragraph::new(self.user_input.get(1).unwrap().as_str())
            .style(Style::default().fg(match self.user_input_ptr {
                1 => Color::LightMagenta,
                _ => Color::White,
            }))
            .block(
                Block::default()
                    .borders(Borders::BOTTOM | Borders::RIGHT | Borders::LEFT)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Color::White))
                    .title("Username"),
            );
        (address, username)
    }

    /// ### draw_popup_quit
    ///
    /// Draw quit select popup
    fn draw_popup_quit(&self) -> Tabs {
        let choices: Vec<Spans> = vec![
            Spans::from("Save"),
            Spans::from("Don't save"),
            Spans::from("Cancel"),
        ];
        let index: usize = match self.quit_opt {
            QuitDialogOption::Save => 0,
            QuitDialogOption::DontSave => 1,
            QuitDialogOption::Cancel => 2,
        };
        Tabs::new(choices)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Exit setup?"),
            )
            .select(index)
            .style(Style::default())
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
    }

    /// ### draw_popup_yesno
    ///
    /// Draw yes/no select popup
    fn draw_popup_yesno(&self, text: String) -> Tabs {
        let choices: Vec<Spans> = vec![Spans::from("Yes"), Spans::from("No")];
        let index: usize = match self.yesno_opt {
            YesNoDialogOption::Yes => 0,
            YesNoDialogOption::No => 1,
        };
        Tabs::new(choices)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(text),
            )
            .select(index)
            .style(Style::default())
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            )
    }

    /// ### draw_popup_help
    ///
    /// Draw authentication page help popup
    fn draw_popup_help(&self) -> List {
        // Write header
        let cmds: Vec<ListItem> = vec![
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<ESC>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("           "),
                Span::raw("Exit setup"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<TAB>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("           "),
                Span::raw("Change setup page"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<RIGHT/LEFT>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("    "),
                Span::raw("Change selected element in tab"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<UP/DOWN>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("       "),
                Span::raw("Change input field"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<ENTER>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("         "),
                Span::raw("Submit / Dismiss popup"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<DEL>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("           "),
                Span::raw("Delete entry"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<CTRL+E>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("        "),
                Span::raw("Delete entry"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<CTRL+H>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("        "),
                Span::raw("Show help"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<CTRL+N>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("        "),
                Span::raw("New SSH key"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<CTRL+R>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("        "),
                Span::raw("Revert changes"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<CTRL+S>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("        "),
                Span::raw("Save configuration"),
            ])),
        ];
        List::new(cmds)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default())
                    .border_type(BorderType::Rounded)
                    .title("Help"),
            )
            .start_corner(Corner::TopLeft)
    }
}
