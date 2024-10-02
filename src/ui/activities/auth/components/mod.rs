//! ## Components
//!
//! auth activity components

use super::{FileTransferProtocol, FormMsg, Msg, UiMsg};

mod bookmarks;
mod form;
mod popup;
mod text;

pub use bookmarks::{
    BookmarkName, BookmarkSavePassword, BookmarksList, DeleteBookmarkPopup, DeleteRecentPopup,
    RecentsList,
};
#[cfg(unix)]
pub use form::InputSmbWorkgroup;
pub use form::{
    InputAddress, InputKubeClientCert, InputKubeClientKey, InputKubeClusterUrl, InputKubeNamespace,
    InputKubeUsername, InputLocalDirectory, InputPassword, InputPort, InputRemoteDirectory,
    InputS3AccessKey, InputS3Bucket, InputS3Endpoint, InputS3Profile, InputS3Region,
    InputS3SecretAccessKey, InputS3SecurityToken, InputS3SessionToken, InputSmbShare,
    InputUsername, InputWebDAVUri, ProtocolRadio, RadioS3NewPathStyle,
};
pub use popup::{
    ErrorPopup, InfoPopup, InstallUpdatePopup, Keybindings, QuitPopup, ReleaseNotes, WaitPopup,
    WindowSizeError,
};
pub use text::{HelpFooter, NewVersionDisclaimer, Subtitle, Title};
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
            }) => Some(Msg::Ui(UiMsg::ShowQuitPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::Form(FormMsg::EnterSetup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('h'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::Ui(UiMsg::ShowKeybindingsPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Function(1),
                ..
            }) => Some(Msg::Ui(UiMsg::ShowKeybindingsPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('r'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::Ui(UiMsg::ShowReleaseNotes)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('s'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::Ui(UiMsg::ShowSaveBookmarkPopup)),
            Event::WindowResize(_, _) => Some(Msg::Ui(UiMsg::WindowResized)),
            _ => None,
        }
    }
}
