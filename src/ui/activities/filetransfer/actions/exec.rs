//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use super::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    pub(crate) fn action_local_exec(&mut self, input: String) -> String {
        // TODO: handle cd

        match self.host_bridge.exec(input.as_str()) {
            Ok(output) => {
                // Reload files
                self.log(LogLevel::Info, format!("\"{input}\": {output}"));
                output
            }
            Err(err) => {
                // Report err
                self.log(
                    LogLevel::Error,
                    format!("Could not execute command \"{input}\": {err}"),
                );
                err.to_string()
            }
        }
    }

    pub(crate) fn action_remote_exec(&mut self, input: String) -> String {
        // TODO: handle cd

        match self.client.as_mut().exec(input.as_str()) {
            Ok((rc, output)) => {
                // Reload files
                self.log(
                    LogLevel::Info,
                    format!("\"{input}\" (exitcode: {rc}): {output}"),
                );
                output
            }
            Err(err) => {
                // Report err
                self.log(
                    LogLevel::Error,
                    format!("Could not execute command \"{input}\": {err}"),
                );
                err.to_string()
            }
        }
    }
}
