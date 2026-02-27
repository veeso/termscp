use bytesize::ByteSize;

use super::super::{FileTransferActivity, TransferPayload};
use crate::system::notifications::Notification;
use crate::utils::fmt::fmt_millis;

impl FileTransferActivity {
    /// Send notification regarding transfer completed
    /// The notification is sent only when these conditions are satisfied:
    ///
    /// - notifications are enabled
    /// - transfer size is greater or equal than notification threshold
    pub(in crate::ui::activities::filetransfer) fn notify_transfer_completed(
        &self,
        payload: &TransferPayload,
    ) {
        if self.config().get_notifications()
            && self.config().get_notification_threshold() as usize <= self.transfer.full_size()
        {
            Notification::transfer_completed(self.transfer_completed_msg(payload));
        }
    }

    /// Send notification regarding transfer error
    /// The notification is sent only when these conditions are satisfied:
    ///
    /// - notifications are enabled
    /// - transfer size is greater or equal than notification threshold
    pub(in crate::ui::activities::filetransfer) fn notify_transfer_error(&self, msg: &str) {
        if self.config().get_notifications()
            && self.config().get_notification_threshold() as usize <= self.transfer.full_size()
        {
            Notification::transfer_error(msg);
        }
    }

    fn transfer_completed_msg(&self, payload: &TransferPayload) -> String {
        let transfer_stats = format!(
            "took {} seconds; at {}/s",
            fmt_millis(self.transfer.partial.started().elapsed()),
            ByteSize(self.transfer.partial.calc_bytes_per_second()),
        );
        match payload {
            TransferPayload::File(file) => {
                format!(
                    "File \"{}\" has been successfully transferred ({})",
                    file.name(),
                    transfer_stats
                )
            }
            TransferPayload::Any(entry) => {
                format!(
                    "\"{}\" has been successfully transferred ({})",
                    entry.name(),
                    transfer_stats
                )
            }
            TransferPayload::TransferQueue(entries) => {
                format!(
                    "{} files has been successfully transferred ({})",
                    entries.len(),
                    transfer_stats
                )
            }
        }
    }
}
