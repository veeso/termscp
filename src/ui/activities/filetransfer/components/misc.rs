//! ## Components
//!
//! file transfer activity components

use super::Msg;

use tui_realm_stdlib::Span;
use tuirealm::props::{Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

#[derive(MockComponent)]
pub struct FooterBar {
    component: Span,
}

impl FooterBar {
    pub fn new(key_color: Color) -> Self {
        Self {
            component: Span::default().spans(&[
                TextSpan::from("<F1|H>").bold().fg(key_color),
                TextSpan::from(" Help "),
                TextSpan::from("<TAB>").bold().fg(key_color),
                TextSpan::from(" Change tab "),
                TextSpan::from("<SPACE>").bold().fg(key_color),
                TextSpan::from(" Transfer "),
                TextSpan::from("<ENTER>").bold().fg(key_color),
                TextSpan::from(" Enter dir "),
                TextSpan::from("<F2|S>").bold().fg(key_color),
                TextSpan::from(" Save as "),
                TextSpan::from("<F3|V>").bold().fg(key_color),
                TextSpan::from(" View "),
                TextSpan::from("<F4|O>").bold().fg(key_color),
                TextSpan::from(" Edit "),
                TextSpan::from("<F5|C>").bold().fg(key_color),
                TextSpan::from(" Copy "),
                TextSpan::from("<F6|R>").bold().fg(key_color),
                TextSpan::from(" Rename "),
                TextSpan::from("<F7|D>").bold().fg(key_color),
                TextSpan::from(" Make dir "),
                TextSpan::from("<F8|DEL>").bold().fg(key_color),
                TextSpan::from(" Delete "),
                TextSpan::from("<F10|Q>").bold().fg(key_color),
                TextSpan::from(" Quit "),
            ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for FooterBar {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
