use remotefs::fs::UnixPex;

use super::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    pub fn action_local_chmod(&mut self, mode: UnixPex) {
        let files = self.get_local_selected_entries().get_files();

        for file in files {
            if let Err(err) = self.host.chmod(file.path(), mode) {
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

    pub fn action_remote_chmod(&mut self, mode: UnixPex) {
        let files = self.get_remote_selected_entries().get_files();

        for file in files {
            let mut metadata = file.metadata.clone();
            metadata.mode = Some(mode);

            if let Err(err) = self.client.setstat(file.path(), metadata) {
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

    pub fn action_find_local_chmod(&mut self, mode: UnixPex) {
        let files = self.get_found_selected_entries().get_files();

        for file in files {
            if let Err(err) = self.host.chmod(file.path(), mode) {
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

    pub fn action_find_remote_chmod(&mut self, mode: UnixPex) {
        let files = self.get_found_selected_entries().get_files();

        for file in files {
            let mut metadata = file.metadata.clone();
            metadata.mode = Some(mode);

            if let Err(err) = self.client.setstat(file.path(), metadata) {
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
