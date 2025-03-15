//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
// externals
use remotefs::fs::File;
use tuirealm::props::{AttrValue, Attribute};
use tuirealm::{State, StateValue, Update};

use super::actions::SelectedFile;
use super::actions::walkdir::WalkdirError;
use super::browser::{FileExplorerTab, FoundExplorerTab};
use super::{ExitReason, FileTransferActivity, Id, Msg, TransferMsg, TransferOpts, UiMsg};

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
                match self.browser.tab() {
                    FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge
                        if self.host_bridge.is_localhost() && cfg!(windows) => {}
                    FileExplorerTab::HostBridge => self.action_local_chmod(mode),
                    FileExplorerTab::FindHostBridge => self.action_find_local_chmod(mode),
                    FileExplorerTab::Remote => self.action_remote_chmod(mode),
                    FileExplorerTab::FindRemote => self.action_find_remote_chmod(mode),
                }
                self.umount_wait();
                self.update_browser_file_list();
            }
            TransferMsg::CopyFileTo(dest) => {
                self.umount_copy();
                self.mount_blocking_wait("Copying file(s)…");
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_local_copy(dest),
                    FileExplorerTab::Remote => self.action_remote_copy(dest),
                    _ => panic!("Found tab doesn't support COPY"),
                }
                self.umount_wait();
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::CreateSymlink(name) => {
                self.umount_symlink();
                self.mount_blocking_wait("Creating symlink…");
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_local_symlink(name),
                    FileExplorerTab::Remote => self.action_remote_symlink(name),
                    _ => panic!("Found tab doesn't support SYMLINK"),
                }
                self.umount_wait();
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::DeleteFile => {
                self.umount_radio_delete();
                self.mount_blocking_wait("Removing file(s)…");
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_local_delete(),
                    FileExplorerTab::Remote => self.action_remote_delete(),
                    FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => {
                        // Get entry
                        self.action_find_delete();
                        // Delete entries
                        match self.app.state(&Id::ExplorerFind) {
                            Ok(State::One(StateValue::Usize(idx))) => {
                                // Reload entries
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
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.update_host_bridge_filelist(),
                    FileExplorerTab::Remote => self.update_remote_filelist(),
                    FileExplorerTab::FindHostBridge => self.update_host_bridge_filelist(),
                    FileExplorerTab::FindRemote => self.update_remote_filelist(),
                }
            }
            TransferMsg::EnterDirectory if self.browser.tab() == FileExplorerTab::HostBridge => {
                if let SelectedFile::One(entry) = self.get_local_selected_entries() {
                    self.action_submit_local(entry);
                    // Update file list if sync
                    if self.browser.sync_browsing && self.browser.found().is_none() {
                        self.update_remote_filelist();
                    }
                    self.update_host_bridge_filelist();
                }
            }
            TransferMsg::EnterDirectory if self.browser.tab() == FileExplorerTab::Remote => {
                if let SelectedFile::One(entry) = self.get_remote_selected_entries() {
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
                // Exex command
                self.umount_exec();
                self.mount_blocking_wait(format!("Executing '{cmd}'…").as_str());
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_local_exec(cmd),
                    FileExplorerTab::Remote => self.action_remote_exec(cmd),
                    _ => panic!("Found tab doesn't support EXEC"),
                }
                self.umount_wait();
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::GoTo(dir) => {
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_change_local_dir(dir),
                    FileExplorerTab::Remote => self.action_change_remote_dir(dir),
                    _ => panic!("Found tab doesn't support GOTO"),
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
                    _ => panic!("Trying to search for files, while already in a find result"),
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
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_local_mkdir(dir),
                    FileExplorerTab::Remote => self.action_remote_mkdir(dir),
                    _ => {}
                }
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
                match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.action_local_rename(dest),
                    FileExplorerTab::Remote => self.action_remote_rename(dest),
                    _ => {}
                }
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
                        assert!(self.app.active(&Id::ExplorerHostBridge).is_ok())
                    }
                    FileExplorerTab::Remote => {
                        assert!(self.app.active(&Id::ExplorerRemote).is_ok())
                    }
                    FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => {
                        assert!(self.app.active(&Id::ExplorerFind).is_ok())
                    }
                }
                self.browser.change_tab(new_tab);
            }
            UiMsg::CloseCopyPopup => self.umount_copy(),
            UiMsg::CloseDeletePopup => self.umount_radio_delete(),
            UiMsg::CloseDisconnectPopup => self.umount_disconnect(),
            UiMsg::CloseErrorPopup => self.umount_error(),
            UiMsg::CloseExecPopup => self.umount_exec(),
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
            UiMsg::ShowLogPanel => {
                assert!(self.app.active(&Id::Log).is_ok());
            }
            UiMsg::LogBackTabbed => {
                assert!(self.app.active(&Id::ExplorerHostBridge).is_ok());
            }
            UiMsg::Quit => {
                self.disconnect_and_quit();
                self.umount_quit();
            }
            UiMsg::ReplacePopupTabbed => {
                if let Ok(Some(AttrValue::Flag(true))) =
                    self.app.query(&Id::ReplacePopup, Attribute::Focus)
                {
                    assert!(self.app.active(&Id::ReplacingFilesListPopup).is_ok());
                } else {
                    assert!(self.app.active(&Id::ReplacePopup).is_ok());
                }
            }
            UiMsg::ShowChmodPopup => {
                let selected_file = match self.browser.tab() {
                    #[cfg(posix)]
                    FileExplorerTab::HostBridge => self.get_local_selected_entries(),
                    #[cfg(posix)]
                    FileExplorerTab::FindHostBridge => self.get_found_selected_entries(),
                    FileExplorerTab::Remote => self.get_remote_selected_entries(),
                    FileExplorerTab::FindRemote => self.get_found_selected_entries(),
                    #[cfg(win)]
                    FileExplorerTab::HostBridge | FileExplorerTab::FindHostBridge => {
                        SelectedFile::None
                    }
                };
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
            UiMsg::ShowExecPopup => self.mount_exec(),
            UiMsg::ShowFileInfoPopup if self.browser.tab() == FileExplorerTab::HostBridge => {
                if let SelectedFile::One(file) = self.get_local_selected_entries() {
                    self.mount_file_info(&file);
                }
            }
            UiMsg::ShowFileInfoPopup if self.browser.tab() == FileExplorerTab::Remote => {
                if let SelectedFile::One(file) = self.get_remote_selected_entries() {
                    self.mount_file_info(&file);
                }
            }
            UiMsg::ShowFileInfoPopup => {
                if let SelectedFile::One(file) = self.get_found_selected_entries() {
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
                if match self.browser.tab() {
                    FileExplorerTab::HostBridge => self.is_local_selected_one(),
                    FileExplorerTab::Remote => self.is_remote_selected_one(),
                    FileExplorerTab::FindHostBridge | FileExplorerTab::FindRemote => false,
                } {
                    // Only if only one entry is selected
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
        }
        None
    }
}
