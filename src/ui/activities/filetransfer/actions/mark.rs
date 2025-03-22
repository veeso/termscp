//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

use super::FileTransferActivity;

impl FileTransferActivity {
    pub(crate) fn action_mark_file(&mut self, index: usize) {
        // get dest
        let dest_path = self.browser.other_explorer_no_found().wrkdir.clone();
        // get file
        let browser = self.browser.explorer_mut();
        let Some(file) = browser.get(index).map(|item| item.path().to_path_buf()) else {
            return;
        };

        if browser.enqueued().contains_key(&file) {
            debug!("File already marked, unmarking {}", file.display());
            browser.dequeue(&file);
        } else {
            debug!("Marking file {}", file.display());
            browser.enqueue(&file, &dest_path);
        }

        self.reload_browser_file_list();
    }

    pub(crate) fn action_mark_all(&mut self) {
        let dest_path = self.browser.other_explorer_no_found().wrkdir.clone();
        let browser = self.browser.explorer_mut();

        let mut files = vec![];
        for file in browser.iter_files().map(|x| x.path()) {
            files.push(file.to_path_buf());
        }
        for file in files {
            debug!("Marking file {}", file.display());
            browser.enqueue(&file, &dest_path);
        }

        self.reload_browser_file_list();
    }

    pub(crate) fn action_mark_clear(&mut self) {
        let browser = self.browser.explorer_mut();
        debug!("Clearing all marked files");
        browser.clear_queue();

        self.reload_browser_file_list();
    }
}
