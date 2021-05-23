//! ## Fmt
//!
//! `fmt` is the module which provides utilities for formatting

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
extern crate chrono;
extern crate textwrap;

use chrono::prelude::*;
use std::path::{Path, PathBuf};
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

/// ### elide_path
///
/// Elide a path if longer than width
/// In this case, the path is formatted to {ANCESTOR[0]}/.../{PARENT[0]}/{BASENAME}
pub fn fmt_path_elide(p: &Path, width: usize) -> String {
    let fmt_path: String = format!("{}", p.display());
    match fmt_path.len() > width as usize {
        false => fmt_path,
        true => {
            // Elide
            let ancestors_len: usize = p.ancestors().count();
            let mut ancestors = p.ancestors();
            let mut elided_path: PathBuf = PathBuf::new();
            // If ancestors_len's size is bigger than 2, push count - 2
            if ancestors_len > 2 {
                elided_path.push(ancestors.nth(ancestors_len - 2).unwrap());
            }
            // If ancestors_len is bigger than 3, push '...' and parent too
            if ancestors_len > 3 {
                elided_path.push("...");
                if let Some(parent) = p.ancestors().nth(1) {
                    elided_path.push(parent.file_name().unwrap());
                }
            }
            // Push file_name
            if let Some(name) = p.file_name() {
                elided_path.push(name);
            }
            format!("{}", elided_path.display())
        }
    }
}

/// ### shadow_password
///
/// Return a string with the same length of input string, but each character is replaced by '*'
pub fn shadow_password(s: &str) -> String {
    (0..s.len()).map(|_| '*').collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

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

    #[test]
    #[cfg(any(target_os = "unix", target_os = "linux", target_os = "macos"))]
    fn test_utils_fmt_path_elide() {
        let p: &Path = &Path::new("/develop/pippo");
        // Under max size
        assert_eq!(fmt_path_elide(p, 16), String::from("/develop/pippo"));
        // Above max size, only one ancestor
        assert_eq!(fmt_path_elide(p, 8), String::from("/develop/pippo"));
        let p: &Path = &Path::new("/develop/pippo/foo/bar");
        assert_eq!(fmt_path_elide(p, 16), String::from("/develop/.../foo/bar"));
    }

    #[test]
    fn test_utils_fmt_shadow_password() {
        assert_eq!(shadow_password("foobar"), String::from("******"));
    }
}
