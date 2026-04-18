//! ## Themes
//!
//! `themes` is the module which provides the themes configurations and the serializers

// locals
// ext
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tuirealm::ratatui::style::Color;

use crate::utils::fmt::fmt_color;
use crate::utils::parser::parse_color;

/// Theme contains all the colors lookup table for termscp
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Theme {
    // -- auth
    #[serde(serialize_with = "serialize_color")]
    pub auth_address: Color,
    #[serde(serialize_with = "serialize_color")]
    pub auth_bookmarks: Color,
    #[serde(serialize_with = "serialize_color")]
    pub auth_password: Color,
    #[serde(serialize_with = "serialize_color")]
    pub auth_port: Color,
    #[serde(serialize_with = "serialize_color")]
    pub auth_protocol: Color,
    #[serde(serialize_with = "serialize_color")]
    pub auth_recents: Color,
    #[serde(serialize_with = "serialize_color")]
    pub auth_username: Color,
    // -- misc
    #[serde(serialize_with = "serialize_color")]
    pub misc_error_dialog: Color,
    #[serde(serialize_with = "serialize_color")]
    pub misc_info_dialog: Color,
    #[serde(serialize_with = "serialize_color")]
    pub misc_input_dialog: Color,
    #[serde(serialize_with = "serialize_color")]
    pub misc_keys: Color,
    #[serde(serialize_with = "serialize_color")]
    pub misc_quit_dialog: Color,
    #[serde(serialize_with = "serialize_color")]
    pub misc_save_dialog: Color,
    #[serde(serialize_with = "serialize_color")]
    pub misc_warn_dialog: Color,
    // -- transfer
    #[serde(serialize_with = "serialize_color")]
    pub transfer_local_explorer_background: Color,
    #[serde(serialize_with = "serialize_color")]
    pub transfer_local_explorer_foreground: Color,
    #[serde(serialize_with = "serialize_color")]
    pub transfer_local_explorer_highlighted: Color,
    #[serde(serialize_with = "serialize_color")]
    pub transfer_log_background: Color,
    #[serde(serialize_with = "serialize_color")]
    pub transfer_log_window: Color,
    #[serde(serialize_with = "serialize_color")]
    pub transfer_progress_bar: Color,
    #[serde(serialize_with = "serialize_color")]
    pub transfer_remote_explorer_background: Color,
    #[serde(serialize_with = "serialize_color")]
    pub transfer_remote_explorer_foreground: Color,
    #[serde(serialize_with = "serialize_color")]
    pub transfer_remote_explorer_highlighted: Color,
    #[serde(serialize_with = "serialize_color")]
    pub transfer_status_hidden: Color,
    #[serde(serialize_with = "serialize_color")]
    pub transfer_status_sorting: Color,
    #[serde(serialize_with = "serialize_color")]
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
            misc_info_dialog: Color::LightYellow,
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
//
// Custom deserialization: every field is optional and falls back to `Theme::default()`
// when missing or when the supplied color string is invalid. This keeps user themes
// backwards compatible even when fields are added, renamed, or contain typos.

#[derive(Deserialize, Default)]
#[serde(default)]
struct ThemeFile {
    auth_address: Option<String>,
    auth_bookmarks: Option<String>,
    auth_password: Option<String>,
    auth_port: Option<String>,
    auth_protocol: Option<String>,
    auth_recents: Option<String>,
    auth_username: Option<String>,
    misc_error_dialog: Option<String>,
    misc_info_dialog: Option<String>,
    misc_input_dialog: Option<String>,
    misc_keys: Option<String>,
    misc_quit_dialog: Option<String>,
    misc_save_dialog: Option<String>,
    misc_warn_dialog: Option<String>,
    transfer_local_explorer_background: Option<String>,
    transfer_local_explorer_foreground: Option<String>,
    transfer_local_explorer_highlighted: Option<String>,
    transfer_log_background: Option<String>,
    transfer_log_window: Option<String>,
    transfer_progress_bar: Option<String>,
    // Legacy aliases for the now-consolidated `transfer_progress_bar` field.
    transfer_progress_bar_full: Option<String>,
    transfer_progress_bar_partial: Option<String>,
    transfer_remote_explorer_background: Option<String>,
    transfer_remote_explorer_foreground: Option<String>,
    transfer_remote_explorer_highlighted: Option<String>,
    transfer_status_hidden: Option<String>,
    transfer_status_sorting: Option<String>,
    transfer_status_sync_browsing: Option<String>,
}

impl ThemeFile {
    fn into_theme(self) -> Theme {
        let defaults = Theme::default();
        fn pick(value: Option<String>, fallback: Color) -> Color {
            value.as_deref().and_then(parse_color).unwrap_or(fallback)
        }
        Theme {
            auth_address: pick(self.auth_address, defaults.auth_address),
            auth_bookmarks: pick(self.auth_bookmarks, defaults.auth_bookmarks),
            auth_password: pick(self.auth_password, defaults.auth_password),
            auth_port: pick(self.auth_port, defaults.auth_port),
            auth_protocol: pick(self.auth_protocol, defaults.auth_protocol),
            auth_recents: pick(self.auth_recents, defaults.auth_recents),
            auth_username: pick(self.auth_username, defaults.auth_username),
            misc_error_dialog: pick(self.misc_error_dialog, defaults.misc_error_dialog),
            misc_info_dialog: pick(self.misc_info_dialog, defaults.misc_info_dialog),
            misc_input_dialog: pick(self.misc_input_dialog, defaults.misc_input_dialog),
            misc_keys: pick(self.misc_keys, defaults.misc_keys),
            misc_quit_dialog: pick(self.misc_quit_dialog, defaults.misc_quit_dialog),
            misc_save_dialog: pick(self.misc_save_dialog, defaults.misc_save_dialog),
            misc_warn_dialog: pick(self.misc_warn_dialog, defaults.misc_warn_dialog),
            transfer_local_explorer_background: pick(
                self.transfer_local_explorer_background,
                defaults.transfer_local_explorer_background,
            ),
            transfer_local_explorer_foreground: pick(
                self.transfer_local_explorer_foreground,
                defaults.transfer_local_explorer_foreground,
            ),
            transfer_local_explorer_highlighted: pick(
                self.transfer_local_explorer_highlighted,
                defaults.transfer_local_explorer_highlighted,
            ),
            transfer_log_background: pick(
                self.transfer_log_background,
                defaults.transfer_log_background,
            ),
            transfer_log_window: pick(self.transfer_log_window, defaults.transfer_log_window),
            transfer_progress_bar: pick(
                self.transfer_progress_bar
                    .or(self.transfer_progress_bar_full)
                    .or(self.transfer_progress_bar_partial),
                defaults.transfer_progress_bar,
            ),
            transfer_remote_explorer_background: pick(
                self.transfer_remote_explorer_background,
                defaults.transfer_remote_explorer_background,
            ),
            transfer_remote_explorer_foreground: pick(
                self.transfer_remote_explorer_foreground,
                defaults.transfer_remote_explorer_foreground,
            ),
            transfer_remote_explorer_highlighted: pick(
                self.transfer_remote_explorer_highlighted,
                defaults.transfer_remote_explorer_highlighted,
            ),
            transfer_status_hidden: pick(
                self.transfer_status_hidden,
                defaults.transfer_status_hidden,
            ),
            transfer_status_sorting: pick(
                self.transfer_status_sorting,
                defaults.transfer_status_sorting,
            ),
            transfer_status_sync_browsing: pick(
                self.transfer_status_sync_browsing,
                defaults.transfer_status_sync_browsing,
            ),
        }
    }
}

impl<'de> Deserialize<'de> for Theme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let file = ThemeFile::deserialize(deserializer)?;
        Ok(file.into_theme())
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

// Kept for backwards compatibility with any external callers; no longer used directly
// by serde derive because `Theme` now uses a custom `Deserialize` impl.
#[allow(dead_code)]
fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match parse_color(&s) {
        None => Err(DeError::custom("Invalid color")),
        Some(color) => Ok(color),
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_get_default_theme() {
        let theme: Theme = Theme::default();
        assert_eq!(theme.auth_address, Color::Yellow);
        assert_eq!(theme.auth_bookmarks, Color::LightGreen);
        assert_eq!(theme.auth_password, Color::LightBlue);
        assert_eq!(theme.auth_port, Color::LightCyan);
        assert_eq!(theme.auth_protocol, Color::LightGreen);
        assert_eq!(theme.auth_recents, Color::LightBlue);
        assert_eq!(theme.auth_username, Color::LightMagenta);
        assert_eq!(theme.misc_error_dialog, Color::Red);
        assert_eq!(theme.misc_info_dialog, Color::LightYellow);
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

    #[test]
    fn should_accept_legacy_progress_bar_fields() {
        let toml = r#"
            auth_protocol = "Yellow"
            transfer_progress_bar_full = "Green"
        "#;
        let theme: Theme = toml::from_str(toml).expect("theme should load");
        assert_eq!(theme.auth_protocol, Color::Yellow);
        assert_eq!(theme.transfer_progress_bar, Color::Green);
    }

    #[test]
    fn should_ignore_duplicated_legacy_progress_bar_fields() {
        let toml = r#"
            auth_protocol = "Yellow"
            transfer_progress_bar_full = "Green"
            transfer_progress_bar_partial = "Red"
        "#;
        let theme: Theme = toml::from_str(toml).expect("theme should load");
        assert_eq!(theme.auth_protocol, Color::Yellow);
        // `_full` wins because `transfer_progress_bar` and `_full` are checked first.
        assert_eq!(theme.transfer_progress_bar, Color::Green);
    }

    #[test]
    fn should_fall_back_to_defaults_on_invalid_values() {
        // Invalid color value should make *that* field fall back to default, without
        // breaking the rest of the theme.
        let toml = r##"
            auth_protocol = "not-a-color"
            auth_username = "#ca9ee6"
        "##;
        let theme: Theme = toml::from_str(toml).expect("theme should load");
        assert_eq!(theme.auth_protocol, Theme::default().auth_protocol);
        assert_eq!(theme.auth_username, Color::Rgb(202, 158, 230));
    }

    #[test]
    fn should_fall_back_to_defaults_on_missing_fields() {
        // Empty file should still produce a theme equal to default.
        let theme: Theme = toml::from_str("").expect("theme should load");
        assert_eq!(theme, Theme::default());
    }
}
