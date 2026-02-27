use tui_realm_stdlib::ProgressBar;
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderSides, BorderType, Borders, Color};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

use crate::ui::activities::filetransfer::{Msg, TransferMsg};

#[derive(MockComponent)]
pub struct ProgressBarFull {
    component: ProgressBar,
}

impl ProgressBarFull {
    pub fn new<S: Into<String>>(prog: f64, label: S, title: S, color: Color) -> Self {
        Self {
            component: ProgressBar::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .sides(BorderSides::TOP | BorderSides::LEFT | BorderSides::RIGHT),
                )
                .foreground(color)
                .label(label)
                .progress(prog)
                .title(title, Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for ProgressBarFull {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
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

#[derive(MockComponent)]
pub struct ProgressBarPartial {
    component: ProgressBar,
}

impl ProgressBarPartial {
    pub fn new<S: Into<String>>(prog: f64, label: S, title: S, color: Color) -> Self {
        Self {
            component: ProgressBar::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .sides(BorderSides::BOTTOM | BorderSides::LEFT | BorderSides::RIGHT),
                )
                .foreground(color)
                .label(label)
                .progress(prog)
                .title(title, Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for ProgressBarPartial {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
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
