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
use super::{AuthActivity, Context, FileTransferProtocol};
use crate::ui::layout::components::{
    bookmark_list::BookmarkList, ctext::CText, input::Input, radio_group::RadioGroup, table::Table,
    text::Text,
};
use crate::ui::layout::props::{
    InputType, PropValue, PropsBuilder, TableBuilder, TextParts, TextSpan, TextSpanBuilder,
};
use crate::ui::layout::utils::draw_area_in;
use crate::ui::layout::Payload;
// Ext
use tui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::Clear,
};

impl AuthActivity {
    /// ### init
    ///
    /// Initialize view, mounting all startup components inside the view
    pub(super) fn init(&mut self) {
        // Header
        self.view.mount(super::COMPONENT_TEXT_HEADER, Box::new(
            Text::new(
                PropsBuilder::default().with_foreground(Color::White).with_texts(
                    TextParts::new(None, Some(vec![TextSpan::from(" _____                   ____   ____ ____  \n|_   _|__ _ __ _ __ ___ / ___| / ___|  _ \\ \n  | |/ _ \\ '__| '_ ` _ \\\\___ \\| |   | |_) |\n  | |  __/ |  | | | | | |___) | |___|  __/ \n  |_|\\___|_|  |_| |_| |_|____/ \\____|_|    \n")]))
                ).bold().build()
            )
        ));
        // Footer
        self.view.mount(
            super::COMPONENT_TEXT_FOOTER,
            Box::new(Text::new(
                PropsBuilder::default()
                    .with_foreground(Color::White)
                    .with_texts(TextParts::new(
                        None,
                        Some(vec![
                            TextSpanBuilder::new("Press ").bold().build(),
                            TextSpanBuilder::new("<CTRL+H>")
                                .bold()
                                .with_foreground(Color::Cyan)
                                .build(),
                            TextSpanBuilder::new(" to show keybindings; ")
                                .bold()
                                .build(),
                            TextSpanBuilder::new("<CTRL+C>")
                                .bold()
                                .with_foreground(Color::Cyan)
                                .build(),
                            TextSpanBuilder::new(" to enter setup").bold().build(),
                        ]),
                    ))
                    .build(),
            )),
        );
        // Address
        self.view.mount(
            super::COMPONENT_INPUT_ADDR,
            Box::new(Input::new(
                PropsBuilder::default()
                    .with_foreground(Color::Yellow)
                    .with_texts(TextParts::new(Some(String::from("Remote address")), None))
                    .build(),
            )),
        );
        // Port
        self.view.mount(
            super::COMPONENT_INPUT_PORT,
            Box::new(Input::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightCyan)
                    .with_texts(TextParts::new(Some(String::from("Port number")), None))
                    .with_input(InputType::Number)
                    .with_input_len(5)
                    .with_value(PropValue::Unsigned(22))
                    .build(),
            )),
        );
        // Protocol
        self.view.mount(
            super::COMPONENT_RADIO_PROTOCOL,
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightGreen)
                    .with_texts(TextParts::new(
                        Some(String::from("Protocol")),
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
        // Username
        self.view.mount(
            super::COMPONENT_INPUT_USERNAME,
            Box::new(Input::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightMagenta)
                    .with_texts(TextParts::new(Some(String::from("Username")), None))
                    .build(),
            )),
        );
        // Password
        self.view.mount(
            super::COMPONENT_INPUT_PASSWORD,
            Box::new(Input::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightBlue)
                    .with_texts(TextParts::new(Some(String::from("Password")), None))
                    .with_input(InputType::Password)
                    .build(),
            )),
        );
        // Version notice
        if let Some(version) = self.new_version.as_ref() {
            self.view.mount(
                super::COMPONENT_TEXT_NEW_VERSION,
                Box::new(Text::new(
                    PropsBuilder::default()
                        .with_foreground(Color::Yellow)
                        .with_texts(TextParts::new(None, Some(vec![TextSpan::from(format!("TermSCP {} is now available! Download it from <https://github.com/veeso/termscp/releases/latest>", version).as_str())])))
                        .bold()
                        .build()
                ))
            );
        }
        // Bookmarks
        self.view.mount(
            super::COMPONENT_BOOKMARKS_LIST,
            Box::new(BookmarkList::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightGreen)
                    .with_texts(TextParts::new(
                        Some(String::from("Bookmarks")),
                        Some(self.view_bookmarks()),
                    ))
                    .build(),
            )),
        );
        // Recents
        self.view.mount(
            super::COMPONENT_RECENTS_LIST,
            Box::new(BookmarkList::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightBlue)
                    .with_texts(TextParts::new(
                        Some(String::from("Recent connections")),
                        Some(self.view_recent_connections()),
                    ))
                    .build(),
            )),
        );
        // Active address
        self.view.active(super::COMPONENT_INPUT_ADDR);
    }

    /// ### view
    ///
    /// Display view on canvas
    pub(super) fn view(&mut self) {
        let mut ctx: Context = self.context.take().unwrap();
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
                        Constraint::Length(5), // header
                        Constraint::Length(1), // Version
                        Constraint::Length(3), // host
                        Constraint::Length(3), // port
                        Constraint::Length(3), // protocol
                        Constraint::Length(3), // username
                        Constraint::Length(3), // password
                        Constraint::Length(3), // footer
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
            // Render
            // Auth chunks
            self.view
                .render(super::COMPONENT_TEXT_HEADER, f, auth_chunks[0]);
            self.view
                .render(super::COMPONENT_TEXT_NEW_VERSION, f, auth_chunks[1]);
            self.view
                .render(super::COMPONENT_INPUT_ADDR, f, auth_chunks[2]);
            self.view
                .render(super::COMPONENT_INPUT_PORT, f, auth_chunks[3]);
            self.view
                .render(super::COMPONENT_RADIO_PROTOCOL, f, auth_chunks[4]);
            self.view
                .render(super::COMPONENT_INPUT_USERNAME, f, auth_chunks[5]);
            self.view
                .render(super::COMPONENT_INPUT_PASSWORD, f, auth_chunks[6]);
            self.view
                .render(super::COMPONENT_TEXT_FOOTER, f, auth_chunks[7]);
            // Bookmark chunks
            self.view
                .render(super::COMPONENT_BOOKMARKS_LIST, f, bookmark_chunks[0]);
            self.view
                .render(super::COMPONENT_RECENTS_LIST, f, bookmark_chunks[1]);
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
                    let popup = draw_area_in(f.size(), 30, 10);
                    f.render_widget(Clear, popup);
                    self.view.render(super::COMPONENT_RADIO_QUIT, f, popup);
                }
            }
            if let Some(mut props) = self
                .view
                .get_props(super::COMPONENT_RADIO_BOOKMARK_DEL_BOOKMARK)
            {
                if props.build().visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 30, 10);
                    f.render_widget(Clear, popup);
                    self.view
                        .render(super::COMPONENT_RADIO_BOOKMARK_DEL_BOOKMARK, f, popup);
                }
            }
            if let Some(mut props) = self
                .view
                .get_props(super::COMPONENT_RADIO_BOOKMARK_DEL_RECENT)
            {
                if props.build().visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 30, 10);
                    f.render_widget(Clear, popup);
                    self.view
                        .render(super::COMPONENT_RADIO_BOOKMARK_DEL_RECENT, f, popup);
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
            if let Some(mut props) = self
                .view
                .get_props(super::COMPONENT_RADIO_BOOKMARK_SAVE_PWD)
            {
                if props.build().visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 20, 20);
                    f.render_widget(Clear, popup);
                    self.view
                        .render(super::COMPONENT_INPUT_BOOKMARK_NAME, f, popup);
                    self.view
                        .render(super::COMPONENT_RADIO_BOOKMARK_SAVE_PWD, f, popup);
                }
            }
        });
        self.context = Some(ctx);
    }

    // -- partials

    /// ### view_bookmarks
    ///
    /// Make text span from bookmarks
    pub(super) fn view_bookmarks(&self) -> Vec<TextSpan> {
        self.bookmarks_list
            .iter()
            .map(|x| TextSpan::from(x.as_str()))
            .collect()
    }

    /// ### view_recent_connections
    ///
    /// View recent connections
    pub(super) fn view_recent_connections(&self) -> Vec<TextSpan> {
        self.recents_list
            .iter()
            .map(|x| TextSpan::from(x.as_str()))
            .collect()
    }

    // -- mount

    /// ### mount_error
    ///
    /// Mount error box
    pub(super) fn mount_error(&mut self, text: &str) {
        // Mount
        self.view.mount(
            super::COMPONENT_TEXT_ERROR,
            Box::new(CText::new(
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

    /// ### mount_quit
    ///
    /// Mount quit popup
    pub(super) fn mount_quit(&mut self) {
        // Protocol
        self.view.mount(
            super::COMPONENT_RADIO_QUIT,
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::Yellow)
                    .with_texts(TextParts::new(
                        Some(String::from("Quit TermSCP?")),
                        Some(vec![TextSpan::from("Yes"), TextSpan::from("No")]),
                    ))
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

    /// ### mount_bookmark_del_dialog
    ///
    /// Mount bookmark delete dialog
    pub(super) fn mount_bookmark_del_dialog(&mut self) {
        self.view.mount(
            super::COMPONENT_RADIO_BOOKMARK_DEL_BOOKMARK,
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::Yellow)
                    .with_texts(TextParts::new(
                        Some(String::from("Delete bookmark?")),
                        Some(vec![TextSpan::from("Yes"), TextSpan::from("No")]),
                    ))
                    .with_value(PropValue::Unsigned(1))
                    .build(),
            )),
        );
        // Active
        self.view
            .active(super::COMPONENT_RADIO_BOOKMARK_DEL_BOOKMARK);
    }

    /// ### umount_bookmark_del_dialog
    ///
    /// umount delete bookmark dialog
    pub(super) fn umount_bookmark_del_dialog(&mut self) {
        self.view
            .umount(super::COMPONENT_RADIO_BOOKMARK_DEL_BOOKMARK);
    }

    /// ### mount_bookmark_del_dialog
    ///
    /// Mount recent delete dialog
    pub(super) fn mount_recent_del_dialog(&mut self) {
        self.view.mount(
            super::COMPONENT_RADIO_BOOKMARK_DEL_RECENT,
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::Yellow)
                    .with_texts(TextParts::new(
                        Some(String::from("Delete bookmark?")),
                        Some(vec![TextSpan::from("Yes"), TextSpan::from("No")]),
                    ))
                    .with_value(PropValue::Unsigned(1))
                    .build(),
            )),
        );
        // Active
        self.view.active(super::COMPONENT_RADIO_BOOKMARK_DEL_RECENT);
    }

    /// ### umount_recent_del_dialog
    ///
    /// umount delete recent dialog
    pub(super) fn umount_recent_del_dialog(&mut self) {
        self.view.umount(super::COMPONENT_RADIO_BOOKMARK_DEL_RECENT);
    }

    /// ### mount_bookmark_save_dialog
    ///
    /// Mount bookmark save dialog
    pub(super) fn mount_bookmark_save_dialog(&mut self) {
        self.view.mount(
            super::COMPONENT_INPUT_BOOKMARK_NAME,
            Box::new(Input::new(
                PropsBuilder::default()
                    .with_texts(TextParts::new(
                        Some(String::from("Save bookmark as...")),
                        None,
                    ))
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_RADIO_BOOKMARK_SAVE_PWD,
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::Red)
                    .with_texts(TextParts::new(
                        Some(String::from("Save password?")),
                        Some(vec![TextSpan::from("Yes"), TextSpan::from("No")]),
                    ))
                    .with_value(PropValue::Unsigned(1))
                    .build(),
            )),
        );
        // Give focus to input bookmark name
        self.view.active(super::COMPONENT_INPUT_BOOKMARK_NAME);
    }

    /// ### umount_bookmark_save_dialog
    ///
    /// Umount bookmark save dialog
    pub(super) fn umount_bookmark_save_dialog(&mut self) {
        self.view.umount(super::COMPONENT_RADIO_BOOKMARK_SAVE_PWD);
        self.view.umount(super::COMPONENT_INPUT_BOOKMARK_NAME);
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
                            .add_col(TextSpan::from("           Quit TermSCP"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<TAB>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("           Switch from form and bookmarks"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<RIGHT/LEFT>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("    Switch bookmark tab"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<UP/DOWN>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("       Move up/down in current tab"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<ENTER>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("         Connect/Load bookmark"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<DEL|E>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("         Delete selected bookmark"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<CTRL+C>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("        Enter setup"))
                            .add_row()
                            .add_col(
                                TextSpanBuilder::new("<CTRL+S>")
                                    .bold()
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("        Save bookmark"))
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

    /// ### get_input
    ///
    /// Collect input values from view
    pub(super) fn get_input(&self) -> (String, u16, FileTransferProtocol, String, String) {
        let addr: String = match self.view.get_value(super::COMPONENT_INPUT_ADDR) {
            Some(Payload::Text(a)) => a,
            _ => String::new(),
        };
        let port: u16 = match self.view.get_value(super::COMPONENT_INPUT_PORT) {
            Some(Payload::Unsigned(p)) => p as u16,
            _ => 0,
        };
        let protocol: FileTransferProtocol =
            match self.view.get_value(super::COMPONENT_RADIO_PROTOCOL) {
                Some(Payload::Unsigned(p)) => match p {
                    1 => FileTransferProtocol::Scp,
                    2 => FileTransferProtocol::Ftp(false),
                    3 => FileTransferProtocol::Ftp(true),
                    _ => FileTransferProtocol::Sftp,
                },
                _ => FileTransferProtocol::Sftp,
            };
        let username: String = match self.view.get_value(super::COMPONENT_INPUT_USERNAME) {
            Some(Payload::Text(a)) => a,
            _ => String::new(),
        };
        let password: String = match self.view.get_value(super::COMPONENT_INPUT_PASSWORD) {
            Some(Payload::Text(a)) => a,
            _ => String::new(),
        };
        (addr, port, protocol, username, password)
    }
}
