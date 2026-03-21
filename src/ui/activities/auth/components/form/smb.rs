use tuirealm::event::NoUserEvent;
use tuirealm::{Component, MockComponent};

use super::*;

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
