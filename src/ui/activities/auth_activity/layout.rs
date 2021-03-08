//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

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
use super::{
    AuthActivity, Context, DialogYesNoOption, FileTransferProtocol, InputField, InputForm, Popup,
};
use crate::ui::store::Store;
use crate::utils::fmt::align_text_center;
// Ext
use std::string::ToString;
use tui::{
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs},
};
use unicode_width::UnicodeWidthStr;

impl AuthActivity {
    /// ### draw
    ///
    /// Draw UI
    pub(super) fn draw(&mut self) {
        let mut ctx: Context = self.context.take().unwrap();
        let store: &Store = &ctx.store;
        let _ = ctx.terminal.draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(70), // Auth Form
                        Constraint::Percentage(30), // Bookmarks
                    ]
                    .as_ref(),
                )
                .split(f.size());
            // Create explorer chunks
            let auth_chunks = Layout::default()
                .constraints(
                    [
                        Constraint::Length(5),
                        Constraint::Length(1), // Version
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .direction(Direction::Vertical)
                .split(chunks[0]);
            // Create bookmark chunks
            let bookmark_chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .split(chunks[1]);
            // Draw header
            f.render_widget(self.draw_header(), auth_chunks[0]);
            f.render_widget(self.draw_new_version(store), auth_chunks[1]);
            // Draw input fields
            f.render_widget(self.draw_remote_address(), auth_chunks[2]);
            f.render_widget(self.draw_remote_port(), auth_chunks[3]);
            f.render_widget(self.draw_protocol_select(), auth_chunks[4]);
            f.render_widget(self.draw_protocol_username(), auth_chunks[5]);
            f.render_widget(self.draw_protocol_password(), auth_chunks[6]);
            // Draw footer
            f.render_widget(self.draw_footer(), auth_chunks[7]);
            // Set cursor
            if let InputForm::AuthCredentials = self.input_form {
                match self.selected_field {
                    InputField::Address => f.set_cursor(
                        auth_chunks[2].x + self.address.width() as u16 + 1,
                        auth_chunks[2].y + 1,
                    ),
                    InputField::Port => f.set_cursor(
                        auth_chunks[3].x + self.port.width() as u16 + 1,
                        auth_chunks[3].y + 1,
                    ),
                    InputField::Username => f.set_cursor(
                        auth_chunks[5].x + self.username.width() as u16 + 1,
                        auth_chunks[5].y + 1,
                    ),
                    InputField::Password => f.set_cursor(
                        auth_chunks[6].x + self.password_placeholder.width() as u16 + 1,
                        auth_chunks[6].y + 1,
                    ),
                    _ => {}
                }
            }
            // Draw bookmarks
            if let Some(tab) = self.draw_bookmarks_tab() {
                let mut bookmarks_state: ListState = ListState::default();
                bookmarks_state.select(Some(self.bookmarks_idx));
                f.render_stateful_widget(tab, bookmark_chunks[0], &mut bookmarks_state);
            }
            if let Some(tab) = self.draw_recents_tab() {
                let mut recents_state: ListState = ListState::default();
                recents_state.select(Some(self.recents_idx));
                f.render_stateful_widget(tab, bookmark_chunks[1], &mut recents_state);
            }
            // Draw popup
            if let Some(popup) = &self.popup {
                // Calculate popup size
                let (width, height): (u16, u16) = match popup {
                    Popup::Alert(_, _) => (50, 10),
                    Popup::Help => (50, 70),
                    Popup::SaveBookmark => (20, 20),
                    Popup::YesNo(_, _, _) => (30, 10),
                };
                let popup_area: Rect = self.draw_popup_area(f.size(), width, height);
                f.render_widget(Clear, popup_area); //this clears out the background
                match popup {
                    Popup::Alert(color, txt) => f.render_widget(
                        self.draw_popup_alert(*color, txt.clone(), popup_area.width),
                        popup_area,
                    ),
                    Popup::Help => f.render_widget(self.draw_popup_help(), popup_area),
                    Popup::SaveBookmark => {
                        let popup_chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints(
                                [
                                    Constraint::Length(3), // Input form
                                    Constraint::Length(2), // Yes/No
                                ]
                                .as_ref(),
                            )
                            .split(popup_area);
                        let (input, yes_no): (Paragraph, Tabs) = self.draw_popup_save_bookmark();
                        // Render parts
                        f.render_widget(input, popup_chunks[0]);
                        f.render_widget(yes_no, popup_chunks[1]);
                        // Set cursor
                        f.set_cursor(
                            popup_chunks[0].x + self.input_txt.width() as u16 + 1,
                            popup_chunks[0].y + 1,
                        )
                    }
                    Popup::YesNo(txt, _, _) => {
                        f.render_widget(self.draw_popup_yesno(txt.clone()), popup_area)
                    }
                }
            }
        });
        self.context = Some(ctx);
    }

    /// ### draw_remote_address
    ///
    /// Draw remote address block
    fn draw_remote_address(&self) -> Paragraph {
        Paragraph::new(self.address.as_ref())
            .style(match self.selected_field {
                InputField::Address => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Remote address"),
            )
    }

    /// ### draw_remote_port
    ///
    /// Draw remote port block
    fn draw_remote_port(&self) -> Paragraph {
        Paragraph::new(self.port.as_ref())
            .style(match self.selected_field {
                InputField::Port => Style::default().fg(Color::Cyan),
                _ => Style::default(),
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Remote port"),
            )
    }

    /// ### draw_protocol_select
    ///
    /// Draw protocol select
    fn draw_protocol_select(&self) -> Tabs {
        let protocols: Vec<Spans> = vec![
            Spans::from("SFTP"),
            Spans::from("SCP"),
            Spans::from("FTP"),
            Spans::from("FTPS"),
        ];
        let index: usize = match self.protocol {
            FileTransferProtocol::Sftp => 0,
            FileTransferProtocol::Scp => 1,
            FileTransferProtocol::Ftp(ftps) => match ftps {
                false => 2,
                true => 3,
            },
        };
        Tabs::new(protocols)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Protocol"),
            )
            .select(index)
            .style(match self.selected_field {
                InputField::Protocol => Style::default().fg(Color::Green),
                _ => Style::default(),
            })
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Green)
                    .fg(Color::Black),
            )
    }

    /// ### draw_protocol_username
    ///
    /// Draw username block
    fn draw_protocol_username(&self) -> Paragraph {
        Paragraph::new(self.username.as_ref())
            .style(match self.selected_field {
                InputField::Username => Style::default().fg(Color::Magenta),
                _ => Style::default(),
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Username"),
            )
    }

    /// ### draw_protocol_password
    ///
    /// Draw password block
    fn draw_protocol_password(&mut self) -> Paragraph {
        // Create password secret
        self.password_placeholder = (0..self.password.width()).map(|_| "*").collect::<String>();
        Paragraph::new(self.password_placeholder.as_ref())
            .style(match self.selected_field {
                InputField::Password => Style::default().fg(Color::LightBlue),
                _ => Style::default(),
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Password"),
            )
    }

    /// ### draw_header
    ///
    /// Draw header
    fn draw_header(&self) -> Paragraph {
        Paragraph::new(" _____                   ____   ____ ____  \n|_   _|__ _ __ _ __ ___ / ___| / ___|  _ \\ \n  | |/ _ \\ '__| '_ ` _ \\\\___ \\| |   | |_) |\n  | |  __/ |  | | | | | |___) | |___|  __/ \n  |_|\\___|_|  |_| |_| |_|____/ \\____|_|    \n")
            .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
    }

    /// ### draw_new_version
    ///
    /// Draw new version disclaimer
    fn draw_new_version(&self, store: &Store) -> Paragraph {
        let content: String = match store.get_string(super::STORE_KEY_LATEST_VERSION) {
            Some(ver) => format!("TermSCP {} is now available! Download it from <https://github.com/veeso/termscp/releases/latest>", ver),
            None => String::new(),
        };
        Paragraph::new(content).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
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
                Span::raw(" to show keybindings; "),
                Span::styled(
                    "<CTRL+C>",
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Cyan),
                ),
                Span::raw(" to enter setup"),
            ],
            Style::default().add_modifier(Modifier::BOLD),
        );
        let mut footer_text = Text::from(Spans::from(footer));
        footer_text.patch_style(h_style);
        Paragraph::new(footer_text)
    }

    /// ### draw_local_explorer
    ///
    /// Draw local explorer list
    fn draw_bookmarks_tab(&self) -> Option<List> {
        self.bookmarks_client.as_ref()?;
        let hosts: Vec<ListItem> = self
            .bookmarks_list
            .iter()
            .map(|key: &String| {
                let entry: (String, u16, FileTransferProtocol, String, _) = self
                    .bookmarks_client
                    .as_ref()
                    .unwrap()
                    .get_bookmark(key)
                    .unwrap();
                ListItem::new(Span::from(format!(
                    "{} ({}://{}@{}:{})",
                    key,
                    entry.2.to_string().to_lowercase(),
                    entry.3,
                    entry.0,
                    entry.1
                )))
            })
            .collect();
        // Get colors to use; highlight element inverting fg/bg only when tab is active
        let (fg, bg): (Color, Color) = match self.input_form {
            InputForm::Bookmarks => (Color::Black, Color::LightGreen),
            _ => (Color::Reset, Color::Reset),
        };
        Some(
            List::new(hosts)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(match self.input_form {
                            InputForm::Bookmarks => Style::default().fg(Color::LightGreen),
                            _ => Style::default(),
                        })
                        .title("Bookmarks"),
                )
                .start_corner(Corner::TopLeft)
                .highlight_style(Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD)),
        )
    }

    /// ### draw_local_explorer
    ///
    /// Draw local explorer list
    fn draw_recents_tab(&self) -> Option<List> {
        self.bookmarks_client.as_ref()?;
        let hosts: Vec<ListItem> = self
            .recents_list
            .iter()
            .map(|key: &String| {
                let entry: (String, u16, FileTransferProtocol, String) = self
                    .bookmarks_client
                    .as_ref()
                    .unwrap()
                    .get_recent(key)
                    .unwrap();
                ListItem::new(Span::from(format!(
                    "{}://{}@{}:{}",
                    entry.2.to_string().to_lowercase(),
                    entry.3,
                    entry.0,
                    entry.1
                )))
            })
            .collect();
        // Get colors to use; highlight element inverting fg/bg only when tab is active
        let (fg, bg): (Color, Color) = match self.input_form {
            InputForm::Recents => (Color::Black, Color::LightBlue),
            _ => (Color::Reset, Color::Reset),
        };
        Some(
            List::new(hosts)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(match self.input_form {
                            InputForm::Recents => Style::default().fg(Color::LightBlue),
                            _ => Style::default(),
                        })
                        .title("Recent connections"),
                )
                .start_corner(Corner::TopLeft)
                .highlight_style(Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD)),
        )
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

    /// ### draw_popup_input
    ///
    /// Draw input popup
    fn draw_popup_save_bookmark(&self) -> (Paragraph, Tabs) {
        let input: Paragraph = Paragraph::new(self.input_txt.as_ref())
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::TOP | Borders::RIGHT | Borders::LEFT)
                    .border_type(BorderType::Rounded)
                    .title("Save bookmark as..."),
            );
        let choices: Vec<Spans> = vec![Spans::from("Yes"), Spans::from("No")];
        let index: usize = match self.choice_opt {
            DialogYesNoOption::Yes => 0,
            DialogYesNoOption::No => 1,
        };
        let tabs: Tabs = Tabs::new(choices)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM | Borders::RIGHT | Borders::LEFT)
                    .border_type(BorderType::Rounded)
                    .title("Save password?"),
            )
            .select(index)
            .style(Style::default())
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightRed),
            );
        (input, tabs)
    }

    /// ### draw_popup_yesno
    ///
    /// Draw yes/no select popup
    fn draw_popup_yesno(&self, text: String) -> Tabs {
        let choices: Vec<Spans> = vec![Spans::from("Yes"), Spans::from("No")];
        let index: usize = match self.choice_opt {
            DialogYesNoOption::Yes => 0,
            DialogYesNoOption::No => 1,
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
                Span::raw("Quit TermSCP"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<TAB>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("           "),
                Span::raw("Switch input form and bookmarks"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<RIGHT/LEFT>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("    "),
                Span::raw("Change bookmark tab"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<UP/DOWN>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("       "),
                Span::raw("Move up/down in current tab"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<ENTER>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("         "),
                Span::raw("Submit"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<DEL>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("           "),
                Span::raw("Delete bookmark"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<E>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("             "),
                Span::raw("Delete selected bookmark"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<CTRL+C>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("        "),
                Span::raw("Enter setup"),
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
                    "<CTRL+S>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("        "),
                Span::raw("Save bookmark"),
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
