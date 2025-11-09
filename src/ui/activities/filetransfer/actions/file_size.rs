use remotefs::File;

use super::{FileTransferActivity, LogLevel};
use crate::ui::activities::filetransfer::lib::browser::FileExplorerTab;

#[derive(Debug, Copy, Clone)]
enum Host {
    HostBridge,
    Remote,
}

impl FileTransferActivity {
    pub(crate) fn action_get_file_size(&mut self) {
        // Get selected file
        self.mount_blocking_wait("Getting total path size...");

        let total_size = match self.browser.tab() {
            FileExplorerTab::HostBridge => {
                let files = self.get_local_selected_entries().get_files();
                self.get_files_size(files, Host::HostBridge)
            }
            FileExplorerTab::Remote => {
                let files = self.get_remote_selected_entries().get_files();
                self.get_files_size(files, Host::Remote)
            }
            FileExplorerTab::FindHostBridge => {
                let files = self.get_found_selected_entries().get_files();
                self.get_files_size(files, Host::HostBridge)
            }
            FileExplorerTab::FindRemote => {
                let files = self.get_found_selected_entries().get_files();
                self.get_files_size(files, Host::Remote)
            }
        };

        self.umount_wait();
        self.mount_info(format!(
            "Total file size: {size}",
            size = bytesize::ByteSize::b(total_size)
        ));
    }

    fn get_files_size(&mut self, files: Vec<File>, host: Host) -> u64 {
        files.into_iter().map(|f| self.get_file_size(f, host)).sum()
    }

    fn get_file_size(&mut self, file: File, host: Host) -> u64 {
        if let Some(symlink) = &file.metadata().symlink {
            // stat
            let stat_res = match host {
                Host::HostBridge => self.host_bridge.stat(&symlink).map_err(|e| e.to_string()),
                Host::Remote => self.client.stat(&symlink).map_err(|e| e.to_string()),
            };
            match stat_res {
                Ok(stat) => stat.metadata().size,
                Err(err_msg) => {
                    self.log(
                        LogLevel::Error,
                        format!(
                            "Failed to stat symlink target {path}: {err_msg}",
                            path = symlink.display(),
                        ),
                    );
                    0
                }
            }
        } else if file.is_dir() {
            // list and sum
            let list_res = match host {
                Host::HostBridge => self
                    .host_bridge
                    .list_dir(&file.path)
                    .map_err(|e| e.to_string()),
                Host::Remote => self.client.list_dir(&file.path).map_err(|e| e.to_string()),
            };

            match list_res {
                Ok(list) => list.into_iter().map(|f| self.get_file_size(f, host)).sum(),
                Err(err_msg) => {
                    self.log(
                        LogLevel::Error,
                        format!(
                            "Failed to list directory {path}: {err_msg}",
                            path = file.path.display(),
                        ),
                    );
                    0
                }
            }
        } else {
            file.metadata().size
        }
    }
}
