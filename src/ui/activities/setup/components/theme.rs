//! ## Theme
//!
//! theme tab components

use tui_realm_stdlib::components::{Input, Label};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{
    AttrValue, Attribute, BorderType, Borders, Color, HorizontalAlignment, InputType, Style,
    TextModifiers, Title,
};
use tuirealm::state::{State, StateValue};

use super::{Msg, ThemeMsg};
use crate::ui::activities::setup::IdTheme;

// -- components

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for AuthTitle {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for MiscTitle {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for TransferTitle {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for TransferTitle2 {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for AuthAddress {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for AuthBookmarks {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for AuthPassword {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for AuthPort {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for AuthProtocol {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for AuthRecentHosts {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}
#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for AuthUsername {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for ExplorerLocalBg {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for ExplorerLocalFg {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for ExplorerLocalHg {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for ExplorerRemoteBg {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for ExplorerRemoteFg {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for ExplorerRemoteHg {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for LogBg {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for LogWindow {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for MiscError {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for MiscInfo {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for MiscInput {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for MiscKeys {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for MiscQuit {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for MiscSave {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for MiscWarn {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
pub struct ProgBar {
    component: InputColor,
}

impl ProgBar {
    pub fn new(color: Color) -> Self {
        Self {
            component: InputColor::new(
                "Progress bar",
                IdTheme::ProgBar,
                color,
                Msg::Theme(ThemeMsg::ProgBarBlurDown),
                Msg::Theme(ThemeMsg::ProgBarBlurUp),
            ),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for ProgBar {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for StatusHidden {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for StatusSorting {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

#[derive(Component)]
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

impl AppComponent<Msg, NoUserEvent> for StatusSync {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        self.component.on(ev)
    }
}

// -- input color

#[derive(Component)]
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
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "#aa33ee",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(Title::from(name.to_string()).alignment(HorizontalAlignment::Left))
                .value(value),
            id,
            on_key_down,
            on_key_up,
        }
    }

    fn update_color(&mut self, result: CmdResult) -> Option<Msg> {
        if let CmdResult::Changed(State::Single(StateValue::String(color))) = result {
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

impl AppComponent<Msg, NoUserEvent> for InputColor {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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
                // NOTE: escaped control sequence
                code: Key::Char('h' | 'r' | 's'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::None),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                ..
            }) => {
                let result = self.perform(Cmd::Type(*ch));
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
