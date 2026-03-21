//! ## Store
//!
//! `Store` is the module which provides the Context Storage.
//! The context storage is a storage indeed which is shared between the activities thanks to the context
//! The storage can be used to store any values which should be cached or shared between activities

use std::collections::HashMap;

// -- store state

/// Store state describes a value in the store
enum StoreState {
    Str(String),   // String
    Boolean(bool), // Boolean value
    Flag,          // Empty value; used to work as a Flag (set unset)
}

// -- store

/// Store represent the context store
/// The store is a key-value hash map. Each key must be unique
/// To each key a `StoreState` is assigned
pub(crate) struct Store {
    store: HashMap<String, StoreState>,
}

impl Store {
    /// Initialize a new Store
    pub fn init() -> Self {
        Store {
            store: HashMap::new(),
        }
    }

    // -- getters

    /// Get string from store
    pub fn get_string(&self, key: &str) -> Option<&str> {
        match self.store.get(key) {
            Some(StoreState::Str(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    /// get boolean from store
    pub fn get_boolean(&self, key: &str) -> Option<bool> {
        match self.store.get(key) {
            Some(StoreState::Boolean(b)) => Some(*b),
            _ => None,
        }
    }

    /// Check if a state is set in the store
    pub fn isset(&self, key: &str) -> bool {
        self.store.contains_key(key)
    }

    // -- setters

    /// Set string into the store
    pub fn set_string(&mut self, key: &str, val: String) {
        self.store.insert(key.to_string(), StoreState::Str(val));
    }

    /// Set boolean
    pub fn set_boolean(&mut self, key: &str, val: bool) {
        self.store.insert(key.to_string(), StoreState::Boolean(val));
    }

    /// Set a key as a flag; has no value
    pub fn set(&mut self, key: &str) {
        self.store.insert(key.to_string(), StoreState::Flag);
    }

    // -- Consumers
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_ui_store() {
        // Create store
        let mut store: Store = Store::init();
        // Test string
        store.set_string("test", String::from("hello"));
        assert_eq!(*store.get_string("test").as_ref().unwrap(), "hello");
        // Test boolean
        store.set_boolean("bool", true);
        assert_eq!(store.get_boolean("bool").unwrap(), true);
        // Test flag
        store.set("myflag");
        assert_eq!(store.isset("myflag"), true);
        // Test unexisting
        assert!(store.get_boolean("unexisting-key").is_none());
        assert!(store.get_string("unexisting-key").is_none());
    }
}
