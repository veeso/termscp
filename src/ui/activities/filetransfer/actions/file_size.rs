use remotefs::File;

use super::{FileTransferActivity, LogLevel};

fn aggregate_sizes(sizes: impl IntoIterator<Item = Option<u64>>) -> Option<u64> {
    sizes
        .into_iter()
        .try_fold(0_u64, |total, size| total.checked_add(size?))
}

impl FileTransferActivity {
    /// Calculate and display the total size of the selected file(s) via the active tab's pane.
    pub(crate) fn action_get_file_size(&mut self) {
        self.mount_blocking_wait("Getting total path size...");

        let files = self.get_selected_entries().get_files();
        let total_size = self.get_files_size(files);

        self.umount_wait();
        let message = total_size.map_or_else(
            || String::from("Total file size: Unknown"),
            |size| {
                format!(
                    "Total file size: {size}",
                    size = bytesize::ByteSize::b(size)
                )
            },
        );
        self.mount_info(message);
    }

    fn get_files_size(&mut self, files: Vec<File>) -> Option<u64> {
        aggregate_sizes(files.into_iter().map(|file| self.get_file_size(file)))
    }

    fn get_file_size(&mut self, file: File) -> Option<u64> {
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
                    None
                }
            }
        } else if file.is_dir() {
            match self.browser.fs_pane_mut().fs.list_dir(&file.path) {
                Ok(list) => self.get_files_size(list),
                Err(err) => {
                    self.log(
                        LogLevel::Error,
                        format!(
                            "Failed to list directory {path}: {err}",
                            path = file.path.display(),
                        ),
                    );
                    None
                }
            }
        } else {
            file.metadata().size
        }
    }
}

#[cfg(test)]
mod test {
    use super::aggregate_sizes;

    #[test]
    fn aggregates_only_known_sizes() {
        assert_eq!(aggregate_sizes([Some(2), Some(3)]), Some(5));
        assert_eq!(aggregate_sizes([Some(2), None, Some(3)]), None);
    }
}
