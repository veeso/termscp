use remotefs::fs::UnixPex;

use super::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    /// Change file mode for the currently selected entries via the active tab's pane.
    pub(crate) fn action_chmod(&mut self, mode: UnixPex) {
        let files = self.get_selected_entries().get_files();

        for file in files {
            if let Err(err) = self.browser.fs_pane_mut().fs.chmod(file.path(), mode) {
                self.log_and_alert(
                    LogLevel::Error,
                    format!(
                        "could not change mode for {}: {}",
                        file.path().display(),
                        err
                    ),
                );
                return;
            }
            self.log(
                LogLevel::Info,
                format!("changed mode to {:#o} for {}", u32::from(mode), file.name()),
            );
        }
    }
}
