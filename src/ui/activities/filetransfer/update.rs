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
use super::{
    actions::SelectedEntry,
    browser::{FileExplorerTab, FoundExplorerTab},
    ExitReason, FileTransferActivity, Id, Msg, TransferMsg, TransferOpts, UiMsg,
};
// externals
use remotefs::fs::Entry;
use tuirealm::{
    props::{AttrValue, Attribute},
    State, StateValue, Update,
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
            TransferMsg::CopyFileTo(dest) => {
                self.umount_copy();
                self.mount_blocking_wait("Copying file(s)…");
                match self.browser.tab() {
                    FileExplorerTab::Local => self.action_local_copy(dest),
                    FileExplorerTab::Remote => self.action_remote_copy(dest),
                    _ => panic!("Found tab doesn't support COPY"),
                }
                self.umount_wait();
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::DeleteFile => {
                self.umount_radio_delete();
                self.mount_blocking_wait("Removing file(s)…");
                match self.browser.tab() {
                    FileExplorerTab::Local => self.action_local_delete(),
                    FileExplorerTab::Remote => self.action_remote_delete(),
                    FileExplorerTab::FindLocal | FileExplorerTab::FindRemote => {
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
                    FileExplorerTab::Local => self.update_local_filelist(),
                    FileExplorerTab::Remote => self.update_remote_filelist(),
                    FileExplorerTab::FindLocal => self.update_local_filelist(),
                    FileExplorerTab::FindRemote => self.update_remote_filelist(),
                }
            }
            TransferMsg::EnterDirectory if self.browser.tab() == FileExplorerTab::Local => {
                if let SelectedEntry::One(entry) = self.get_local_selected_entries() {
                    self.action_submit_local(entry);
                    // Update file list if sync
                    if self.browser.sync_browsing && self.browser.found().is_none() {
                        let _ = self.update_remote_filelist();
                    }
                    self.update_local_filelist();
                }
            }
            TransferMsg::EnterDirectory if self.browser.tab() == FileExplorerTab::Remote => {
                if let SelectedEntry::One(entry) = self.get_remote_selected_entries() {
                    self.action_submit_remote(entry);
                    // Update file list if sync
                    if self.browser.sync_browsing && self.browser.found().is_none() {
                        let _ = self.update_local_filelist();
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
                self.mount_blocking_wait(format!("Executing '{}'…", cmd).as_str());
                match self.browser.tab() {
                    FileExplorerTab::Local => self.action_local_exec(cmd),
                    FileExplorerTab::Remote => self.action_remote_exec(cmd),
                    _ => panic!("Found tab doesn't support EXEC"),
                }
                self.umount_wait();
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::GoTo(dir) => {
                match self.browser.tab() {
                    FileExplorerTab::Local => self.action_change_local_dir(dir),
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
                    FileExplorerTab::Local => {
                        self.action_go_to_local_upper_dir();
                        if self.browser.sync_browsing && self.browser.found().is_none() {
                            let _ = self.update_remote_filelist();
                        }
                        // Reload file list component
                        self.update_local_filelist()
                    }
                    FileExplorerTab::Remote => {
                        self.action_go_to_remote_upper_dir();
                        if self.browser.sync_browsing && self.browser.found().is_none() {
                            let _ = self.update_local_filelist();
                        }
                        // Reload file list component
                        self.update_remote_filelist()
                    }
                    _ => {}
                }
            }
            TransferMsg::GoToPreviousDirectory => {
                match self.browser.tab() {
                    FileExplorerTab::Local => {
                        self.action_go_to_previous_local_dir();
                        if self.browser.sync_browsing && self.browser.found().is_none() {
                            let _ = self.update_remote_filelist();
                        }
                        // Reload file list component
                        self.update_local_filelist()
                    }
                    FileExplorerTab::Remote => {
                        self.action_go_to_previous_remote_dir();
                        if self.browser.sync_browsing && self.browser.found().is_none() {
                            let _ = self.update_local_filelist();
                        }
                        // Reload file list component
                        self.update_remote_filelist()
                    }
                    _ => {}
                }
            }
            TransferMsg::Mkdir(dir) => {
                match self.browser.tab() {
                    FileExplorerTab::Local => self.action_local_mkdir(dir),
                    FileExplorerTab::Remote => self.action_remote_mkdir(dir),
                    _ => {}
                }
                self.umount_mkdir();
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::NewFile(name) => {
                match self.browser.tab() {
                    FileExplorerTab::Local => self.action_local_newfile(name),
                    FileExplorerTab::Remote => self.action_remote_newfile(name),
                    _ => {}
                }
                self.umount_newfile();
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::OpenFile => match self.browser.tab() {
                FileExplorerTab::Local => self.action_open_local(),
                FileExplorerTab::Remote => self.action_open_remote(),
                FileExplorerTab::FindLocal | FileExplorerTab::FindRemote => self.action_find_open(),
            },
            TransferMsg::OpenFileWith(prog) => {
                match self.browser.tab() {
                    FileExplorerTab::Local => self.action_local_open_with(&prog),
                    FileExplorerTab::Remote => self.action_remote_open_with(&prog),
                    FileExplorerTab::FindLocal | FileExplorerTab::FindRemote => {
                        self.action_find_open_with(&prog)
                    }
                }
                self.umount_openwith();
            }
            TransferMsg::OpenTextFile => {
                match self.browser.tab() {
                    FileExplorerTab::Local => self.action_edit_local_file(),
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
                    FileExplorerTab::Local => self.action_local_rename(dest),
                    FileExplorerTab::Remote => self.action_remote_rename(dest),
                    _ => {}
                }
                self.umount_wait();
                // Reload files
                self.update_browser_file_list()
            }
            TransferMsg::SaveFileAs(dest) => {
                self.umount_saveas();
                match self.browser.tab() {
                    FileExplorerTab::Local => self.action_local_saveas(dest),
                    FileExplorerTab::Remote => self.action_remote_saveas(dest),
                    FileExplorerTab::FindLocal | FileExplorerTab::FindRemote => {
                        // Get entry
                        self.action_find_transfer(TransferOpts::default().save_as(Some(dest)));
                    }
                }
                self.umount_saveas();
                // Reload files
                self.update_browser_file_list_swapped();
            }
            TransferMsg::SearchFile(search) => {
                self.umount_find_input();
                // Mount wait
                self.mount_blocking_wait(format!(r#"Searching for "{}"…"#, search).as_str());
                // Find
                let res: Result<Vec<Entry>, String> = match self.browser.tab() {
                    FileExplorerTab::Local => self.action_local_find(search.clone()),
                    FileExplorerTab::Remote => self.action_remote_find(search.clone()),
                    _ => panic!("Trying to search for files, while already in a find result"),
                };
                // Umount wait
                self.umount_wait();
                // Match result
                match res {
                    Err(err) => {
                        // Mount error
                        self.mount_error(err.as_str());
                    }
                    Ok(files) if files.is_empty() => {
                        // If no file has been found notify user
                        self.mount_info(
                            format!(r#"Could not find any file matching "{}""#, search).as_str(),
                        );
                    }
                    Ok(files) => {
                        // Get wrkdir
                        let wrkdir = match self.browser.tab() {
                            FileExplorerTab::Local => self.local().wrkdir.clone(),
                            _ => self.remote().wrkdir.clone(),
                        };
                        // Create explorer and load files
                        self.browser.set_found(
                            match self.browser.tab() {
                                FileExplorerTab::Local => FoundExplorerTab::Local,
                                _ => FoundExplorerTab::Remote,
                            },
                            files,
                            wrkdir.as_path(),
                        );
                        // Mount result widget
                        self.mount_find(&search);
                        self.update_find_list();
                        // Initialize tab
                        self.browser.change_tab(match self.browser.tab() {
                            FileExplorerTab::Local => FileExplorerTab::FindLocal,
                            FileExplorerTab::Remote => FileExplorerTab::FindRemote,
                            _ => FileExplorerTab::FindLocal,
                        });
                    }
                }
            }
            TransferMsg::TransferFile => {
                match self.browser.tab() {
                    FileExplorerTab::Local => self.action_local_send(),
                    FileExplorerTab::Remote => self.action_remote_recv(),
                    FileExplorerTab::FindLocal | FileExplorerTab::FindRemote => {
                        self.action_find_transfer(TransferOpts::default())
                    }
                }
                self.update_browser_file_list_swapped();
            }
            TransferMsg::TransferPendingFile => {
                self.umount_radio_replace();
                self.action_finalize_pending_transfer();
            }
        }
        // Force redraw
        self.redraw = true;
        None
    }

    fn update_ui(&mut self, msg: UiMsg) -> Option<Msg> {
        match msg {
            UiMsg::ChangeFileSorting(sorting) => {
                match self.browser.tab() {
                    FileExplorerTab::Local | FileExplorerTab::FindLocal => {
                        self.local_mut().sort_by(sorting);
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
                    FileExplorerTab::Local if self.browser.found().is_some() => {
                        FileExplorerTab::FindRemote
                    }
                    FileExplorerTab::FindLocal | FileExplorerTab::Local => FileExplorerTab::Remote,
                    FileExplorerTab::Remote if self.browser.found().is_some() => {
                        FileExplorerTab::FindLocal
                    }
                    FileExplorerTab::FindRemote | FileExplorerTab::Remote => FileExplorerTab::Local,
                };
                // Set focus
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
            UiMsg::CloseFindExplorer => {
                self.finalize_find();
                self.umount_find();
            }
            UiMsg::CloseFindPopup => self.umount_find_input(),
            UiMsg::CloseGotoPopup => self.umount_goto(),
            UiMsg::CloseKeybindingsPopup => self.umount_help(),
            UiMsg::CloseMkdirPopup => self.umount_mkdir(),
            UiMsg::CloseNewFilePopup => self.umount_newfile(),
            UiMsg::CloseOpenWithPopup => self.umount_openwith(),
            UiMsg::CloseQuitPopup => self.umount_quit(),
            UiMsg::CloseRenamePopup => self.umount_rename(),
            UiMsg::CloseReplacePopups => {
                self.umount_radio_replace();
            }
            UiMsg::CloseSaveAsPopup => self.umount_saveas(),
            UiMsg::Disconnect => {
                self.disconnect();
                self.umount_disconnect();
            }
            UiMsg::ExplorerBackTabbed => {
                assert!(self.app.active(&Id::Log).is_ok());
            }
            UiMsg::LogBackTabbed => {
                assert!(self.app.active(&Id::ExplorerLocal).is_ok());
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
            UiMsg::ShowCopyPopup => self.mount_copy(),
            UiMsg::ShowDeletePopup => self.mount_radio_delete(),
            UiMsg::ShowDisconnectPopup => self.mount_disconnect(),
            UiMsg::ShowExecPopup => self.mount_exec(),
            UiMsg::ShowFileInfoPopup if self.browser.tab() == FileExplorerTab::Local => {
                if let SelectedEntry::One(file) = self.get_local_selected_entries() {
                    self.mount_file_info(&file);
                }
            }
            UiMsg::ShowFileInfoPopup if self.browser.tab() == FileExplorerTab::Remote => {
                if let SelectedEntry::One(file) = self.get_remote_selected_entries() {
                    self.mount_file_info(&file);
                }
            }
            UiMsg::ShowFileInfoPopup => {
                if let SelectedEntry::One(file) = self.get_found_selected_entries() {
                    self.mount_file_info(&file);
                }
            }
            UiMsg::ShowFileSortingPopup => self.mount_file_sorting(),
            UiMsg::ShowFindPopup => self.mount_find_input(),
            UiMsg::ShowGotoPopup => self.mount_goto(),
            UiMsg::ShowKeybindingsPopup => self.mount_help(),
            UiMsg::ShowMkdirPopup => self.mount_mkdir(),
            UiMsg::ShowNewFilePopup => self.mount_newfile(),
            UiMsg::ShowOpenWithPopup => self.mount_openwith(),
            UiMsg::ShowQuitPopup => self.mount_quit(),
            UiMsg::ShowRenamePopup => self.mount_rename(),
            UiMsg::ShowSaveAsPopup => self.mount_saveas(),
            UiMsg::ToggleHiddenFiles => match self.browser.tab() {
                FileExplorerTab::FindLocal | FileExplorerTab::Local => {
                    self.browser.local_mut().toggle_hidden_files();
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
        }
        None
    }
}
