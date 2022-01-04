//! ## Theme
//!
//! theme tab components

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
use super::{Msg, ThemeMsg};
use crate::ui::activities::setup::IdTheme;

use tui_realm_stdlib::{Input, Label};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color, InputType, Style, TextModifiers};
use tuirealm::{
    AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

// -- components

#[derive(MockComponent)]
pub struct AuthTitle {
    component: Label,
}

impl Default for AuthTitle {
    fn default() -> Self {
        Self {
            component: Label::default()
                .modifiers(TextModifiers::BOLD)
                .text("Authentication styles"),
        }
    }
}

impl Component<Msg, NoUserEvent> for AuthTitle {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct MiscTitle {
    component: Label,
}

impl Default for MiscTitle {
    fn default() -> Self {
        Self {
            component: Label::default()
                .modifiers(TextModifiers::BOLD)
                .text("Misc styles"),
        }
    }
}

impl Component<Msg, NoUserEvent> for MiscTitle {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct TransferTitle {
    component: Label,
}

impl Default for TransferTitle {
    fn default() -> Self {
        Self {
            component: Label::default()
                .modifiers(TextModifiers::BOLD)
                .text("Transfer styles"),
        }
    }
}

impl Component<Msg, NoUserEvent> for TransferTitle {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct TransferTitle2 {
    component: Label,
}

impl Default for TransferTitle2 {
    fn default() -> Self {
        Self {
            component: Label::default()
                .modifiers(TextModifiers::BOLD)
                .text("Transfer styles (2)"),
        }
    }
}

impl Component<Msg, NoUserEvent> for TransferTitle2 {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct AuthAddress {
    component: InputColor,
}

impl AuthAddress {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Ip Address",
                IdTheme::AuthAddress,
                value,
                Msg::Theme(ThemeMsg::AuthAddressBlurDown),
                Msg::Theme(ThemeMsg::AuthAddressBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for AuthAddress {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct AuthBookmarks {
    component: InputColor,
}

impl AuthBookmarks {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Bookmarks",
                IdTheme::AuthBookmarks,
                value,
                Msg::Theme(ThemeMsg::AuthBookmarksBlurDown),
                Msg::Theme(ThemeMsg::AuthBookmarksBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for AuthBookmarks {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct AuthPassword {
    component: InputColor,
}

impl AuthPassword {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Password",
                IdTheme::AuthPassword,
                value,
                Msg::Theme(ThemeMsg::AuthPasswordBlurDown),
                Msg::Theme(ThemeMsg::AuthPasswordBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for AuthPassword {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct AuthPort {
    component: InputColor,
}

impl AuthPort {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Port",
                IdTheme::AuthPort,
                value,
                Msg::Theme(ThemeMsg::AuthPortBlurDown),
                Msg::Theme(ThemeMsg::AuthPortBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for AuthPort {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct AuthProtocol {
    component: InputColor,
}

impl AuthProtocol {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Protocol",
                IdTheme::AuthProtocol,
                value,
                Msg::Theme(ThemeMsg::AuthProtocolBlurDown),
                Msg::Theme(ThemeMsg::AuthProtocolBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for AuthProtocol {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct AuthRecentHosts {
    component: InputColor,
}

impl AuthRecentHosts {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Recent connections",
                IdTheme::AuthRecentHosts,
                value,
                Msg::Theme(ThemeMsg::AuthRecentHostsBlurDown),
                Msg::Theme(ThemeMsg::AuthRecentHostsBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for AuthRecentHosts {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}
#[derive(MockComponent)]
pub struct AuthUsername {
    component: InputColor,
}

impl AuthUsername {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Username",
                IdTheme::AuthUsername,
                value,
                Msg::Theme(ThemeMsg::AuthUsernameBlurDown),
                Msg::Theme(ThemeMsg::AuthUsernameBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for AuthUsername {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct ExplorerLocalBg {
    component: InputColor,
}

impl ExplorerLocalBg {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Local explorer background",
                IdTheme::ExplorerLocalBg,
                value,
                Msg::Theme(ThemeMsg::ExplorerLocalBgBlurDown),
                Msg::Theme(ThemeMsg::ExplorerLocalBgBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for ExplorerLocalBg {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct ExplorerLocalFg {
    component: InputColor,
}

impl ExplorerLocalFg {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Local explorer foreground",
                IdTheme::ExplorerLocalFg,
                value,
                Msg::Theme(ThemeMsg::ExplorerLocalFgBlurDown),
                Msg::Theme(ThemeMsg::ExplorerLocalFgBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for ExplorerLocalFg {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct ExplorerLocalHg {
    component: InputColor,
}

impl ExplorerLocalHg {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Local explorer highlighted",
                IdTheme::ExplorerLocalHg,
                value,
                Msg::Theme(ThemeMsg::ExplorerLocalHgBlurDown),
                Msg::Theme(ThemeMsg::ExplorerLocalHgBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for ExplorerLocalHg {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct ExplorerRemoteBg {
    component: InputColor,
}

impl ExplorerRemoteBg {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Remote explorer background",
                IdTheme::ExplorerRemoteBg,
                value,
                Msg::Theme(ThemeMsg::ExplorerRemoteBgBlurDown),
                Msg::Theme(ThemeMsg::ExplorerRemoteBgBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for ExplorerRemoteBg {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct ExplorerRemoteFg {
    component: InputColor,
}

impl ExplorerRemoteFg {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Remote explorer foreground",
                IdTheme::ExplorerRemoteFg,
                value,
                Msg::Theme(ThemeMsg::ExplorerRemoteFgBlurDown),
                Msg::Theme(ThemeMsg::ExplorerRemoteFgBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for ExplorerRemoteFg {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct ExplorerRemoteHg {
    component: InputColor,
}

impl ExplorerRemoteHg {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Remote explorer highlighted",
                IdTheme::ExplorerRemoteHg,
                value,
                Msg::Theme(ThemeMsg::ExplorerRemoteHgBlurDown),
                Msg::Theme(ThemeMsg::ExplorerRemoteHgBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for ExplorerRemoteHg {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct LogBg {
    component: InputColor,
}

impl LogBg {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Log window background",
                IdTheme::LogBg,
                value,
                Msg::Theme(ThemeMsg::LogBgBlurDown),
                Msg::Theme(ThemeMsg::LogBgBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for LogBg {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct LogWindow {
    component: InputColor,
}

impl LogWindow {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Log window",
                IdTheme::LogWindow,
                value,
                Msg::Theme(ThemeMsg::LogWindowBlurDown),
                Msg::Theme(ThemeMsg::LogWindowBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for LogWindow {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct MiscError {
    component: InputColor,
}

impl MiscError {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Error",
                IdTheme::MiscError,
                value,
                Msg::Theme(ThemeMsg::MiscErrorBlurDown),
                Msg::Theme(ThemeMsg::MiscErrorBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for MiscError {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct MiscInfo {
    component: InputColor,
}

impl MiscInfo {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Info",
                IdTheme::MiscInfo,
                value,
                Msg::Theme(ThemeMsg::MiscInfoBlurDown),
                Msg::Theme(ThemeMsg::MiscInfoBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for MiscInfo {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct MiscInput {
    component: InputColor,
}

impl MiscInput {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Input",
                IdTheme::MiscInput,
                value,
                Msg::Theme(ThemeMsg::MiscInputBlurDown),
                Msg::Theme(ThemeMsg::MiscInputBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for MiscInput {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct MiscKeys {
    component: InputColor,
}

impl MiscKeys {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Key strokes",
                IdTheme::MiscKeys,
                value,
                Msg::Theme(ThemeMsg::MiscKeysBlurDown),
                Msg::Theme(ThemeMsg::MiscKeysBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for MiscKeys {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct MiscQuit {
    component: InputColor,
}

impl MiscQuit {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Quit dialogs",
                IdTheme::MiscQuit,
                value,
                Msg::Theme(ThemeMsg::MiscQuitBlurDown),
                Msg::Theme(ThemeMsg::MiscQuitBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for MiscQuit {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct MiscSave {
    component: InputColor,
}

impl MiscSave {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Save confirmations",
                IdTheme::MiscSave,
                value,
                Msg::Theme(ThemeMsg::MiscSaveBlurDown),
                Msg::Theme(ThemeMsg::MiscSaveBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for MiscSave {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct MiscWarn {
    component: InputColor,
}

impl MiscWarn {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Warnings",
                IdTheme::MiscWarn,
                value,
                Msg::Theme(ThemeMsg::MiscWarnBlurDown),
                Msg::Theme(ThemeMsg::MiscWarnBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for MiscWarn {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct ProgBarFull {
    component: InputColor,
}

impl ProgBarFull {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "'Full transfer' Progress bar",
                IdTheme::ProgBarFull,
                value,
                Msg::Theme(ThemeMsg::ProgBarFullBlurDown),
                Msg::Theme(ThemeMsg::ProgBarFullBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for ProgBarFull {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct ProgBarPartial {
    component: InputColor,
}

impl ProgBarPartial {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "'Partial transfer' Progress bar",
                IdTheme::ProgBarPartial,
                value,
                Msg::Theme(ThemeMsg::ProgBarPartialBlurDown),
                Msg::Theme(ThemeMsg::ProgBarPartialBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for ProgBarPartial {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct StatusHidden {
    component: InputColor,
}

impl StatusHidden {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Hidden files",
                IdTheme::StatusHidden,
                value,
                Msg::Theme(ThemeMsg::StatusHiddenBlurDown),
                Msg::Theme(ThemeMsg::StatusHiddenBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for StatusHidden {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct StatusSorting {
    component: InputColor,
}

impl StatusSorting {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "File sorting",
                IdTheme::StatusSorting,
                value,
                Msg::Theme(ThemeMsg::StatusSortingBlurDown),
                Msg::Theme(ThemeMsg::StatusSortingBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for StatusSorting {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(MockComponent)]
pub struct StatusSync {
    component: InputColor,
}

impl StatusSync {
    pub fn new(value: Color) -> Self {
        Self {
            component: InputColor::new(
                "Synchronized browsing",
                IdTheme::StatusSync,
                value,
                Msg::Theme(ThemeMsg::StatusSyncBlurDown),
                Msg::Theme(ThemeMsg::StatusSyncBlurUp),
            ),
        }
    }
}

impl Component<Msg, NoUserEvent> for StatusSync {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

// -- input color

#[derive(MockComponent)]
struct InputColor {
    component: Input,
    id: IdTheme,
    on_key_down: Msg,
    on_key_up: Msg,
}

impl InputColor {
    pub fn new(name: &str, id: IdTheme, color: Color, on_key_down: Msg, on_key_up: Msg) -> Self {
        let value = crate::utils::fmt::fmt_color(&color);
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .input_type(InputType::Color)
                .placeholder("#aa33ee", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title(name, Alignment::Left)
                .value(value),
            id,
            on_key_down,
            on_key_up,
        }
    }

    fn update_color(&mut self, result: CmdResult) -> Option<Msg> {
        if let CmdResult::Changed(State::One(StateValue::String(color))) = result {
            let color = tuirealm::utils::parser::parse_color(&color).unwrap();
            self.attr(Attribute::Foreground, AttrValue::Color(color));
            self.attr(
                Attribute::Borders,
                AttrValue::Borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(color),
                ),
            );
            Some(Msg::Theme(ThemeMsg::ColorChanged(self.id.clone(), color)))
        } else {
            self.attr(Attribute::Foreground, AttrValue::Color(Color::Red));
            self.attr(
                Attribute::Borders,
                AttrValue::Borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Red),
                ),
            );
            Some(Msg::None)
        }
    }
}

impl Component<Msg, NoUserEvent> for InputColor {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                let result = self.perform(Cmd::Cancel);
                self.update_color(result)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                let result = self.perform(Cmd::Delete);
                self.update_color(result)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            }) => {
                let result = self.perform(Cmd::Type(ch));
                self.update_color(result)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(self.on_key_down.clone()),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(self.on_key_up.clone()),
            _ => None,
        }
    }
}
