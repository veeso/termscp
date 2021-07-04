//! ## Themes
//!
//! `themes` is the module which provides the themes configurations and the serializers

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
// locals
use crate::utils::fmt::fmt_color;
use crate::utils::parser::parse_color;
// ext
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};
use tuirealm::tui::style::Color;

/// ### Theme
///
/// Theme contains all the colors lookup table for termscp
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Theme {
    // -- auth
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub auth_address: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub auth_bookmarks: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub auth_password: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub auth_port: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub auth_protocol: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub auth_recents: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub auth_username: Color,
    // -- misc
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub misc_error_dialog: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub misc_input_dialog: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub misc_keys: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub misc_quit_dialog: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub misc_save_dialog: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub misc_warn_dialog: Color,
    // -- transfer
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub transfer_local_explorer_background: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub transfer_local_explorer_foreground: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub transfer_local_explorer_highlighted: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub transfer_log_background: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub transfer_log_window: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub transfer_progress_bar: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub transfer_remote_explorer_background: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub transfer_remote_explorer_foreground: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub transfer_remote_explorer_highlighted: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub transfer_status_hidden: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub transfer_status_sorting: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub transfer_status_sync_browsing: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            auth_address: Color::Yellow,
            auth_bookmarks: Color::LightGreen,
            auth_password: Color::LightBlue,
            auth_port: Color::LightCyan,
            auth_protocol: Color::LightGreen,
            auth_recents: Color::LightBlue,
            auth_username: Color::LightMagenta,
            misc_error_dialog: Color::Red,
            misc_input_dialog: Color::Reset,
            misc_keys: Color::Cyan,
            misc_quit_dialog: Color::Yellow,
            misc_save_dialog: Color::LightCyan,
            misc_warn_dialog: Color::LightRed,
            transfer_local_explorer_background: Color::Reset,
            transfer_local_explorer_foreground: Color::Reset,
            transfer_local_explorer_highlighted: Color::Yellow,
            transfer_log_background: Color::Reset,
            transfer_log_window: Color::LightGreen,
            transfer_progress_bar: Color::Green,
            transfer_remote_explorer_background: Color::Reset,
            transfer_remote_explorer_foreground: Color::Reset,
            transfer_remote_explorer_highlighted: Color::LightBlue,
            transfer_status_hidden: Color::LightBlue,
            transfer_status_sorting: Color::LightYellow,
            transfer_status_sync_browsing: Color::LightGreen,
        }
    }
}

// -- deserializer

fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    // Parse color
    match parse_color(s) {
        None => Err(DeError::custom("Invalid color")),
        Some(color) => Ok(color),
    }
}

fn serialize_color<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Convert color to string
    let s: String = fmt_color(color);
    serializer.serialize_str(s.as_str())
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_config_themes_default() {
        let theme: Theme = Theme::default();
        assert_eq!(theme.auth_address, Color::Yellow);
        assert_eq!(theme.auth_bookmarks, Color::LightGreen);
        assert_eq!(theme.auth_password, Color::LightBlue);
        assert_eq!(theme.auth_port, Color::LightCyan);
        assert_eq!(theme.auth_protocol, Color::LightGreen);
        assert_eq!(theme.auth_recents, Color::LightBlue);
        assert_eq!(theme.auth_username, Color::LightMagenta);
        assert_eq!(theme.misc_error_dialog, Color::Red);
        assert_eq!(theme.misc_input_dialog, Color::Reset);
        assert_eq!(theme.misc_keys, Color::Cyan);
        assert_eq!(theme.misc_quit_dialog, Color::Yellow);
        assert_eq!(theme.misc_save_dialog, Color::LightCyan);
        assert_eq!(theme.misc_warn_dialog, Color::LightRed);
        assert_eq!(theme.transfer_local_explorer_background, Color::Reset);
        assert_eq!(theme.transfer_local_explorer_foreground, Color::Reset);
        assert_eq!(theme.transfer_local_explorer_highlighted, Color::Yellow);
        assert_eq!(theme.transfer_log_background, Color::Reset);
        assert_eq!(theme.transfer_log_window, Color::LightGreen);
        assert_eq!(theme.transfer_progress_bar, Color::Green);
        assert_eq!(theme.transfer_remote_explorer_background, Color::Reset);
        assert_eq!(theme.transfer_remote_explorer_foreground, Color::Reset);
        assert_eq!(theme.transfer_remote_explorer_highlighted, Color::LightBlue);
        assert_eq!(theme.transfer_status_hidden, Color::LightBlue);
        assert_eq!(theme.transfer_status_sorting, Color::LightYellow);
        assert_eq!(theme.transfer_status_sync_browsing, Color::LightGreen);
    }
}
