use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};

// 1:1 map to tuirealm, but serializing to literal words instead of ints
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize, Hash)]
enum Modifier {
    #[serde(rename = "Ctrl")]
    Control = KeyModifiers::CONTROL.bits() as isize,
    #[serde(rename = "Alt")]
    Alt = KeyModifiers::ALT.bits() as isize,
    #[serde(rename = "Shift")]
    Shift = KeyModifiers::SHIFT.bits() as isize,
}

// Custom struct to make the deserialize one as a single toml entry per binding, as tuirealm is not flattened
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct KeyBinding {
    #[serde(flatten)]
    code: Key,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    modifiers: HashSet<Modifier>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Bookmarks {
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub load: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub delete: Vec<KeyEvent>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Recents {
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub load: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub delete: Vec<KeyEvent>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Auth {
    pub bookmarks: Bookmarks,
    pub recents: Recents,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub help: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub enter_setup: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub switch_tab: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub right: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub left: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub save_bookmark: Vec<KeyEvent>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct KeyBindings {
    pub auth: Auth,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub close: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub up: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub down: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub left: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub right: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub confirm: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub yes: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub no: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub page_down: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub page_up: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub begin: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub end: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub switch_left: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub switch_right: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub switch_down: Vec<KeyEvent>,
    #[serde(with = "key_event_vec", default, skip_serializing_if = "Vec::is_empty")]
    pub switch_up: Vec<KeyEvent>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            close: vec![KeyEvent::new(Key::Esc, KeyModifiers::NONE)],
            up: vec![KeyEvent::new(Key::Up, KeyModifiers::NONE)],
            down: vec![KeyEvent::new(Key::Down, KeyModifiers::NONE)],
            left: vec![KeyEvent::new(Key::Left, KeyModifiers::NONE)],
            right: vec![KeyEvent::new(Key::Right, KeyModifiers::NONE)],
            yes: vec![KeyEvent::new(Key::Char('y'), KeyModifiers::NONE)],
            no: vec![KeyEvent::new(Key::Char('n'), KeyModifiers::NONE)],
            confirm: vec![KeyEvent::new(Key::Enter, KeyModifiers::NONE)],
            page_down: vec![KeyEvent::new(Key::PageDown, KeyModifiers::NONE)],
            page_up: vec![KeyEvent::new(Key::PageUp, KeyModifiers::NONE)],
            begin: vec![KeyEvent::new(Key::Home, KeyModifiers::NONE)],
            end: vec![KeyEvent::new(Key::End, KeyModifiers::NONE)],
            switch_down: vec![KeyEvent::new(Key::Tab, KeyModifiers::NONE)],
            switch_up: vec![KeyEvent::new(Key::Tab, KeyModifiers::SHIFT)],
            switch_left: vec![KeyEvent::new(Key::Left, KeyModifiers::NONE)],
            switch_right: vec![KeyEvent::new(Key::Right, KeyModifiers::NONE)],
            auth: Auth {
                bookmarks: Bookmarks {
                    load: vec![KeyEvent::new(Key::Enter, KeyModifiers::NONE)],
                    delete: vec![KeyEvent::new(Key::Delete, KeyModifiers::NONE)],
                },
                recents: Recents {
                    load: vec![KeyEvent::new(Key::Enter, KeyModifiers::NONE)],
                    delete: vec![KeyEvent::new(Key::Delete, KeyModifiers::NONE)],
                },
                help: vec![
                    KeyEvent::new(Key::Char('h'), KeyModifiers::CONTROL),
                    KeyEvent::new(Key::Function(1), KeyModifiers::NONE),
                ],
                enter_setup: vec![KeyEvent::new(Key::Char('c'), KeyModifiers::CONTROL)],
                switch_tab: vec![KeyEvent::new(Key::Tab, KeyModifiers::NONE)],
                right: vec![KeyEvent::new(Key::Right, KeyModifiers::NONE)],
                left: vec![KeyEvent::new(Key::Left, KeyModifiers::NONE)],
                save_bookmark: vec![KeyEvent::new(Key::Char('s'), KeyModifiers::CONTROL)],
            },
        }
    }
}

impl From<KeyEvent> for KeyBinding {
    fn from(value: KeyEvent) -> Self {
        let mut modifiers = HashSet::new();
        if value.modifiers.bits() & Modifier::Control as u8 != 0 {
            modifiers.insert(Modifier::Control);
        }
        if value.modifiers.bits() & Modifier::Alt as u8 != 0 {
            modifiers.insert(Modifier::Alt);
        }
        if value.modifiers.bits() & Modifier::Shift as u8 != 0 {
            modifiers.insert(Modifier::Shift);
        }
        KeyBinding {
            code: value.code,
            modifiers: modifiers,
        }
    }
}

impl Into<KeyEvent> for KeyBinding {
    fn into(self) -> KeyEvent {
        let as_bits = self
            .modifiers
            .iter()
            .map(|el| *el as isize)
            .reduce(|el, acc| el | acc)
            .unwrap_or(0) as u8;

        KeyEvent::new(
            self.code,
            KeyModifiers::from_bits(as_bits).unwrap_or(KeyModifiers::NONE),
        )
    }
}

mod key_event_vec {
    use serde::{Deserializer, Serializer};

    use super::*;

    pub fn serialize<S>(key_events: &Vec<KeyEvent>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let key_bindings: Vec<KeyBinding> = key_events
            .iter()
            .map(|event| KeyBinding::from(*event)) // Use KeyBinding::from(KeyEvent)
            .collect();

        key_bindings.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<KeyEvent>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let key_bindings = Vec::<KeyBinding>::deserialize(deserializer)?;

        let key_events: Vec<KeyEvent> = key_bindings
            .into_iter()
            .map(|binding| binding.into())
            .collect();

        Ok(key_events)
    }
}
