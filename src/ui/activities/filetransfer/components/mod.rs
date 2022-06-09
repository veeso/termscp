//! ## Components
//!
//! file transfer activity components

use super::{Msg, PendingActionMsg, TransferMsg, UiMsg};

use tui_realm_stdlib::Phantom;
use tuirealm::{
    event::{Event, Key, KeyEvent, KeyModifiers},
    Component, MockComponent, NoUserEvent,
};

// -- export
mod log;
mod misc;
mod popups;
mod transfer;

pub use self::log::Log;
pub use misc::FooterBar;
pub use popups::{
    CopyPopup, DeletePopup, DisconnectPopup, ErrorPopup, ExecPopup, FatalPopup, FileInfoPopup,
    FindPopup, GoToPopup, KeybindingsPopup, MkdirPopup, NewfilePopup, OpenWithPopup,
    ProgressBarFull, ProgressBarPartial, QuitPopup, RenamePopup, ReplacePopup,
    ReplacingFilesListPopup, SaveAsPopup, SortingPopup, StatusBarLocal, StatusBarRemote,
    SymlinkPopup, SyncBrowsingMkdirPopup, WaitPopup, WatchedPathsList, WatcherPopup,
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
                code: Key::Char('q') | Key::Function(10),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowQuitPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('h') | Key::Function(1),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowKeybindingsPopup)),
            Event::WindowResize(_, _) => Some(Msg::Ui(UiMsg::WindowResized)),
            _ => None,
        }
    }
}
