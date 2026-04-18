use tui_realm_stdlib::components::Paragraph;
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{BorderType, Borders, Color, HorizontalAlignment, SpanStatic};
use tuirealm::ratatui::text::Text;

use crate::ui::activities::filetransfer::{Msg, UiMsg};

#[derive(Component)]
pub struct ErrorPopup {
    component: Paragraph,
}

impl ErrorPopup {
    pub fn new<S: AsRef<str>>(text: S, color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment_horizontal(HorizontalAlignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .text(Text::from_iter([SpanStatic::from(
                    text.as_ref().to_string(),
                )]))
                .wrap_trim(true),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for ErrorPopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Ui(UiMsg::CloseErrorPopup)),
            _ => None,
        }
    }
}

#[derive(Component)]
pub struct FatalPopup {
    component: Paragraph,
}

impl FatalPopup {
    pub fn new<S: AsRef<str>>(text: S, color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment_horizontal(HorizontalAlignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .text(Text::from_iter([SpanStatic::from(
                    text.as_ref().to_string(),
                )]))
                .wrap_trim(true),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for FatalPopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Ui(UiMsg::CloseFatalPopup)),
            _ => None,
        }
    }
}
