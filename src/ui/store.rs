//! ## Store
//!
//! `Store` is the module which provides the Context Storage.
//! The context storage is a storage indeed which is shared between the activities thanks to the context
//! The storage can be used to store any values which should be cached or shared between activities

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use std::collections::HashMap;

// -- store state

/// ## StoreState
///
/// Store state describes a value in the store
#[allow(dead_code)]
enum StoreState {
    Str(String),     // String
    Signed(isize),   // Signed number
    Unsigned(usize), // Unsigned number
    Float(f64),      // Floating point number
    Boolean(bool),   // Boolean value
    Flag,            // Empty value; used to work as a Flag (set unset)
}

// -- store

/// ## Store
///
/// Store represent the context store
/// The store is a key-value hash map. Each key must be unique
/// To each key a `StoreState` is assigned
pub(crate) struct Store {
    store: HashMap<String, StoreState>,
}

#[allow(dead_code)]
impl Store {
    /// ### init
    ///
    /// Initialize a new Store
    pub fn init() -> Self {
        Store {
            store: HashMap::new(),
        }
    }

    // -- getters
    /// ### get_string
    ///
    /// Get string from store
    pub fn get_string(&self, key: &str) -> Option<&str> {
        match self.store.get(key) {
            Some(StoreState::Str(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    /// ### get_signed
    ///
    /// Get signed from store
    pub fn get_signed(&self, key: &str) -> Option<isize> {
        match self.store.get(key) {
            Some(StoreState::Signed(i)) => Some(*i),
            _ => None,
        }
    }

    /// ### get_unsigned
    ///
    /// Get unsigned from store
    pub fn get_unsigned(&self, key: &str) -> Option<usize> {
        match self.store.get(key) {
            Some(StoreState::Unsigned(u)) => Some(*u),
            _ => None,
        }
    }

    /// ### get_float
    ///
    /// get float from store
    pub fn get_float(&self, key: &str) -> Option<f64> {
        match self.store.get(key) {
            Some(StoreState::Float(f)) => Some(*f),
            _ => None,
        }
    }

    /// ### get_boolean
    ///
    /// get boolean from store
    pub fn get_boolean(&self, key: &str) -> Option<bool> {
        match self.store.get(key) {
            Some(StoreState::Boolean(b)) => Some(*b),
            _ => None,
        }
    }

    /// ### isset
    ///
    /// Check if a state is set in the store
    pub fn isset(&self, key: &str) -> bool {
        self.store.get(key).is_some()
    }

    // -- setters

    /// ### set_string
    ///
    /// Set string into the store
    pub fn set_string(&mut self, key: &str, val: String) {
        self.store.insert(key.to_string(), StoreState::Str(val));
    }

    /// ### set_signed
    ///
    /// Set signed number
    pub fn set_signed(&mut self, key: &str, val: isize) {
        self.store.insert(key.to_string(), StoreState::Signed(val));
    }

    /// ### set_signed
    ///
    /// Set unsigned number
    pub fn set_unsigned(&mut self, key: &str, val: usize) {
        self.store
            .insert(key.to_string(), StoreState::Unsigned(val));
    }

    /// ### set_float
    ///
    /// Set floating point number
    pub fn set_float(&mut self, key: &str, val: f64) {
        self.store.insert(key.to_string(), StoreState::Float(val));
    }

    /// ### set_boolean
    ///
    /// Set boolean
    pub fn set_boolean(&mut self, key: &str, val: bool) {
        self.store.insert(key.to_string(), StoreState::Boolean(val));
    }

    /// ### set
    ///
    /// Set a key as a flag; has no value
    pub fn set(&mut self, key: &str) {
        self.store.insert(key.to_string(), StoreState::Flag);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_ui_store() {
        // Create store
        let mut store: Store = Store::init();
        // Test string
        store.set_string("test", String::from("hello"));
        assert_eq!(*store.get_string("test").as_ref().unwrap(), "hello");
        // Test isize
        store.set_signed("number", 3005);
        assert_eq!(store.get_signed("number").unwrap(), 3005);
        store.set_signed("number", -123);
        assert_eq!(store.get_signed("number").unwrap(), -123);
        // Test usize
        store.set_unsigned("unumber", 1024);
        assert_eq!(store.get_unsigned("unumber").unwrap(), 1024);
        // Test float
        store.set_float("float", 3.33);
        assert_eq!(store.get_float("float").unwrap(), 3.33);
        // Test boolean
        store.set_boolean("bool", true);
        assert_eq!(store.get_boolean("bool").unwrap(), true);
        // Test flag
        store.set("myflag");
        assert_eq!(store.isset("myflag"), true);
        // Test unexisting
        assert!(store.get_boolean("unexisting-key").is_none());
        assert!(store.get_float("unexisting-key").is_none());
        assert!(store.get_signed("unexisting-key").is_none());
        assert!(store.get_signed("unexisting-key").is_none());
        assert!(store.get_string("unexisting-key").is_none());
        assert!(store.get_unsigned("unexisting-key").is_none());
    }
}
