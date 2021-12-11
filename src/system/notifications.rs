//! # Notifications
//!
//! This module exposes the function to send notifications to the guest OS

#[cfg(all(unix, not(target_os = "macos")))]
use notify_rust::Hint;
use notify_rust::{Notification as OsNotification, Timeout};

/// A notification helper which provides all the functions to send the available notifications for termscp
pub struct Notification;

impl Notification {
    /// Notify a transfer has been completed with success
    pub fn transfer_completed<S: AsRef<str>>(body: S) {
        Self::notify(
            "Transfer completed ✅",
            body.as_ref(),
            Some("transfer.complete"),
        );
    }

    /// Notify a transfer has failed
    pub fn transfer_error<S: AsRef<str>>(body: S) {
        Self::notify("Transfer failed ❌", body.as_ref(), Some("transfer.error"));
    }

    /// Notify a new version of termscp is available for download
    pub fn update_available<S: AsRef<str>>(version: S) {
        Self::notify(
            "New version available ⬇️",
            format!("termscp {} is now available for download", version.as_ref()).as_str(),
            None,
        );
    }

    /// Notify the update has been correctly installed
    pub fn update_installed<S: AsRef<str>>(version: S) {
        Self::notify(
        "Update installed 🎉",
            format!("termscp {} has been installed! Restart termscp to enjoy the latest version of termscp 🙂", version.as_ref()).as_str(),
        None,
        );
    }

    /// Notify the update installation has failed
    pub fn update_failed<S: AsRef<str>>(err: S) {
        Self::notify("Update installation failed ❌", err.as_ref(), None);
    }

    /// Notify guest OS with provided Summary, body and optional category
    /// e.g. Category is supported on FreeBSD/Linux only
    #[allow(unused_variables)]
    fn notify(summary: &str, body: &str, category: Option<&str>) {
        let mut notification = OsNotification::new();
        // Set common params
        notification
            .appname(env!("CARGO_PKG_NAME"))
            .summary(summary)
            .body(body)
            .timeout(Timeout::Milliseconds(10000));
        // Set category if any
        #[cfg(all(unix, not(target_os = "macos")))]
        if let Some(category) = category {
            notification.hint(Hint::Category(category.to_string()));
        }
        let _ = notification.show();
    }
}
