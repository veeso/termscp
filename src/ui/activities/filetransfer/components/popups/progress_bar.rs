use tui_realm_stdlib::components::Gauge;
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{BorderSides, BorderType, Borders, Color, HorizontalAlignment, Title};

use crate::ui::activities::filetransfer::{Msg, TransferMsg};

#[derive(Component)]
pub struct TransferProgressBar {
    component: Gauge,
}

impl TransferProgressBar {
    /// Build a gauge. `sides` selects which borders to draw so two gauges can be
    /// stacked into a single seamless panel (e.g. the upper bar omits its bottom
    /// edge and the lower bar its top edge).
    pub fn new<S: Into<String>>(
        prog: f64,
        label: S,
        title: S,
        color: Color,
        sides: BorderSides,
    ) -> Self {
        let mut component = Gauge::default()
            .borders(
                Borders::default()
                    .modifiers(BorderType::Rounded)
                    .sides(sides),
            )
            .foreground(color)
            .label(label)
            .progress(prog);
        // Only set a title when there's actually text. An empty title still
        // registers as a top-positioned title in the block, which forces a 1-row
        // top inset even when the TOP border is omitted (see `Block::inner`). On
        // the bottom bar — whose top edge is dropped to join the seam — that
        // inset would shrink its inner area to 1 row while the top bar keeps 2,
        // making the two gauges visibly unequal in height.
        let title = title.into();
        if !title.is_empty() {
            component = component.title(Title::from(title).alignment(HorizontalAlignment::Center));
        }
        Self { component }
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
