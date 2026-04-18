use tuirealm::component::{AppComponent, Component};
use tuirealm::event::NoUserEvent;

use super::*;

#[derive(Component)]
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
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "http://localhost:8080",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(Title::from("HTTP url").alignment(HorizontalAlignment::Left))
                .input_type(InputType::Text)
                .value(host),
            form_tab,
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputWebDAVUri {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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
