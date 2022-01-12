//! ## Components
//!
//! file transfer activity components

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
