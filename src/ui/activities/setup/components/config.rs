//! ## Config
//!
//! config tab components

use tui_realm_stdlib::{Input, Radio};
use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color, InputType, Style};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

use super::{ConfigMsg, Msg};
use crate::explorer::GroupDirs as GroupDirsEnum;
use crate::filetransfer::FileTransferProtocol;
use crate::ui::activities::setup::{
    RADIO_PROTOCOL_FTP, RADIO_PROTOCOL_FTPS, RADIO_PROTOCOL_KUBE, RADIO_PROTOCOL_S3,
    RADIO_PROTOCOL_SCP, RADIO_PROTOCOL_SFTP, RADIO_PROTOCOL_SMB, RADIO_PROTOCOL_WEBDAV,
};
use crate::utils::parser::parse_bytesize;

// -- components

#[derive(MockComponent)]
pub struct CheckUpdates {
    component: Radio,
}

impl CheckUpdates {
    pub fn new(enabled: bool) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(Color::LightYellow)
                        .modifiers(BorderType::Rounded),
                )
                .choices(["Yes", "No"])
                .foreground(Color::LightYellow)
                .rewind(true)
                .title("Check for updates?", Alignment::Left)
                .value(usize::from(!enabled)),
        }
    }
}

impl Component<Msg, NoUserEvent> for CheckUpdates {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_radio_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::CheckUpdatesBlurDown),
            Msg::Config(ConfigMsg::CheckUpdatesBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct DefaultProtocol {
    component: Radio,
}

impl DefaultProtocol {
    pub fn new(protocol: FileTransferProtocol) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(Color::Cyan)
                        .modifiers(BorderType::Rounded),
                )
                .choices(["SFTP", "SCP", "FTP", "FTPS", "Kube", "S3", "SMB", "WebDAV"])
                .foreground(Color::Cyan)
                .rewind(true)
                .title("Default protocol", Alignment::Left)
                .value(match protocol {
                    FileTransferProtocol::Sftp => RADIO_PROTOCOL_SFTP,
                    FileTransferProtocol::Scp => RADIO_PROTOCOL_SCP,
                    FileTransferProtocol::Ftp(false) => RADIO_PROTOCOL_FTP,
                    FileTransferProtocol::Ftp(true) => RADIO_PROTOCOL_FTPS,
                    FileTransferProtocol::Kube => RADIO_PROTOCOL_KUBE,
                    FileTransferProtocol::AwsS3 => RADIO_PROTOCOL_S3,
                    FileTransferProtocol::Smb => RADIO_PROTOCOL_SMB,
                    FileTransferProtocol::WebDAV => RADIO_PROTOCOL_WEBDAV,
                }),
        }
    }
}

impl Component<Msg, NoUserEvent> for DefaultProtocol {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_radio_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::DefaultProtocolBlurDown),
            Msg::Config(ConfigMsg::DefaultProtocolBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct GroupDirs {
    component: Radio,
}

impl GroupDirs {
    pub fn new(opt: Option<GroupDirsEnum>) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(Color::LightMagenta)
                        .modifiers(BorderType::Rounded),
                )
                .choices(["Display first", "Display last", "No"])
                .foreground(Color::LightMagenta)
                .rewind(true)
                .title("Group directories", Alignment::Left)
                .value(match opt {
                    Some(GroupDirsEnum::First) => 0,
                    Some(GroupDirsEnum::Last) => 1,
                    None => 2,
                }),
        }
    }
}

impl Component<Msg, NoUserEvent> for GroupDirs {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_radio_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::GroupDirsBlurDown),
            Msg::Config(ConfigMsg::GroupDirsBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct HiddenFiles {
    component: Radio,
}

impl HiddenFiles {
    pub fn new(enabled: bool) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(Color::LightRed)
                        .modifiers(BorderType::Rounded),
                )
                .choices(["Yes", "No"])
                .foreground(Color::LightRed)
                .rewind(true)
                .title("Show hidden files? (by default)", Alignment::Left)
                .value(usize::from(!enabled)),
        }
    }
}

impl Component<Msg, NoUserEvent> for HiddenFiles {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_radio_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::HiddenFilesBlurDown),
            Msg::Config(ConfigMsg::HiddenFilesBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct NotificationsEnabled {
    component: Radio,
}

impl NotificationsEnabled {
    pub fn new(enabled: bool) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(Color::LightRed)
                        .modifiers(BorderType::Rounded),
                )
                .choices(["Yes", "No"])
                .foreground(Color::LightRed)
                .rewind(true)
                .title("Enable notifications?", Alignment::Left)
                .value(usize::from(!enabled)),
        }
    }
}

impl Component<Msg, NoUserEvent> for NotificationsEnabled {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_radio_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::NotificationsEnabledBlurDown),
            Msg::Config(ConfigMsg::NotificationsEnabledBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct PromptOnFileReplace {
    component: Radio,
}

impl PromptOnFileReplace {
    pub fn new(enabled: bool) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(Color::LightBlue)
                        .modifiers(BorderType::Rounded),
                )
                .choices(["Yes", "No"])
                .foreground(Color::LightBlue)
                .rewind(true)
                .title("Prompt when replacing existing files?", Alignment::Left)
                .value(usize::from(!enabled)),
        }
    }
}

impl Component<Msg, NoUserEvent> for PromptOnFileReplace {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_radio_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::PromptOnFileReplaceBlurDown),
            Msg::Config(ConfigMsg::PromptOnFileReplaceBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct LocalFileFmt {
    component: Input,
}

impl LocalFileFmt {
    pub fn new(value: &str) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(Color::LightGreen)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::LightGreen)
                .input_type(InputType::Text)
                .placeholder(
                    "{NAME:36} {PEX} {SIZE} {MTIME:17:%b %d %Y %H:%M}",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title("File formatter syntax (local)", Alignment::Left)
                .value(value),
        }
    }
}

impl Component<Msg, NoUserEvent> for LocalFileFmt {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::LocalFileFmtBlurDown),
            Msg::Config(ConfigMsg::LocalFileFmtBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct NotificationsThreshold {
    component: Input,
}

impl NotificationsThreshold {
    pub fn new(value: &str) -> Self {
        // -- validators
        fn validate(bytes: &str) -> bool {
            parse_bytesize(bytes).is_some()
        }
        fn char_valid(_input: &str, incoming: char) -> bool {
            incoming.is_ascii_digit() || ['B', 'K', 'M', 'G', 'T', 'P'].contains(&incoming)
        }
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(Color::LightYellow)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::LightYellow)
                .invalid_style(Style::default().fg(Color::Red))
                .input_type(InputType::Custom(validate, char_valid))
                .placeholder("64 MB", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Notifications: minimum transfer size", Alignment::Left)
                .value(value),
        }
    }
}

impl Component<Msg, NoUserEvent> for NotificationsThreshold {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::NotificationsThresholdBlurDown),
            Msg::Config(ConfigMsg::NotificationsThresholdBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct RemoteFileFmt {
    component: Input,
}

impl RemoteFileFmt {
    pub fn new(value: &str) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(Color::Cyan)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::Cyan)
                .input_type(InputType::Text)
                .placeholder(
                    "{NAME:36} {PEX} {SIZE} {MTIME:17:%b %d %Y %H:%M}",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title("File formatter syntax (remote)", Alignment::Left)
                .value(value),
        }
    }
}

impl Component<Msg, NoUserEvent> for RemoteFileFmt {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::RemoteFileFmtBlurDown),
            Msg::Config(ConfigMsg::RemoteFileFmtBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct SshConfig {
    component: Input,
}

impl SshConfig {
    pub fn new(value: &str) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(Color::LightBlue)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::LightBlue)
                .input_type(InputType::Text)
                .placeholder(
                    "~/.ssh/config",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title("SSH configuration path", Alignment::Left)
                .value(value),
        }
    }
}

impl Component<Msg, NoUserEvent> for SshConfig {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::SshConfigBlurDown),
            Msg::Config(ConfigMsg::SshConfigBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct TextEditor {
    component: Input,
}

impl TextEditor {
    pub fn new(value: &str) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(Color::LightGreen)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::LightGreen)
                .input_type(InputType::Text)
                .placeholder("vim", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Text editor", Alignment::Left)
                .value(value),
        }
    }
}

impl Component<Msg, NoUserEvent> for TextEditor {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::TextEditorBlurDown),
            Msg::Config(ConfigMsg::TextEditorBlurUp),
        )
    }
}

// -- event handler

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
            // NOTE: escaped control sequence
            code: Key::Char('h') | Key::Char('r') | Key::Char('s'),
            modifiers: KeyModifiers::CONTROL,
        }) => Some(Msg::None),
        Event::Keyboard(KeyEvent {
            code: Key::Char(ch),
            ..
        }) => {
            component.perform(Cmd::Type(ch));
            Some(Msg::Config(ConfigMsg::ConfigChanged))
        }
        Event::Keyboard(KeyEvent {
            code: Key::Down, ..
        }) => Some(on_key_down),
        Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(on_key_up),
        _ => None,
    }
}

fn handle_radio_ev(
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
            Some(Msg::Config(ConfigMsg::ConfigChanged))
        }
        Event::Keyboard(KeyEvent {
            code: Key::Right, ..
        }) => {
            component.perform(Cmd::Move(Direction::Right));
            Some(Msg::Config(ConfigMsg::ConfigChanged))
        }
        Event::Keyboard(KeyEvent {
            code: Key::Down, ..
        }) => Some(on_key_down),
        Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(on_key_up),
        _ => None,
    }
}
