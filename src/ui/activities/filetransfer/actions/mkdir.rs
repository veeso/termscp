//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// locals
use super::{FileTransferActivity, LogLevel};
use remotefs::fs::UnixPex;
use std::path::PathBuf;

impl FileTransferActivity {
    pub(crate) fn action_local_mkdir(&mut self, input: String) {
        match self.host.mkdir(PathBuf::from(input.as_str()).as_path()) {
            Ok(_) => {
                // Reload files
                self.log(LogLevel::Info, format!("Created directory \"{}\"", input));
                // Reload entries
                self.reload_local_dir();
            }
            Err(err) => {
                // Report err
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not create directory \"{}\": {}", input, err),
                );
            }
        }
    }
    pub(crate) fn action_remote_mkdir(&mut self, input: String) {
        match self.client.as_mut().create_dir(
            PathBuf::from(input.as_str()).as_path(),
            UnixPex::from(0o755),
        ) {
            Ok(_) => {
                // Reload files
                self.log(LogLevel::Info, format!("Created directory \"{}\"", input));
                self.reload_remote_dir();
            }
            Err(err) => {
                // Report err
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not create directory \"{}\": {}", input, err),
                );
            }
        }
    }
}
