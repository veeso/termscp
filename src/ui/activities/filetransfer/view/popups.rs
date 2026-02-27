//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

use remotefs::fs::{File, UnixPex};
use tuirealm::props::{PropPayload, PropValue, TextSpan};
use tuirealm::{AttrValue, Attribute};

use crate::explorer::FileSorting;
use crate::ui::activities::filetransfer::browser::FileExplorerTab;
use crate::ui::activities::filetransfer::components::ATTR_FILES;
use crate::ui::activities::filetransfer::{FileTransferActivity, Id, components, ui_result};

impl FileTransferActivity {
    // -- partials

    /// Mount info box
    pub(in crate::ui::activities::filetransfer) fn mount_info<S: AsRef<str>>(&mut self, text: S) {
        // Mount
        let info_color = self.theme().misc_info_dialog;
        ui_result(self.app.remount(
            Id::ErrorPopup,
            Box::new(components::ErrorPopup::new(text, info_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::ErrorPopup));
    }

    /// Mount error box
    pub(in crate::ui::activities::filetransfer) fn mount_error<S: AsRef<str>>(&mut self, text: S) {
        // Mount
        let error_color = self.theme().misc_error_dialog;
        ui_result(self.app.remount(
            Id::ErrorPopup,
            Box::new(components::ErrorPopup::new(text, error_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::ErrorPopup));
    }

    /// Umount error message
    pub(in crate::ui::activities::filetransfer) fn umount_error(&mut self) {
        let _ = self.app.umount(&Id::ErrorPopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_fatal<S: AsRef<str>>(&mut self, text: S) {
        self.umount_wait();
        // Mount
        let error_color = self.theme().misc_error_dialog;
        ui_result(self.app.remount(
            Id::FatalPopup,
            Box::new(components::FatalPopup::new(text, error_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::FatalPopup));
    }

    /// Umount fatal error message
    pub(in crate::ui::activities::filetransfer) fn umount_fatal(&mut self) {
        let _ = self.app.umount(&Id::FatalPopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_wait<S: AsRef<str>>(&mut self, text: S) {
        let color = self.theme().misc_info_dialog;
        ui_result(self.app.remount(
            Id::WaitPopup,
            Box::new(components::WaitPopup::new(text, color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::WaitPopup));
    }

    pub(in crate::ui::activities::filetransfer) fn mount_walkdir_wait(&mut self) {
        let color = self.theme().misc_info_dialog;
        ui_result(self.app.remount(
            Id::WaitPopup,
            Box::new(components::WalkdirWaitPopup::new(
                "Scanning current directory…",
                color,
            )),
            vec![],
        ));
        ui_result(self.app.active(&Id::WaitPopup));

        self.view();
    }

    pub(in crate::ui::activities::filetransfer) fn update_walkdir_entries(
        &mut self,
        entries: usize,
    ) {
        let text = format!("Scanning current directory… ({entries} items found)",);
        let _ = self.app.attr(
            &Id::WaitPopup,
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(vec![
                PropValue::TextSpan(TextSpan::from(text)),
                PropValue::TextSpan(TextSpan::from("Press 'CTRL+C' to abort")),
            ])),
        );

        self.view();
    }

    pub(in crate::ui::activities::filetransfer) fn mount_blocking_wait<S: AsRef<str>>(
        &mut self,
        text: S,
    ) {
        self.mount_wait(text);
        self.view();
    }

    pub(in crate::ui::activities::filetransfer) fn umount_wait(&mut self) {
        let _ = self.app.umount(&Id::WaitPopup);
    }

    /// Mount quit popup
    pub(in crate::ui::activities::filetransfer) fn mount_quit(&mut self) {
        // Protocol
        let quit_color = self.theme().misc_quit_dialog;
        ui_result(self.app.remount(
            Id::QuitPopup,
            Box::new(components::QuitPopup::new(quit_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::QuitPopup));
    }

    /// Umount quit popup
    pub(in crate::ui::activities::filetransfer) fn umount_quit(&mut self) {
        let _ = self.app.umount(&Id::QuitPopup);
    }

    /// Mount disconnect popup
    pub(in crate::ui::activities::filetransfer) fn mount_disconnect(&mut self) {
        // Protocol
        let quit_color = self.theme().misc_quit_dialog;
        ui_result(self.app.remount(
            Id::DisconnectPopup,
            Box::new(components::DisconnectPopup::new(quit_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::DisconnectPopup));
    }

    /// Umount disconnect popup
    pub(in crate::ui::activities::filetransfer) fn umount_disconnect(&mut self) {
        let _ = self.app.umount(&Id::DisconnectPopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_chmod(
        &mut self,
        mode: UnixPex,
        title: String,
    ) {
        // Mount
        let color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::ChmodPopup,
            Box::new(components::ChmodPopup::new(mode, color, title)),
            vec![],
        ));
        ui_result(self.app.active(&Id::ChmodPopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_chmod(&mut self) {
        let _ = self.app.umount(&Id::ChmodPopup);
    }

    pub(in crate::ui::activities::filetransfer) fn umount_filter(&mut self) {
        let _ = self.app.umount(&Id::FilterPopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_filter(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::FilterPopup,
            Box::new(components::FilterPopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::FilterPopup));
    }

    pub(in crate::ui::activities::filetransfer) fn mount_copy(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::CopyPopup,
            Box::new(components::CopyPopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::CopyPopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_copy(&mut self) {
        let _ = self.app.umount(&Id::CopyPopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_exec(&mut self) {
        let tab = self.browser.tab();
        let id = match tab {
            FileExplorerTab::HostBridge => Id::TerminalHostBridge,
            FileExplorerTab::Remote => Id::TerminalRemote,
            _ => {
                error!("Cannot mount terminal on this tab");
                return;
            }
        };

        let border = match tab {
            FileExplorerTab::HostBridge => self.theme().transfer_local_explorer_highlighted,
            FileExplorerTab::Remote => self.theme().transfer_remote_explorer_highlighted,
            _ => {
                error!("Cannot mount terminal on this tab");
                return;
            }
        };

        let input_color = self.theme().misc_input_dialog;
        ui_result(
            self.app.remount(
                id.clone(),
                Box::new(
                    components::Terminal::default()
                        .foreground(input_color)
                        .prompt(self.terminal_prompt())
                        .title(format!("Terminal - {}", self.get_tab_hostname()))
                        .border_color(border),
                ),
                vec![],
            ),
        );
        ui_result(self.app.active(&id));
    }

    /// Update the terminal prompt based on the current directory
    pub(in crate::ui::activities::filetransfer) fn update_terminal_prompt(&mut self) {
        let prompt = self.terminal_prompt();
        let id = match self.browser.tab() {
            FileExplorerTab::HostBridge => Id::TerminalHostBridge,
            FileExplorerTab::Remote => Id::TerminalRemote,
            _ => {
                error!("Cannot update terminal prompt on this tab");
                return;
            }
        };
        let _ = self
            .app
            .attr(&id, Attribute::Content, AttrValue::String(prompt));
    }

    /// Print output to terminal
    pub(in crate::ui::activities::filetransfer) fn print_terminal(&mut self, text: String) {
        // get id
        let focus = self.app.focus().unwrap().clone();

        // replace all \n with \r\n
        let mut text = text.replace('\n', "\r\n");
        if !text.ends_with("\r\n") && !text.is_empty() {
            text.push_str("\r\n");
        }
        let _ = self
            .app
            .attr(&focus, Attribute::Text, AttrValue::String(text));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_exec(&mut self) {
        let focus = self.app.focus().unwrap().clone();
        let _ = self.app.umount(&focus);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_find(
        &mut self,
        msg: impl ToString,
        fuzzy_search: bool,
    ) {
        // Get color
        let (bg, fg, hg) = match self.browser.tab() {
            FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => (
                self.theme().transfer_local_explorer_background,
                self.theme().transfer_local_explorer_foreground,
                self.theme().transfer_local_explorer_highlighted,
            ),
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => (
                self.theme().transfer_remote_explorer_background,
                self.theme().transfer_remote_explorer_foreground,
                self.theme().transfer_remote_explorer_highlighted,
            ),
        };

        // Mount component
        ui_result(self.app.remount(
            Id::ExplorerFind,
            if fuzzy_search {
                Box::new(components::ExplorerFuzzy::new(
                    msg.to_string(),
                    &[],
                    bg,
                    fg,
                    hg,
                ))
            } else {
                Box::new(components::ExplorerFind::new(
                    msg.to_string(),
                    &[],
                    bg,
                    fg,
                    hg,
                ))
            },
            vec![],
        ));
        ui_result(self.app.active(&Id::ExplorerFind));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_find(&mut self) {
        let _ = self.app.umount(&Id::ExplorerFind);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_goto(&mut self) {
        // get files
        let files = self
            .browser
            .explorer()
            .iter_files()
            .filter(|f| f.is_dir() || f.is_symlink())
            .map(|f| f.path().to_string_lossy().to_string())
            .collect::<Vec<String>>();

        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::GotoPopup,
            Box::new(components::GotoPopup::new(input_color, files)),
            vec![],
        ));
        ui_result(self.app.active(&Id::GotoPopup));
    }

    pub(in crate::ui::activities::filetransfer) fn update_goto(&mut self, files: Vec<String>) {
        let payload = files
            .into_iter()
            .map(PropValue::Str)
            .collect::<Vec<PropValue>>();

        let _ = self.app.attr(
            &Id::GotoPopup,
            Attribute::Custom(ATTR_FILES),
            AttrValue::Payload(PropPayload::Vec(payload)),
        );
    }

    pub(in crate::ui::activities::filetransfer) fn umount_goto(&mut self) {
        let _ = self.app.umount(&Id::GotoPopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_mkdir(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::MkdirPopup,
            Box::new(components::MkdirPopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::MkdirPopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_mkdir(&mut self) {
        let _ = self.app.umount(&Id::MkdirPopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_newfile(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::NewfilePopup,
            Box::new(components::NewfilePopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::NewfilePopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_newfile(&mut self) {
        let _ = self.app.umount(&Id::NewfilePopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_openwith(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::OpenWithPopup,
            Box::new(components::OpenWithPopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::OpenWithPopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_openwith(&mut self) {
        let _ = self.app.umount(&Id::OpenWithPopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_rename(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::RenamePopup,
            Box::new(components::RenamePopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::RenamePopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_rename(&mut self) {
        let _ = self.app.umount(&Id::RenamePopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_saveas(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::SaveAsPopup,
            Box::new(components::SaveAsPopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::SaveAsPopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_saveas(&mut self) {
        let _ = self.app.umount(&Id::SaveAsPopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_progress_bar(
        &mut self,
        root_name: String,
    ) {
        let prog_color_full = self.theme().transfer_progress_bar_full;
        let prog_color_partial = self.theme().transfer_progress_bar_partial;
        ui_result(self.app.remount(
            Id::ProgressBarFull,
            Box::new(components::ProgressBarFull::new(
                0.0,
                "",
                &root_name,
                prog_color_full,
            )),
            vec![],
        ));
        ui_result(self.app.remount(
            Id::ProgressBarPartial,
            Box::new(components::ProgressBarPartial::new(
                0.0,
                "",
                "Please wait",
                prog_color_partial,
            )),
            vec![],
        ));
        ui_result(self.app.active(&Id::ProgressBarPartial));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_progress_bar(&mut self) {
        let _ = self.app.umount(&Id::ProgressBarPartial);
        let _ = self.app.umount(&Id::ProgressBarFull);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_file_sorting(&mut self) {
        let sorting_color = self.theme().transfer_status_sorting;
        let sorting: FileSorting = match self.browser.tab() {
            FileExplorerTab::HostBridge => self.host_bridge().get_file_sorting(),
            FileExplorerTab::Remote => self.remote().get_file_sorting(),
            _ => return,
        };
        ui_result(self.app.remount(
            Id::SortingPopup,
            Box::new(components::SortingPopup::new(sorting, sorting_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::SortingPopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_file_sorting(&mut self) {
        let _ = self.app.umount(&Id::SortingPopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_radio_delete(&mut self) {
        let warn_color = self.theme().misc_warn_dialog;
        ui_result(self.app.remount(
            Id::DeletePopup,
            Box::new(components::DeletePopup::new(warn_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::DeletePopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_radio_delete(&mut self) {
        let _ = self.app.umount(&Id::DeletePopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_radio_watch(
        &mut self,
        watch: bool,
        local: &str,
        remote: &str,
    ) {
        let info_color = self.theme().misc_info_dialog;
        ui_result(self.app.remount(
            Id::WatcherPopup,
            Box::new(components::WatcherPopup::new(
                watch, local, remote, info_color,
            )),
            vec![],
        ));
        ui_result(self.app.active(&Id::WatcherPopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_radio_watcher(&mut self) {
        let _ = self.app.umount(&Id::WatcherPopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_watched_paths_list(
        &mut self,
        paths: &[std::path::PathBuf],
    ) {
        let info_color = self.theme().misc_info_dialog;
        ui_result(self.app.remount(
            Id::WatchedPathsList,
            Box::new(components::WatchedPathsList::new(paths, info_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::WatchedPathsList));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_watched_paths_list(&mut self) {
        let _ = self.app.umount(&Id::WatchedPathsList);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_radio_replace(&mut self, file_name: &str) {
        let warn_color = self.theme().misc_warn_dialog;
        ui_result(self.app.remount(
            Id::ReplacePopup,
            Box::new(components::ReplacePopup::new(Some(file_name), warn_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::ReplacePopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_radio_replace(&mut self) {
        let _ = self.app.umount(&Id::ReplacePopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_file_info(&mut self, file: &File) {
        ui_result(self.app.remount(
            Id::FileInfoPopup,
            Box::new(components::FileInfoPopup::new(file)),
            vec![],
        ));
        ui_result(self.app.active(&Id::FileInfoPopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_file_info(&mut self) {
        let _ = self.app.umount(&Id::FileInfoPopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_symlink(&mut self) {
        let input_color = self.theme().misc_input_dialog;
        ui_result(self.app.remount(
            Id::SymlinkPopup,
            Box::new(components::SymlinkPopup::new(input_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::SymlinkPopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_symlink(&mut self) {
        let _ = self.app.umount(&Id::SymlinkPopup);
    }

    pub(in crate::ui::activities::filetransfer) fn mount_sync_browsing_mkdir_popup(
        &mut self,
        dir_name: &str,
    ) {
        let color = self.theme().misc_info_dialog;
        ui_result(self.app.remount(
            Id::SyncBrowsingMkdirPopup,
            Box::new(components::SyncBrowsingMkdirPopup::new(color, dir_name)),
            vec![],
        ));
        ui_result(self.app.active(&Id::SyncBrowsingMkdirPopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_sync_browsing_mkdir_popup(&mut self) {
        let _ = self.app.umount(&Id::SyncBrowsingMkdirPopup);
    }

    /// Mount help
    pub(in crate::ui::activities::filetransfer) fn mount_help(&mut self) {
        let key_color = self.theme().misc_keys;
        ui_result(self.app.remount(
            Id::KeybindingsPopup,
            Box::new(components::KeybindingsPopup::new(key_color)),
            vec![],
        ));
        ui_result(self.app.active(&Id::KeybindingsPopup));
    }

    pub(in crate::ui::activities::filetransfer) fn umount_help(&mut self) {
        let _ = self.app.umount(&Id::KeybindingsPopup);
    }
}
