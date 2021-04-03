//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

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
use super::{AuthActivity, Context, FileTransferProtocol};
use crate::ui::layout::components::{
    bookmark_list::BookmarkList, input::Input, msgbox::MsgBox, radio_group::RadioGroup,
    table::Table, text::Text, title::Title,
};
use crate::ui::layout::props::{
    InputType, PropValue, PropsBuilder, TableBuilder, TextParts, TextSpan, TextSpanBuilder,
};
use crate::ui::layout::utils::draw_area_in;
use crate::ui::layout::{Msg, Payload};
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
            Title::new(
                PropsBuilder::default().with_foreground(Color::White).with_texts(
                    TextParts::new(Some(String::from(" _____                   ____   ____ ____  \n|_   _|__ _ __ _ __ ___ / ___| / ___|  _ \\ \n  | |/ _ \\ '__| '_ ` _ \\\\___ \\| |   | |_) |\n  | |  __/ |  | | | | | |___) | |___|  __/ \n  |_|\\___|_|  |_| |_| |_|____/ \\____|_|    \n")), None)
                ).bold().build()
            )
        ));
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
                    .with_value(PropValue::Str(String::from("22")))
                    .build(),
            )),
        );
        // Protocol
        self.view.mount(
            super::COMPONENT_RADIO_PROTOCOL,
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::LightGreen)
                    .with_background(Color::Black)
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
        if let Some(version) = self
            .context
            .as_ref()
            .unwrap()
            .store
            .get_string(super::STORE_KEY_LATEST_VERSION)
        {
            self.view.mount(
                super::COMPONENT_TEXT_NEW_VERSION,
                Box::new(Text::new(
                        PropsBuilder::default()
                        .with_foreground(Color::Yellow)
                        .with_texts(TextParts::new(None, Some(vec![TextSpan::from(format!("TermSCP {} is now available! Download it from <https://github.com/veeso/termscp/releases/latest>", version))])))
                        .build()
                ))
            );
        }
        // Bookmarks
        self.view.mount(
            super::COMPONENT_BOOKMARKS_LIST,
            Box::new(BookmarkList::new(
                PropsBuilder::default()
                    .with_background(Color::LightGreen)
                    .with_foreground(Color::Black)
                    .with_texts(TextParts::new(Some(String::from("Bookmarks")), None))
                    .build(),
            )),
        );
        let _ = self.view_bookmarks();
        // Recents
        self.view.mount(
            super::COMPONENT_RECENTS_LIST,
            Box::new(BookmarkList::new(
                PropsBuilder::default()
                    .with_background(Color::LightBlue)
                    .with_foreground(Color::Black)
                    .with_texts(TextParts::new(
                        Some(String::from("Recent connections")),
                        None,
                    ))
                    .build(),
            )),
        );
        let _ = self.view_recent_connections();
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
                        Constraint::Length(6), // header
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
                    let popup_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Length(3), // Input form
                                Constraint::Length(2), // Yes/No
                            ]
                            .as_ref(),
                        )
                        .split(popup);
                    self.view
                        .render(super::COMPONENT_INPUT_BOOKMARK_NAME, f, popup_chunks[0]);
                    self.view
                        .render(super::COMPONENT_RADIO_BOOKMARK_SAVE_PWD, f, popup_chunks[1]);
                }
            }
        });
        self.context = Some(ctx);
    }

    // -- partials

    /// ### view_bookmarks
    ///
    /// Make text span from bookmarks
    pub(super) fn view_bookmarks(&mut self) -> Option<(String, Msg)> {
        let bookmarks: Vec<TextSpan> = self
            .bookmarks_list
            .iter()
            .map(|x| {
                let entry: (String, u16, FileTransferProtocol, String, _) = self
                    .bookmarks_client
                    .as_ref()
                    .unwrap()
                    .get_bookmark(x)
                    .unwrap();
                TextSpan::from(
                    format!(
                        "{} ({}://{}@{}:{})",
                        x,
                        entry.2.to_string().to_lowercase(),
                        entry.3,
                        entry.0,
                        entry.1
                    )
                    .as_str(),
                )
            })
            .collect();
        match self
            .view
            .get_props(super::COMPONENT_BOOKMARKS_LIST)
            .as_mut()
        {
            None => None,
            Some(props) => {
                let msg = self.view.update(
                    super::COMPONENT_BOOKMARKS_LIST,
                    props
                        .with_texts(TextParts::new(
                            Some(String::from("Bookmarks")),
                            Some(bookmarks),
                        ))
                        .build(),
                );
                msg
            }
        }
    }

    /// ### view_recent_connections
    ///
    /// View recent connections
    pub(super) fn view_recent_connections(&mut self) -> Option<(String, Msg)> {
        let bookmarks: Vec<TextSpan> = self
            .recents_list
            .iter()
            .map(|x| {
                let entry: (String, u16, FileTransferProtocol, String) = self
                    .bookmarks_client
                    .as_ref()
                    .unwrap()
                    .get_recent(x)
                    .unwrap();
                TextSpan::from(
                    format!(
                        "{}://{}@{}:{}",
                        entry.2.to_string().to_lowercase(),
                        entry.3,
                        entry.0,
                        entry.1
                    )
                    .as_str(),
                )
            })
            .collect();
        match self.view.get_props(super::COMPONENT_RECENTS_LIST).as_mut() {
            None => None,
            Some(props) => {
                let msg = self.view.update(
                    super::COMPONENT_RECENTS_LIST,
                    props
                        .with_texts(TextParts::new(
                            Some(String::from("Recent connections")),
                            Some(bookmarks),
                        ))
                        .build(),
                );
                msg
            }
        }
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
                    .with_background(Color::Black)
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
                    .with_background(Color::Black)
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
                    .with_background(Color::Black)
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
                    .with_foreground(Color::LightCyan)
                    .with_texts(TextParts::new(
                        Some(String::from("Save bookmark as...")),
                        None,
                    ))
                    //.with_borders(Borders::TOP | Borders::RIGHT | Borders::LEFT)
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_RADIO_BOOKMARK_SAVE_PWD,
            Box::new(RadioGroup::new(
                PropsBuilder::default()
                    .with_foreground(Color::Red)
                    //.with_borders(Borders::BOTTOM | Borders::RIGHT | Borders::LEFT)
                    .with_texts(TextParts::new(
                        Some(String::from("Save password?")),
                        Some(vec![TextSpan::from("Yes"), TextSpan::from("No")]),
                    ))
                    //.with_value(PropValue::Unsigned(1))
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
