//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
// externals
use remotefs::fs::File;
use tuirealm::{State, StateValue, Update};

use super::actions::SelectedFile;
use super::actions::walkdir::WalkdirError;
use super::browser::{FileExplorerTab, FoundExplorerTab};
use super::{
    ExitReason, FileTransferActivity, Id, MarkQueue, Msg, TransferMsg, TransferOpts, UiMsg,
    ui_result,
};

impl Update<Msg> for FileTransferActivity {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        match msg.unwrap_or(Msg::None) {
            Msg::None => None,
            Msg::PendingAction(_) => {
                // NOTE: Pending actions must be handled directly in the action
                None
            }
            Msg::Transfer(msg) => self.update_transfer(msg),
            Msg::Ui(msg) => self.update_ui(msg),
        }
    }
}

impl FileTransferActivity {
    fn update_transfer(&mut self, msg: TransferMsg) -> Option<Msg> {
        match msg {
            TransferMsg::AbortTransfer => {
                self.transfer.abort();
            }
            TransferMsg::AbortWalkdir => {
                self.walkdir.aborted = true;
            }
            TransferMsg::Chmod(mode) => {
                self.umount_chmod();
                self.mount_blocking_wait("Applying new file mode…");
                // Skip chmod on Windows localhost
                if !(self.is_local_tab() && self.host_bridge.is_localhost() && cfg!(windows)) {
                    self.action_chmod(mode);
                }
                self.umount_wait();
                self.update_browser_file_list();
            }
            TransferMsg::CopyFileTo(dest) => {
                self.umount_copy();
                self.mount_blocking_wait("Copying file(s)…");
                self.action_copy(dest);
                self.umount_wait();
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::CreateSymlink(name) => {
                self.umount_symlink();
                self.mount_blocking_wait("Creating symlink…");
                self.action_symlink(name);
                self.umount_wait();
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::DeleteFile => {
                self.umount_radio_delete();
                self.mount_blocking_wait("Removing file(s)…");
                match self.browser.tab() {
                    FileExplorerTab::HostBridge | FileExplorerTab::Remote => {
                        self.action_delete();
                    }
                    FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => {
                        self.action_find_delete();
                        // Remove deleted entries from the find-result list
                        match self.app.state(&Id::ExplorerFind) {
                            Ok(State::One(StateValue::Usize(idx))) => {
                                self.found_mut().unwrap().del_entry(idx);
                            }
                            Ok(State::Vec(values)) => {
                                values
                                    .iter()
                                    .map(|x| match x {
                                        StateValue::Usize(v) => *v,
                                        _ => 0,
                                    })
                                    .for_each(|x| self.found_mut().unwrap().del_entry(x));
                            }
                            _ => {}
                        }
                        self.update_find_list();
                    }
                }
                self.umount_wait();
                // Reload files
                self.update_browser_file_list();
            }
            TransferMsg::EnterDirectory if self.browser.tab() == FileExplorerTab::HostBridge => {
                if let Some(entry) = self.get_local_selected_file() {
                    self.action_submit_local(entry);
                    // Update file list if sync
                    if self.browser.sync_browsing && self.browser.found().is_none() {
                        self.update_remote_filelist();
                    }
                    self.update_host_bridge_filelist();
                }
            }
            TransferMsg::EnterDirectory if self.browser.tab() == FileExplorerTab::Remote => {
                if let Some(entry) = self.get_remote_selected_file() {
                    self.action_submit_remote(entry);
                    // Update file list if sync
                    if self.browser.sync_browsing && self.browser.found().is_none() {
                        self.update_host_bridge_filelist();
                    }
                    self.update_remote_filelist();
                }
            }
            TransferMsg::EnterDirectory => {
                // NOTE: is find explorer
                // Find changedir
                self.action_find_changedir();
                // Umount find
                self.umount_find();
                // Finalize find
                self.finalize_find();
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::ExecuteCmd(cmd) => {
                // Exec command
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_local_exec(cmd),
                    FileExplorerTab::Remote => self.action_remote_exec(cmd),
                    _ => error!("Found tab doesn't support EXEC"),
                };
            }
            TransferMsg::GetFileSize => {
                self.action_get_file_size();
            }
            TransferMsg::GoTo(dir) => {
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_change_local_dir(dir),
                    FileExplorerTab::Remote => self.action_change_remote_dir(dir),
                    _ => error!("Found tab doesn't support GOTO"),
                }
                // Umount
                self.umount_goto();
                // Reload files if sync
                if self.browser.sync_browsing && self.browser.found().is_none() {
                    self.update_browser_file_list_swapped();
                }
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::GoToParentDirectory => {
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => {
                        self.action_go_to_local_upper_dir();
                        if self.browser.sync_browsing && self.browser.found().is_none() {
                            self.update_remote_filelist();
                        }
                        // Reload file list component
                        self.update_host_bridge_filelist()
                    }
                    FileExplorerTab::Remote => {
                        self.action_go_to_remote_upper_dir();
                        if self.browser.sync_browsing && self.browser.found().is_none() {
                            self.update_host_bridge_filelist();
                        }
                        // Reload file list component
                        self.update_remote_filelist()
                    }
                    _ => {}
                }
            }
            TransferMsg::GoToPreviousDirectory => {
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => {
                        self.action_go_to_previous_local_dir();
                        if self.browser.sync_browsing && self.browser.found().is_none() {
                            self.update_remote_filelist();
                        }
                        // Reload file list component
                        self.update_host_bridge_filelist()
                    }
                    FileExplorerTab::Remote => {
                        self.action_go_to_previous_remote_dir();
                        if self.browser.sync_browsing && self.browser.found().is_none() {
                            self.update_host_bridge_filelist();
                        }
                        // Reload file list component
                        self.update_remote_filelist()
                    }
                    _ => {}
                }
            }
            TransferMsg::InitFuzzySearch => {
                // Mount wait
                self.mount_walkdir_wait();
                // Find
                let res: Result<Vec<File>, WalkdirError> = match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_walkdir_local(),
                    FileExplorerTab::Remote => self.action_walkdir_remote(),
                    _ => {
                        error!("Trying to search for files, while already in a find result");
                        self.umount_wait();
                        self.redraw = true;
                        return None;
                    }
                };
                // Umount wait
                self.umount_wait();
                // Match result
                match res {
                    Err(WalkdirError::Error(err)) => {
                        // Mount error
                        self.mount_error(err.as_str());
                    }
                    Err(WalkdirError::Aborted) => {
                        self.mount_info("Search aborted");
                    }
                    Ok(files) if files.is_empty() => {
                        // If no file has been found notify user
                        self.mount_info("There are no files in the current directory");
                    }
                    Ok(files) => {
                        // Get wrkdir
                        let wrkdir = match self.browser.tab() {
                            FileExplorerTab::HostBridge => self.host_bridge().wrkdir.clone(),
                            _ => self.remote().wrkdir.clone(),
                        };
                        // Create explorer and load files
                        self.browser.set_found(
                            match self.browser.tab() {
                                FileExplorerTab::HostBridge => FoundExplorerTab::Local,
                                _ => FoundExplorerTab::Remote,
                            },
                            files,
                            wrkdir.as_path(),
                        );
                        // init fuzzy search to display nothing
                        self.browser.init_fuzzy_search();
                        // Mount result widget
                        self.mount_find(format!(r#"Searching at "{}""#, wrkdir.display()), true);
                        self.update_find_list();
                        // Initialize tab
                        self.browser.change_tab(match self.browser.tab() {
                            FileExplorerTab::HostBridge => FileExplorerTab::FindHostBridge,
                            FileExplorerTab::Remote => FileExplorerTab::FindRemote,
                            _ => FileExplorerTab::FindHostBridge,
                        });
                    }
                }
            }
            TransferMsg::Mkdir(dir) => {
                self.action_mkdir(dir);
                self.umount_mkdir();
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::NewFile(name) => {
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_local_newfile(name),
                    FileExplorerTab::Remote => self.action_remote_newfile(name),
                    _ => {}
                }
                self.umount_newfile();
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::OpenFile => match self.browser.tab() {
                FileExplorerTab::HostBridge => self.action_open_local(),
                FileExplorerTab::Remote => self.action_open_remote(),
                FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => {
                    self.action_find_open()
                }
            },
            TransferMsg::OpenFileWith(prog) => {
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_local_open_with(&prog),
                    FileExplorerTab::Remote => self.action_remote_open_with(&prog),
                    FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => {
                        self.action_find_open_with(&prog)
                    }
                }
                self.umount_openwith();
            }
            TransferMsg::OpenTextFile => {
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_edit_local_file(),
                    FileExplorerTab::Remote => self.action_edit_remote_file(),
                    _ => {}
                }
                self.update_browser_file_list()
            }
            TransferMsg::ReloadDir => self.update_browser_file_list(),
            TransferMsg::RenameFile(dest) => {
                self.umount_rename();
                self.mount_blocking_wait("Moving file(s)…");
                self.action_rename(dest);
                self.umount_wait();
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::RescanGotoFiles(path) => {
                let files = self.action_scan(&path).unwrap_or_default();
                let files = files
                    .into_iter()
                    .filter(|f| f.is_dir() || f.is_symlink())
                    .map(|f| f.path().to_string_lossy().to_string())
                    .collect();
                self.update_goto(files);
            }
            TransferMsg::SaveFileAs(dest) => {
                self.umount_saveas();
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_local_saveas(dest),
                    FileExplorerTab::Remote => self.action_remote_saveas(dest),
                    FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => {
                        // Get entry
                        self.action_find_transfer(TransferOpts::default().save_as(Some(dest)));
                    }
                }
                self.umount_saveas();
                // Reload files
                self.update_browser_file_list_swapped();
            }

            TransferMsg::ToggleWatch => self.action_toggle_watch(),
            TransferMsg::ToggleWatchFor(index) => self.action_toggle_watch_for(index),
            TransferMsg::TransferFile => {
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_local_send(),
                    FileExplorerTab::Remote => self.action_remote_recv(),
                    FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => {
                        self.action_find_transfer(TransferOpts::default())
                    }
                }
                self.update_browser_file_list_swapped();
            }
        }
        // Force redraw
        self.redraw = true;
        None
    }

    fn update_ui(&mut self, msg: UiMsg) -> Option<Msg> {
        match msg {
            UiMsg::CloseChmodPopup => self.umount_chmod(),
            UiMsg::ChangeFileSorting(sorting) => {
                match self.browser.tab() {
                    FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => {
                        self.host_bridge_mut().sort_by(sorting);
                        self.refresh_local_status_bar();
                    }
                    FileExplorerTab::Remote | FileExplorerTab::FindRemote => {
                        self.remote_mut().sort_by(sorting);
                        self.refresh_remote_status_bar()
                    }
                }
                self.update_browser_file_list();
            }
            UiMsg::ChangeTransferWindow => {
                let new_tab = match self.browser.tab() {
                    FileExplorerTab::HostBridge if self.browser.found().is_some() => {
                        FileExplorerTab::FindRemote
                    }
                    FileExplorerTab::FindHostBridge | FileExplorerTab::HostBridge => {
                        FileExplorerTab::Remote
                    }
                    FileExplorerTab::Remote if self.browser.found().is_some() => {
                        FileExplorerTab::FindHostBridge
                    }
                    FileExplorerTab::FindRemote | FileExplorerTab::Remote => {
                        FileExplorerTab::HostBridge
                    }
                };
                // Set focus
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
            UiMsg::CloseCopyPopup => self.umount_copy(),
            UiMsg::CloseDeletePopup => self.umount_radio_delete(),
            UiMsg::CloseDisconnectPopup => self.umount_disconnect(),
            UiMsg::CloseErrorPopup => self.umount_error(),
            UiMsg::CloseExecPopup => {
                self.browser.toggle_terminal(false);
                self.umount_exec();
            }
            UiMsg::CloseFatalPopup => {
                self.umount_fatal();
                self.exit_reason = Some(ExitReason::Disconnect);
            }
            UiMsg::CloseFileInfoPopup => self.umount_file_info(),
            UiMsg::CloseFileSortingPopup => self.umount_file_sorting(),
            UiMsg::CloseFilterPopup => self.umount_filter(),
            UiMsg::CloseFindExplorer => {
                self.finalize_find();
                self.umount_find();
            }
            UiMsg::CloseGotoPopup => self.umount_goto(),
            UiMsg::CloseKeybindingsPopup => self.umount_help(),
            UiMsg::CloseMkdirPopup => self.umount_mkdir(),
            UiMsg::CloseNewFilePopup => self.umount_newfile(),
            UiMsg::CloseOpenWithPopup => self.umount_openwith(),
            UiMsg::CloseQuitPopup => self.umount_quit(),
            UiMsg::CloseRenamePopup => self.umount_rename(),
            UiMsg::CloseSaveAsPopup => self.umount_saveas(),
            UiMsg::CloseSymlinkPopup => self.umount_symlink(),
            UiMsg::CloseWatchedPathsList => self.umount_watched_paths_list(),
            UiMsg::CloseWatcherPopup => self.umount_radio_watcher(),
            UiMsg::Disconnect => {
                self.disconnect();
                self.umount_disconnect();
            }
            UiMsg::FilterFiles(filter) => {
                self.umount_filter();
                let files = self.filter(&filter);
                // Get wrkdir
                let wrkdir = match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.host_bridge().wrkdir.clone(),
                    _ => self.remote().wrkdir.clone(),
                };
                // Create explorer and load files
                self.browser.set_found(
                    match self.browser.tab() {
                        FileExplorerTab::HostBridge => FoundExplorerTab::Local,
                        _ => FoundExplorerTab::Remote,
                    },
                    files,
                    wrkdir.as_path(),
                );
                // Mount result widget
                self.mount_find(&filter, false);
                self.update_find_list();
                // Initialize tab
                self.browser.change_tab(match self.browser.tab() {
                    FileExplorerTab::HostBridge => FileExplorerTab::FindHostBridge,
                    FileExplorerTab::Remote => FileExplorerTab::FindRemote,
                    _ => FileExplorerTab::FindHostBridge,
                });
            }
            UiMsg::FuzzySearch(needle) => {
                self.browser.fuzzy_search(&needle);
                self.update_find_list();
            }
            UiMsg::GoToTransferQueue => {
                ui_result(self.app.active(&Id::TransferQueueHostBridge));
            }
            UiMsg::LogBackTabbed => {
                ui_result(self.app.active(&Id::ExplorerHostBridge));
            }
            UiMsg::MarkFile(index) => {
                self.action_mark_file(index);
            }
            UiMsg::MarkAll => {
                self.action_mark_all();
            }
            UiMsg::MarkClear => {
                self.action_mark_clear();
            }
            UiMsg::MarkRemove(tab, path) => match tab {
                MarkQueue::Local => {
                    self.host_bridge_mut().dequeue(&path);
                    self.reload_host_bridge_filelist();
                    self.refresh_host_bridge_transfer_queue();
                }
                MarkQueue::Remote => {
                    self.remote_mut().dequeue(&path);
                    self.reload_remote_filelist();
                    self.refresh_remote_transfer_queue();
                }
            },
            UiMsg::Quit => {
                self.disconnect_and_quit();
                self.umount_quit();
            }
            UiMsg::ShowChmodPopup => {
                // On Windows localhost, chmod is not supported
                #[cfg(win)]
                let selected_file = if self.is_local_tab() {
                    SelectedFile::None
                } else {
                    self.get_selected_entries()
                };
                #[cfg(posix)]
                let selected_file = self.get_selected_entries();

                if let Some(mode) = selected_file.unix_pex() {
                    self.mount_chmod(
                        mode,
                        match selected_file {
                            SelectedFile::Many(files) => {
                                format!("changing mode for {} files…", files.len())
                            }
                            SelectedFile::One(file) => {
                                format!("changing mode for {}…", file.name())
                            }
                            SelectedFile::None => "".to_string(),
                        },
                    );
                }
            }
            UiMsg::ShowCopyPopup => self.mount_copy(),
            UiMsg::ShowDeletePopup => self.mount_radio_delete(),
            UiMsg::ShowDisconnectPopup => self.mount_disconnect(),
            UiMsg::ShowTerminal => {
                self.browser.toggle_terminal(true);
                self.mount_exec()
            }
            UiMsg::ShowFileInfoPopup => {
                if let SelectedFile::One(file) = self.get_selected_entries() {
                    self.mount_file_info(&file);
                }
            }
            UiMsg::ShowFileSortingPopup => self.mount_file_sorting(),
            UiMsg::ShowFilterPopup => self.mount_filter(),
            UiMsg::ShowGotoPopup => self.mount_goto(),
            UiMsg::ShowKeybindingsPopup => self.mount_help(),
            UiMsg::ShowMkdirPopup => self.mount_mkdir(),
            UiMsg::ShowNewFilePopup => self.mount_newfile(),
            UiMsg::ShowOpenWithPopup => self.mount_openwith(),
            UiMsg::ShowQuitPopup => self.mount_quit(),
            UiMsg::ShowRenamePopup => self.mount_rename(),
            UiMsg::ShowSaveAsPopup => self.mount_saveas(),
            UiMsg::ShowSymlinkPopup => {
                // Symlink is not available from find-result tabs
                let can_symlink = match self.browser.tab() {
                    FileExplorerTab::HostBridge | FileExplorerTab::Remote => self.is_selected_one(),
                    FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => false,
                };
                if can_symlink {
                    self.mount_symlink();
                } else {
                    self.mount_error(
                        "Symlink cannot be performed if more than one file is selected",
                    );
                }
            }
            UiMsg::ShowWatchedPathsList => self.action_show_watched_paths_list(),
            UiMsg::ShowWatcherPopup => self.action_show_radio_watch(),
            UiMsg::ToggleHiddenFiles => match self.browser.tab() {
                FileExplorerTab::FindHostBridge | FileExplorerTab::HostBridge => {
                    self.browser.host_bridge_mut().toggle_hidden_files();
                    self.refresh_local_status_bar();
                    self.update_browser_file_list();
                }
                FileExplorerTab::FindRemote | FileExplorerTab::Remote => {
                    self.browser.remote_mut().toggle_hidden_files();
                    self.refresh_remote_status_bar();
                    self.update_browser_file_list();
                }
            },
            UiMsg::ToggleSyncBrowsing => {
                self.browser.toggle_sync_browsing();
                self.refresh_remote_status_bar();
            }
            UiMsg::WindowResized => {
                self.redraw = true;
            }

            UiMsg::BottomPanelLeft => match self.app.focus() {
                Some(Id::TransferQueueHostBridge) => {
                    ui_result(self.app.active(&Id::Log));
                }
                Some(Id::TransferQueueRemote) => {
                    ui_result(self.app.active(&Id::TransferQueueHostBridge));
                }
                Some(Id::Log) => {
                    ui_result(self.app.active(&Id::TransferQueueRemote));
                }
                _ => {}
            },
            UiMsg::BottomPanelRight => match self.app.focus() {
                Some(Id::TransferQueueHostBridge) => {
                    ui_result(self.app.active(&Id::TransferQueueRemote));
                }
                Some(Id::TransferQueueRemote) => {
                    ui_result(self.app.active(&Id::Log));
                }
                Some(Id::Log) => {
                    ui_result(self.app.active(&Id::TransferQueueHostBridge));
                }
                _ => {}
            },
        }
        None
    }
}
