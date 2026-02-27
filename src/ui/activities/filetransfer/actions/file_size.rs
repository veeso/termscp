use remotefs::File;

use super::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    /// Calculate and display the total size of the selected file(s) via the active tab's pane.
    pub(crate) fn action_get_file_size(&mut self) {
        self.mount_blocking_wait("Getting total path size...");

        let files = self.get_selected_entries().get_files();
        let total_size = self.get_files_size(files);

        self.umount_wait();
        self.mount_info(format!(
            "Total file size: {size}",
            size = bytesize::ByteSize::b(total_size)
        ));
    }

    fn get_files_size(&mut self, files: Vec<File>) -> u64 {
        files.into_iter().map(|f| self.get_file_size(f)).sum()
    }

    fn get_file_size(&mut self, file: File) -> u64 {
        if let Some(symlink) = &file.metadata().symlink {
            match self.browser.fs_pane_mut().fs.stat(symlink) {
                Ok(stat) => stat.metadata().size,
                Err(err) => {
                    self.log(
                        LogLevel::Error,
                        format!(
                            "Failed to stat symlink target {path}: {err}",
                            path = symlink.display(),
                        ),
                    );
                    0
                }
            }
        } else if file.is_dir() {
            match self.browser.fs_pane_mut().fs.list_dir(&file.path) {
                Ok(list) => list.into_iter().map(|f| self.get_file_size(f)).sum(),
                Err(err) => {
                    self.log(
                        LogLevel::Error,
                        format!(
                            "Failed to list directory {path}: {err}",
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
