/// A simple line for the shell, which keeps track of the current
/// content and the cursor position.
#[derive(Debug, Default)]
pub struct Line {
    content: String,
    cursor: usize,
}

impl Line {
    /// Set the content of the line and reset the cursor to the end.
    pub fn set(&mut self, content: String) {
        self.cursor = content.len();
        self.content = content;
    }

    // Push a character to the line at the current cursor position.
    pub fn push(&mut self, c: char) {
        self.content.insert(self.cursor, c);
        self.cursor += c.len_utf8();
    }

    /// Take the current line content and reset the cursor.
    pub fn take(&mut self) -> String {
        self.cursor = 0;
        std::mem::take(&mut self.content)
    }

    /// Get a reference to the current line content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Move the cursor to the left, if possible.
    ///
    /// Returns `true` if the cursor was moved, `false` if it was already at the beginning.
    pub fn left(&mut self) -> bool {
        if self.cursor > 0 {
            // get the previous character length
            let prev_char_len = self
                .content
                .chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    if i < self.cursor {
                        Some(c.len_utf8())
                    } else {
                        None
                    }
                })
                .last()
                .unwrap();
            self.cursor -= prev_char_len;
            true
        } else {
            false
        }
    }

    /// Move the cursor to the right, if possible.
    ///
    /// Returns `true` if the cursor was moved, `false` if it was already at the end.
    pub fn right(&mut self) -> bool {
        if self.cursor < self.content.len() {
            // get the next character length
            let next_char_len = self.content[self.cursor..]
                .chars()
                .next()
                .unwrap()
                .len_utf8();
            self.cursor += next_char_len;
            true
        } else {
            false
        }
    }

    /// Move the cursor to the beginning of the line.
    ///
    /// Returns the previous cursor position.
    pub fn begin(&mut self) -> usize {
        std::mem::take(&mut self.cursor)
    }

    /// Move the cursor to the end of the line.
    ///
    /// Returns the difference between the previous cursor position and the new position.
    pub fn end(&mut self) -> usize {
        let diff = self.content.len() - self.cursor;
        self.cursor = self.content.len();

        diff
    }

    /// Remove the previous character from the line at the current cursor position.
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            let prev_char_len = self
                .content
                .chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    if i < self.cursor {
                        Some(c.len_utf8())
                    } else {
                        None
                    }
                })
                .last()
                .unwrap();
            self.content.remove(self.cursor - prev_char_len);
            self.cursor -= prev_char_len;
        }
    }

    /// Deletes the character at the current cursor position.
    pub fn delete(&mut self) {
        if self.cursor < self.content.len() {
            self.content.remove(self.cursor);
        }
    }

    /// Returns whether the line is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_line() {
        let mut line = Line::default();
        assert!(line.is_empty());

        line.push('H');
        line.push('e');
        line.push('l');
        line.push('l');
        line.push('o');
        assert_eq!(line.content(), "Hello");

        line.left();
        line.left();
        line.push(' ');
        assert_eq!(line.content(), "Hel lo");

        line.begin();
        line.push('W');
        assert_eq!(line.content(), "WHel lo");

        line.end();
        line.push('!');
        assert_eq!(line.content(), "WHel lo!");

        let taken = line.take();
        assert_eq!(taken, "WHel lo!");
        assert!(line.is_empty());

        line.set("New Line".to_string());
        assert_eq!(line.content(), "New Line");

        line.backspace();
        assert_eq!(line.content(), "New Lin");
        line.left();
        line.delete();
        assert_eq!(line.content(), "New Li");
        line.left();
        line.left();
        line.right();
        assert_eq!(line.content(), "New Li");
        line.end();
        assert_eq!(line.content(), "New Li");
    }

    #[test]
    fn test_should_return_whether_the_cursor_was_moved() {
        let mut line = Line::default();
        line.set("Hello".to_string());

        assert!(line.left());
        assert_eq!(line.content(), "Hello");
        assert_eq!(line.cursor, 4);

        assert!(line.left());
        assert_eq!(line.content(), "Hello");
        assert_eq!(line.cursor, 3);

        assert!(line.right());
        assert_eq!(line.content(), "Hello");
        assert_eq!(line.cursor, 4);
        assert!(line.right());
        assert_eq!(line.content(), "Hello");
        assert!(!line.right());
        assert_eq!(line.cursor, 5);
        assert!(!line.right());

        line.end();
        assert!(!line.right());
        assert_eq!(line.content(), "Hello");
        assert_eq!(line.cursor, 5);
    }

    #[test]
    fn test_should_allow_utf8_cursors() {
        let mut line = Line::default();
        line.set("Hello, 世界".to_string());
        assert_eq!(line.content(), "Hello, 世界");
        assert_eq!(line.cursor, 13); // "Hello, " is 7 bytes, "世界" is 6 bytes

        assert!(line.left());
        assert_eq!(line.content(), "Hello, 世界");
        assert_eq!(line.cursor, 10); // Move left to '世'
        assert!(line.left());
        assert_eq!(line.content(), "Hello, 世界");
        assert_eq!(line.cursor, 7); // Move left to ','
    }
}
