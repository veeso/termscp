use std::collections::VecDeque;

/// Shell history management module.
#[derive(Debug)]
pub struct History {
    /// Entries in the history.
    entries: VecDeque<String>,
    /// Maximum size of the history.
    max_size: usize,
    /// Current index in the history for navigation.
    index: usize,
}

impl History {
    /// Create a new [`History`] with a specified maximum size.
    pub fn new(max_size: usize) -> Self {
        History {
            entries: VecDeque::with_capacity(max_size),
            max_size,
            index: 0,
        }
    }

    /// Push a new command into the history.
    pub fn push(&mut self, cmd: &str) {
        if self.entries.len() == self.max_size {
            self.entries.pop_front();
        }
        self.entries.push_back(cmd.to_string());
        self.index = self.entries.len(); // Reset index to the end after adding a new command
    }

    /// Get the previous command in the history.
    ///
    /// Set also the index to the last command if it exists.
    pub fn previous(&mut self) -> Option<String> {
        if self.index > 0 {
            self.index -= 1;
            self.entries.get(self.index).cloned()
        } else {
            None
        }
    }

    /// Get the next command in the history.
    ///
    /// Set also the index to the next command if it exists.
    pub fn next(&mut self) -> Option<String> {
        if self.index < self.entries.len() {
            let cmd = self.entries.get(self.index).cloned();
            self.index += 1;
            cmd
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::History;

    #[test]
    fn test_history() {
        let mut history = History::new(5);
        history.push("first");
        history.push("second");
        history.push("third");

        assert_eq!(history.previous(), Some("third".to_string()));
        assert_eq!(history.previous(), Some("second".to_string()));
        assert_eq!(history.previous(), Some("first".to_string()));
        assert_eq!(history.previous(), None); // No more previous commands
        assert_eq!(history.next(), Some("first".to_string()));
        assert_eq!(history.next(), Some("second".to_string()));
        assert_eq!(history.next(), Some("third".to_string()));
        assert_eq!(history.next(), None); // No more next commands
        history.push("fourth");
        assert_eq!(history.previous(), Some("fourth".to_string()));
    }
}
