//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

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

use super::{AuthActivity, InputForm};

impl AuthActivity {
    /// ### callback_nothing_to_do
    ///
    /// Self titled
    pub(super) fn callback_nothing_to_do(&mut self) {}

    /// ### callback_quit
    ///
    /// Self titled
    pub(super) fn callback_quit(&mut self) {
        self.quit = true;
    }

    /// ### callback_del_bookmark
    ///
    /// Callback which deletes recent or bookmark based on current form
    pub(super) fn callback_del_bookmark(&mut self) {
        match self.input_form {
            InputForm::Bookmarks => self.del_bookmark(self.bookmarks_idx),
            InputForm::Recents => self.del_recent(self.recents_idx),
            _ => { /* Nothing to do */ }
        }
    }

    /// ### callback_save_bookmark
    ///
    /// Callback used to save bookmark with name
    pub(super) fn callback_save_bookmark(&mut self, input: String) {
        self.save_bookmark(input);
    }
}
