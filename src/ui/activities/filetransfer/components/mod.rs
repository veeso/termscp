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
use super::{Msg, TransferMsg, UiMsg};

use tui_realm_stdlib::Phantom;
use tuirealm::{
    event::{Event, Key, KeyEvent, KeyModifiers},
    Component, MockComponent, NoUserEvent,
};

// -- export
mod log;
mod popups;
mod transfer;

pub use self::log::Log;
pub use popups::{
    CopyPopup, DeletePopup, DisconnectPopup, ErrorPopup, ExecPopup, FatalPopup, FileInfoPopup,
    FindPopup, GoToPopup, KeybindingsPopup, MkdirPopup, NewfilePopup, OpenWithPopup,
    ProgressBarFull, ProgressBarPartial, QuitPopup, RenamePopup, ReplacePopup,
    ReplacingFilesListPopup, SaveAsPopup, SortingPopup, StatusBarLocal, StatusBarRemote, WaitPopup,
};
pub use transfer::{ExplorerFind, ExplorerLocal, ExplorerRemote};

#[derive(Default, MockComponent)]
pub struct GlobalListener {
    component: Phantom,
}

impl Component<Msg, NoUserEvent> for GlobalListener {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::ShowDisconnectPopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowQuitPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('h'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowKeybindingsPopup)),
            _ => None,
        }
    }
}
