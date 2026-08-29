//! ## Google Cloud Storage Form
//!
//! Input components for Google Cloud Storage authentication parameters.

use tuirealm::component::{AppComponent, Component};

use super::*;

#[derive(Component)]
pub struct InputGcsBucket {
    component: Input,
    form_tab: FormTab,
}

impl InputGcsBucket {
    pub fn new(bucket: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "my-bucket",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(Title::from("Bucket").alignment(HorizontalAlignment::Left))
                .input_type(InputType::Text)
                .value(bucket),
            form_tab,
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputGcsBucket {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let (on_key_down, on_key_up) = match self.form_tab {
            FormTab::Remote => (
                Msg::Ui(UiMsg::Remote(UiAuthFormMsg::GcsBucketBlurDown)),
                Msg::Ui(UiMsg::Remote(UiAuthFormMsg::GcsBucketBlurUp)),
            ),
            FormTab::HostBridge => (
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::GcsBucketBlurDown)),
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::GcsBucketBlurUp)),
            ),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

#[derive(Component)]
pub struct InputGcsEndpoint {
    component: Input,
    form_tab: FormTab,
}

impl InputGcsEndpoint {
    pub fn new(endpoint: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "https://storage.googleapis.com",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(Title::from("Endpoint").alignment(HorizontalAlignment::Left))
                .input_type(InputType::Text)
                .value(endpoint),
            form_tab,
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputGcsEndpoint {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let (on_key_down, on_key_up) = match self.form_tab {
            FormTab::Remote => (
                Msg::Ui(UiMsg::Remote(UiAuthFormMsg::GcsEndpointBlurDown)),
                Msg::Ui(UiMsg::Remote(UiAuthFormMsg::GcsEndpointBlurUp)),
            ),
            FormTab::HostBridge => (
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::GcsEndpointBlurDown)),
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::GcsEndpointBlurUp)),
            ),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

#[derive(Component)]
pub struct InputGcsServiceAccountKey {
    component: Input,
    form_tab: FormTab,
}

impl InputGcsServiceAccountKey {
    pub fn new(path: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "Optional service-account JSON path",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(Title::from("Service account JSON").alignment(HorizontalAlignment::Left))
                .input_type(InputType::Text)
                .value(path),
            form_tab,
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputGcsServiceAccountKey {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let (on_key_down, on_key_up) = match self.form_tab {
            FormTab::Remote => (
                Msg::Ui(UiMsg::Remote(UiAuthFormMsg::GcsServiceAccountKeyBlurDown)),
                Msg::Ui(UiMsg::Remote(UiAuthFormMsg::GcsServiceAccountKeyBlurUp)),
            ),
            FormTab::HostBridge => (
                Msg::Ui(UiMsg::HostBridge(
                    UiAuthFormMsg::GcsServiceAccountKeyBlurDown,
                )),
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::GcsServiceAccountKeyBlurUp)),
            ),
        };
        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}
