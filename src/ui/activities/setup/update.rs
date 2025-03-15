//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

// locals
// ext
use tuirealm::Update;

use super::{
    CommonMsg, ConfigMsg, Id, IdConfig, IdSsh, IdTheme, Msg, SetupActivity, SshMsg, ThemeMsg,
    ViewLayout,
};

impl Update<Msg> for SetupActivity {
    /// Update auth activity model based on msg
    /// The function exits when returns None
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        match msg.unwrap_or(Msg::None) {
            Msg::Common(msg) => self.common_update(msg),
            Msg::Config(msg) => self.config_update(msg),
            Msg::Ssh(msg) => self.ssh_update(msg),
            Msg::Theme(msg) => self.theme_update(msg),
            Msg::None => None,
        }
    }
}

impl SetupActivity {
    fn common_update(&mut self, msg: CommonMsg) -> Option<Msg> {
        match msg {
            CommonMsg::ChangeLayout => {
                let new_layout = match self.layout {
                    ViewLayout::SetupForm => ViewLayout::SshKeys,
                    ViewLayout::SshKeys => ViewLayout::Theme,
                    ViewLayout::Theme => ViewLayout::SetupForm,
                };
                if let Err(err) = self.action_change_tab(new_layout) {
                    self.mount_error(err.as_str());
                }
            }
            CommonMsg::CloseErrorPopup => {
                self.umount_error();
            }
            CommonMsg::CloseKeybindingsPopup => {
                self.umount_help();
            }
            CommonMsg::CloseQuitPopup => {
                self.umount_quit();
            }
            CommonMsg::CloseSavePopup => {
                self.umount_save_popup();
            }
            CommonMsg::Quit => {
                self.exit_reason = Some(super::ExitReason::Quit);
            }
            CommonMsg::RevertChanges => match self.layout {
                ViewLayout::Theme => {
                    if let Err(err) = self.action_reset_theme() {
                        error!("Failed to reset theme: {}", err);
                        self.mount_error(err);
                    }
                }
                ViewLayout::SshKeys | ViewLayout::SetupForm => {
                    if let Err(err) = self.action_reset_config() {
                        error!("Failed to reset config: {}", err);
                        self.mount_error(err);
                    }
                }
            },
            CommonMsg::SaveAndQuit => {
                // Save changes
                if let Err(err) = self.action_save_all() {
                    error!("Failed to save config: {}", err);
                    self.mount_error(err.as_str());
                }
                // Exit
                self.exit_reason = Some(super::ExitReason::Quit);
            }
            CommonMsg::SaveConfig => {
                if let Err(err) = self.action_save_all() {
                    error!("Failed to save config: {}", err);
                    self.mount_error(err.as_str());
                }
                self.umount_save_popup();
            }
            CommonMsg::ShowKeybindings => {
                self.mount_help();
            }
            CommonMsg::ShowQuitPopup => {
                self.action_on_esc();
            }
            CommonMsg::ShowSavePopup => {
                self.mount_save_popup();
            }
            CommonMsg::WindowResized => {
                self.redraw = true;
            }
        }
        None
    }

    fn config_update(&mut self, msg: ConfigMsg) -> Option<Msg> {
        match msg {
            ConfigMsg::CheckUpdatesBlurDown => {
                assert!(
                    self.app
                        .active(&Id::Config(IdConfig::PromptOnFileReplace))
                        .is_ok()
                );
            }
            ConfigMsg::CheckUpdatesBlurUp => {
                assert!(self.app.active(&Id::Config(IdConfig::HiddenFiles)).is_ok());
            }
            ConfigMsg::DefaultProtocolBlurDown => {
                assert!(self.app.active(&Id::Config(IdConfig::HiddenFiles)).is_ok());
            }
            ConfigMsg::DefaultProtocolBlurUp => {
                assert!(self.app.active(&Id::Config(IdConfig::TextEditor)).is_ok());
            }
            ConfigMsg::GroupDirsBlurDown => {
                assert!(self.app.active(&Id::Config(IdConfig::LocalFileFmt)).is_ok());
            }
            ConfigMsg::GroupDirsBlurUp => {
                assert!(
                    self.app
                        .active(&Id::Config(IdConfig::PromptOnFileReplace))
                        .is_ok()
                );
            }
            ConfigMsg::HiddenFilesBlurDown => {
                assert!(self.app.active(&Id::Config(IdConfig::CheckUpdates)).is_ok());
            }
            ConfigMsg::HiddenFilesBlurUp => {
                assert!(
                    self.app
                        .active(&Id::Config(IdConfig::DefaultProtocol))
                        .is_ok()
                );
            }
            ConfigMsg::LocalFileFmtBlurDown => {
                assert!(
                    self.app
                        .active(&Id::Config(IdConfig::RemoteFileFmt))
                        .is_ok()
                );
            }
            ConfigMsg::LocalFileFmtBlurUp => {
                assert!(self.app.active(&Id::Config(IdConfig::GroupDirs)).is_ok());
            }
            ConfigMsg::NotificationsEnabledBlurDown => {
                assert!(
                    self.app
                        .active(&Id::Config(IdConfig::NotificationsThreshold))
                        .is_ok()
                );
            }
            ConfigMsg::NotificationsEnabledBlurUp => {
                assert!(
                    self.app
                        .active(&Id::Config(IdConfig::RemoteFileFmt))
                        .is_ok()
                );
            }
            ConfigMsg::NotificationsThresholdBlurDown => {
                assert!(self.app.active(&Id::Config(IdConfig::SshConfig)).is_ok());
            }
            ConfigMsg::NotificationsThresholdBlurUp => {
                assert!(
                    self.app
                        .active(&Id::Config(IdConfig::NotificationsEnabled))
                        .is_ok()
                );
            }
            ConfigMsg::PromptOnFileReplaceBlurDown => {
                assert!(self.app.active(&Id::Config(IdConfig::GroupDirs)).is_ok());
            }
            ConfigMsg::PromptOnFileReplaceBlurUp => {
                assert!(self.app.active(&Id::Config(IdConfig::CheckUpdates)).is_ok());
            }
            ConfigMsg::RemoteFileFmtBlurDown => {
                assert!(
                    self.app
                        .active(&Id::Config(IdConfig::NotificationsEnabled))
                        .is_ok()
                );
            }
            ConfigMsg::RemoteFileFmtBlurUp => {
                assert!(self.app.active(&Id::Config(IdConfig::LocalFileFmt)).is_ok());
            }
            ConfigMsg::TextEditorBlurDown => {
                assert!(
                    self.app
                        .active(&Id::Config(IdConfig::DefaultProtocol))
                        .is_ok()
                );
            }
            ConfigMsg::TextEditorBlurUp => {
                assert!(self.app.active(&Id::Config(IdConfig::SshConfig)).is_ok());
            }
            ConfigMsg::SshConfigBlurDown => {
                assert!(self.app.active(&Id::Config(IdConfig::TextEditor)).is_ok());
            }
            ConfigMsg::SshConfigBlurUp => {
                assert!(
                    self.app
                        .active(&Id::Config(IdConfig::NotificationsThreshold))
                        .is_ok()
                );
            }
            ConfigMsg::ConfigChanged => {
                self.set_config_changed(true);
            }
        }
        None
    }

    fn ssh_update(&mut self, msg: SshMsg) -> Option<Msg> {
        match msg {
            SshMsg::CloseDelSshKeyPopup => {
                self.umount_del_ssh_key();
            }
            SshMsg::CloseNewSshKeyPopup => {
                self.umount_new_ssh_key();
            }
            SshMsg::DeleteSshKey => {
                self.action_delete_ssh_key();
                self.umount_del_ssh_key();
                self.reload_ssh_keys();
            }
            SshMsg::EditSshKey(i) => {
                if let Err(err) = self.edit_ssh_key(i) {
                    error!("Failed to edit ssh key: {}", err);
                    self.mount_error(err.as_str());
                }
            }
            SshMsg::SaveSshKey => {
                let res = self.action_new_ssh_key();
                self.umount_new_ssh_key();
                match res {
                    Ok(_) => {
                        self.reload_ssh_keys();
                    }
                    Err(err) => self.mount_error(&err),
                }
            }
            SshMsg::ShowDelSshKeyPopup => {
                self.mount_del_ssh_key();
            }
            SshMsg::ShowNewSshKeyPopup => {
                self.mount_new_ssh_key();
            }
            SshMsg::SshHostBlur => {
                assert!(self.app.active(&Id::Ssh(IdSsh::SshUsername)).is_ok());
            }
            SshMsg::SshUsernameBlur => {
                assert!(self.app.active(&Id::Ssh(IdSsh::SshHost)).is_ok());
            }
        }
        None
    }

    fn theme_update(&mut self, msg: ThemeMsg) -> Option<Msg> {
        match msg {
            ThemeMsg::AuthAddressBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::AuthPort)).is_ok());
            }
            ThemeMsg::AuthAddressBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::AuthProtocol)).is_ok());
            }
            ThemeMsg::AuthBookmarksBlurDown => {
                assert!(
                    self.app
                        .active(&Id::Theme(IdTheme::AuthRecentHosts))
                        .is_ok()
                );
            }
            ThemeMsg::AuthBookmarksBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::AuthPassword)).is_ok());
            }
            ThemeMsg::AuthPasswordBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::AuthBookmarks)).is_ok());
            }
            ThemeMsg::AuthPasswordBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::AuthUsername)).is_ok());
            }
            ThemeMsg::AuthPortBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::AuthUsername)).is_ok());
            }
            ThemeMsg::AuthPortBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::AuthAddress)).is_ok());
            }
            ThemeMsg::AuthProtocolBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::AuthAddress)).is_ok());
            }
            ThemeMsg::AuthProtocolBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::StatusSync)).is_ok());
            }
            ThemeMsg::AuthRecentHostsBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::MiscError)).is_ok());
            }
            ThemeMsg::AuthRecentHostsBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::AuthBookmarks)).is_ok());
            }
            ThemeMsg::AuthUsernameBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::AuthPassword)).is_ok());
            }
            ThemeMsg::AuthUsernameBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::AuthPort)).is_ok());
            }
            ThemeMsg::MiscErrorBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::MiscInfo)).is_ok());
            }
            ThemeMsg::MiscErrorBlurUp => {
                assert!(
                    self.app
                        .active(&Id::Theme(IdTheme::AuthRecentHosts))
                        .is_ok()
                );
            }
            ThemeMsg::MiscInfoBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::MiscInput)).is_ok());
            }
            ThemeMsg::MiscInfoBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::MiscError)).is_ok());
            }
            ThemeMsg::MiscInputBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::MiscKeys)).is_ok());
            }
            ThemeMsg::MiscInputBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::MiscInfo)).is_ok());
            }
            ThemeMsg::MiscKeysBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::MiscQuit)).is_ok());
            }
            ThemeMsg::MiscKeysBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::MiscInput)).is_ok());
            }
            ThemeMsg::MiscQuitBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::MiscSave)).is_ok());
            }
            ThemeMsg::MiscQuitBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::MiscKeys)).is_ok());
            }
            ThemeMsg::MiscSaveBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::MiscWarn)).is_ok());
            }
            ThemeMsg::MiscSaveBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::MiscQuit)).is_ok());
            }
            ThemeMsg::MiscWarnBlurDown => {
                assert!(
                    self.app
                        .active(&Id::Theme(IdTheme::ExplorerLocalBg))
                        .is_ok()
                );
            }
            ThemeMsg::MiscWarnBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::MiscSave)).is_ok());
            }
            ThemeMsg::ExplorerLocalBgBlurDown => {
                assert!(
                    self.app
                        .active(&Id::Theme(IdTheme::ExplorerLocalFg))
                        .is_ok()
                );
            }
            ThemeMsg::ExplorerLocalBgBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::MiscWarn)).is_ok());
            }
            ThemeMsg::ExplorerLocalFgBlurDown => {
                assert!(
                    self.app
                        .active(&Id::Theme(IdTheme::ExplorerLocalHg))
                        .is_ok()
                );
            }
            ThemeMsg::ExplorerLocalFgBlurUp => {
                assert!(
                    self.app
                        .active(&Id::Theme(IdTheme::ExplorerLocalBg))
                        .is_ok()
                );
            }
            ThemeMsg::ExplorerLocalHgBlurDown => {
                assert!(
                    self.app
                        .active(&Id::Theme(IdTheme::ExplorerRemoteBg))
                        .is_ok()
                );
            }
            ThemeMsg::ExplorerLocalHgBlurUp => {
                assert!(
                    self.app
                        .active(&Id::Theme(IdTheme::ExplorerLocalFg))
                        .is_ok()
                );
            }
            ThemeMsg::ExplorerRemoteBgBlurDown => {
                assert!(
                    self.app
                        .active(&Id::Theme(IdTheme::ExplorerRemoteFg))
                        .is_ok()
                );
            }
            ThemeMsg::ExplorerRemoteBgBlurUp => {
                assert!(
                    self.app
                        .active(&Id::Theme(IdTheme::ExplorerLocalHg))
                        .is_ok()
                );
            }
            ThemeMsg::ExplorerRemoteFgBlurDown => {
                assert!(
                    self.app
                        .active(&Id::Theme(IdTheme::ExplorerRemoteHg))
                        .is_ok()
                );
            }
            ThemeMsg::ExplorerRemoteFgBlurUp => {
                assert!(
                    self.app
                        .active(&Id::Theme(IdTheme::ExplorerRemoteBg))
                        .is_ok()
                );
            }
            ThemeMsg::ExplorerRemoteHgBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::ProgBarFull)).is_ok());
            }
            ThemeMsg::ExplorerRemoteHgBlurUp => {
                assert!(
                    self.app
                        .active(&Id::Theme(IdTheme::ExplorerRemoteFg))
                        .is_ok()
                );
            }
            ThemeMsg::ProgBarFullBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::ProgBarPartial)).is_ok());
            }
            ThemeMsg::ProgBarFullBlurUp => {
                assert!(
                    self.app
                        .active(&Id::Theme(IdTheme::ExplorerRemoteHg))
                        .is_ok()
                );
            }
            ThemeMsg::ProgBarPartialBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::LogBg)).is_ok());
            }
            ThemeMsg::ProgBarPartialBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::ProgBarFull)).is_ok());
            }
            ThemeMsg::LogBgBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::LogWindow)).is_ok());
            }
            ThemeMsg::LogBgBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::ProgBarPartial)).is_ok());
            }
            ThemeMsg::LogWindowBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::StatusSorting)).is_ok());
            }
            ThemeMsg::LogWindowBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::LogBg)).is_ok());
            }
            ThemeMsg::StatusSortingBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::StatusHidden)).is_ok());
            }
            ThemeMsg::StatusSortingBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::LogWindow)).is_ok());
            }
            ThemeMsg::StatusHiddenBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::StatusSync)).is_ok());
            }
            ThemeMsg::StatusHiddenBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::StatusSorting)).is_ok());
            }
            ThemeMsg::StatusSyncBlurDown => {
                assert!(self.app.active(&Id::Theme(IdTheme::AuthProtocol)).is_ok());
            }
            ThemeMsg::StatusSyncBlurUp => {
                assert!(self.app.active(&Id::Theme(IdTheme::StatusHidden)).is_ok());
            }
            ThemeMsg::ColorChanged(id, color) => {
                self.action_save_color(id, color);
                // Set unsaved changes to true
                self.set_config_changed(true);
            }
        }
        None
    }
}
