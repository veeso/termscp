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
                if let Err(err) = self.app.active(&Id::Config(IdConfig::PromptOnFileReplace)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::CheckUpdatesBlurUp => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::HiddenFiles)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::DefaultProtocolBlurDown => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::HiddenFiles)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::DefaultProtocolBlurUp => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::TextEditor)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::GroupDirsBlurDown => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::LocalFileFmt)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::GroupDirsBlurUp => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::PromptOnFileReplace)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::HiddenFilesBlurDown => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::CheckUpdates)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::HiddenFilesBlurUp => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::DefaultProtocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::LocalFileFmtBlurDown => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::RemoteFileFmt)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::LocalFileFmtBlurUp => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::GroupDirs)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::NotificationsEnabledBlurDown => {
                if let Err(err) = self
                    .app
                    .active(&Id::Config(IdConfig::NotificationsThreshold))
                {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::NotificationsEnabledBlurUp => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::RemoteFileFmt)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::NotificationsThresholdBlurDown => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::SshConfig)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::NotificationsThresholdBlurUp => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::NotificationsEnabled)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::PromptOnFileReplaceBlurDown => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::GroupDirs)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::PromptOnFileReplaceBlurUp => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::CheckUpdates)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::RemoteFileFmtBlurDown => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::NotificationsEnabled)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::RemoteFileFmtBlurUp => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::LocalFileFmt)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::TextEditorBlurDown => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::DefaultProtocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::TextEditorBlurUp => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::SshConfig)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::SshConfigBlurDown => {
                if let Err(err) = self.app.active(&Id::Config(IdConfig::TextEditor)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ConfigMsg::SshConfigBlurUp => {
                if let Err(err) = self
                    .app
                    .active(&Id::Config(IdConfig::NotificationsThreshold))
                {
                    error!("Failed to activate component: {err}");
                }
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
                if let Err(err) = self.app.active(&Id::Ssh(IdSsh::SshUsername)) {
                    error!("Failed to activate component: {err}");
                }
            }
            SshMsg::SshUsernameBlur => {
                if let Err(err) = self.app.active(&Id::Ssh(IdSsh::SshHost)) {
                    error!("Failed to activate component: {err}");
                }
            }
        }
        None
    }

    fn theme_update(&mut self, msg: ThemeMsg) -> Option<Msg> {
        match msg {
            ThemeMsg::AuthAddressBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::AuthPort)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::AuthAddressBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::AuthProtocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::AuthBookmarksBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::AuthRecentHosts)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::AuthBookmarksBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::AuthPassword)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::AuthPasswordBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::AuthBookmarks)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::AuthPasswordBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::AuthUsername)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::AuthPortBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::AuthUsername)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::AuthPortBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::AuthAddress)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::AuthProtocolBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::AuthAddress)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::AuthProtocolBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::StatusSync)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::AuthRecentHostsBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::MiscError)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::AuthRecentHostsBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::AuthBookmarks)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::AuthUsernameBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::AuthPassword)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::AuthUsernameBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::AuthPort)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::MiscErrorBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::MiscInfo)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::MiscErrorBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::AuthRecentHosts)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::MiscInfoBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::MiscInput)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::MiscInfoBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::MiscError)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::MiscInputBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::MiscKeys)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::MiscInputBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::MiscInfo)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::MiscKeysBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::MiscQuit)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::MiscKeysBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::MiscInput)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::MiscQuitBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::MiscSave)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::MiscQuitBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::MiscKeys)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::MiscSaveBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::MiscWarn)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::MiscSaveBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::MiscQuit)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::MiscWarnBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ExplorerLocalBg)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::MiscWarnBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::MiscSave)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ExplorerLocalBgBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ExplorerLocalFg)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ExplorerLocalBgBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::MiscWarn)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ExplorerLocalFgBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ExplorerLocalHg)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ExplorerLocalFgBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ExplorerLocalBg)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ExplorerLocalHgBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ExplorerRemoteBg)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ExplorerLocalHgBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ExplorerLocalFg)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ExplorerRemoteBgBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ExplorerRemoteFg)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ExplorerRemoteBgBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ExplorerLocalHg)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ExplorerRemoteFgBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ExplorerRemoteHg)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ExplorerRemoteFgBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ExplorerRemoteBg)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ExplorerRemoteHgBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ProgBarFull)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ExplorerRemoteHgBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ExplorerRemoteFg)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ProgBarFullBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ProgBarPartial)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ProgBarFullBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ExplorerRemoteHg)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ProgBarPartialBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::LogBg)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::ProgBarPartialBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ProgBarFull)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::LogBgBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::LogWindow)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::LogBgBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::ProgBarPartial)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::LogWindowBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::StatusSorting)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::LogWindowBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::LogBg)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::StatusSortingBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::StatusHidden)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::StatusSortingBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::LogWindow)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::StatusHiddenBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::StatusSync)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::StatusHiddenBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::StatusSorting)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::StatusSyncBlurDown => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::AuthProtocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            ThemeMsg::StatusSyncBlurUp => {
                if let Err(err) = self.app.active(&Id::Theme(IdTheme::StatusHidden)) {
                    error!("Failed to activate component: {err}");
                }
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
