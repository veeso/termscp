use remotefs::fs::UnixPex;

use super::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    /// Change file mode for the currently selected entries.
    /// Branches on the active tab (local vs remote) to choose the right API.
    /// Works for both regular and find-result tabs thanks to `get_selected_entries()`.
    pub(crate) fn action_chmod(&mut self, mode: UnixPex) {
        let files = self.get_selected_entries().get_files();

        for file in files {
            let result: Result<(), String> = if self.is_local_tab() {
                self.host_bridge
                    .chmod(file.path(), mode)
                    .map_err(|e| e.to_string())
            } else {
                let mut metadata = file.metadata.clone();
                metadata.mode = Some(mode);
                self.client
                    .setstat(file.path(), metadata)
                    .map_err(|e| e.to_string())
            };
            if let Err(err) = result {
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
