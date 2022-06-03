//! # watcher actions
//!
//! actions associated to the file watcher

use super::{FileTransferActivity, LogLevel, SelectedFile};

use std::path::{Path, PathBuf};

impl FileTransferActivity {
    pub fn action_show_radio_watch(&mut self) {
        // return if fswatcher is not working
        if self.fswatcher.is_none() {
            return;
        }
        // get local entry
        if let Some((watched, local, remote)) = self.get_watcher_dirs() {
            self.mount_radio_watch(
                watched,
                local.to_string_lossy().to_string().as_str(),
                remote.to_string_lossy().to_string().as_str(),
            );
        }
    }

    pub fn action_toggle_watch(&mut self) {
        // umount radio
        self.umount_radio_watcher();
        // return if fswatcher is not working
        if self.fswatcher.is_none() {
            return;
        }
        match self.get_watcher_dirs() {
            Some((true, local, _)) => self.unwatch_path(&local),
            Some((false, local, remote)) => self.watch_path(&local, &remote),
            None => {}
        }
    }

    fn watch_path(&mut self, local: &Path, remote: &Path) {
        debug!(
            "tracking changes at {} to {}",
            local.display(),
            remote.display()
        );
        match self.map_on_fswatcher(|w| w.watch(local, remote)) {
            Some(Ok(())) => {
                self.log(
                    LogLevel::Info,
                    format!(
                        "changes to {} will now be synced with {}",
                        local.display(),
                        remote.display()
                    ),
                );
            }
            Some(Err(err)) => {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("could not track changes to {}: {}", local.display(), err),
                );
            }
            None => {}
        }
    }

    fn unwatch_path(&mut self, path: &Path) {
        debug!("unwatching path at {}", path.display());
        match self.map_on_fswatcher(|w| w.unwatch(path)) {
            Some(Ok(path)) => {
                self.log(
                    LogLevel::Info,
                    format!("{} is no longer watched", path.display()),
                );
            }
            Some(Err(err)) => {
                self.log_and_alert(LogLevel::Error, format!("could not unwatch path: {}", err));
            }
            None => {}
        }
    }

    fn get_watcher_dirs(&mut self) -> Option<(bool, PathBuf, PathBuf)> {
        if let SelectedFile::One(file) = self.get_local_selected_entries() {
            // check if entry is already watched
            let watched = self
                .map_on_fswatcher(|w| w.watched(file.path()))
                .unwrap_or(false);
            // mount dialog
            let mut remote = self.remote().wrkdir.clone();
            remote.push(file.name().as_str());
            Some((watched, file.path().to_path_buf(), remote))
        } else {
            None
        }
    }
}
