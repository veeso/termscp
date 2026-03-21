use tuirealm::command::{Cmd, Direction};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::{Component, MockComponent};

use super::*;

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
