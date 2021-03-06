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
use crate::ui::components::{
    bookmark_list::{BookmarkList, BookmarkListPropsBuilder},
    msgbox::{MsgBox, MsgBoxPropsBuilder},
};
use crate::utils::ui::draw_area_in;
// Ext
use tuirealm::components::{
    input::{Input, InputPropsBuilder},
    label::{Label, LabelPropsBuilder},
    radio::{Radio, RadioPropsBuilder},
    scrolltable::{ScrollTablePropsBuilder, Scrolltable},
    span::{Span, SpanPropsBuilder},
};
use tuirealm::tui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{BorderType, Borders, Clear},
};
use tuirealm::{
    props::{InputType, PropsBuilder, TableBuilder, TextSpan, TextSpanBuilder},
    Msg, Payload, Value,
};

impl AuthActivity {
    /// ### init
    ///
    /// Initialize view, mounting all startup components inside the view
    pub(super) fn init(&mut self) {
        // Headers
        self.view.mount(
            super::COMPONENT_TEXT_H1,
            Box::new(Label::new(
                LabelPropsBuilder::default()
                    .bold()
                    .italic()
                    .with_text(String::from("$ termscp"))
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_TEXT_H2,
            Box::new(Label::new(
                LabelPropsBuilder::default()
                    .bold()
                    .italic()
                    .with_text(format!("$ version {}", env!("CARGO_PKG_VERSION")))
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
                        TextSpanBuilder::new(" to show keybindings; ")
                            .bold()
                            .build(),
                        TextSpanBuilder::new("<CTRL+C>")
                            .bold()
                            .with_foreground(Color::Cyan)
                            .build(),
                        TextSpanBuilder::new(" to enter setup").bold().build(),
                    ])
                    .build(),
            )),
        );
        // Get default protocol
        let default_protocol: FileTransferProtocol =
            match self.context.as_ref().unwrap().config_client.as_ref() {
                Some(cli) => cli.get_default_protocol(),
                None => FileTransferProtocol::Sftp,
            };
        // Protocol
        self.view.mount(
            super::COMPONENT_RADIO_PROTOCOL,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::LightGreen)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightGreen)
                    .with_options(
                        Some(String::from("Protocol")),
                        vec![
                            String::from("SFTP"),
                            String::from("SCP"),
                            String::from("FTP"),
                            String::from("FTPS"),
                        ],
                    )
                    .with_value(Self::protocol_enum_to_opt(default_protocol))
                    .build(),
            )),
        );
        // Address
        self.view.mount(
            super::COMPONENT_INPUT_ADDR,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_foreground(Color::Yellow)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightYellow)
                    .with_label(String::from("Remote address"))
                    .build(),
            )),
        );
        // Port
        self.view.mount(
            super::COMPONENT_INPUT_PORT,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_foreground(Color::LightCyan)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightCyan)
                    .with_label(String::from("Port number"))
                    .with_input(InputType::Number)
                    .with_input_len(5)
                    .with_value(Self::get_default_port_for_protocol(default_protocol).to_string())
                    .build(),
            )),
        );
        // Username
        self.view.mount(
            super::COMPONENT_INPUT_USERNAME,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_foreground(Color::LightMagenta)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightMagenta)
                    .with_label(String::from("Username"))
                    .build(),
            )),
        );
        // Password
        self.view.mount(
            super::COMPONENT_INPUT_PASSWORD,
            Box::new(Input::new(
                InputPropsBuilder::default()
                    .with_foreground(Color::LightBlue)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightBlue)
                    .with_label(String::from("Password"))
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
                Box::new(Span::new(
                        SpanPropsBuilder::default()
                        .with_foreground(Color::Yellow)
                        .with_spans(
                            vec![
                                TextSpan::from("termscp "),
                                TextSpanBuilder::new(version).underlined().bold().build(),
                                TextSpan::from(" is now available! Download it from <https://github.com/veeso/termscp/releases/latest>")
                            ]
                        )
                        .build()
                ))
            );
        }
        // Bookmarks
        self.view.mount(
            super::COMPONENT_BOOKMARKS_LIST,
            Box::new(BookmarkList::new(
                BookmarkListPropsBuilder::default()
                    .with_background(Color::LightGreen)
                    .with_foreground(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Plain, Color::LightGreen)
                    .with_bookmarks(Some(String::from("Bookmarks")), vec![])
                    .build(),
            )),
        );
        let _ = self.view_bookmarks();
        // Recents
        self.view.mount(
            super::COMPONENT_RECENTS_LIST,
            Box::new(BookmarkList::new(
                BookmarkListPropsBuilder::default()
                    .with_background(Color::LightBlue)
                    .with_foreground(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Plain, Color::LightBlue)
                    .with_bookmarks(Some(String::from("Recent connections")), vec![])
                    .build(),
            )),
        );
        let _ = self.view_recent_connections();
        // Active protocol
        self.view.active(super::COMPONENT_RADIO_PROTOCOL);
    }

    /// ### view
    ///
    /// Display view on canvas
    pub(super) fn view(&mut self) {
        let mut ctx: Context = self.context.take().unwrap();
        let _ = ctx.terminal.draw(|f| {
            // Check window size
            let height: u16 = f.size().height;
            self.check_minimum_window_size(height);
            // Prepare chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(21), // Auth Form
                        Constraint::Min(3),     // Bookmarks
                    ]
                    .as_ref(),
                )
                .split(f.size());
            // Create explorer chunks
            let auth_chunks = Layout::default()
                .constraints(
                    [
                        Constraint::Length(1), // h1
                        Constraint::Length(1), // h2
                        Constraint::Length(1), // Version
                        Constraint::Length(3), // protocol
                        Constraint::Length(3), // host
                        Constraint::Length(3), // port
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
                .render(super::COMPONENT_TEXT_H1, f, auth_chunks[0]);
            self.view
                .render(super::COMPONENT_TEXT_H2, f, auth_chunks[1]);
            self.view
                .render(super::COMPONENT_TEXT_NEW_VERSION, f, auth_chunks[2]);
            self.view
                .render(super::COMPONENT_RADIO_PROTOCOL, f, auth_chunks[3]);
            self.view
                .render(super::COMPONENT_INPUT_ADDR, f, auth_chunks[4]);
            self.view
                .render(super::COMPONENT_INPUT_PORT, f, auth_chunks[5]);
            self.view
                .render(super::COMPONENT_INPUT_USERNAME, f, auth_chunks[6]);
            self.view
                .render(super::COMPONENT_INPUT_PASSWORD, f, auth_chunks[7]);
            self.view
                .render(super::COMPONENT_TEXT_FOOTER, f, auth_chunks[8]);
            // Bookmark chunks
            self.view
                .render(super::COMPONENT_BOOKMARKS_LIST, f, bookmark_chunks[0]);
            self.view
                .render(super::COMPONENT_RECENTS_LIST, f, bookmark_chunks[1]);
            // Popups
            if let Some(props) = self.view.get_props(super::COMPONENT_TEXT_ERROR) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 50, 10);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_TEXT_ERROR, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_TEXT_SIZE_ERR) {
                if props.visible {
                    let popup = draw_area_in(f.size(), 80, 20);
                    f.render_widget(Clear, popup);
                    // make popup
                    self.view.render(super::COMPONENT_TEXT_SIZE_ERR, f, popup);
                }
            }
            if let Some(props) = self.view.get_props(super::COMPONENT_RADIO_QUIT) {
                if props.visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 30, 10);
                    f.render_widget(Clear, popup);
                    self.view.render(super::COMPONENT_RADIO_QUIT, f, popup);
                }
            }
            if let Some(props) = self
                .view
                .get_props(super::COMPONENT_RADIO_BOOKMARK_DEL_BOOKMARK)
            {
                if props.visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 30, 10);
                    f.render_widget(Clear, popup);
                    self.view
                        .render(super::COMPONENT_RADIO_BOOKMARK_DEL_BOOKMARK, f, popup);
                }
            }
            if let Some(props) = self
                .view
                .get_props(super::COMPONENT_RADIO_BOOKMARK_DEL_RECENT)
            {
                if props.visible {
                    // make popup
                    let popup = draw_area_in(f.size(), 30, 10);
                    f.render_widget(Clear, popup);
                    self.view
                        .render(super::COMPONENT_RADIO_BOOKMARK_DEL_RECENT, f, popup);
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
            if let Some(props) = self
                .view
                .get_props(super::COMPONENT_RADIO_BOOKMARK_SAVE_PWD)
            {
                if props.visible {
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
        let bookmarks: Vec<String> = self
            .bookmarks_list
            .iter()
            .map(|x| {
                let entry: (String, u16, FileTransferProtocol, String, _) = self
                    .bookmarks_client
                    .as_ref()
                    .unwrap()
                    .get_bookmark(x)
                    .unwrap();
                format!(
                    "{} ({}://{}@{}:{})",
                    x,
                    entry.2.to_string().to_lowercase(),
                    entry.3,
                    entry.0,
                    entry.1
                )
            })
            .collect();
        match self.view.get_props(super::COMPONENT_BOOKMARKS_LIST) {
            None => None,
            Some(props) => {
                let msg = self.view.update(
                    super::COMPONENT_BOOKMARKS_LIST,
                    BookmarkListPropsBuilder::from(props)
                        .with_bookmarks(Some(String::from("Bookmarks")), bookmarks)
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
        let bookmarks: Vec<String> = self
            .recents_list
            .iter()
            .map(|x| {
                let entry: (String, u16, FileTransferProtocol, String) = self
                    .bookmarks_client
                    .as_ref()
                    .unwrap()
                    .get_recent(x)
                    .unwrap();

                format!(
                    "{}://{}@{}:{}",
                    entry.2.to_string().to_lowercase(),
                    entry.3,
                    entry.0,
                    entry.1
                )
            })
            .collect();
        match self.view.get_props(super::COMPONENT_RECENTS_LIST) {
            None => None,
            Some(props) => {
                let msg = self.view.update(
                    super::COMPONENT_RECENTS_LIST,
                    BookmarkListPropsBuilder::from(props)
                        .with_bookmarks(Some(String::from("Recent connections")), bookmarks)
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
                MsgBoxPropsBuilder::default()
                    .with_foreground(Color::Red)
                    .with_borders(Borders::ALL, BorderType::Thick, Color::Red)
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

    /// ### mount_size_err
    ///
    /// Mount size error
    pub(super) fn mount_size_err(&mut self) {
        // Mount
        self.view.mount(
            super::COMPONENT_TEXT_SIZE_ERR,
            Box::new(MsgBox::new(
                MsgBoxPropsBuilder::default()
                    .with_foreground(Color::Red)
                    .with_borders(Borders::ALL, BorderType::Thick, Color::Red)
                    .bold()
                    .with_texts(
                        None,
                        vec![TextSpan::from(
                            "termscp requires at least 24 lines of height to run",
                        )],
                    )
                    .build(),
            )),
        );
        // Give focus to error
        self.view.active(super::COMPONENT_TEXT_SIZE_ERR);
    }

    /// ### umount_size_err
    ///
    /// Umount error size error
    pub(super) fn umount_size_err(&mut self) {
        self.view.umount(super::COMPONENT_TEXT_SIZE_ERR);
    }

    /// ### mount_quit
    ///
    /// Mount quit popup
    pub(super) fn mount_quit(&mut self) {
        // Protocol
        self.view.mount(
            super::COMPONENT_RADIO_QUIT,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::Yellow)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::Yellow)
                    .with_inverted_color(Color::Black)
                    .with_options(
                        Some(String::from("Quit termscp?")),
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

    /// ### mount_bookmark_del_dialog
    ///
    /// Mount bookmark delete dialog
    pub(super) fn mount_bookmark_del_dialog(&mut self) {
        self.view.mount(
            super::COMPONENT_RADIO_BOOKMARK_DEL_BOOKMARK,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::Yellow)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::Yellow)
                    .with_options(
                        Some(String::from("Delete bookmark?")),
                        vec![String::from("Yes"), String::from("No")],
                    )
                    .with_value(1)
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
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::Yellow)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::Yellow)
                    .with_options(
                        Some(String::from("Delete bookmark?")),
                        vec![String::from("Yes"), String::from("No")],
                    )
                    .with_value(1)
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
                InputPropsBuilder::default()
                    .with_foreground(Color::LightCyan)
                    .with_label(String::from("Save bookmark as..."))
                    .with_borders(
                        Borders::TOP | Borders::RIGHT | Borders::LEFT,
                        BorderType::Rounded,
                        Color::Reset,
                    )
                    .build(),
            )),
        );
        self.view.mount(
            super::COMPONENT_RADIO_BOOKMARK_SAVE_PWD,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::Red)
                    .with_borders(
                        Borders::BOTTOM | Borders::RIGHT | Borders::LEFT,
                        BorderType::Rounded,
                        Color::Reset,
                    )
                    .with_options(
                        Some(String::from("Save password?")),
                        vec![String::from("Yes"), String::from("No")],
                    )
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
                                    .with_foreground(Color::Cyan)
                                    .build(),
                            )
                            .add_col(TextSpan::from("           Quit termscp"))
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

    /// ### get_input
    ///
    /// Collect input values from view
    pub(super) fn get_input(&self) -> (String, u16, FileTransferProtocol, String, String) {
        let addr: String = self.get_input_addr();
        let port: u16 = self.get_input_port();
        let protocol: FileTransferProtocol = self.get_input_protocol();
        let username: String = self.get_input_username();
        let password: String = self.get_input_password();
        (addr, port, protocol, username, password)
    }

    pub(super) fn get_input_addr(&self) -> String {
        match self.view.get_state(super::COMPONENT_INPUT_ADDR) {
            Some(Payload::One(Value::Str(x))) => x,
            _ => String::new(),
        }
    }

    pub(super) fn get_input_port(&self) -> u16 {
        match self.view.get_state(super::COMPONENT_INPUT_PORT) {
            Some(Payload::One(Value::Usize(x))) => x as u16,
            _ => Self::get_default_port_for_protocol(FileTransferProtocol::Sftp),
        }
    }

    pub(super) fn get_input_protocol(&self) -> FileTransferProtocol {
        match self.view.get_state(super::COMPONENT_RADIO_PROTOCOL) {
            Some(Payload::One(Value::Usize(x))) => Self::protocol_opt_to_enum(x),
            _ => FileTransferProtocol::Sftp,
        }
    }

    pub(super) fn get_input_username(&self) -> String {
        match self.view.get_state(super::COMPONENT_INPUT_USERNAME) {
            Some(Payload::One(Value::Str(x))) => x,
            _ => String::new(),
        }
    }

    pub(super) fn get_input_password(&self) -> String {
        match self.view.get_state(super::COMPONENT_INPUT_PASSWORD) {
            Some(Payload::One(Value::Str(x))) => x,
            _ => String::new(),
        }
    }
}
