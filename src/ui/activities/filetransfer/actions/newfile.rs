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
use super::{FileTransferActivity, FsEntry, LogLevel};
use std::fs::File;
use std::path::PathBuf;

impl FileTransferActivity {
    pub(crate) fn action_local_newfile(&mut self, input: String) {
        // Check if file exists
        let mut file_exists: bool = false;
        for file in self.local().iter_files_all() {
            if input == file.get_name() {
                file_exists = true;
            }
        }
        if file_exists {
            self.log_and_alert(
                LogLevel::Warn,
                format!("File \"{}\" already exists", input,),
            );
            return;
        }
        // Create file
        let file_path: PathBuf = PathBuf::from(input.as_str());
        if let Err(err) = self.host.open_file_write(file_path.as_path()) {
            self.log_and_alert(
                LogLevel::Error,
                format!("Could not create file \"{}\": {}", file_path.display(), err),
            );
        } else {
            self.log(
                LogLevel::Info,
                format!("Created file \"{}\"", file_path.display()),
            );
        }
        // Reload files
        self.reload_local_dir();
    }

    pub(crate) fn action_remote_newfile(&mut self, input: String) {
        // Check if file exists
        let mut file_exists: bool = false;
        for file in self.remote().iter_files_all() {
            if input == file.get_name() {
                file_exists = true;
            }
        }
        if file_exists {
            self.log_and_alert(
                LogLevel::Warn,
                format!("File \"{}\" already exists", input,),
            );
            return;
        }
        // Get path on remote
        let file_path: PathBuf = PathBuf::from(input.as_str());
        // Create file (on local)
        match tempfile::NamedTempFile::new() {
            Err(err) => self.log_and_alert(
                LogLevel::Error,
                format!("Could not create tempfile: {}", err),
            ),
            Ok(tfile) => {
                // Stat tempfile
                let local_file: FsEntry = match self.host.stat(tfile.path()) {
                    Err(err) => {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not stat tempfile: {}", err),
                        );
                        return;
                    }
                    Ok(f) => f,
                };
                if let FsEntry::File(local_file) = local_file {
                    // Create file
                    let reader = Box::new(match File::open(tfile.path()) {
                        Ok(f) => f,
                        Err(err) => {
                            self.log_and_alert(
                                LogLevel::Error,
                                format!("Could not open tempfile: {}", err),
                            );
                            return;
                        }
                    });
                    match self
                        .client
                        .send_file_wno_stream(&local_file, file_path.as_path(), reader)
                    {
                        Err(err) => self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not create file \"{}\": {}", file_path.display(), err),
                        ),
                        Ok(_) => {
                            self.log(
                                LogLevel::Info,
                                format!("Created file \"{}\"", file_path.display()),
                            );
                            // Reload files
                            self.reload_remote_dir();
                        }
                    }
                }
            }
        }
    }
}
