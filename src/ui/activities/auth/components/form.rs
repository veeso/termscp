//! ## Form
//!
//! auth activity components for file transfer params form

use tui_realm_stdlib::{Input, Radio};
use tuirealm::Component;
use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, InputType, Style};

use super::{FileTransferProtocol, FormMsg, Msg, UiMsg};
use crate::ui::activities::auth::{
    FormTab, HOST_BRIDGE_RADIO_PROTOCOL_FTP, HOST_BRIDGE_RADIO_PROTOCOL_FTPS,
    HOST_BRIDGE_RADIO_PROTOCOL_KUBE, HOST_BRIDGE_RADIO_PROTOCOL_LOCALHOST,
    HOST_BRIDGE_RADIO_PROTOCOL_S3, HOST_BRIDGE_RADIO_PROTOCOL_SCP, HOST_BRIDGE_RADIO_PROTOCOL_SFTP,
    HOST_BRIDGE_RADIO_PROTOCOL_SMB, HOST_BRIDGE_RADIO_PROTOCOL_WEBDAV, HostBridgeProtocol,
    REMOTE_RADIO_PROTOCOL_FTP, REMOTE_RADIO_PROTOCOL_FTPS, REMOTE_RADIO_PROTOCOL_KUBE,
    REMOTE_RADIO_PROTOCOL_S3, REMOTE_RADIO_PROTOCOL_SCP, REMOTE_RADIO_PROTOCOL_SFTP,
    REMOTE_RADIO_PROTOCOL_SMB, REMOTE_RADIO_PROTOCOL_WEBDAV, UiAuthFormMsg,
};

#[path = "form/generic.rs"]
mod generic;
#[path = "form/kube.rs"]
mod kube;
#[path = "form/paths.rs"]
mod paths;
#[path = "form/protocol.rs"]
mod protocol;
#[path = "form/s3.rs"]
mod s3;
#[path = "form/smb.rs"]
mod smb;
#[path = "form/webdav.rs"]
mod webdav;

pub use generic::{InputAddress, InputPassword, InputPort, InputUsername};
pub use kube::{
    InputKubeClientCert, InputKubeClientKey, InputKubeClusterUrl, InputKubeNamespace,
    InputKubeUsername,
};
pub use paths::{InputLocalDirectory, InputRemoteDirectory};
pub use protocol::{HostBridgeProtocolRadio, RemoteProtocolRadio};
pub use s3::{
    InputS3AccessKey, InputS3Bucket, InputS3Endpoint, InputS3Profile, InputS3Region,
    InputS3SecretAccessKey, InputS3SecurityToken, InputS3SessionToken, RadioS3NewPathStyle,
};
pub use smb::InputSmbShare;
#[cfg(posix)]
pub use smb::InputSmbWorkgroup;
pub use webdav::InputWebDAVUri;

fn handle_input_ev(
    component: &mut dyn Component<Msg, NoUserEvent>,
    ev: Event<NoUserEvent>,
    on_key_down: Msg,
    on_key_up: Msg,
    form_tab: FormTab,
) -> Option<Msg> {
    match ev {
        Event::Keyboard(KeyEvent {
            code: Key::Left, ..
        }) => {
            component.perform(Cmd::Move(Direction::Left));
            Some(Msg::None)
        }
        Event::Keyboard(KeyEvent {
            code: Key::Right, ..
        }) => {
            component.perform(Cmd::Move(Direction::Right));
            Some(Msg::None)
        }
        Event::Keyboard(KeyEvent {
            code: Key::Home, ..
        }) => {
            component.perform(Cmd::GoTo(Position::Begin));
            Some(Msg::None)
        }
        Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
            component.perform(Cmd::GoTo(Position::End));
            Some(Msg::None)
        }
        Event::Keyboard(KeyEvent {
            code: Key::Delete, ..
        }) => {
            component.perform(Cmd::Cancel);
            Some(Msg::None)
        }
        Event::Keyboard(KeyEvent {
            code: Key::Backspace,
            ..
        }) => {
            component.perform(Cmd::Delete);
            Some(Msg::None)
        }
        Event::Keyboard(KeyEvent {
            code: Key::Enter, ..
        }) => Some(Msg::Form(FormMsg::Connect)),
        Event::Keyboard(KeyEvent {
            code: Key::Char('c' | 'h' | 'r' | 's'),
            modifiers: KeyModifiers::CONTROL,
        }) => Some(Msg::None),
        Event::Keyboard(KeyEvent {
            code: Key::Char(ch),
            ..
        }) => {
            component.perform(Cmd::Type(ch));
            Some(Msg::None)
        }
        Event::Keyboard(KeyEvent {
            code: Key::Down, ..
        }) => Some(on_key_down),
        Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(on_key_up),
        Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => match form_tab {
            FormTab::HostBridge => Some(Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::ParamsFormBlur))),
            FormTab::Remote => Some(Msg::Ui(UiMsg::Remote(UiAuthFormMsg::ParamsFormBlur))),
        },
        Event::Keyboard(KeyEvent {
            code: Key::BackTab, ..
        }) => match form_tab {
            FormTab::HostBridge => Some(Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::ChangeFormTab))),
            FormTab::Remote => Some(Msg::Ui(UiMsg::Remote(UiAuthFormMsg::ChangeFormTab))),
        },
        _ => None,
    }
}
