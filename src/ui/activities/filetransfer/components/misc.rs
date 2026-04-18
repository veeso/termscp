//! ## Components
//!
//! file transfer activity components

use tui_realm_stdlib::components::Span;
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, NoUserEvent};
use tuirealm::props::{Color, SpanStatic};
use tuirealm::ratatui::style::Stylize;

use super::Msg;

#[derive(Component)]
pub struct FooterBar {
    component: Span,
}

impl FooterBar {
    pub fn new(key_color: Color) -> Self {
        Self {
            component: Span::default().spans([
                SpanStatic::from("<F1|H>").bold().fg(key_color),
                SpanStatic::from(" Help "),
                SpanStatic::from("<TAB>").bold().fg(key_color),
                SpanStatic::from(" Change tab "),
                SpanStatic::from("<SPACE>").bold().fg(key_color),
                SpanStatic::from(" Transfer "),
                SpanStatic::from("<ENTER>").bold().fg(key_color),
                SpanStatic::from(" Enter dir "),
                SpanStatic::from("<F2|S>").bold().fg(key_color),
                SpanStatic::from(" Save as "),
                SpanStatic::from("<F3|V>").bold().fg(key_color),
                SpanStatic::from(" View "),
                SpanStatic::from("<F4|O>").bold().fg(key_color),
                SpanStatic::from(" Edit "),
                SpanStatic::from("<F5|C>").bold().fg(key_color),
                SpanStatic::from(" Copy "),
                SpanStatic::from("<F6|R>").bold().fg(key_color),
                SpanStatic::from(" Rename "),
                SpanStatic::from("<F7|D>").bold().fg(key_color),
                SpanStatic::from(" Make dir "),
                SpanStatic::from("<F8|DEL>").bold().fg(key_color),
                SpanStatic::from(" Delete "),
                SpanStatic::from("<F10|Q>").bold().fg(key_color),
                SpanStatic::from(" Quit "),
            ]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for FooterBar {
    fn on(&mut self, _: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
