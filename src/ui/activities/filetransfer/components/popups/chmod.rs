use remotefs::fs::{UnixPex, UnixPexClass};
use tui_realm_stdlib::components::Checkbox;
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{
    AttrValue, Attribute, BorderSides, Borders, Color, HorizontalAlignment, Props, Title,
};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::state::{State, StateValue};

use crate::ui::activities::filetransfer::{Msg, TransferMsg, UiMsg};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    #[default]
    User,
    Group,
    Others,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct States {
    focus: Item,
}

/// Permissions popup for chmod command
pub struct ChmodPopup {
    props: Props,
    states: States,
    title: String,
    color: Color,
    user: Checkbox,
    group: Checkbox,
    others: Checkbox,
}

/// Make checkbox values from unix pex class
fn make_pex_values(mode: UnixPexClass) -> Vec<usize> {
    let mut values = Vec::with_capacity(3);
    if mode.read() {
        values.push(0);
    }
    if mode.write() {
        values.push(1);
    }
    if mode.execute() {
        values.push(2);
    }

    values
}

impl ChmodPopup {
    pub fn new(pex: UnixPex, color: Color, title: String) -> Self {
        Self {
            props: Props::default(),
            color,
            title,
            states: States {
                focus: Item::default(),
            },
            user: Checkbox::default()
                .foreground(color)
                .choices(["Read", "Write", "Execute"])
                .title(Title::from("User").alignment(HorizontalAlignment::Left))
                .borders(Borders::default().sides(BorderSides::NONE))
                .values(&make_pex_values(pex.user()))
                .rewind(true),
            group: Checkbox::default()
                .foreground(color)
                .choices(["Read", "Write", "Execute"])
                .title(Title::from("Group").alignment(HorizontalAlignment::Left))
                .borders(Borders::default().sides(BorderSides::NONE))
                .values(&make_pex_values(pex.group()))
                .rewind(true),
            others: Checkbox::default()
                .foreground(color)
                .choices(["Read", "Write", "Execute"])
                .title(Title::from("Others").alignment(HorizontalAlignment::Left))
                .borders(Borders::default().sides(BorderSides::NONE))
                .values(&make_pex_values(pex.others()))
                .rewind(true),
        }
    }

    fn get_active_checkbox(&mut self) -> &'_ mut Checkbox {
        match self.states.focus {
            Item::Group => &mut self.group,
            Item::Others => &mut self.others,
            Item::User => &mut self.user,
        }
    }

    fn toggle_checkbox_focus(&mut self, value: bool) {
        match self.states.focus {
            Item::User => self.user.attr(Attribute::Focus, AttrValue::Flag(value)),
            Item::Group => self.group.attr(Attribute::Focus, AttrValue::Flag(value)),
            Item::Others => self.others.attr(Attribute::Focus, AttrValue::Flag(value)),
        }
    }

    fn active_checkbox_up(&mut self) {
        self.toggle_checkbox_focus(false);
        let next = match self.states.focus {
            Item::User => Item::Others,
            Item::Group => Item::User,
            Item::Others => Item::Group,
        };

        self.states.focus = next;

        self.toggle_checkbox_focus(true);
    }

    fn active_checkbox_down(&mut self) {
        self.toggle_checkbox_focus(false);
        let next = match self.states.focus {
            Item::User => Item::Group,
            Item::Group => Item::Others,
            Item::Others => Item::User,
        };

        self.states.focus = next;

        self.toggle_checkbox_focus(true);
    }

    fn checkbox_state_to_pex_class(state: State) -> UnixPexClass {
        let values: Vec<usize> = state
            .unwrap_vec()
            .into_iter()
            .map(StateValue::unwrap_usize)
            .collect();

        UnixPexClass::new(
            values.contains(&0),
            values.contains(&1),
            values.contains(&2),
        )
    }

    fn get_mode(&self) -> UnixPex {
        UnixPex::new(
            Self::checkbox_state_to_pex_class(self.user.state()),
            Self::checkbox_state_to_pex_class(self.group.state()),
            Self::checkbox_state_to_pex_class(self.others.state()),
        )
    }
}

impl Component for ChmodPopup {
    fn attr(&mut self, attr: tuirealm::props::Attribute, value: AttrValue) {
        self.props.set(attr, value.clone());

        if attr == Attribute::Focus {
            self.get_active_checkbox().attr(attr, value);
        } else {
            self.user.attr(attr, value.clone());
            self.group.attr(attr, value.clone());
            self.others.attr(attr, value);
        }
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Left | Direction::Right) => {
                self.get_active_checkbox().perform(cmd)
            }
            Cmd::Move(Direction::Up) => {
                self.active_checkbox_up();
                CmdResult::NoChange
            }
            Cmd::Move(Direction::Down) => {
                self.active_checkbox_down();
                CmdResult::NoChange
            }
            Cmd::Toggle => self.get_active_checkbox().perform(cmd),
            Cmd::Submit => CmdResult::Submit(self.state()),
            _ => CmdResult::NoChange,
        }
    }

    fn query<'a>(
        &'a self,
        attr: tuirealm::props::Attribute,
    ) -> Option<tuirealm::props::QueryResult<'a>> {
        self.props.get_for_query(attr)
    }

    fn state(&self) -> State {
        State::Single(StateValue::U32(self.get_mode().into()))
    }

    fn view(
        &mut self,
        frame: &mut tuirealm::ratatui::Frame,
        area: tuirealm::ratatui::layout::Rect,
    ) {
        if !self
            .props
            .get(Attribute::Display)
            .and_then(AttrValue::as_flag)
            .unwrap_or(true)
        {
            return;
        }
        let chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(area);

        let focus = self
            .props
            .get(Attribute::Focus)
            .and_then(AttrValue::as_flag)
            .unwrap_or(false);

        let div_title = Title::from(self.title.clone()).alignment(HorizontalAlignment::Center);

        let div = tui_realm_stdlib::utils::get_block(
            Borders::default().color(self.color),
            Some(&div_title),
            focus,
            None,
        );

        frame.render_widget(div, area);

        self.user.view(frame, chunks[0]);
        self.group.view(frame, chunks[1]);
        self.others.view(frame, chunks[2]);
    }
}

impl AppComponent<Msg, NoUserEvent> for ChmodPopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseChmodPopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => {
                self.perform(Cmd::Move(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(' '),
                ..
            }) => {
                self.perform(Cmd::Toggle);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Transfer(TransferMsg::Chmod(self.get_mode()))),
            _ => None,
        }
    }
}
