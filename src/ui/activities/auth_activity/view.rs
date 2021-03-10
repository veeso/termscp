//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

/*
*
*   Copyright (C) 2020-2021 Christian Visintin - christian.visintin1997@gmail.com
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

// Locals
use super::{AuthActivity, Context};
use crate::ui::layout::components::{
    bookmark_list::BookmarkList, input::Input, radio_group::RadioGroup, table::Table, text::Text,
};
use crate::ui::layout::props::{
    PropValue, Props, PropsBuilder, TextParts, TextSpan, TextSpanBuilder,
};
use crate::utils::fmt::align_text_center;
// Ext
use std::string::ToString;
use tui::{
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs},
};
use unicode_width::UnicodeWidthStr;

impl AuthActivity {
    /// ### init
    ///
    /// Initialize view, mounting all startup components inside the view
    pub fn init(&mut self) {
        // Header
        self.view.mount(super::COMPONENT_TEXT_HEADER, Box::new(
            Text::new(
                PropsBuilder::default().with_foreground(Color::White).with_texts(
                    TextParts::new(None, Some(vec![TextSpan::from(" _____                   ____   ____ ____  \n|_   _|__ _ __ _ __ ___ / ___| / ___|  _ \\ \n  | |/ _ \\ '__| '_ ` _ \\\\___ \\| |   | |_) |\n  | |  __/ |  | | | | | |___) | |___|  __/ \n  |_|\\___|_|  |_| |_| |_|____/ \\____|_|    \n")]))
                ).bold().build()
            )
        ));
        // Footer
        self.view.mount(super::COMPONENT_TEXT_FOOTER, Box::new(
            Text::new(
                PropsBuilder::default().with_foreground(Color::White).with_texts(
                    TextParts::new(None, Some(vec![
                        TextSpanBuilder::new("Press ").bold().build(),
                        TextSpanBuilder::new("<CTRL+H>").bold().with_foreground(Color::Cyan).build(),
                        TextSpanBuilder::new(" to show keybindings; ").bold().build(),
                        TextSpanBuilder::new("<CTRL+C>").bold().with_foreground(Color::Cyan).build(),
                        TextSpanBuilder::new(" to enter setup").bold().build(),
                    ]))
                ).build()
            )
        ));
    }

    /// ### view
    /// 
    /// Display view on canvas
    pub fn view(&mut self) {

    }

}
