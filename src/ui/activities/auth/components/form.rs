//! ## Form
//!
//! auth activity components for file transfer params form

use tui_realm_stdlib::{Input, Radio};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color, InputType, Style};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

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

// -- protocol

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

    /// Convert radio index for protocol into a `FileTransferProtocol`
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

    /// Convert `FileTransferProtocol` enum into radio group index
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

    /// Convert radio index for protocol into a `FileTransferProtocol`
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

// -- remote directory

#[derive(MockComponent)]
pub struct InputRemoteDirectory {
    component: Input,
    form_tab: FormTab,
}

impl InputRemoteDirectory {
    pub fn new(remote_dir: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("/home/foo", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Default remote working directory", Alignment::Left)
                .input_type(InputType::Text)
                .value(remote_dir),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputRemoteDirectory {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::RemoteDirectoryBlurDown)),
            FormTab::HostBridge => {
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::RemoteDirectoryBlurDown))
            }
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::RemoteDirectoryBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::RemoteDirectoryBlurUp)),
        };

        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

// -- remote directory

#[derive(MockComponent)]
pub struct InputLocalDirectory {
    component: Input,
    form_tab: FormTab,
}

impl InputLocalDirectory {
    pub fn new(local_dir: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("/home/foo", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Default local working directory", Alignment::Left)
                .input_type(InputType::Text)
                .value(local_dir),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputLocalDirectory {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::LocalDirectoryBlurDown)),
            FormTab::HostBridge => {
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::LocalDirectoryBlurDown))
            }
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::LocalDirectoryBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::LocalDirectoryBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

// -- address

#[derive(MockComponent)]
pub struct InputAddress {
    component: Input,
    form_tab: FormTab,
}

impl InputAddress {
    pub fn new(host: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("127.0.0.1", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Remote host", Alignment::Left)
                .input_type(InputType::Text)
                .value(host),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputAddress {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::AddressBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::AddressBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::AddressBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::AddressBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

// -- port number

#[derive(MockComponent)]
pub struct InputPort {
    component: Input,
    form_tab: FormTab,
}

impl InputPort {
    pub fn new(port: u16, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("22", Style::default().fg(Color::Rgb(128, 128, 128)))
                .input_type(InputType::UnsignedInteger)
                .input_len(5)
                .title("Port number", Alignment::Left)
                .value(port.to_string()),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputPort {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::PortBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::PortBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::PortBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::PortBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

// -- username

#[derive(MockComponent)]
pub struct InputUsername {
    component: Input,
    form_tab: FormTab,
}

impl InputUsername {
    pub fn new(username: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("root", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Username", Alignment::Left)
                .input_type(InputType::Text)
                .value(username),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputUsername {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::UsernameBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::UsernameBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::UsernameBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::UsernameBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

// -- password

#[derive(MockComponent)]
pub struct InputPassword {
    component: Input,
    form_tab: FormTab,
}

impl InputPassword {
    pub fn new(password: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .title("Password", Alignment::Left)
                .input_type(InputType::Password('*'))
                .value(password),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputPassword {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::PasswordBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::PasswordBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::PasswordBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::PasswordBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

// -- s3 bucket

#[derive(MockComponent)]
pub struct InputS3Bucket {
    component: Input,
    form_tab: FormTab,
}

impl InputS3Bucket {
    pub fn new(bucket: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("my-bucket", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Bucket name", Alignment::Left)
                .input_type(InputType::Text)
                .value(bucket),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3Bucket {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3BucketBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3BucketBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3BucketBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3BucketBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

// -- s3 region

#[derive(MockComponent)]
pub struct InputS3Region {
    component: Input,
    form_tab: FormTab,
}

impl InputS3Region {
    pub fn new(region: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("eu-west-1", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Region", Alignment::Left)
                .input_type(InputType::Text)
                .value(region),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3Region {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3RegionBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3RegionBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3RegionBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3RegionBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

// -- s3 endpoint

#[derive(MockComponent)]
pub struct InputS3Endpoint {
    component: Input,
    form_tab: FormTab,
}

impl InputS3Endpoint {
    pub fn new(endpoint: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder(
                    "http://localhost:9000",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title("Endpoint", Alignment::Left)
                .input_type(InputType::Text)
                .value(endpoint),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3Endpoint {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3EndpointBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3EndpointBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3EndpointBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3EndpointBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

// -- s3 new path style

#[derive(MockComponent)]
pub struct RadioS3NewPathStyle {
    component: Radio,
    form_tab: FormTab,
}

impl RadioS3NewPathStyle {
    pub fn new(new_path_style: bool, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .choices(["Yes", "No"])
                .foreground(color)
                .rewind(true)
                .title("New path style", Alignment::Left)
                .value(usize::from(!new_path_style)),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for RadioS3NewPathStyle {
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
                code: Key::Enter, ..
            }) => Some(Msg::Form(FormMsg::Connect)),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(if self.form_tab == FormTab::Remote {
                Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3NewPathStyleBlurDown))
            } else {
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3NewPathStyleBlurDown))
            }),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                Some(if self.form_tab == FormTab::Remote {
                    Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3NewPathStyleBlurUp))
                } else {
                    Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3NewPathStyleBlurUp))
                })
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                Some(if self.form_tab == FormTab::Remote {
                    Msg::Ui(UiMsg::Remote(UiAuthFormMsg::ParamsFormBlur))
                } else {
                    Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::ParamsFormBlur))
                })
            }
            _ => None,
        }
    }
}

// -- s3 profile

#[derive(MockComponent)]
pub struct InputS3Profile {
    component: Input,
    form_tab: FormTab,
}

impl InputS3Profile {
    pub fn new(profile: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("default", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Profile", Alignment::Left)
                .input_type(InputType::Text)
                .value(profile),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3Profile {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3ProfileBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3ProfileBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3ProfileBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3ProfileBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

// -- s3 access key

#[derive(MockComponent)]
pub struct InputS3AccessKey {
    component: Input,
    form_tab: FormTab,
}

impl InputS3AccessKey {
    pub fn new(access_key: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("AKIA...", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Access key", Alignment::Left)
                .input_type(InputType::Text)
                .value(access_key),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3AccessKey {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3AccessKeyBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3AccessKeyBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3AccessKeyBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3AccessKeyBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

#[derive(MockComponent)]
pub struct InputS3SecretAccessKey {
    component: Input,
    form_tab: FormTab,
}

impl InputS3SecretAccessKey {
    pub fn new(secret_access_key: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .title("Secret access key", Alignment::Left)
                .input_type(InputType::Password('*'))
                .value(secret_access_key),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3SecretAccessKey {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3SecretAccessKeyBlurDown)),
            FormTab::HostBridge => {
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3SecretAccessKeyBlurDown))
            }
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3SecretAccessKeyBlurUp)),
            FormTab::HostBridge => {
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3SecretAccessKeyBlurUp))
            }
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

#[derive(MockComponent)]
pub struct InputS3SecurityToken {
    component: Input,
    form_tab: FormTab,
}

impl InputS3SecurityToken {
    pub fn new(security_token: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .title("Security token", Alignment::Left)
                .input_type(InputType::Password('*'))
                .value(security_token),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3SecurityToken {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3SecurityTokenBlurDown)),
            FormTab::HostBridge => {
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3SecurityTokenBlurDown))
            }
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3SecurityTokenBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3SecurityTokenBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

#[derive(MockComponent)]
pub struct InputS3SessionToken {
    component: Input,
    form_tab: FormTab,
}

impl InputS3SessionToken {
    pub fn new(session_token: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .title("Session token", Alignment::Left)
                .input_type(InputType::Password('*'))
                .value(session_token),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3SessionToken {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3SessionTokenBlurDown)),
            FormTab::HostBridge => {
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3SessionTokenBlurDown))
            }
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::S3SessionTokenBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::S3SessionTokenBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

#[derive(MockComponent)]
pub struct InputSmbShare {
    component: Input,
    form_tab: FormTab,
}

impl InputSmbShare {
    pub fn new(host: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .title("Share", Alignment::Left)
                .input_type(InputType::Text)
                .value(host),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputSmbShare {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::SmbShareBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::SmbShareBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::SmbShareBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::SmbShareBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

#[cfg(posix)]
#[derive(MockComponent)]
pub struct InputSmbWorkgroup {
    component: Input,
    form_tab: FormTab,
}

#[cfg(posix)]
impl InputSmbWorkgroup {
    pub fn new(host: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .title("Workgroup", Alignment::Left)
                .input_type(InputType::Text)
                .value(host),
            form_tab,
        }
    }
}

#[cfg(posix)]
impl Component<Msg, NoUserEvent> for InputSmbWorkgroup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::SmbWorkgroupDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::SmbWorkgroupDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::SmbWorkgroupUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::SmbWorkgroupUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

#[derive(MockComponent)]
pub struct InputWebDAVUri {
    component: Input,
    form_tab: FormTab,
}

impl InputWebDAVUri {
    pub fn new(host: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder(
                    "http://localhost:8080",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title("HTTP url", Alignment::Left)
                .input_type(InputType::Text)
                .value(host),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputWebDAVUri {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::WebDAVUriBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::WebDAVUriBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::WebDAVUriBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::WebDAVUriBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

// kube

#[derive(MockComponent)]
pub struct InputKubeNamespace {
    component: Input,
    form_tab: FormTab,
}

impl InputKubeNamespace {
    pub fn new(bucket: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("namespace", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Pod namespace (optional)", Alignment::Left)
                .input_type(InputType::Text)
                .value(bucket),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputKubeNamespace {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::KubeNamespaceBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::KubeNamespaceBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::KubeNamespaceBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::KubeNamespaceBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

#[derive(MockComponent)]
pub struct InputKubeClusterUrl {
    component: Input,
    form_tab: FormTab,
}

impl InputKubeClusterUrl {
    pub fn new(bucket: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder(
                    "cluster url",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title("Kube cluster url (optional)", Alignment::Left)
                .input_type(InputType::Text)
                .value(bucket),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputKubeClusterUrl {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::KubeClusterUrlBlurDown)),
            FormTab::HostBridge => {
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::KubeClusterUrlBlurDown))
            }
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::KubeClusterUrlBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::KubeClusterUrlBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

#[derive(MockComponent)]
pub struct InputKubeUsername {
    component: Input,
    form_tab: FormTab,
}

impl InputKubeUsername {
    pub fn new(bucket: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("username", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Kube username (optional)", Alignment::Left)
                .input_type(InputType::Text)
                .value(bucket),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputKubeUsername {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::KubeUsernameBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::KubeUsernameBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::KubeUsernameBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::KubeUsernameBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

#[derive(MockComponent)]
pub struct InputKubeClientCert {
    component: Input,
    form_tab: FormTab,
}

impl InputKubeClientCert {
    pub fn new(bucket: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder(
                    "/home/user/.kube/client.crt",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title("Kube client cert path (optional)", Alignment::Left)
                .input_type(InputType::Text)
                .value(bucket),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputKubeClientCert {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::KubeClientCertBlurDown)),
            FormTab::HostBridge => {
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::KubeClientCertBlurDown))
            }
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::KubeClientCertBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::KubeClientCertBlurUp)),
        };

        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

#[derive(MockComponent)]
pub struct InputKubeClientKey {
    component: Input,
    form_tab: FormTab,
}

impl InputKubeClientKey {
    pub fn new(bucket: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder(
                    "/home/user/.kube/client.key",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title("Kube client key path (optional)", Alignment::Left)
                .input_type(InputType::Text)
                .value(bucket),
            form_tab,
        }
    }
}

impl Component<Msg, NoUserEvent> for InputKubeClientKey {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::KubeClientKeyBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::KubeClientKeyBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::KubeClientKeyBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::KubeClientKeyBlurUp)),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

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
            // NOTE: escaped control sequence
            code: Key::Char('c') | Key::Char('h') | Key::Char('r') | Key::Char('s'),
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
