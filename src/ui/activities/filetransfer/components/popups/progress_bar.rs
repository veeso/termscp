use tui_realm_stdlib::components::Gauge;
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{BorderType, Borders, Color, HorizontalAlignment, Title};

use crate::ui::activities::filetransfer::{Msg, TransferMsg};

#[derive(Component)]
pub struct TransferProgressBar {
    component: Gauge,
}

impl TransferProgressBar {
    pub fn new<S: Into<String>>(prog: f64, label: S, title: S, color: Color) -> Self {
        Self {
            component: Gauge::default()
                .borders(Borders::default().modifiers(BorderType::Rounded))
                .foreground(color)
                .label(label)
                .progress(prog)
                .title(Title::from(title.into()).alignment(HorizontalAlignment::Center)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for TransferProgressBar {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        if matches!(
            ev,
            Event::Keyboard(KeyEvent {
                code: Key::Char('c'),
                modifiers: KeyModifiers::CONTROL
            })
        ) {
            Some(Msg::Transfer(TransferMsg::AbortTransfer))
        } else {
            None
        }
    }
}
