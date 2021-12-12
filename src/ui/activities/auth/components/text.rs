//! ## Text
//!
//! auth activity texts

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

use tui_realm_stdlib::{Label, Span};
use tuirealm::props::{Color, TextModifiers, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

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
