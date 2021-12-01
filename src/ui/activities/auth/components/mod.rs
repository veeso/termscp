//! ## Components
//!
//! auth activity components

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
use super::{FileTransferProtocol, Msg};

mod bookmarks;
mod form;
mod popup;
mod text;

pub use bookmarks::{
    BookmarkName, BookmarkSavePassword, BookmarksList, DeleteBookmarkPopup, DeleteRecentPopup,
    RecentsList,
};
pub use form::{
    InputAddress, InputPassword, InputPort, InputS3Bucket, InputS3Profile, InputS3Region,
    InputUsername, ProtocolRadio,
};
pub use popup::{
    ErrorPopup, InfoPopup, InstallUpdatePopup, Keybindings, QuitPopup, ReleaseNotes, WaitPopup,
    WindowSizeError,
};
pub use text::{HelpText, NewVersionDisclaimer, Subtitle, Title};

use tui_realm_stdlib::Phantom;
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::{Component, MockComponent};

// -- global listener

#[derive(MockComponent)]
pub struct GlobalListener {
    component: Phantom,
}

impl Default for GlobalListener {
    fn default() -> Self {
        Self {
            component: Phantom::default(),
        }
    }
}

impl Component<Msg, NoUserEvent> for GlobalListener {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => Some(Msg::ShowQuitPopup),
            Event::Keyboard(KeyEvent {
                code: Key::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::EnterSetup),
            Event::Keyboard(KeyEvent {
                code: Key::Char('h'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::ShowKeybindingsPopup),
            Event::Keyboard(KeyEvent {
                code: Key::Char('r'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::ShowReleaseNotes),
            Event::Keyboard(KeyEvent {
                code: Key::Char('s'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::ShowSaveBookmarkPopup),
            _ => None,
        }
    }
}
