//! ## Components
//!
//! setup activity components

use super::{CommonMsg, ConfigMsg, Msg, SshMsg, ThemeMsg, ViewLayout};

mod commons;
mod config;
mod ssh;
mod theme;

pub(super) use commons::{ErrorPopup, Footer, Header, Keybindings, QuitPopup, SavePopup};
pub(super) use config::{
    CheckUpdates, DefaultProtocol, GroupDirs, HiddenFiles, LocalFileFmt, NotificationsEnabled,
    NotificationsThreshold, PromptOnFileReplace, RemoteFileFmt, SshConfig, TextEditor,
};
pub(super) use ssh::{DelSshKeyPopup, SshHost, SshKeys, SshUsername};
pub(super) use theme::*;

use tui_realm_stdlib::Phantom;
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::{Component, MockComponent};

// -- global listener

#[derive(Default, MockComponent)]
pub struct GlobalListener {
    component: Phantom,
}

impl Component<Msg, NoUserEvent> for GlobalListener {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Function(10),
                ..
            }) => Some(Msg::Common(CommonMsg::ShowQuitPopup)),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                Some(Msg::Common(CommonMsg::ChangeLayout))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('h'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::Common(CommonMsg::ShowKeybindings)),
            Event::Keyboard(KeyEvent {
                code: Key::Function(1),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Common(CommonMsg::ShowKeybindings)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('r'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::Common(CommonMsg::RevertChanges)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('s'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::Common(CommonMsg::ShowSavePopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Function(4),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Common(CommonMsg::ShowSavePopup)),
            Event::WindowResize(_, _) => Some(Msg::Common(CommonMsg::WindowResized)),
            _ => None,
        }
    }
}
