//! ## Text
//!
//! auth activity texts

use tui_realm_stdlib::components::{Label, Span};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, NoUserEvent};
use tuirealm::props::{Color, SpanStatic, TextModifiers};
use tuirealm::ratatui::style::Stylize;

use super::Msg;

// -- Title

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for Title {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

// -- subtitle

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for Subtitle {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

// -- new version disclaimer

#[derive(Component)]
pub struct NewVersionDisclaimer {
    component: Span,
}

impl NewVersionDisclaimer {
    pub fn new(new_version: &str, color: Color) -> Self {
        Self {
            component: Span::default().foreground(color).spans([
                SpanStatic::from("termscp "),
                SpanStatic::raw(new_version.to_string()).underlined().bold(),
                SpanStatic::from(
                    " is NOW available! Install update and view release notes with <CTRL+R>",
                ),
            ]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for NewVersionDisclaimer {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

// -- HelpFooter

#[derive(Component)]
pub struct HelpFooter {
    component: Span,
}

impl HelpFooter {
    pub fn new(key_color: Color) -> Self {
        Self {
            component: Span::default().spans([
                SpanStatic::from("<F1|CTRL+H>").bold().fg(key_color),
                SpanStatic::from(" Help "),
                SpanStatic::from("<CTRL+C>").bold().fg(key_color),
                SpanStatic::from(" Enter setup "),
                SpanStatic::from("<UP/DOWN>").bold().fg(key_color),
                SpanStatic::from(" Change field "),
                SpanStatic::from("<TAB>").bold().fg(key_color),
                SpanStatic::from(" Switch tab "),
                SpanStatic::from("<BACKTAB>").bold().fg(key_color),
                SpanStatic::from(" Switch form "),
                SpanStatic::from("<ENTER>").bold().fg(key_color),
                SpanStatic::from(" Submit form "),
                SpanStatic::from("<F10|ESC>").bold().fg(key_color),
                SpanStatic::from(" Quit "),
            ]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for HelpFooter {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
