use tui_realm_stdlib::components::Paragraph;
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{BorderType, Borders, Color, HorizontalAlignment, SpanStatic};
use tuirealm::ratatui::text::Text;

use crate::ui::activities::filetransfer::{Msg, TransferMsg};

#[derive(Component)]
pub struct WaitPopup {
    component: Paragraph,
}

impl WaitPopup {
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

impl AppComponent<Msg, NoUserEvent> for WaitPopup {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(Component)]
pub struct WalkdirWaitPopup {
    component: Paragraph,
}

impl WalkdirWaitPopup {
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
                .text(Text::from_iter([
                    SpanStatic::from(text.as_ref().to_string()),
                    SpanStatic::from("Press 'CTRL+C' to abort"),
                ]))
                .wrap_trim(true),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for WalkdirWaitPopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        if matches!(
            ev,
            Event::Keyboard(KeyEvent {
                code: Key::Char('c'),
                modifiers: KeyModifiers::CONTROL
            })
        ) {
            Some(Msg::Transfer(TransferMsg::AbortWalkdir))
        } else {
            None
        }
    }
}
