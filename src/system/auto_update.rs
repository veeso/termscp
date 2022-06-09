//! ## Auto update
//!
//! Automatic update module. This module is used to upgrade the current version of termscp to the latest available on Github

use crate::utils::parser::parse_semver;

pub use self_update::errors::Error as UpdateError;
use self_update::{
    backends::github::Update as GithubUpdater, cargo_crate_version, update::Release as UpdRelease,
    Status,
};

/// ### UpdateStatus
///
/// The status of the update in case of success
#[derive(Debug, Eq, PartialEq)]
pub enum UpdateStatus {
    /// Termscp is already up to date
    AlreadyUptodate,
    /// The update has been correctly installed
    UpdateInstalled(String),
}

/// Info related to a github release
#[derive(Debug)]
pub struct Release {
    pub version: String,
    pub body: String,
}

/// The update structure defines the options used to install the update.
/// Once you're fine with the options, just call the `upgrade()` method to upgrade termscp.
#[derive(Debug, Default)]
pub struct Update {
    ask_confirm: bool,
    progress: bool,
}

impl Update {
    /// Set whether to show or not the progress bar
    pub fn show_progress(mut self, opt: bool) -> Self {
        self.progress = opt;
        self
    }

    /// Set whether to ask for confirm when updating
    pub fn ask_confirm(mut self, opt: bool) -> Self {
        self.ask_confirm = opt;
        self
    }

    pub fn upgrade(self) -> Result<UpdateStatus, UpdateError> {
        info!("Updating termscp...");
        GithubUpdater::configure()
            // Set default options
            .repo_owner("veeso")
            .repo_name("termscp")
            .bin_name("termscp")
            .current_version(cargo_crate_version!())
            .no_confirm(!self.ask_confirm)
            .show_download_progress(self.progress)
            .show_output(self.progress)
            .build()?
            .update()
            .map(UpdateStatus::from)
    }

    /// Returns whether a new version of termscp is available
    /// In case of success returns Ok(Option<Release>), where the Option is Some(new_version);
    /// otherwise if no version is available, return None
    /// In case of error returns Error with the error description
    pub fn is_new_version_available() -> Result<Option<Release>, UpdateError> {
        info!("Checking whether a new version is available...");
        GithubUpdater::configure()
            // Set default options
            .repo_owner("veeso")
            .repo_name("termscp")
            .bin_name("termscp")
            .current_version(cargo_crate_version!())
            .no_confirm(true)
            .show_download_progress(false)
            .show_output(false)
            .build()?
            .get_latest_release()
            .map(Release::from)
            .map(Self::check_version)
    }

    /// In case received version is newer than current one, version as Some is returned; otherwise None
    fn check_version(r: Release) -> Option<Release> {
        match parse_semver(r.version.as_str()) {
            Some(new_version) => {
                // Check if version is different
                debug!(
                    "New version: {}; current version: {}",
                    new_version,
                    cargo_crate_version!()
                );
                if new_version.as_str() > cargo_crate_version!() {
                    Some(r) // New version is available
                } else {
                    None // No new version
                }
            }
            None => None,
        }
    }
}
impl From<Status> for UpdateStatus {
    fn from(s: Status) -> Self {
        match s {
            Status::UpToDate(_) => Self::AlreadyUptodate,
            Status::Updated(v) => Self::UpdateInstalled(v),
        }
    }
}

impl From<UpdRelease> for Release {
    fn from(r: UpdRelease) -> Self {
        Self {
            version: r.version,
            body: r.body.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn auto_update_default() {
        let upd: Update = Update::default();
        assert_eq!(upd.ask_confirm, false);
        assert_eq!(upd.progress, false);
        let upd = upd.ask_confirm(true).show_progress(true);
        assert_eq!(upd.ask_confirm, true);
        assert_eq!(upd.progress, true);
    }

    #[test]
    #[cfg(not(all(
        any(target_os = "macos", target_os = "freebsd"),
        feature = "github-actions"
    )))]
    fn auto_update() {
        // Wno version
        assert_eq!(
            Update::default()
                .show_progress(true)
                .upgrade()
                .ok()
                .unwrap(),
            UpdateStatus::AlreadyUptodate,
        );
    }

    #[test]
    #[cfg(not(all(
        any(target_os = "macos", target_os = "freebsd"),
        feature = "github-actions"
    )))]
    fn check_for_updates() {
        println!("{:?}", Update::is_new_version_available());
        assert!(Update::is_new_version_available().is_ok());
    }

    #[test]
    fn update_status() {
        assert_eq!(
            UpdateStatus::from(Status::Updated(String::from("0.6.0"))),
            UpdateStatus::UpdateInstalled(String::from("0.6.0"))
        );
        assert_eq!(
            UpdateStatus::from(Status::UpToDate(String::from("0.6.0"))),
            UpdateStatus::AlreadyUptodate
        );
    }

    #[test]
    fn release() {
        let release: UpdRelease = UpdRelease {
            name: String::from("termscp 0.7.0"),
            version: String::from("0.7.0"),
            date: String::from("2021-09-12T00:00:00Z"),
            body: Some(String::from("fixed everything")),
            assets: vec![],
        };
        let release: Release = Release::from(release);
        assert_eq!(release.body.as_str(), "fixed everything");
        assert_eq!(release.version.as_str(), "0.7.0");
    }
}
