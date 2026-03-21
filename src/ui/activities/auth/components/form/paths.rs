use tuirealm::event::NoUserEvent;
use tuirealm::{Component, MockComponent};

use super::*;

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
