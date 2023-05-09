//! ## Update
//!
//! Update impl

use tuirealm::{State, StateValue};

use super::{AuthActivity, ExitReason, FormMsg, Id, InputMask, Msg, UiMsg, Update};

impl Update<Msg> for AuthActivity {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.redraw = true;
        match msg.unwrap_or(Msg::None) {
            Msg::Form(msg) => self.update_form(msg),
            Msg::Ui(msg) => self.update_ui(msg),
            Msg::None => None,
        }
    }
}

impl AuthActivity {
    fn update_form(&mut self, msg: FormMsg) -> Option<Msg> {
        match msg {
            FormMsg::Connect => {
                match self.collect_host_params() {
                    Err(err) => {
                        // mount error
                        self.mount_error(err);
                    }
                    Ok(params) => {
                        self.save_recent();
                        // Set file transfer params to context
                        self.context_mut().set_ftparams(params);
                        // Set exit reason
                        self.exit_reason = Some(super::ExitReason::Connect);
                    }
                }
            }
            FormMsg::DeleteBookmark => {
                if let Ok(State::One(StateValue::Usize(idx))) = self.app.state(&Id::BookmarksList) {
                    // Umount dialog
                    self.umount_bookmark_del_dialog();
                    // Delete bookmark
                    self.del_bookmark(idx);
                    // Update bookmarks
                    self.view_bookmarks()
                }
            }
            FormMsg::DeleteRecent => {
                if let Ok(State::One(StateValue::Usize(idx))) = self.app.state(&Id::RecentsList) {
                    // Umount dialog
                    self.umount_recent_del_dialog();
                    // Delete recent
                    self.del_recent(idx);
                    // Update recents
                    self.view_recent_connections();
                }
            }
            FormMsg::EnterSetup => {
                self.exit_reason = Some(ExitReason::EnterSetup);
            }
            FormMsg::InstallUpdate => {
                self.install_update();
            }
            FormMsg::LoadBookmark(i) => {
                self.load_bookmark(i);
                // Give focus to input password (or to protocol if not generic)
                assert!(self
                    .app
                    .active(match self.input_mask() {
                        InputMask::Generic => &Id::Password,
                        InputMask::AwsS3 => &Id::S3Bucket,
                    })
                    .is_ok());
            }
            FormMsg::LoadRecent(i) => {
                self.load_recent(i);
                // Give focus to input password (or to protocol if not generic)
                assert!(self
                    .app
                    .active(match self.input_mask() {
                        InputMask::Generic => &Id::Password,
                        InputMask::AwsS3 => &Id::S3Bucket,
                    })
                    .is_ok());
            }
            FormMsg::ProtocolChanged(protocol) => {
                self.protocol = protocol;
                // Update port
                let port: u16 = self.get_input_port();
                if Self::is_port_standard(port) {
                    self.mount_port(Self::get_default_port_for_protocol(protocol));
                }
            }
            FormMsg::Quit => {
                self.exit_reason = Some(ExitReason::Quit);
            }
            FormMsg::SaveBookmark => {
                // get bookmark name
                let (name, save_password) = self.get_new_bookmark();
                // Save bookmark
                if !name.is_empty() {
                    self.save_bookmark(name, save_password);
                }
                // Umount popup
                self.umount_bookmark_save_dialog();
                // Reload bookmarks
                self.view_bookmarks()
            }
        }
        None
    }

    fn update_ui(&mut self, msg: UiMsg) -> Option<Msg> {
        match msg {
            UiMsg::AddressBlurDown => {
                assert!(self.app.active(&Id::Port).is_ok());
            }
            UiMsg::AddressBlurUp => {
                assert!(self.app.active(&Id::Protocol).is_ok());
            }
            UiMsg::BookmarksListBlur => {
                assert!(self.app.active(&Id::RecentsList).is_ok());
            }
            UiMsg::BookmarkNameBlur => {
                assert!(self.app.active(&Id::BookmarkSavePassword).is_ok());
            }
            UiMsg::BookmarksTabBlur => {
                assert!(self.app.active(&Id::Protocol).is_ok());
            }
            UiMsg::CloseDeleteBookmark => {
                assert!(self.app.umount(&Id::DeleteBookmarkPopup).is_ok());
            }
            UiMsg::CloseDeleteRecent => {
                assert!(self.app.umount(&Id::DeleteRecentPopup).is_ok());
            }
            UiMsg::CloseErrorPopup => {
                self.umount_error();
            }
            UiMsg::CloseInfoPopup => {
                self.umount_info();
            }
            UiMsg::CloseInstallUpdatePopup => {
                assert!(self.app.umount(&Id::NewVersionChangelog).is_ok());
                assert!(self.app.umount(&Id::InstallUpdatePopup).is_ok());
            }
            UiMsg::CloseKeybindingsPopup => {
                self.umount_help();
            }
            UiMsg::CloseQuitPopup => self.umount_quit(),
            UiMsg::CloseSaveBookmark => {
                assert!(self.app.umount(&Id::BookmarkName).is_ok());
                assert!(self.app.umount(&Id::BookmarkSavePassword).is_ok());
            }
            UiMsg::ParamsFormBlur => {
                assert!(self.app.active(&Id::BookmarksList).is_ok());
            }
            UiMsg::PasswordBlurDown => {
                assert!(self.app.active(&Id::RemoteDirectory).is_ok());
            }
            UiMsg::PasswordBlurUp => {
                assert!(self.app.active(&Id::Username).is_ok());
            }
            UiMsg::PortBlurDown => {
                assert!(self.app.active(&Id::Username).is_ok());
            }
            UiMsg::PortBlurUp => {
                assert!(self.app.active(&Id::Address).is_ok());
            }
            UiMsg::ProtocolBlurDown => {
                assert!(self
                    .app
                    .active(match self.input_mask() {
                        InputMask::Generic => &Id::Address,
                        InputMask::AwsS3 => &Id::S3Bucket,
                    })
                    .is_ok());
            }
            UiMsg::ProtocolBlurUp => {
                assert!(self.app.active(&Id::RemoteDirectory).is_ok());
            }
            UiMsg::RececentsListBlur => {
                assert!(self.app.active(&Id::BookmarksList).is_ok());
            }
            UiMsg::RemoteDirectoryBlurDown => {
                assert!(self.app.active(&Id::Protocol).is_ok());
            }
            UiMsg::RemoteDirectoryBlurUp => {
                assert!(self
                    .app
                    .active(match self.input_mask() {
                        InputMask::Generic => &Id::Password,
                        InputMask::AwsS3 => &Id::S3NewPathStyle,
                    })
                    .is_ok());
            }
            UiMsg::S3BucketBlurDown => {
                assert!(self.app.active(&Id::S3Region).is_ok());
            }
            UiMsg::S3BucketBlurUp => {
                assert!(self.app.active(&Id::Protocol).is_ok());
            }
            UiMsg::S3RegionBlurDown => {
                assert!(self.app.active(&Id::S3Endpoint).is_ok());
            }
            UiMsg::S3RegionBlurUp => {
                assert!(self.app.active(&Id::S3Bucket).is_ok());
            }
            UiMsg::S3EndpointBlurDown => {
                assert!(self.app.active(&Id::S3Profile).is_ok());
            }
            UiMsg::S3EndpointBlurUp => {
                assert!(self.app.active(&Id::S3Region).is_ok());
            }
            UiMsg::S3ProfileBlurDown => {
                assert!(self.app.active(&Id::S3AccessKey).is_ok());
            }
            UiMsg::S3ProfileBlurUp => {
                assert!(self.app.active(&Id::S3Endpoint).is_ok());
            }
            UiMsg::S3AccessKeyBlurDown => {
                assert!(self.app.active(&Id::S3SecretAccessKey).is_ok());
            }
            UiMsg::S3AccessKeyBlurUp => {
                assert!(self.app.active(&Id::S3Profile).is_ok());
            }
            UiMsg::S3SecretAccessKeyBlurDown => {
                assert!(self.app.active(&Id::S3SecurityToken).is_ok());
            }
            UiMsg::S3SecretAccessKeyBlurUp => {
                assert!(self.app.active(&Id::S3AccessKey).is_ok());
            }
            UiMsg::S3SecurityTokenBlurDown => {
                assert!(self.app.active(&Id::S3SessionToken).is_ok());
            }
            UiMsg::S3SecurityTokenBlurUp => {
                assert!(self.app.active(&Id::S3SecretAccessKey).is_ok());
            }
            UiMsg::S3SessionTokenBlurDown => {
                assert!(self.app.active(&Id::S3NewPathStyle).is_ok());
            }
            UiMsg::S3SessionTokenBlurUp => {
                assert!(self.app.active(&Id::S3SecurityToken).is_ok());
            }
            UiMsg::S3NewPathStyleBlurDown => {
                assert!(self.app.active(&Id::RemoteDirectory).is_ok());
            }
            UiMsg::S3NewPathStyleBlurUp => {
                assert!(self.app.active(&Id::S3SessionToken).is_ok());
            }
            UiMsg::SaveBookmarkPasswordBlur => {
                assert!(self.app.active(&Id::BookmarkName).is_ok());
            }
            UiMsg::ShowDeleteBookmarkPopup => {
                self.mount_bookmark_del_dialog();
            }
            UiMsg::ShowDeleteRecentPopup => {
                self.mount_recent_del_dialog();
            }
            UiMsg::ShowKeybindingsPopup => {
                self.mount_keybindings();
            }
            UiMsg::ShowQuitPopup => {
                self.mount_quit();
            }
            UiMsg::ShowReleaseNotes => {
                self.mount_release_notes();
            }
            UiMsg::ShowSaveBookmarkPopup => {
                self.mount_bookmark_save_dialog();
            }
            UiMsg::UsernameBlurDown => {
                assert!(self.app.active(&Id::Password).is_ok());
            }
            UiMsg::UsernameBlurUp => {
                assert!(self.app.active(&Id::Port).is_ok());
            }
            UiMsg::WindowResized => {
                self.redraw = true;
            }
        }

        None
    }
}
