//! ## Components
//!
//! setup activity components

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
