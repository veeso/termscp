use tui_realm_stdlib::Paragraph;
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

use crate::ui::activities::filetransfer::{Msg, TransferMsg};

#[derive(MockComponent)]
pub struct WaitPopup {
    component: Paragraph,
}

impl WaitPopup {
    pub fn new<S: AsRef<str>>(text: S, color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .text([TextSpan::from(text.as_ref())])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for WaitPopup {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct WalkdirWaitPopup {
    component: Paragraph,
}

impl WalkdirWaitPopup {
    pub fn new<S: AsRef<str>>(text: S, color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .text([
                    TextSpan::from(text.as_ref()),
                    TextSpan::from("Press 'CTRL+C' to abort"),
                ])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for WalkdirWaitPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
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
