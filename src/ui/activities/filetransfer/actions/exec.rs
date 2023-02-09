//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use super::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    pub(crate) fn action_local_exec(&mut self, input: String) {
        match self.host.exec(input.as_str()) {
            Ok(output) => {
                // Reload files
                self.log(LogLevel::Info, format!("\"{input}\": {output}"));
            }
            Err(err) => {
                // Report err
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not execute command \"{input}\": {err}"),
                );
            }
        }
    }

    pub(crate) fn action_remote_exec(&mut self, input: String) {
        match self.client.as_mut().exec(input.as_str()) {
            Ok((rc, output)) => {
                // Reload files
                self.log(
                    LogLevel::Info,
                    format!("\"{input}\" (exitcode: {rc}): {output}"),
                );
            }
            Err(err) => {
                // Report err
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not execute command \"{input}\": {err}"),
                );
            }
        }
    }
}
