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
// Locals
use super::{
    browser::FileExplorerTab, ConfigClient, FileTransferActivity, Id, LogLevel, LogRecord,
    TransferPayload,
};
use crate::filetransfer::ProtocolParams;
use crate::system::environment;
use crate::system::notifications::Notification;
use crate::utils::fmt::{fmt_millis, fmt_path_elide_ex};
use crate::utils::path;
// Ext
use bytesize::ByteSize;
use std::env;
use std::path::{Path, PathBuf};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Color, PropPayload, PropValue, TableBuilder, TextSpan,
};
use tuirealm::{PollStrategy, Update};

const LOG_CAPACITY: usize = 256;

impl FileTransferActivity {
    /// Call `Application::tick()` and process messages in `Update`
    pub(super) fn tick(&mut self) {
        match self.app.tick(PollStrategy::UpTo(3)) {
            Ok(messages) => {
                if !messages.is_empty() {
                    self.redraw = true;
                }
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = self.update(msg);
                    }
                }
            }
            Err(err) => {
                self.mount_error(format!("Application error: {}", err));
            }
        }
    }

    /// Add message to log events
    pub(super) fn log(&mut self, level: LogLevel, msg: String) {
        // Log to file
        match level {
            LogLevel::Error => error!("{}", msg),
            LogLevel::Info => info!("{}", msg),
            LogLevel::Warn => warn!("{}", msg),
        }
        // Create log record
        let record: LogRecord = LogRecord::new(level, msg);
        //Check if history overflows the size
        if self.log_records.len() + 1 > LOG_CAPACITY {
            self.log_records.pop_back(); // Start cleaning events from back
        }
        // Eventually push front the new record
        self.log_records.push_front(record);
        // Update log
        self.update_logbox();
    }

    /// Add message to log events and also display it as an alert
    pub(super) fn log_and_alert(&mut self, level: LogLevel, msg: String) {
        self.mount_error(msg.as_str());
        self.log(level, msg);
        // Update log
        self.update_logbox();
    }

    /// Initialize configuration client if possible.
    /// This function doesn't return errors.
    pub(super) fn init_config_client() -> ConfigClient {
        match environment::init_config_dir() {
            Ok(termscp_dir) => match termscp_dir {
                Some(termscp_dir) => {
                    // Make configuration file path and ssh keys path
                    let (config_path, ssh_keys_path): (PathBuf, PathBuf) =
                        environment::get_config_paths(termscp_dir.as_path());
                    match ConfigClient::new(config_path.as_path(), ssh_keys_path.as_path()) {
                        Ok(config_client) => config_client,
                        Err(_) => ConfigClient::degraded(),
                    }
                }
                None => ConfigClient::degraded(),
            },
            Err(_) => ConfigClient::degraded(),
        }
    }

    /// Set text editor to use
    pub(super) fn setup_text_editor(&self) {
        env::set_var("EDITOR", self.config().get_text_editor());
    }

    /// Convert a path to absolute according to local explorer
    pub(super) fn local_to_abs_path(&self, path: &Path) -> PathBuf {
        path::absolutize(self.local().wrkdir.as_path(), path)
    }

    /// Convert a path to absolute according to remote explorer
    pub(super) fn remote_to_abs_path(&self, path: &Path) -> PathBuf {
        path::absolutize(self.remote().wrkdir.as_path(), path)
    }

    /// Get remote hostname
    pub(super) fn get_remote_hostname(&self) -> String {
        let ft_params = self.context().ft_params().unwrap();
        match &ft_params.params {
            ProtocolParams::Generic(params) => params.address.clone(),
            ProtocolParams::AwsS3(params) => params.bucket_name.clone(),
        }
    }

    /// Get connection message to show to client
    pub(super) fn get_connection_msg(params: &ProtocolParams) -> String {
        match params {
            ProtocolParams::Generic(params) => {
                info!(
                    "Client is not connected to remote; connecting to {}:{}",
                    params.address, params.port
                );
                format!("Connecting to {}:{}…", params.address, params.port)
            }
            ProtocolParams::AwsS3(params) => {
                info!(
                    "Client is not connected to remote; connecting to {}{} ({})",
                    params.endpoint.as_deref().unwrap_or(""),
                    params.bucket_name,
                    params.region.as_deref().unwrap_or("custom")
                );
                format!("Connecting to {}…", params.bucket_name)
            }
        }
    }

    /// Send notification regarding transfer completed
    /// The notification is sent only when these conditions are satisfied:
    ///
    /// - notifications are enabled
    /// - transfer size is greater or equal than notification threshold
    pub(super) fn notify_transfer_completed(&self, payload: &TransferPayload) {
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
    pub(super) fn notify_transfer_error(&self, msg: &str) {
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
            TransferPayload::Many(entries) => {
                format!(
                    "{} files has been successfully transferred ({})",
                    entries.len(),
                    transfer_stats
                )
            }
        }
    }

    /// Update local file list
    pub(super) fn update_local_filelist(&mut self) {
        self.reload_local_dir();
        // Get width
        let width = self
            .context_mut()
            .terminal()
            .raw()
            .size()
            .map(|x| (x.width / 2) - 2)
            .unwrap_or(0) as usize;
        let hostname: String = match hostname::get() {
            Ok(h) => {
                let hostname: String = h.as_os_str().to_string_lossy().to_string();
                let tokens: Vec<&str> = hostname.split('.').collect();
                String::from(*tokens.get(0).unwrap_or(&"localhost"))
            }
            Err(_) => String::from("localhost"),
        };
        let hostname: String = format!(
            "{}:{} ",
            hostname,
            fmt_path_elide_ex(self.local().wrkdir.as_path(), width, hostname.len() + 3) // 3 because of '/…/'
        );
        let files: Vec<Vec<TextSpan>> = self
            .local()
            .iter_files()
            .map(|x| vec![TextSpan::from(self.local().fmt_file(x))])
            .collect();
        // Update content and title
        assert!(self
            .app
            .attr(
                &Id::ExplorerLocal,
                Attribute::Content,
                AttrValue::Table(files)
            )
            .is_ok());
        assert!(self
            .app
            .attr(
                &Id::ExplorerLocal,
                Attribute::Title,
                AttrValue::Title((hostname, Alignment::Left))
            )
            .is_ok());
    }

    /// Update remote file list
    pub(super) fn update_remote_filelist(&mut self) {
        self.reload_remote_dir();
        let width = self
            .context_mut()
            .terminal()
            .raw()
            .size()
            .map(|x| (x.width / 2) - 2)
            .unwrap_or(0) as usize;
        let hostname = self.get_remote_hostname();
        let hostname: String = format!(
            "{}:{} ",
            hostname,
            fmt_path_elide_ex(
                self.remote().wrkdir.as_path(),
                width,
                hostname.len() + 3 // 3 because of '/…/'
            )
        );
        let files: Vec<Vec<TextSpan>> = self
            .remote()
            .iter_files()
            .map(|x| vec![TextSpan::from(self.remote().fmt_file(x))])
            .collect();
        // Update content and title
        assert!(self
            .app
            .attr(
                &Id::ExplorerRemote,
                Attribute::Content,
                AttrValue::Table(files)
            )
            .is_ok());
        assert!(self
            .app
            .attr(
                &Id::ExplorerRemote,
                Attribute::Title,
                AttrValue::Title((hostname, Alignment::Left))
            )
            .is_ok());
    }

    /// Update log box
    pub(super) fn update_logbox(&mut self) {
        let mut table: TableBuilder = TableBuilder::default();
        for (idx, record) in self.log_records.iter().enumerate() {
            // Add row if not first row
            if idx > 0 {
                table.add_row();
            }
            let fg = match record.level {
                LogLevel::Error => Color::Red,
                LogLevel::Warn => Color::Yellow,
                LogLevel::Info => Color::Green,
            };
            table
                .add_col(TextSpan::from(format!(
                    "{}",
                    record.time.format("%Y-%m-%dT%H:%M:%S%Z")
                )))
                .add_col(TextSpan::from(" ["))
                .add_col(
                    TextSpan::new(
                        format!(
                            "{:5}",
                            match record.level {
                                LogLevel::Error => "ERROR",
                                LogLevel::Warn => "WARN",
                                LogLevel::Info => "INFO",
                            }
                        )
                        .as_str(),
                    )
                    .fg(fg),
                )
                .add_col(TextSpan::from("]: "))
                .add_col(TextSpan::from(record.msg.as_str()));
        }
        assert!(self
            .app
            .attr(
                &Id::Log,
                Attribute::Content,
                AttrValue::Table(table.build())
            )
            .is_ok());
    }

    pub(super) fn update_progress_bar(&mut self, filename: String) {
        assert!(self
            .app
            .attr(
                &Id::ProgressBarFull,
                Attribute::Text,
                AttrValue::String(self.transfer.full.to_string())
            )
            .is_ok());
        assert!(self
            .app
            .attr(
                &Id::ProgressBarFull,
                Attribute::Value,
                AttrValue::Payload(PropPayload::One(PropValue::F64(
                    self.transfer.full.calc_progress()
                )))
            )
            .is_ok());
        assert!(self
            .app
            .attr(
                &Id::ProgressBarPartial,
                Attribute::Text,
                AttrValue::String(self.transfer.partial.to_string())
            )
            .is_ok());
        assert!(self
            .app
            .attr(
                &Id::ProgressBarPartial,
                Attribute::Value,
                AttrValue::Payload(PropPayload::One(PropValue::F64(
                    self.transfer.partial.calc_progress()
                )))
            )
            .is_ok());
        assert!(self
            .app
            .attr(
                &Id::ProgressBarPartial,
                Attribute::Title,
                AttrValue::Title((filename, Alignment::Center))
            )
            .is_ok());
    }

    /// Finalize find process
    pub(super) fn finalize_find(&mut self) {
        // Set found to none
        self.browser.del_found();
        // Restore tab
        let new_tab = match self.browser.tab() {
            FileExplorerTab::FindLocal => FileExplorerTab::Local,
            FileExplorerTab::FindRemote => FileExplorerTab::Remote,
            _ => FileExplorerTab::Local,
        };
        // Give focus to new tab
        match new_tab {
            FileExplorerTab::Local => assert!(self.app.active(&Id::ExplorerLocal).is_ok()),
            FileExplorerTab::Remote => {
                assert!(self.app.active(&Id::ExplorerRemote).is_ok())
            }
            FileExplorerTab::FindLocal | FileExplorerTab::FindRemote => {
                assert!(self.app.active(&Id::ExplorerFind).is_ok())
            }
        }
        self.browser.change_tab(new_tab);
    }

    pub(super) fn update_find_list(&mut self) {
        let files: Vec<Vec<TextSpan>> = self
            .found()
            .unwrap()
            .iter_files()
            .map(|x| vec![TextSpan::from(self.found().unwrap().fmt_file(x))])
            .collect();
        assert!(self
            .app
            .attr(
                &Id::ExplorerFind,
                Attribute::Content,
                AttrValue::Table(files)
            )
            .is_ok());
    }

    pub(super) fn update_browser_file_list(&mut self) {
        match self.browser.tab() {
            FileExplorerTab::Local | FileExplorerTab::FindLocal => self.update_local_filelist(),
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => self.update_remote_filelist(),
        }
    }

    pub(super) fn update_browser_file_list_swapped(&mut self) {
        match self.browser.tab() {
            FileExplorerTab::Local | FileExplorerTab::FindLocal => self.update_remote_filelist(),
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => self.update_local_filelist(),
        }
    }
}
