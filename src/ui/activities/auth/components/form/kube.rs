use tuirealm::event::NoUserEvent;
use tuirealm::{Component, MockComponent};

use super::*;

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
