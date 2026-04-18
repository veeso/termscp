use tuirealm::component::{AppComponent, Component};
use tuirealm::event::NoUserEvent;

use super::*;

#[derive(Component)]
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
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "127.0.0.1",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(Title::from("Remote host").alignment(HorizontalAlignment::Left))
                .input_type(InputType::Text)
                .value(host),
            form_tab,
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputAddress {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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

#[derive(Component)]
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
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "22",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .input_type(InputType::UnsignedInteger)
                .input_len(5)
                .title(Title::from("Port number").alignment(HorizontalAlignment::Left))
                .value(port.to_string()),
            form_tab,
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputPort {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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

#[derive(Component)]
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
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "root",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(Title::from("Username").alignment(HorizontalAlignment::Left))
                .input_type(InputType::Text)
                .value(username),
            form_tab,
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputUsername {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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

#[derive(Component)]
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
                .title(Title::from("Password").alignment(HorizontalAlignment::Left))
                .input_type(InputType::Password('*'))
                .value(password),
            form_tab,
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputPassword {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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
