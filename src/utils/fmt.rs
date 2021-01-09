//! ## Fmt
//!
//! `fmt` is the module which provides utilities for formatting

/*
*
*   Copyright (C) 2020-2021Christian Visintin - christian.visintin1997@gmail.com
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

extern crate chrono;
extern crate textwrap;

use chrono::prelude::*;
use std::time::{Duration, SystemTime};

/// ### fmt_pex
///
/// Convert 3 bytes of permissions value into ls notation (e.g. rwx-wx--x)
pub fn fmt_pex(owner: u8, group: u8, others: u8) -> String {
    let mut mode: String = String::with_capacity(9);
    let read: u8 = (owner >> 2) & 0x1;
    let write: u8 = (owner >> 1) & 0x1;
    let exec: u8 = owner & 0x1;
    mode.push_str(match read {
        1 => "r",
        _ => "-",
    });
    mode.push_str(match write {
        1 => "w",
        _ => "-",
    });
    mode.push_str(match exec {
        1 => "x",
        _ => "-",
    });
    let read: u8 = (group >> 2) & 0x1;
    let write: u8 = (group >> 1) & 0x1;
    let exec: u8 = group & 0x1;
    mode.push_str(match read {
        1 => "r",
        _ => "-",
    });
    mode.push_str(match write {
        1 => "w",
        _ => "-",
    });
    mode.push_str(match exec {
        1 => "x",
        _ => "-",
    });
    let read: u8 = (others >> 2) & 0x1;
    let write: u8 = (others >> 1) & 0x1;
    let exec: u8 = others & 0x1;
    mode.push_str(match read {
        1 => "r",
        _ => "-",
    });
    mode.push_str(match write {
        1 => "w",
        _ => "-",
    });
    mode.push_str(match exec {
        1 => "x",
        _ => "-",
    });
    mode
}

/// ### instant_to_str
///
/// Format a `Instant` into a time string
pub fn fmt_time(time: SystemTime, fmt: &str) -> String {
    let datetime: DateTime<Local> = time.into();
    format!("{}", datetime.format(fmt))
}

/// ### fmt_millis
///
/// Format duration as {secs}.{millis}
pub fn fmt_millis(duration: Duration) -> String {
    let seconds: u128 = duration.as_millis() / 1000;
    let millis: u128 = duration.as_millis() % 1000;
    format!("{}.{:0width$}", seconds, millis, width = 3)
}

/// align_text_center
///
/// Align text to center for a given width
pub fn align_text_center(text: &str, width: u16) -> String {
    let indent_size: usize = match (width as usize) >= text.len() {
        // NOTE: The check prevents underflow
        true => (width as usize - text.len()) / 2,
        false => 0,
    };
    textwrap::indent(
        text,
        (0..indent_size).map(|_| " ").collect::<String>().as_str(),
    )
    .trim_end()
    .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_utils_fmt_pex() {
        assert_eq!(fmt_pex(7, 7, 7), String::from("rwxrwxrwx"));
        assert_eq!(fmt_pex(7, 5, 5), String::from("rwxr-xr-x"));
        assert_eq!(fmt_pex(6, 6, 6), String::from("rw-rw-rw-"));
        assert_eq!(fmt_pex(6, 4, 4), String::from("rw-r--r--"));
        assert_eq!(fmt_pex(6, 0, 0), String::from("rw-------"));
        assert_eq!(fmt_pex(0, 0, 0), String::from("---------"));
        assert_eq!(fmt_pex(4, 4, 4), String::from("r--r--r--"));
        assert_eq!(fmt_pex(1, 2, 1), String::from("--x-w---x"));
    }

    #[test]
    fn test_utils_fmt_time() {
        let system_time: SystemTime = SystemTime::from(SystemTime::UNIX_EPOCH);
        assert_eq!(
            fmt_time(system_time, "%Y-%m-%d"),
            String::from("1970-01-01")
        );
    }

    #[test]
    fn test_utils_align_text_center() {
        assert_eq!(
            align_text_center("hello world!", 24),
            String::from("      hello world!")
        );
        // Bad case
        assert_eq!(
            align_text_center("hello world!", 8),
            String::from("hello world!")
        );
    }
    #[test]
    fn test_utils_fmt_millis() {
        assert_eq!(
            fmt_millis(Duration::from_millis(2048)),
            String::from("2.048")
        );
        assert_eq!(
            fmt_millis(Duration::from_millis(8192)),
            String::from("8.192")
        );
        assert_eq!(
            fmt_millis(Duration::from_millis(18192)),
            String::from("18.192")
        );
    }
}
