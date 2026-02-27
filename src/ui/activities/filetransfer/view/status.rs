//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

use crate::ui::activities::filetransfer::{
    FileTransferActivity, Id, MarkQueue, components, ui_result,
};

impl FileTransferActivity {
    pub(in crate::ui::activities::filetransfer) fn refresh_host_bridge_transfer_queue(&mut self) {
        let enqueued = self
            .host_bridge()
            .enqueued()
            .iter()
            .map(|(src, dest)| (src.clone(), dest.clone()))
            .collect::<Vec<_>>();
        let log_panel = self.theme().transfer_log_window;

        ui_result(self.app.remount(
            Id::TransferQueueHostBridge,
            Box::new(components::SelectedFilesList::new(
                &enqueued,
                MarkQueue::Local,
                log_panel,
                "Host Bridge selected files",
            )),
            vec![],
        ));
    }

    pub(in crate::ui::activities::filetransfer) fn refresh_remote_transfer_queue(&mut self) {
        let enqueued = self
            .remote()
            .enqueued()
            .iter()
            .map(|(src, dest)| (src.clone(), dest.clone()))
            .collect::<Vec<_>>();
        let log_panel = self.theme().transfer_log_window;

        ui_result(self.app.remount(
            Id::TransferQueueRemote,
            Box::new(components::SelectedFilesList::new(
                &enqueued,
                MarkQueue::Remote,
                log_panel,
                "Remote transfer selected files",
            )),
            vec![],
        ));
    }

    pub(in crate::ui::activities::filetransfer) fn refresh_local_status_bar(&mut self) {
        let sorting_color = self.theme().transfer_status_sorting;
        let hidden_color = self.theme().transfer_status_hidden;
        ui_result(self.app.remount(
            Id::StatusBarHostBridge,
            Box::new(components::StatusBarLocal::new(
                &self.browser,
                sorting_color,
                hidden_color,
            )),
            vec![],
        ));
    }

    pub(in crate::ui::activities::filetransfer) fn refresh_remote_status_bar(&mut self) {
        let sorting_color = self.theme().transfer_status_sorting;
        let hidden_color = self.theme().transfer_status_hidden;
        let sync_color = self.theme().transfer_status_sync_browsing;
        ui_result(self.app.remount(
            Id::StatusBarRemote,
            Box::new(components::StatusBarRemote::new(
                &self.browser,
                sorting_color,
                hidden_color,
                sync_color,
            )),
            vec![],
        ));
    }
}
