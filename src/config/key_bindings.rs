use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize, Hash)]
enum Modifier {
    #[serde(rename = "Ctrl")]
    Control = KeyModifiers::CONTROL.bits() as isize,
    #[serde(rename = "Alt")]
    Alt = KeyModifiers::ALT.bits() as isize,
    #[serde(rename = "Shift")]
    Shift = KeyModifiers::SHIFT.bits() as isize,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct KeyBinding {
    #[serde(flatten)]
    code: Key,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    modifiers: HashSet<Modifier>,
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

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct KeyBindings {
    // Each action becomes an optional field containing a vector of its bindings
    #[serde(default)] // Use default so the field is omitted if no [[]] tables exist
    pub exit: Vec<KeyBinding>,
    #[serde(default)]
    pub up: Vec<KeyBinding>,
    #[serde(default)]
    pub down: Vec<KeyBinding>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            exit: vec![
                KeyEvent::new(Key::Char('c'), KeyModifiers::CONTROL).into(),
                KeyEvent::new(Key::Esc, KeyModifiers::NONE).into(),
            ],
            up: vec![
                KeyEvent::new(Key::Up, KeyModifiers::NONE).into(),
                KeyEvent::new(Key::Char('k'), KeyModifiers::NONE).into(),
            ],
            down: vec![],
        }
    }
}
