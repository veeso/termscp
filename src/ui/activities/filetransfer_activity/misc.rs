/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "TermSCP"
*
*   TermSCP is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   TermSCP is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with TermSCP.  If not, see <http://www.gnu.org/licenses/>.
*
*/

use super::{Color, FileTransferActivity, InputField, InputMode, LogLevel, LogRecord, PopupType};

impl FileTransferActivity {
    /// ### log
    ///
    /// Add message to log events
    pub(super) fn log(&mut self, level: LogLevel, msg: &str) {
        // Create log record
        let record: LogRecord = LogRecord::new(level, msg);
        //Check if history overflows the size
        if self.log_records.len() + 1 > self.log_size {
            self.log_records.pop_back(); // Start cleaning events from back
        }
        // Eventually push front the new record
        self.log_records.push_front(record);
        // Set log index
        self.log_index = 0;
    }

    /// ### log_and_alert
    ///
    /// Add message to log events and also display it as an alert
    pub(super) fn log_and_alert(&mut self, level: LogLevel, msg: String) {
        // Set input mode
        let color: Color = match level {
            LogLevel::Error => Color::Red,
            LogLevel::Info => Color::Green,
            LogLevel::Warn => Color::Yellow,
        };
        self.log(level, msg.as_str());
        self.input_mode = InputMode::Popup(PopupType::Alert(color, msg));
    }

    /// ### create_quit_popup
    ///
    /// Create quit popup input mode (since must be shared between different input handlers)
    pub(super) fn create_disconnect_popup(&mut self) -> InputMode {
        InputMode::Popup(PopupType::YesNo(
            String::from("Are you sure you want to disconnect?"),
            FileTransferActivity::disconnect,
            FileTransferActivity::callback_nothing_to_do,
        ))
    }

    /// ### create_quit_popup
    ///
    /// Create quit popup input mode (since must be shared between different input handlers)
    pub(super) fn create_quit_popup(&mut self) -> InputMode {
        InputMode::Popup(PopupType::YesNo(
            String::from("Are you sure you want to quit?"),
            FileTransferActivity::disconnect_and_quit,
            FileTransferActivity::callback_nothing_to_do,
        ))
    }

    /// ### switch_input_field
    ///
    /// Switch input field based on current input field
    pub(super) fn switch_input_field(&mut self) {
        self.input_field = match self.input_field {
            InputField::Explorer => InputField::Logs,
            InputField::Logs => InputField::Explorer,
        }
    }
}
