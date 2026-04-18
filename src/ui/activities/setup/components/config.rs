//! ## Config
//!
//! config tab components

use tui_realm_stdlib::components::{Input, Radio};
use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{
    BorderType, Borders, Color, HorizontalAlignment, InputType, Style, TextModifiers, Title,
};

use super::{ConfigMsg, Msg};
use crate::explorer::GroupDirs as GroupDirsEnum;
use crate::filetransfer::FileTransferProtocol;
use crate::ui::activities::setup::{
    RADIO_PROTOCOL_FTP, RADIO_PROTOCOL_FTPS, RADIO_PROTOCOL_KUBE, RADIO_PROTOCOL_S3,
    RADIO_PROTOCOL_SCP, RADIO_PROTOCOL_SFTP, RADIO_PROTOCOL_SMB, RADIO_PROTOCOL_WEBDAV,
};
use crate::utils::parser::parse_bytesize;

// -- components

#[derive(Component)]
pub struct CheckUpdates {
    component: Radio,
}

impl CheckUpdates {
    pub fn new(enabled: bool) -> Self {
        Self {
            component: Radio::default()
                .highlight_style(
                    Style::default()
                        .fg(Color::LightYellow)
                        .add_modifier(TextModifiers::REVERSED),
                )
                .borders(
                    Borders::default()
                        .color(Color::LightYellow)
                        .modifiers(BorderType::Rounded),
                )
                .choices(["Yes", "No"])
                .rewind(true)
                .title(Title::from("Check for updates?").alignment(HorizontalAlignment::Left))
                .value(usize::from(!enabled)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for CheckUpdates {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        handle_radio_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::CheckUpdatesBlurDown),
            Msg::Config(ConfigMsg::CheckUpdatesBlurUp),
        )
    }
}

#[derive(Component)]
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
                .title(Title::from("Default protocol").alignment(HorizontalAlignment::Left))
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

impl AppComponent<Msg, NoUserEvent> for DefaultProtocol {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        handle_radio_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::DefaultProtocolBlurDown),
            Msg::Config(ConfigMsg::DefaultProtocolBlurUp),
        )
    }
}

#[derive(Component)]
pub struct GroupDirs {
    component: Radio,
}

impl GroupDirs {
    pub fn new(opt: Option<GroupDirsEnum>) -> Self {
        Self {
            component: Radio::default()
                .highlight_style(
                    Style::default()
                        .fg(Color::LightMagenta)
                        .add_modifier(TextModifiers::REVERSED),
                )
                .borders(
                    Borders::default()
                        .color(Color::LightMagenta)
                        .modifiers(BorderType::Rounded),
                )
                .choices(["Display first", "Display last", "No"])
                .rewind(true)
                .title(Title::from("Group directories").alignment(HorizontalAlignment::Left))
                .value(match opt {
                    Some(GroupDirsEnum::First) => 0,
                    Some(GroupDirsEnum::Last) => 1,
                    None => 2,
                }),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for GroupDirs {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        handle_radio_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::GroupDirsBlurDown),
            Msg::Config(ConfigMsg::GroupDirsBlurUp),
        )
    }
}

#[derive(Component)]
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
                .title(
                    Title::from("Show hidden files? (by default)")
                        .alignment(HorizontalAlignment::Left),
                )
                .value(usize::from(!enabled)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for HiddenFiles {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        handle_radio_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::HiddenFilesBlurDown),
            Msg::Config(ConfigMsg::HiddenFilesBlurUp),
        )
    }
}

#[derive(Component)]
pub struct NotificationsEnabled {
    component: Radio,
}

impl NotificationsEnabled {
    pub fn new(enabled: bool) -> Self {
        Self {
            component: Radio::default()
                .highlight_style(
                    Style::default()
                        .fg(Color::LightRed)
                        .add_modifier(TextModifiers::REVERSED),
                )
                .borders(
                    Borders::default()
                        .color(Color::LightRed)
                        .modifiers(BorderType::Rounded),
                )
                .choices(["Yes", "No"])
                .rewind(true)
                .title(Title::from("Enable notifications?").alignment(HorizontalAlignment::Left))
                .value(usize::from(!enabled)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for NotificationsEnabled {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        handle_radio_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::NotificationsEnabledBlurDown),
            Msg::Config(ConfigMsg::NotificationsEnabledBlurUp),
        )
    }
}

#[derive(Component)]
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
                .title(
                    Title::from("Prompt when replacing existing files?")
                        .alignment(HorizontalAlignment::Left),
                )
                .value(usize::from(!enabled)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for PromptOnFileReplace {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        handle_radio_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::PromptOnFileReplaceBlurDown),
            Msg::Config(ConfigMsg::PromptOnFileReplaceBlurUp),
        )
    }
}

#[derive(Component)]
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
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "{NAME:36} {PEX} {SIZE} {MTIME:17:%b %d %Y %H:%M}",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(
                    Title::from("File formatter syntax (local)")
                        .alignment(HorizontalAlignment::Left),
                )
                .value(value),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for LocalFileFmt {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::LocalFileFmtBlurDown),
            Msg::Config(ConfigMsg::LocalFileFmtBlurUp),
        )
    }
}

#[derive(Component)]
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
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "64 MB",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(
                    Title::from("Notifications: minimum transfer size")
                        .alignment(HorizontalAlignment::Left),
                )
                .value(value),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for NotificationsThreshold {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::NotificationsThresholdBlurDown),
            Msg::Config(ConfigMsg::NotificationsThresholdBlurUp),
        )
    }
}

#[derive(Component)]
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
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "{NAME:36} {PEX} {SIZE} {MTIME:17:%b %d %Y %H:%M}",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(
                    Title::from("File formatter syntax (remote)")
                        .alignment(HorizontalAlignment::Left),
                )
                .value(value),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for RemoteFileFmt {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::RemoteFileFmtBlurDown),
            Msg::Config(ConfigMsg::RemoteFileFmtBlurUp),
        )
    }
}

#[derive(Component)]
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
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "~/.ssh/config",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(Title::from("SSH configuration path").alignment(HorizontalAlignment::Left))
                .value(value),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for SshConfig {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        handle_input_ev(
            self,
            ev,
            Msg::Config(ConfigMsg::SshConfigBlurDown),
            Msg::Config(ConfigMsg::SshConfigBlurUp),
        )
    }
}

#[derive(Component)]
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
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "vim",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(Title::from("Text editor").alignment(HorizontalAlignment::Left))
                .value(value),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for TextEditor {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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
    component: &mut dyn AppComponent<Msg, NoUserEvent>,
    ev: &Event<NoUserEvent>,
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
            code: Key::Char('h' | 'r' | 's'),
            modifiers: KeyModifiers::CONTROL,
        }) => Some(Msg::None),
        Event::Keyboard(KeyEvent {
            code: Key::Char(ch),
            ..
        }) => {
            component.perform(Cmd::Type(*ch));
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
    component: &mut dyn AppComponent<Msg, NoUserEvent>,
    ev: &Event<NoUserEvent>,
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
