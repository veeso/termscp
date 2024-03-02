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
    RADIO_PROTOCOL_FTP, RADIO_PROTOCOL_FTPS, RADIO_PROTOCOL_S3, RADIO_PROTOCOL_SCP,
    RADIO_PROTOCOL_SFTP, RADIO_PROTOCOL_SMB, RADIO_PROTOCOL_WEBDAV,
};

// -- protocol

#[derive(MockComponent)]
pub struct ProtocolRadio {
    component: Radio,
}

impl ProtocolRadio {
    pub fn new(default_protocol: FileTransferProtocol, color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .choices(if cfg!(smb) {
                    &["SFTP", "SCP", "FTP", "FTPS", "S3", "WebDAV", "SMB"]
                } else {
                    &["SFTP", "SCP", "FTP", "FTPS", "S3", "WebDAV"]
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
            RADIO_PROTOCOL_SCP => FileTransferProtocol::Scp,
            RADIO_PROTOCOL_FTP => FileTransferProtocol::Ftp(false),
            RADIO_PROTOCOL_FTPS => FileTransferProtocol::Ftp(true),
            RADIO_PROTOCOL_S3 => FileTransferProtocol::AwsS3,
            RADIO_PROTOCOL_SMB => FileTransferProtocol::Smb,
            RADIO_PROTOCOL_WEBDAV => FileTransferProtocol::WebDAV,
            _ => FileTransferProtocol::Sftp,
        }
    }

    /// Convert `FileTransferProtocol` enum into radio group index
    fn protocol_enum_to_opt(protocol: FileTransferProtocol) -> usize {
        match protocol {
            FileTransferProtocol::Sftp => RADIO_PROTOCOL_SFTP,
            FileTransferProtocol::Scp => RADIO_PROTOCOL_SCP,
            FileTransferProtocol::Ftp(false) => RADIO_PROTOCOL_FTP,
            FileTransferProtocol::Ftp(true) => RADIO_PROTOCOL_FTPS,
            FileTransferProtocol::AwsS3 => RADIO_PROTOCOL_S3,
            FileTransferProtocol::Smb => RADIO_PROTOCOL_SMB,
            FileTransferProtocol::WebDAV => RADIO_PROTOCOL_WEBDAV,
        }
    }
}

impl Component<Msg, NoUserEvent> for ProtocolRadio {
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
            }) => return Some(Msg::Ui(UiMsg::ProtocolBlurDown)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                return Some(Msg::Ui(UiMsg::ProtocolBlurUp))
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                return Some(Msg::Ui(UiMsg::ParamsFormBlur))
            }
            _ => return None,
        };
        match result {
            CmdResult::Changed(State::One(StateValue::Usize(choice))) => Some(Msg::Form(
                FormMsg::ProtocolChanged(Self::protocol_opt_to_enum(choice)),
            )),
            _ => Some(Msg::None),
        }
    }
}

// -- remote directory

#[derive(MockComponent)]
pub struct InputRemoteDirectory {
    component: Input,
}

impl InputRemoteDirectory {
    pub fn new(remote_dir: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputRemoteDirectory {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::RemoteDirectoryBlurDown),
            Msg::Ui(UiMsg::RemoteDirectoryBlurUp),
        )
    }
}

// -- remote directory

#[derive(MockComponent)]
pub struct InputLocalDirectory {
    component: Input,
}

impl InputLocalDirectory {
    pub fn new(local_dir: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputLocalDirectory {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::LocalDirectoryBlurDown),
            Msg::Ui(UiMsg::LocalDirectoryBlurUp),
        )
    }
}

// -- address

#[derive(MockComponent)]
pub struct InputAddress {
    component: Input,
}

impl InputAddress {
    pub fn new(host: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputAddress {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::AddressBlurDown),
            Msg::Ui(UiMsg::AddressBlurUp),
        )
    }
}

// -- port number

#[derive(MockComponent)]
pub struct InputPort {
    component: Input,
}

impl InputPort {
    pub fn new(port: u16, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputPort {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::PortBlurDown),
            Msg::Ui(UiMsg::PortBlurUp),
        )
    }
}

// -- username

#[derive(MockComponent)]
pub struct InputUsername {
    component: Input,
}

impl InputUsername {
    pub fn new(username: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputUsername {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::UsernameBlurDown),
            Msg::Ui(UiMsg::UsernameBlurUp),
        )
    }
}

// -- password

#[derive(MockComponent)]
pub struct InputPassword {
    component: Input,
}

impl InputPassword {
    pub fn new(password: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputPassword {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::PasswordBlurDown),
            Msg::Ui(UiMsg::PasswordBlurUp),
        )
    }
}

// -- s3 bucket

#[derive(MockComponent)]
pub struct InputS3Bucket {
    component: Input,
}

impl InputS3Bucket {
    pub fn new(bucket: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3Bucket {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::S3BucketBlurDown),
            Msg::Ui(UiMsg::S3BucketBlurUp),
        )
    }
}

// -- s3 region

#[derive(MockComponent)]
pub struct InputS3Region {
    component: Input,
}

impl InputS3Region {
    pub fn new(region: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3Region {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::S3RegionBlurDown),
            Msg::Ui(UiMsg::S3RegionBlurUp),
        )
    }
}

// -- s3 endpoint

#[derive(MockComponent)]
pub struct InputS3Endpoint {
    component: Input,
}

impl InputS3Endpoint {
    pub fn new(endpoint: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3Endpoint {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::S3EndpointBlurDown),
            Msg::Ui(UiMsg::S3EndpointBlurUp),
        )
    }
}

// -- s3 new path style

#[derive(MockComponent)]
pub struct RadioS3NewPathStyle {
    component: Radio,
}

impl RadioS3NewPathStyle {
    pub fn new(new_path_style: bool, color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .choices(&["Yes", "No"])
                .foreground(color)
                .rewind(true)
                .title("New path style", Alignment::Left)
                .value(usize::from(!new_path_style)),
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
            }) => Some(Msg::Ui(UiMsg::S3NewPathStyleBlurDown)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                Some(Msg::Ui(UiMsg::S3NewPathStyleBlurUp))
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                Some(Msg::Ui(UiMsg::ParamsFormBlur))
            }
            _ => None,
        }
    }
}

// -- s3 profile

#[derive(MockComponent)]
pub struct InputS3Profile {
    component: Input,
}

impl InputS3Profile {
    pub fn new(profile: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3Profile {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::S3ProfileBlurDown),
            Msg::Ui(UiMsg::S3ProfileBlurUp),
        )
    }
}

// -- s3 access key

#[derive(MockComponent)]
pub struct InputS3AccessKey {
    component: Input,
}

impl InputS3AccessKey {
    pub fn new(access_key: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3AccessKey {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::S3AccessKeyBlurDown),
            Msg::Ui(UiMsg::S3AccessKeyBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct InputS3SecretAccessKey {
    component: Input,
}

impl InputS3SecretAccessKey {
    pub fn new(secret_access_key: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3SecretAccessKey {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::S3SecretAccessKeyBlurDown),
            Msg::Ui(UiMsg::S3SecretAccessKeyBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct InputS3SecurityToken {
    component: Input,
}

impl InputS3SecurityToken {
    pub fn new(security_token: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3SecurityToken {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::S3SecurityTokenBlurDown),
            Msg::Ui(UiMsg::S3SecurityTokenBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct InputS3SessionToken {
    component: Input,
}

impl InputS3SessionToken {
    pub fn new(session_token: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3SessionToken {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::S3SessionTokenBlurDown),
            Msg::Ui(UiMsg::S3SessionTokenBlurUp),
        )
    }
}

fn handle_input_ev(
    component: &mut dyn Component<Msg, NoUserEvent>,
    ev: Event<NoUserEvent>,
    on_key_down: Msg,
    on_key_up: Msg,
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
        Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => Some(Msg::Ui(UiMsg::ParamsFormBlur)),
        _ => None,
    }
}

#[derive(MockComponent)]
pub struct InputSmbShare {
    component: Input,
}

impl InputSmbShare {
    pub fn new(host: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputSmbShare {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::SmbShareBlurDown),
            Msg::Ui(UiMsg::SmbShareBlurUp),
        )
    }
}

#[cfg(unix)]
#[derive(MockComponent)]
pub struct InputSmbWorkgroup {
    component: Input,
}

#[cfg(unix)]
impl InputSmbWorkgroup {
    pub fn new(host: &str, color: Color) -> Self {
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
        }
    }
}

#[cfg(unix)]
impl Component<Msg, NoUserEvent> for InputSmbWorkgroup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::SmbWorkgroupDown),
            Msg::Ui(UiMsg::SmbWorkgroupUp),
        )
    }
}

#[derive(MockComponent)]
pub struct InputWebDAVUri {
    component: Input,
}

impl InputWebDAVUri {
    pub fn new(host: &str, color: Color) -> Self {
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
        }
    }
}

impl Component<Msg, NoUserEvent> for InputWebDAVUri {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Ui(UiMsg::WebDAVUriBlurDown),
            Msg::Ui(UiMsg::WebDAVUriBlurUp),
        )
    }
}
