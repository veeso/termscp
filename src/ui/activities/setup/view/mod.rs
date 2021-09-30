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
pub mod setup;
pub mod ssh_keys;
pub mod theme;

use super::*;
pub use setup::*;
pub use ssh_keys::*;
pub use theme::*;
// Ext
use tui_realm_stdlib::{
    Input, InputPropsBuilder, List, ListPropsBuilder, Paragraph, ParagraphPropsBuilder, Radio,
    RadioPropsBuilder, Span, SpanPropsBuilder,
};
use tuirealm::props::{Alignment, InputType, PropsBuilder, TableBuilder, TextSpan};
use tuirealm::tui::{
    style::Color,
    widgets::{BorderType, Borders},
};

impl SetupActivity {
    // -- view

    pub(super) fn init(&mut self, layout: ViewLayout) {
        self.layout = layout;
        match self.layout {
            ViewLayout::SetupForm => self.init_setup(),
            ViewLayout::SshKeys => self.init_ssh_keys(),
            ViewLayout::Theme => self.init_theme(),
        }
    }

    /// ### view
    ///
    /// View gui
    pub(super) fn view(&mut self) {
        match self.layout {
            ViewLayout::SetupForm => self.view_setup(),
            ViewLayout::SshKeys => self.view_ssh_keys(),
            ViewLayout::Theme => self.view_theme(),
        }
    }

    // -- mount

    /// ### mount_error
    ///
    /// Mount error box
    pub(super) fn mount_error(&mut self, text: &str) {
        self.mount_text_dialog(super::COMPONENT_TEXT_ERROR, text, Color::Red);
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
        self.mount_radio_dialog(
            super::COMPONENT_RADIO_QUIT,
            "There are unsaved changes! Save changes before leaving?",
            &["Save", "Don't save", "Cancel"],
            0,
            Color::LightRed,
        );
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
        self.mount_radio_dialog(
            super::COMPONENT_RADIO_SAVE,
            "Save changes?",
            &["Yes", "No"],
            0,
            Color::LightYellow,
        );
    }

    /// ### umount_quit
    ///
    /// Umount quit
    pub(super) fn umount_save_popup(&mut self) {
        self.view.umount(super::COMPONENT_RADIO_SAVE);
    }

    pub(self) fn mount_header_tab(&mut self, idx: usize) {
        self.view.mount(
            super::COMPONENT_RADIO_TAB,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(Color::LightYellow)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::BOTTOM, BorderType::Thick, Color::White)
                    .with_options(&[
                        String::from("User Interface"),
                        String::from("SSH Keys"),
                        String::from("Theme"),
                    ])
                    .with_value(idx)
                    .rewind(true)
                    .build(),
            )),
        );
    }

    pub(self) fn mount_footer(&mut self) {
        self.view.mount(
            super::COMPONENT_TEXT_FOOTER,
            Box::new(Span::new(
                SpanPropsBuilder::default()
                    .with_spans(vec![
                        TextSpan::new("Press ").bold(),
                        TextSpan::new("<CTRL+H>").bold().fg(Color::Cyan),
                        TextSpan::new(" to show keybindings").bold(),
                    ])
                    .build(),
            )),
        );
    }

    /// ### mount_help
    ///
    /// Mount help
    pub(super) fn mount_help(&mut self) {
        self.view.mount(
            super::COMPONENT_TEXT_HELP,
            Box::new(List::new(
                ListPropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::White)
                    .with_highlighted_str(Some("?"))
                    .with_max_scroll_step(8)
                    .bold()
                    .with_title("Help", Alignment::Center)
                    .scrollable(true)
                    .with_rows(
                        TableBuilder::default()
                            .add_col(TextSpan::new("<ESC>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("           Exit setup"))
                            .add_row()
                            .add_col(TextSpan::new("<TAB>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("           Change setup page"))
                            .add_row()
                            .add_col(TextSpan::new("<RIGHT/LEFT>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("    Change cursor"))
                            .add_row()
                            .add_col(TextSpan::new("<UP/DOWN>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("       Change input field"))
                            .add_row()
                            .add_col(TextSpan::new("<ENTER>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("         Select / Dismiss popup"))
                            .add_row()
                            .add_col(TextSpan::new("<DEL|E>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("         Delete SSH key"))
                            .add_row()
                            .add_col(TextSpan::new("<CTRL+N>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("        New SSH key"))
                            .add_row()
                            .add_col(TextSpan::new("<CTRL+R>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("        Revert changes"))
                            .add_row()
                            .add_col(TextSpan::new("<CTRL+S>").bold().fg(Color::Cyan))
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

    // -- mount helpers

    fn mount_text_dialog(&mut self, id: &str, text: &str, color: Color) {
        // Mount
        self.view.mount(
            id,
            Box::new(Paragraph::new(
                ParagraphPropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Thick, color)
                    .with_foreground(color)
                    .bold()
                    .with_text_alignment(Alignment::Center)
                    .with_texts(vec![TextSpan::from(text)])
                    .build(),
            )),
        );
        // Give focus to error
        self.view.active(id);
    }

    fn mount_radio_dialog(
        &mut self,
        id: &str,
        text: &str,
        opts: &[&str],
        default: usize,
        color: Color,
    ) {
        self.view.mount(
            id,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(color)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, color)
                    .with_title(text, Alignment::Center)
                    .with_options(opts)
                    .with_value(default)
                    .rewind(true)
                    .build(),
            )),
        );
        // Active
        self.view.active(id);
    }

    fn mount_radio(&mut self, id: &str, text: &str, opts: &[&str], default: usize, color: Color) {
        self.view.mount(
            id,
            Box::new(Radio::new(
                RadioPropsBuilder::default()
                    .with_color(color)
                    .with_inverted_color(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, color)
                    .with_title(text, Alignment::Left)
                    .with_options(opts)
                    .with_value(default)
                    .rewind(true)
                    .build(),
            )),
        );
    }

    fn mount_input(&mut self, id: &str, label: &str, fg: Color, typ: InputType) {
        self.mount_input_ex(id, label, fg, typ, None, None);
    }

    fn mount_input_ex(
        &mut self,
        id: &str,
        label: &str,
        fg: Color,
        typ: InputType,
        len: Option<usize>,
        value: Option<String>,
    ) {
        let mut props = InputPropsBuilder::default();
        props
            .with_foreground(fg)
            .with_borders(Borders::ALL, BorderType::Rounded, fg)
            .with_label(label, Alignment::Left)
            .with_input(typ);
        if let Some(len) = len {
            props.with_input_len(len);
        }
        if let Some(value) = value {
            props.with_value(value);
        }
        self.view.mount(id, Box::new(Input::new(props.build())));
    }
}
