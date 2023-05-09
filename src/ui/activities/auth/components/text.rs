//! ## Text
//!
//! auth activity texts

use tui_realm_stdlib::{Label, Span};
use tuirealm::props::{Color, TextModifiers, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

use super::Msg;

// -- Title

#[derive(MockComponent)]
pub struct Title {
    component: Label,
}

impl Default for Title {
    fn default() -> Self {
        Self {
            component: Label::default()
                .modifiers(TextModifiers::BOLD | TextModifiers::ITALIC)
                .text("$ termscp"),
        }
    }
}

impl Component<Msg, NoUserEvent> for Title {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

// -- subtitle

#[derive(MockComponent)]
pub struct Subtitle {
    component: Label,
}

impl Default for Subtitle {
    fn default() -> Self {
        Self {
            component: Label::default()
                .modifiers(TextModifiers::BOLD | TextModifiers::ITALIC)
                .text(format!("$ version {}", env!("CARGO_PKG_VERSION"))),
        }
    }
}

impl Component<Msg, NoUserEvent> for Subtitle {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

// -- new version disclaimer

#[derive(MockComponent)]
pub struct NewVersionDisclaimer {
    component: Span,
}

impl NewVersionDisclaimer {
    pub fn new(new_version: &str, color: Color) -> Self {
        Self {
            component: Span::default().foreground(color).spans(&[
                TextSpan::from("termscp "),
                TextSpan::new(new_version).underlined().bold(),
                TextSpan::from(
                    " is NOW available! Install update and view release notes with <CTRL+R>",
                ),
            ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for NewVersionDisclaimer {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

// -- HelpFooter

#[derive(MockComponent)]
pub struct HelpFooter {
    component: Span,
}

impl HelpFooter {
    pub fn new(key_color: Color) -> Self {
        Self {
            component: Span::default().spans(&[
                TextSpan::from("<F1|CTRL+H>").bold().fg(key_color),
                TextSpan::from(" Help "),
                TextSpan::from("<CTRL+C>").bold().fg(key_color),
                TextSpan::from(" Enter setup "),
                TextSpan::from("<UP/DOWN>").bold().fg(key_color),
                TextSpan::from(" Change field "),
                TextSpan::from("<TAB>").bold().fg(key_color),
                TextSpan::from(" Switch tab "),
                TextSpan::from("<ENTER>").bold().fg(key_color),
                TextSpan::from(" Submit form "),
                TextSpan::from("<F10|ESC>").bold().fg(key_color),
                TextSpan::from(" Quit "),
            ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for HelpFooter {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
