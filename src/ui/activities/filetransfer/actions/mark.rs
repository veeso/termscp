//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

use super::FileTransferActivity;

impl FileTransferActivity {
    pub(crate) fn action_mark_file(&mut self, index: usize) {
        self.enqueue_file(index);
    }

    pub(crate) fn action_mark_all(&mut self) {
        self.enqueue_all();
    }

    pub(crate) fn action_mark_clear(&mut self) {
        self.clear_queue();
    }
}
