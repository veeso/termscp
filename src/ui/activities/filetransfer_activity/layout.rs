/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
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

extern crate bytesize;
extern crate hostname;
#[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
extern crate users;

use super::{
    Context, DialogYesNoOption, FileExplorerTab, FileTransferActivity, FsEntry, InputField,
    InputMode, LogLevel, LogRecord, PopupType,
};
use crate::utils::time_to_str;

use bytesize::ByteSize;
use std::path::{Path, PathBuf};
use tui::{
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Tabs},
};
use unicode_width::UnicodeWidthStr;
#[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
use users::{get_group_by_gid, get_user_by_uid};

impl FileTransferActivity {
    /// ### draw
    ///
    /// Draw UI
    pub(super) fn draw(&mut self) {
        let mut ctx: Context = self.context.take().unwrap();
        let local_wrkdir: PathBuf = ctx.local.pwd();
        let _ = ctx.terminal.draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(20), // Explorer
                        Constraint::Length(16), // Log
                    ]
                    .as_ref(),
                )
                .split(f.size());
            // Create explorer chunks
            let tabs_chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .split(chunks[0]);
            // Set localhost state
            let mut localhost_state: ListState = ListState::default();
            localhost_state.select(Some(self.local.index));
            // Set remote state
            let mut remote_state: ListState = ListState::default();
            remote_state.select(Some(self.remote.index));
            // Draw tabs
            f.render_stateful_widget(
                self.draw_local_explorer(local_wrkdir, tabs_chunks[0].width),
                tabs_chunks[0],
                &mut localhost_state,
            );
            // Get pwd
            let remote_wrkdir: PathBuf = match self.client.pwd() {
                Ok(p) => p,
                Err(_) => PathBuf::from("/"),
            };
            f.render_stateful_widget(
                self.draw_remote_explorer(remote_wrkdir, tabs_chunks[1].width),
                tabs_chunks[1],
                &mut remote_state,
            );
            // Set log state
            let mut log_state: ListState = ListState::default();
            log_state.select(Some(self.log_index));
            // Draw log
            f.render_stateful_widget(
                self.draw_log_list(chunks[1].width),
                chunks[1],
                &mut log_state,
            );
            // Draw popup
            if let InputMode::Popup(popup) = &self.input_mode {
                // Calculate popup size
                let (width, height): (u16, u16) = match popup {
                    PopupType::Alert(_, _) => (50, 10),
                    PopupType::Fatal(_) => (50, 10),
                    PopupType::FileInfo => (50, 50),
                    PopupType::Help => (50, 70),
                    PopupType::Input(_, _) => (40, 10),
                    PopupType::Progress(_) => (40, 10),
                    PopupType::Wait(_) => (50, 10),
                    PopupType::YesNo(_, _, _) => (30, 10),
                };
                let popup_area: Rect = self.draw_popup_area(f.size(), width, height);
                f.render_widget(Clear, popup_area); //this clears out the background
                match popup {
                    PopupType::Alert(color, txt) => f.render_widget(
                        self.draw_popup_alert(*color, txt.clone(), popup_area.width),
                        popup_area,
                    ),
                    PopupType::Fatal(txt) => f.render_widget(
                        self.draw_popup_fatal(txt.clone(), popup_area.width),
                        popup_area,
                    ),
                    PopupType::FileInfo => f.render_widget(self.draw_popup_fileinfo(), popup_area),
                    PopupType::Help => f.render_widget(self.draw_popup_help(), popup_area),
                    PopupType::Input(txt, _) => {
                        f.render_widget(self.draw_popup_input(txt.clone()), popup_area);
                        // Set cursor
                        f.set_cursor(
                            popup_area.x + self.input_txt.width() as u16 + 1,
                            popup_area.y + 1,
                        )
                    }
                    PopupType::Progress(txt) => {
                        f.render_widget(self.draw_popup_progress(txt.clone()), popup_area)
                    }
                    PopupType::Wait(txt) => f.render_widget(
                        self.draw_popup_wait(txt.clone(), popup_area.width),
                        popup_area,
                    ),
                    PopupType::YesNo(txt, _, _) => {
                        f.render_widget(self.draw_popup_yesno(txt.clone()), popup_area)
                    }
                }
            }
        });
        self.context = Some(ctx);
    }

    /// ### draw_local_explorer
    ///
    /// Draw local explorer list
    pub(super) fn draw_local_explorer(&self, local_wrkdir: PathBuf, width: u16) -> List {
        let hostname: String = match hostname::get() {
            Ok(h) => String::from(h.as_os_str().to_string_lossy()),
            Err(_) => String::from("localhost"),
        };
        let files: Vec<ListItem> = self
            .local
            .files
            .iter()
            .map(|entry: &FsEntry| ListItem::new(Span::from(format!("{}", entry))))
            .collect();
        List::new(files)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(match self.input_field {
                        InputField::Explorer => match self.tab {
                            FileExplorerTab::Local => Style::default().fg(Color::Yellow),
                            _ => Style::default(),
                        },
                        _ => Style::default(),
                    })
                    .title(format!(
                        "{}:{} ",
                        hostname,
                        FileTransferActivity::elide_wrkdir_path(
                            local_wrkdir.as_path(),
                            hostname.as_str(),
                            width
                        )
                        .display()
                    )),
            )
            .start_corner(Corner::TopLeft)
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
    }

    /// ### draw_remote_explorer
    ///
    /// Draw remote explorer list
    pub(super) fn draw_remote_explorer(&self, remote_wrkdir: PathBuf, width: u16) -> List {
        let files: Vec<ListItem> = self
            .remote
            .files
            .iter()
            .map(|entry: &FsEntry| ListItem::new(Span::from(format!("{}", entry))))
            .collect();
        List::new(files)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(match self.input_field {
                        InputField::Explorer => match self.tab {
                            FileExplorerTab::Remote => Style::default().fg(Color::LightBlue),
                            _ => Style::default(),
                        },
                        _ => Style::default(),
                    })
                    .title(format!(
                        "{}:{} ",
                        self.params.address,
                        FileTransferActivity::elide_wrkdir_path(
                            remote_wrkdir.as_path(),
                            self.params.address.as_str(),
                            width
                        )
                        .display()
                    )),
            )
            .start_corner(Corner::TopLeft)
            .highlight_style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )
    }

    /// ### draw_log_list
    ///
    /// Draw log list
    /// Chunk width must be provided to wrap text
    pub(super) fn draw_log_list(&self, width: u16) -> List {
        let events: Vec<ListItem> = self
            .log_records
            .iter()
            .map(|record: &LogRecord| {
                let record_rows = textwrap::wrap(record.msg.as_str(), (width as usize) - 35); // -35 'cause log prefix
                let s = match record.level {
                    LogLevel::Error => Style::default().fg(Color::Red),
                    LogLevel::Warn => Style::default().fg(Color::Yellow),
                    LogLevel::Info => Style::default().fg(Color::Green),
                };
                let mut rows: Vec<Spans> = Vec::with_capacity(record_rows.len());
                // Iterate over remaining rows
                for (idx, row) in record_rows.iter().enumerate() {
                    let row: Spans = match idx {
                        0 => Spans::from(vec![
                            Span::from(format!("{}", record.time.format("%Y-%m-%dT%H:%M:%S%Z"))),
                            Span::raw(" ["),
                            Span::styled(
                                format!(
                                    "{:5}",
                                    match record.level {
                                        LogLevel::Error => "ERROR",
                                        LogLevel::Warn => "WARN",
                                        LogLevel::Info => "INFO",
                                    }
                                ),
                                s,
                            ),
                            Span::raw("]: "),
                            Span::from(String::from(row.as_ref())),
                        ]),
                        _ => Spans::from(vec![Span::from(textwrap::indent(
                            row.as_ref(),
                            "                                   ",
                        ))]),
                    };
                    rows.push(row);
                }
                ListItem::new(rows)
            })
            .collect();
        List::new(events)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(match self.input_field {
                        InputField::Logs => Style::default().fg(Color::LightGreen),
                        _ => Style::default(),
                    })
                    .title("Log"),
            )
            .start_corner(Corner::BottomLeft)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    }

    /// ### draw_popup_area
    ///
    /// Draw popup area
    pub(super) fn draw_popup_area(&self, area: Rect, width: u16, height: u16) -> Rect {
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
    pub(super) fn draw_popup_alert(&self, color: Color, text: String, width: u16) -> List {
        // Wraps texts
        let message_rows = textwrap::wrap(text.as_str(), width as usize);
        let mut lines: Vec<ListItem> = Vec::new();
        for msg in message_rows.iter() {
            lines.push(ListItem::new(Spans::from(
                FileTransferActivity::align_text_center(msg, width),
            )));
        }
        List::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(color))
                    .title("Alert"),
            )
            .start_corner(Corner::TopLeft)
            .style(Style::default().fg(color))
    }

    /// ### draw_popup_fatal
    ///
    /// Draw fatal error popup
    pub(super) fn draw_popup_fatal(&self, text: String, width: u16) -> List {
        // Wraps texts
        let message_rows = textwrap::wrap(text.as_str(), width as usize);
        let mut lines: Vec<ListItem> = Vec::new();
        for msg in message_rows.iter() {
            lines.push(ListItem::new(Spans::from(
                FileTransferActivity::align_text_center(msg, width),
            )));
        }
        List::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red))
                    .title("Fatal error"),
            )
            .start_corner(Corner::TopLeft)
            .style(Style::default().fg(Color::Red))
    }
    /// ### draw_popup_input
    ///
    /// Draw input popup
    pub(super) fn draw_popup_input(&self, text: String) -> Paragraph {
        Paragraph::new(self.input_txt.as_ref())
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title(text))
    }

    /// ### draw_popup_progress
    ///
    /// Draw progress popup
    pub(super) fn draw_popup_progress(&self, text: String) -> Gauge {
        // Calculate ETA
        let eta: String = match self.transfer_progress as u64 {
            0 => String::from("--:--"), // NOTE: would divide by 0 :D
            _ => {
                let elapsed_secs: u64 = self.transfer_started.elapsed().as_secs();
                let eta: u64 =
                    ((elapsed_secs * 100) / (self.transfer_progress as u64)) - elapsed_secs;
                format!("{:0width$}:{:0width$}", (eta / 60), (eta % 60), width = 2)
            }
        };
        let label = format!("{:.2}% - ETA {}", self.transfer_progress, eta);
        Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(text))
            .gauge_style(
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .label(label)
            .ratio(self.transfer_progress / 100.0)
    }

    /// ### draw_popup_wait
    ///
    /// Draw wait popup
    pub(super) fn draw_popup_wait(&self, text: String, width: u16) -> List {
        // Wraps texts
        let message_rows = textwrap::wrap(text.as_str(), width as usize);
        let mut lines: Vec<ListItem> = Vec::new();
        for msg in message_rows.iter() {
            lines.push(ListItem::new(Spans::from(
                FileTransferActivity::align_text_center(msg, width),
            )));
        }
        List::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White))
                    .title("Please wait"),
            )
            .start_corner(Corner::TopLeft)
            .style(Style::default().add_modifier(Modifier::BOLD))
    }

    /// ### draw_popup_yesno
    ///
    /// Draw yes/no select popup
    pub(super) fn draw_popup_yesno(&self, text: String) -> Tabs {
        let choices: Vec<Spans> = vec![Spans::from("Yes"), Spans::from("No")];
        let index: usize = match self.choice_opt {
            DialogYesNoOption::Yes => 0,
            DialogYesNoOption::No => 1,
        };
        Tabs::new(choices)
            .block(Block::default().borders(Borders::ALL).title(text))
            .select(index)
            .style(Style::default())
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            )
    }

    /// ### draw_popup_fileinfo
    ///
    /// Draw popup containing info about selected fsentry
    pub(super) fn draw_popup_fileinfo(&self) -> List {
        let mut info: Vec<ListItem> = Vec::new();
        // Get current fsentry
        let fsentry: Option<&FsEntry> = match self.tab {
            FileExplorerTab::Local => {
                // Get selected file
                match self.local.files.get(self.local.index) {
                    Some(entry) => Some(entry),
                    None => None,
                }
            }
            FileExplorerTab::Remote => match self.remote.files.get(self.remote.index) {
                Some(entry) => Some(entry),
                None => None,
            },
        };
        // Get file_name and fill info list
        let file_name: String = match fsentry {
            Some(fsentry) => {
                // Get name and path
                let abs_path: PathBuf = fsentry.get_abs_path();
                let name: String = fsentry.get_name();
                let ctime: String = time_to_str(fsentry.get_creation_time(), "%b %d %Y %H:%M:%S");
                let atime: String =
                    time_to_str(fsentry.get_last_access_time(), "%b %d %Y %H:%M:%S");
                let mtime: String = time_to_str(fsentry.get_creation_time(), "%b %d %Y %H:%M:%S");
                let (bsize, size): (ByteSize, usize) =
                    (ByteSize(fsentry.get_size() as u64), fsentry.get_size());
                let user: Option<u32> = fsentry.get_user();
                let group: Option<u32> = fsentry.get_group();
                let real_path: Option<PathBuf> = {
                    let real_file: FsEntry = fsentry.get_realfile();
                    match real_file.get_abs_path() != abs_path {
                        true => Some(real_file.get_abs_path()),
                        false => None,
                    }
                };
                // Push path
                info.push(ListItem::new(Spans::from(vec![
                    Span::styled("Path: ", Style::default()),
                    Span::styled(
                        match real_path {
                            Some(symlink) => {
                                format!("{} => {}", abs_path.display(), symlink.display())
                            }
                            None => abs_path.to_string_lossy().to_string(),
                        },
                        Style::default()
                            .fg(Color::LightYellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])));
                // Push file type
                if let Some(ftype) = fsentry.get_ftype() {
                    info.push(ListItem::new(Spans::from(vec![
                        Span::styled("File type: ", Style::default()),
                        Span::styled(
                            ftype,
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ])));
                }
                // Push size
                info.push(ListItem::new(Spans::from(vec![
                    Span::styled("Size: ", Style::default()),
                    Span::styled(
                        format!("{} ({})", bsize, size),
                        Style::default()
                            .fg(Color::LightBlue)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])));
                // Push creation time
                info.push(ListItem::new(Spans::from(vec![
                    Span::styled("Creation time: ", Style::default()),
                    Span::styled(
                        ctime,
                        Style::default()
                            .fg(Color::LightGreen)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])));
                // Push Last change
                info.push(ListItem::new(Spans::from(vec![
                    Span::styled("Last change time: ", Style::default()),
                    Span::styled(
                        mtime,
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                ])));
                // Push Last access
                info.push(ListItem::new(Spans::from(vec![
                    Span::styled("Last access time: ", Style::default()),
                    Span::styled(
                        atime,
                        Style::default()
                            .fg(Color::LightMagenta)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])));
                // User
                #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
                let username: String = match user {
                    Some(uid) => match get_user_by_uid(uid) {
                        Some(user) => user.name().to_string_lossy().to_string(),
                        None => uid.to_string(),
                    },
                    None => String::from("0"),
                };
                #[cfg(target_os = "windows")]
                let username: String = format!("{}", user.unwrap_or(0));
                info.push(ListItem::new(Spans::from(vec![
                    Span::styled("User: ", Style::default()),
                    Span::styled(
                        username,
                        Style::default()
                            .fg(Color::LightRed)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])));
                // Group
                #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
                let group: String = match group {
                    Some(gid) => match get_group_by_gid(gid) {
                        Some(group) => group.name().to_string_lossy().to_string(),
                        None => gid.to_string(),
                    },
                    None => String::from("0"),
                };
                #[cfg(target_os = "windows")]
                let group: String = format!("{}", group.unwrap_or(0));
                info.push(ListItem::new(Spans::from(vec![
                    Span::styled("Group: ", Style::default()),
                    Span::styled(
                        group,
                        Style::default()
                            .fg(Color::LightBlue)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])));
                // Finally return file name
                name
            }
            None => String::from(""),
        };
        List::new(info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default())
                    .title(file_name),
            )
            .start_corner(Corner::TopLeft)
    }

    /// ### draw_footer
    ///
    /// Draw authentication page footer
    pub(super) fn draw_popup_help(&self) -> List {
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
                Span::raw("disconnect"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<TAB>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("           "),
                Span::raw("Switch between log tab and explorer"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<BACKSPACE>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("     "),
                Span::raw("Go to previous directory in stack"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<RIGHT/LEFT>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("    "),
                Span::raw("change explorer tab"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<UP/DOWN>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("       "),
                Span::raw("move up/down in list"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<PGUP/PGDOWN>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("   "),
                Span::raw("scroll up/down in list quickly"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<ENTER>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("         "),
                Span::raw("enter directory"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<SPACE>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("         "),
                Span::raw("upload/download file"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<CANC>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("          "),
                Span::raw("delete file"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<D>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("             "),
                Span::raw("make directory"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<G>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("             "),
                Span::raw("goto path"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<H>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("             "),
                Span::raw("show help"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<I>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("             "),
                Span::raw("show info about the selected file or directory"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<Q>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("             "),
                Span::raw("Quit TermSCP"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<R>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("             "),
                Span::raw("rename file"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<U>",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("             "),
                Span::raw("go to parent directory"),
            ])),
        ];
        List::new(cmds)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default())
                    .title("Help"),
            )
            .start_corner(Corner::TopLeft)
    }

    /// align_text_center
    ///
    /// Align text to center for a given width
    fn align_text_center(text: &str, width: u16) -> String {
        let indent_size: usize = match (width as usize) >= text.len() {
            // NOTE: The check prevents underflow
            true => (width as usize - text.len()) / 2,
            false => 0,
        };
        textwrap::indent(
            text,
            (0..indent_size).map(|_| " ").collect::<String>().as_str(),
        )
    }

    /// ### elide_wrkdir_path
    ///
    /// Elide working directory path if longer than width + host.len
    /// In this case, the path is formatted to {ANCESTOR[0]}/.../{PARENT[0]}/{BASENAME}
    fn elide_wrkdir_path(wrkdir: &Path, host: &str, width: u16) -> PathBuf {
        let fmt_path: String = format!("{}", wrkdir.display());
        // NOTE: +5 is const
        match fmt_path.len() + host.len() + 5 > width as usize {
            false => PathBuf::from(wrkdir),
            true => {
                // Elide
                let ancestors_len: usize = wrkdir.ancestors().count();
                let mut ancestors = wrkdir.ancestors();
                let mut elided_path: PathBuf = PathBuf::new();
                // If ancestors_len's size is bigger than 2, push count - 2
                if ancestors_len > 2 {
                    elided_path.push(ancestors.nth(ancestors_len - 2).unwrap());
                }
                // If ancestors_len is bigger than 3, push '...' and parent too
                if ancestors_len > 3 {
                    elided_path.push("...");
                    if let Some(parent) = wrkdir.ancestors().nth(1) {
                        elided_path.push(parent.file_name().unwrap());
                    }
                }
                // Push file_name
                if let Some(name) = wrkdir.file_name() {
                    elided_path.push(name);
                }
                elided_path
            }
        }
    }
}
