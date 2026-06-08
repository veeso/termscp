use tuirealm::props::{
    AttrValue, Attribute, HorizontalAlignment, LineStatic, PropPayload, PropValue, SpanStatic,
    TextModifiers, Title,
};
use tuirealm::terminal::TerminalAdapter;

use super::super::browser::FileExplorerTab;
use super::super::{FileTransferActivity, Id, ui_result};
use crate::utils::fmt::fmt_path_elide_ex;

impl FileTransferActivity {
    /// Update host bridge file list
    pub(in crate::ui::activities::filetransfer) fn update_host_bridge_filelist(&mut self) {
        self.reload_host_bridge_dir();
        self.reload_host_bridge_filelist();
    }

    /// Update host bridge file list
    pub(in crate::ui::activities::filetransfer) fn reload_host_bridge_filelist(&mut self) {
        // Get width
        let width = self
            .context_mut()
            .terminal()
            .raw()
            .size()
            .map(|x| (x.width / 2) - 2)
            .unwrap_or(0) as usize;
        let hostname = self.get_hostbridge_hostname();

        let hostname: String = format!(
            "{hostname}:{} ",
            fmt_path_elide_ex(
                self.host_bridge().wrkdir.as_path(),
                width,
                hostname.len() + 3
            ) // 3 because of '/…/'
        );
        let files: Vec<Vec<LineStatic>> = self
            .host_bridge()
            .iter_files()
            .map(|x| {
                let mut span = SpanStatic::from(self.host_bridge().fmt_file(x));
                if self.host_bridge().enqueued().contains_key(x.path()) {
                    span.style = span.style.add_modifier(
                        TextModifiers::REVERSED | TextModifiers::UNDERLINED | TextModifiers::ITALIC,
                    );
                }

                vec![LineStatic::from(span)]
            })
            .collect();
        // Update content and title
        ui_result(self.app.attr(
            &Id::ExplorerHostBridge,
            Attribute::Content,
            AttrValue::Table(files),
        ));
        ui_result(self.app.attr(
            &Id::ExplorerHostBridge,
            Attribute::Title,
            AttrValue::Title(Title::from(hostname).alignment(HorizontalAlignment::Left)),
        ));
    }

    /// Update remote file list
    pub(in crate::ui::activities::filetransfer) fn update_remote_filelist(&mut self) {
        self.reload_remote_dir();
        self.reload_remote_filelist();
    }

    pub(in crate::ui::activities::filetransfer) fn get_tab_hostname(&self) -> String {
        match self.browser.tab() {
            FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => {
                self.get_hostbridge_hostname()
            }
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => self.get_remote_hostname(),
        }
    }

    pub(in crate::ui::activities::filetransfer) fn terminal_prompt(&self) -> String {
        const TERM_CYAN: &str = "\x1b[36m";
        const TERM_GREEN: &str = "\x1b[32m";
        const TERM_YELLOW: &str = "\x1b[33m";
        const TERM_RESET: &str = "\x1b[0m";

        let panel = self.browser.tab();
        match panel {
            FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => {
                let username = self
                    .context()
                    .host_bridge_params()
                    .and_then(|params| {
                        params
                            .username()
                            .map(|u| format!("{TERM_CYAN}{u}{TERM_RESET}@"))
                    })
                    .unwrap_or_default();
                let hostname = self.get_hostbridge_hostname();
                format!(
                    "{username}{TERM_GREEN}{hostname}:{TERM_YELLOW}{}{TERM_RESET}$ ",
                    fmt_path_elide_ex(
                        self.host_bridge().wrkdir.as_path(),
                        0,
                        hostname.len() + 3 // 3 because of '/…/'
                    )
                )
            }
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => {
                let username = self
                    .context()
                    .remote_params()
                    .and_then(|params| {
                        params
                            .username()
                            .map(|u| format!("{TERM_CYAN}{u}{TERM_RESET}@"))
                    })
                    .unwrap_or_default();
                let hostname = self.get_remote_hostname();
                let fmt_path = fmt_path_elide_ex(
                    self.remote().wrkdir.as_path(),
                    0,
                    hostname.len() + 3, // 3 because of '/…/'
                );
                let fmt_path = if fmt_path.starts_with('/') {
                    fmt_path
                } else {
                    format!("/{}", fmt_path)
                };

                format!("{username}{TERM_GREEN}{hostname}:{TERM_YELLOW}{fmt_path}{TERM_RESET}$ ",)
            }
        }
    }

    pub(in crate::ui::activities::filetransfer) fn reload_remote_filelist(&mut self) {
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
        let files: Vec<Vec<LineStatic>> = self
            .remote()
            .iter_files()
            .map(|x| {
                let mut span = SpanStatic::from(self.remote().fmt_file(x));
                if self.remote().enqueued().contains_key(x.path()) {
                    span.style = span.style.add_modifier(
                        TextModifiers::REVERSED | TextModifiers::UNDERLINED | TextModifiers::ITALIC,
                    );
                }

                vec![LineStatic::from(span)]
            })
            .collect();
        // Update content and title
        ui_result(self.app.attr(
            &Id::ExplorerRemote,
            Attribute::Content,
            AttrValue::Table(files),
        ));
        ui_result(self.app.attr(
            &Id::ExplorerRemote,
            Attribute::Title,
            AttrValue::Title(Title::from(hostname).alignment(HorizontalAlignment::Left)),
        ));
    }

    pub(in crate::ui::activities::filetransfer) fn update_progress_bar(
        &mut self,
        filename: String,
    ) {
        // Update the partial bar with the current file progress. The filename
        // goes into the gauge *label*, not a block title: in a multi-file
        // transfer the partial bar omits its top border to join the seam with the
        // full bar, but any top-positioned title forces a 1-row top inset in
        // `Block::inner`, which would shrink the partial bar to one inner row
        // while the full bar keeps two — making the two gauges unequal in height.
        ui_result(self.app.attr(
            &Id::TransferProgressBarPartial,
            Attribute::Text,
            AttrValue::String(format!("{filename} — {}", self.transfer.progress)),
        ));
        ui_result(self.app.attr(
            &Id::TransferProgressBarPartial,
            Attribute::Value,
            AttrValue::Payload(PropPayload::Single(PropValue::F64(
                self.transfer.progress.calc_partial_progress(),
            ))),
        ));
        ui_result(self.app.attr(
            &Id::TransferProgressBarPartial,
            Attribute::Title,
            AttrValue::Title(Title::from(filename).alignment(HorizontalAlignment::Center)),
        ));
        // Update the full bar with the overall progress (only for multi-file transfers)
        if !self.transfer.progress.is_single_file() {
            ui_result(self.app.attr(
                &Id::TransferProgressBarFull,
                Attribute::Value,
                AttrValue::Payload(PropPayload::Single(PropValue::F64(
                    self.transfer.progress.calc_full_progress(),
                ))),
            ));
            ui_result(
                self.app.attr(
                    &Id::TransferProgressBarFull,
                    Attribute::Title,
                    AttrValue::Title(
                        Title::from(format!(
                            "Total {}",
                            self.transfer.progress.file_count_display()
                        ))
                        .alignment(HorizontalAlignment::Center),
                    ),
                ),
            );
        }
    }

    /// Update the progress bar to reflect the pre-transfer scan state.
    ///
    /// Shows how many directories and files have been discovered so far and keeps
    /// the progress value at `0.0` since the total is not yet known.
    pub(in crate::ui::activities::filetransfer) fn update_scan_progress(
        &mut self,
        dirs: usize,
        files: usize,
    ) {
        // During the scan only the partial bar is rendered (the progress model
        // reports `is_single_file()`), so write the scan text there.
        ui_result(self.app.attr(
            &Id::TransferProgressBarPartial,
            Attribute::Text,
            AttrValue::String(format!("Scanning… {dirs} dirs, {files} files")),
        ));
        ui_result(self.app.attr(
            &Id::TransferProgressBarPartial,
            Attribute::Value,
            AttrValue::Payload(PropPayload::Single(PropValue::F64(0.0))),
        ));
    }

    /// Finalize find process
    pub(in crate::ui::activities::filetransfer) fn finalize_find(&mut self) {
        // Set found to none
        self.browser.del_found();
        // Restore tab
        let new_tab = match self.browser.tab() {
            FileExplorerTab::FindHostBridge => FileExplorerTab::HostBridge,
            FileExplorerTab::FindRemote => FileExplorerTab::Remote,
            _ => FileExplorerTab::HostBridge,
        };
        // Give focus to new tab
        match new_tab {
            FileExplorerTab::HostBridge => {
                ui_result(self.app.active(&Id::ExplorerHostBridge));
            }
            FileExplorerTab::Remote => {
                ui_result(self.app.active(&Id::ExplorerRemote));
            }
            FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => {
                ui_result(self.app.active(&Id::ExplorerFind));
            }
        }
        self.browser.change_tab(new_tab);
    }

    pub(in crate::ui::activities::filetransfer) fn update_find_list(&mut self) {
        let files: Vec<Vec<LineStatic>> = self
            .found()
            .unwrap()
            .iter_files()
            .map(|x| {
                let mut span = SpanStatic::from(self.found().unwrap().fmt_file(x));
                if self.found().unwrap().enqueued().contains_key(x.path()) {
                    span.style = span.style.add_modifier(
                        TextModifiers::REVERSED | TextModifiers::UNDERLINED | TextModifiers::ITALIC,
                    );
                }
                vec![LineStatic::from(span)]
            })
            .collect();
        ui_result(self.app.attr(
            &Id::ExplorerFind,
            Attribute::Content,
            AttrValue::Table(files),
        ));
    }

    pub(in crate::ui::activities::filetransfer) fn update_browser_file_list(&mut self) {
        match self.browser.tab() {
            FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => {
                self.update_host_bridge_filelist()
            }
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => self.update_remote_filelist(),
        }
    }

    pub(in crate::ui::activities::filetransfer) fn reload_browser_file_list(&mut self) {
        match self.browser.tab() {
            FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => {
                self.reload_host_bridge_filelist()
            }
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => self.reload_remote_filelist(),
        }
    }

    pub(in crate::ui::activities::filetransfer) fn update_browser_file_list_swapped(&mut self) {
        match self.browser.tab() {
            FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => {
                self.update_remote_filelist()
            }
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => {
                self.update_host_bridge_filelist()
            }
        }
    }
}
