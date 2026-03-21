use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color};
use tuirealm::{Component, MockComponent, State, StateValue};

use super::*;

#[derive(MockComponent)]
pub struct RemoteProtocolRadio {
    component: Radio,
}

impl RemoteProtocolRadio {
    pub fn new(default_protocol: FileTransferProtocol, color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .choices(if cfg!(smb) {
                    vec!["SFTP", "SCP", "FTP", "FTPS", "S3", "Kube", "WebDAV", "SMB"].into_iter()
                } else {
                    vec!["SFTP", "SCP", "FTP", "FTPS", "S3", "Kube", "WebDAV"].into_iter()
                })
                .foreground(color)
                .rewind(true)
                .title("Protocol", Alignment::Left)
                .value(Self::protocol_enum_to_opt(default_protocol)),
        }
    }

    fn protocol_opt_to_enum(protocol: usize) -> FileTransferProtocol {
        match protocol {
            REMOTE_RADIO_PROTOCOL_SCP => FileTransferProtocol::Scp,
            REMOTE_RADIO_PROTOCOL_FTP => FileTransferProtocol::Ftp(false),
            REMOTE_RADIO_PROTOCOL_FTPS => FileTransferProtocol::Ftp(true),
            REMOTE_RADIO_PROTOCOL_S3 => FileTransferProtocol::AwsS3,
            REMOTE_RADIO_PROTOCOL_SMB => FileTransferProtocol::Smb,
            REMOTE_RADIO_PROTOCOL_KUBE => FileTransferProtocol::Kube,
            REMOTE_RADIO_PROTOCOL_WEBDAV => FileTransferProtocol::WebDAV,
            _ => FileTransferProtocol::Sftp,
        }
    }

    fn protocol_enum_to_opt(protocol: FileTransferProtocol) -> usize {
        match protocol {
            FileTransferProtocol::Sftp => REMOTE_RADIO_PROTOCOL_SFTP,
            FileTransferProtocol::Scp => REMOTE_RADIO_PROTOCOL_SCP,
            FileTransferProtocol::Ftp(false) => REMOTE_RADIO_PROTOCOL_FTP,
            FileTransferProtocol::Ftp(true) => REMOTE_RADIO_PROTOCOL_FTPS,
            FileTransferProtocol::AwsS3 => REMOTE_RADIO_PROTOCOL_S3,
            FileTransferProtocol::Kube => REMOTE_RADIO_PROTOCOL_KUBE,
            FileTransferProtocol::Smb => REMOTE_RADIO_PROTOCOL_SMB,
            FileTransferProtocol::WebDAV => REMOTE_RADIO_PROTOCOL_WEBDAV,
        }
    }
}

impl Component<Msg, NoUserEvent> for RemoteProtocolRadio {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let result = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => return Some(Msg::Form(FormMsg::Connect)),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => return Some(Msg::Ui(UiMsg::Remote(UiAuthFormMsg::ProtocolBlurDown))),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                return Some(Msg::Ui(UiMsg::Remote(UiAuthFormMsg::ProtocolBlurUp)));
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                return Some(Msg::Ui(UiMsg::Remote(UiAuthFormMsg::ParamsFormBlur)));
            }
            Event::Keyboard(KeyEvent {
                code: Key::BackTab, ..
            }) => return Some(Msg::Ui(UiMsg::Remote(UiAuthFormMsg::ChangeFormTab))),
            _ => return None,
        };

        match result {
            CmdResult::Changed(State::One(StateValue::Usize(choice))) => Some(Msg::Form(
                FormMsg::RemoteProtocolChanged(Self::protocol_opt_to_enum(choice)),
            )),
            _ => Some(Msg::None),
        }
    }
}

#[derive(MockComponent)]
pub struct HostBridgeProtocolRadio {
    component: Radio,
}

impl HostBridgeProtocolRadio {
    pub fn new(protocol: HostBridgeProtocol, color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .choices(if cfg!(smb) {
                    vec![
                        "Localhost",
                        "SFTP",
                        "SCP",
                        "FTP",
                        "FTPS",
                        "S3",
                        "Kube",
                        "WebDAV",
                        "SMB",
                    ]
                    .into_iter()
                } else {
                    vec![
                        "Localhost",
                        "SFTP",
                        "SCP",
                        "FTP",
                        "FTPS",
                        "S3",
                        "Kube",
                        "WebDAV",
                    ]
                    .into_iter()
                })
                .foreground(color)
                .rewind(true)
                .title("Host type", Alignment::Left)
                .value(Self::protocol_to_opt(protocol)),
        }
    }

    fn protocol_to_opt(protocol: HostBridgeProtocol) -> usize {
        match protocol {
            HostBridgeProtocol::Localhost => HOST_BRIDGE_RADIO_PROTOCOL_LOCALHOST,
            HostBridgeProtocol::Remote(FileTransferProtocol::Sftp) => {
                HOST_BRIDGE_RADIO_PROTOCOL_SFTP
            }
            HostBridgeProtocol::Remote(FileTransferProtocol::Scp) => HOST_BRIDGE_RADIO_PROTOCOL_SCP,
            HostBridgeProtocol::Remote(FileTransferProtocol::Ftp(false)) => {
                HOST_BRIDGE_RADIO_PROTOCOL_FTP
            }
            HostBridgeProtocol::Remote(FileTransferProtocol::Ftp(true)) => {
                HOST_BRIDGE_RADIO_PROTOCOL_FTPS
            }
            HostBridgeProtocol::Remote(FileTransferProtocol::AwsS3) => {
                HOST_BRIDGE_RADIO_PROTOCOL_S3
            }
            HostBridgeProtocol::Remote(FileTransferProtocol::Smb) => HOST_BRIDGE_RADIO_PROTOCOL_SMB,
            HostBridgeProtocol::Remote(FileTransferProtocol::Kube) => {
                HOST_BRIDGE_RADIO_PROTOCOL_KUBE
            }
            HostBridgeProtocol::Remote(FileTransferProtocol::WebDAV) => {
                HOST_BRIDGE_RADIO_PROTOCOL_WEBDAV
            }
        }
    }

    fn protocol_opt_to_enum(protocol: usize) -> HostBridgeProtocol {
        match protocol {
            HOST_BRIDGE_RADIO_PROTOCOL_LOCALHOST => HostBridgeProtocol::Localhost,
            HOST_BRIDGE_RADIO_PROTOCOL_SFTP => {
                HostBridgeProtocol::Remote(FileTransferProtocol::Sftp)
            }
            HOST_BRIDGE_RADIO_PROTOCOL_SCP => HostBridgeProtocol::Remote(FileTransferProtocol::Scp),
            HOST_BRIDGE_RADIO_PROTOCOL_FTP => {
                HostBridgeProtocol::Remote(FileTransferProtocol::Ftp(false))
            }
            HOST_BRIDGE_RADIO_PROTOCOL_FTPS => {
                HostBridgeProtocol::Remote(FileTransferProtocol::Ftp(true))
            }
            HOST_BRIDGE_RADIO_PROTOCOL_S3 => {
                HostBridgeProtocol::Remote(FileTransferProtocol::AwsS3)
            }
            HOST_BRIDGE_RADIO_PROTOCOL_SMB => HostBridgeProtocol::Remote(FileTransferProtocol::Smb),
            HOST_BRIDGE_RADIO_PROTOCOL_KUBE => {
                HostBridgeProtocol::Remote(FileTransferProtocol::Kube)
            }
            HOST_BRIDGE_RADIO_PROTOCOL_WEBDAV => {
                HostBridgeProtocol::Remote(FileTransferProtocol::WebDAV)
            }
            _ => HostBridgeProtocol::Localhost,
        }
    }
}

impl Component<Msg, NoUserEvent> for HostBridgeProtocolRadio {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let result = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => return Some(Msg::Form(FormMsg::Connect)),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => return Some(Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::ProtocolBlurDown))),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                return Some(Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::ProtocolBlurUp)));
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                return Some(Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::ParamsFormBlur)));
            }
            Event::Keyboard(KeyEvent {
                code: Key::BackTab, ..
            }) => return Some(Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::ChangeFormTab))),
            _ => return None,
        };

        match result {
            CmdResult::Changed(State::One(StateValue::Usize(choice))) => Some(Msg::Form(
                FormMsg::HostBridgeProtocolChanged(Self::protocol_opt_to_enum(choice)),
            )),
            _ => Some(Msg::None),
        }
    }
}
