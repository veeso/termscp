//! ## Update
//!
//! Update impl

use tuirealm::{State, StateValue};

use super::{
    AuthActivity, AuthFormId, ExitReason, FormMsg, FormTab, HostBridgeProtocol, Id, InputMask, Msg,
    UiAuthFormMsg, UiMsg, Update,
};

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
                let Ok(remote_params) = self.collect_remote_host_params() else {
                    // mount error
                    self.mount_error("Invalid remote params parameters");
                    return None;
                };

                let Ok(host_bridge_params) = self.collect_host_bridge_params() else {
                    // mount error
                    self.mount_error("Invalid host bridge params parameters");
                    return None;
                };

                self.save_recent();
                // Set file transfer params to context
                self.context_mut().set_remote_params(remote_params);
                // set host bridge params
                self.context_mut()
                    .set_host_bridge_params(host_bridge_params);
                // Set exit reason
                self.exit_reason = Some(super::ExitReason::Connect);
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
                self.load_bookmark(self.last_form_tab, i);
                // Give focus to input password (or to protocol if not generic)
                let focus = match self.last_form_tab {
                    FormTab::Remote => match self.remote_input_mask() {
                        InputMask::Localhost => &Id::Remote(AuthFormId::LocalDirectory),
                        InputMask::Generic => &Id::Remote(AuthFormId::Password),
                        InputMask::Smb => &Id::Remote(AuthFormId::Password),
                        InputMask::AwsS3 => &Id::Remote(AuthFormId::S3Bucket),
                        InputMask::Kube => &Id::Remote(AuthFormId::KubeNamespace),
                        InputMask::WebDAV => &Id::Remote(AuthFormId::Password),
                    },
                    FormTab::HostBridge => match self.host_bridge_input_mask() {
                        InputMask::Localhost => &Id::HostBridge(AuthFormId::LocalDirectory),
                        InputMask::Generic => &Id::HostBridge(AuthFormId::Password),
                        InputMask::Smb => &Id::HostBridge(AuthFormId::Password),
                        InputMask::AwsS3 => &Id::HostBridge(AuthFormId::S3Bucket),
                        InputMask::Kube => &Id::HostBridge(AuthFormId::KubeNamespace),
                        InputMask::WebDAV => &Id::HostBridge(AuthFormId::Password),
                    },
                };

                assert!(self.app.active(focus).is_ok());
            }
            FormMsg::LoadRecent(i) => {
                self.load_recent(self.last_form_tab, i);
                // Give focus to input password (or to protocol if not generic)
                let focus = match self.last_form_tab {
                    FormTab::Remote => match self.remote_input_mask() {
                        InputMask::Localhost => &Id::Remote(AuthFormId::LocalDirectory),
                        InputMask::Generic => &Id::Remote(AuthFormId::Password),
                        InputMask::Smb => &Id::Remote(AuthFormId::Password),
                        InputMask::AwsS3 => &Id::Remote(AuthFormId::S3Bucket),
                        InputMask::Kube => &Id::Remote(AuthFormId::KubeNamespace),
                        InputMask::WebDAV => &Id::Remote(AuthFormId::Password),
                    },
                    FormTab::HostBridge => match self.host_bridge_input_mask() {
                        InputMask::Localhost => &Id::HostBridge(AuthFormId::LocalDirectory),
                        InputMask::Generic => &Id::HostBridge(AuthFormId::Password),
                        InputMask::Smb => &Id::HostBridge(AuthFormId::Password),
                        InputMask::AwsS3 => &Id::HostBridge(AuthFormId::S3Bucket),
                        InputMask::Kube => &Id::HostBridge(AuthFormId::KubeNamespace),
                        InputMask::WebDAV => &Id::HostBridge(AuthFormId::Password),
                    },
                };

                assert!(self.app.active(focus).is_ok());
            }
            FormMsg::HostBridgeProtocolChanged(protocol) => {
                self.host_bridge_protocol = protocol;
                // Update port
                let port: u16 = self.get_input_port(FormTab::HostBridge);
                if let HostBridgeProtocol::Remote(remote_protocol) = protocol {
                    if Self::is_port_standard(port) {
                        self.mount_port(
                            FormTab::HostBridge,
                            Self::get_default_port_for_protocol(remote_protocol),
                        );
                    }
                }
            }
            FormMsg::RemoteProtocolChanged(protocol) => {
                self.remote_protocol = protocol;
                // Update port
                let port: u16 = self.get_input_port(FormTab::Remote);
                if Self::is_port_standard(port) {
                    self.mount_port(
                        FormTab::Remote,
                        Self::get_default_port_for_protocol(protocol),
                    );
                }
            }
            FormMsg::Quit => {
                self.exit_reason = Some(ExitReason::Quit);
            }
            FormMsg::SaveBookmark(form_tab) => {
                // get bookmark name
                let (name, save_password) = self.get_new_bookmark();
                // Save bookmark
                if !name.is_empty() {
                    self.save_bookmark(form_tab, name, save_password);
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
            UiMsg::HostBridge(UiAuthFormMsg::AddressBlurDown) => {
                let id = if cfg!(windows) && self.host_bridge_input_mask() == InputMask::Smb {
                    &Id::HostBridge(AuthFormId::SmbShare)
                } else {
                    &Id::HostBridge(AuthFormId::Port)
                };
                assert!(self.app.active(id).is_ok());
            }
            UiMsg::Remote(UiAuthFormMsg::AddressBlurDown) => {
                let id = if cfg!(windows) && self.remote_input_mask() == InputMask::Smb {
                    &Id::Remote(AuthFormId::SmbShare)
                } else {
                    &Id::Remote(AuthFormId::Port)
                };
                assert!(self.app.active(id).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::AddressBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::Protocol))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::AddressBlurUp) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::Protocol)).is_ok());
            }
            UiMsg::BookmarksListBlur => {
                assert!(self.app.active(&Id::RecentsList).is_ok());
            }
            UiMsg::BookmarkNameBlur => {
                assert!(self.app.active(&Id::BookmarkSavePassword).is_ok());
            }
            UiMsg::BookmarksTabBlur => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::Protocol))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::ChangeFormTab) => {
                self.last_form_tab = FormTab::Remote;
                assert!(self.app.active(&Id::Remote(AuthFormId::Protocol)).is_ok());
            }
            UiMsg::Remote(UiAuthFormMsg::ChangeFormTab) => {
                self.last_form_tab = FormTab::HostBridge;
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::Protocol))
                        .is_ok()
                );
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
            UiMsg::HostBridge(UiAuthFormMsg::LocalDirectoryBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::Protocol))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::LocalDirectoryBlurDown) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::Protocol)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::LocalDirectoryBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::RemoteDirectory))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::LocalDirectoryBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::RemoteDirectory))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::ParamsFormBlur) => {
                assert!(self.app.active(&Id::BookmarksList).is_ok());
            }
            UiMsg::Remote(UiAuthFormMsg::ParamsFormBlur) => {
                assert!(self.app.active(&Id::BookmarksList).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::PasswordBlurDown) => {
                assert!(
                    self.app
                        .active(match self.host_bridge_input_mask() {
                            InputMask::Localhost => unreachable!(),
                            InputMask::Generic => &Id::HostBridge(AuthFormId::RemoteDirectory),
                            #[cfg(posix)]
                            InputMask::Smb => &Id::HostBridge(AuthFormId::SmbWorkgroup),
                            #[cfg(win)]
                            InputMask::Smb => &Id::HostBridge(AuthFormId::RemoteDirectory),
                            InputMask::AwsS3 =>
                                unreachable!("this shouldn't happen (password on s3)"),
                            InputMask::Kube =>
                                unreachable!("this shouldn't happen (password on kube)"),
                            InputMask::WebDAV => &Id::HostBridge(AuthFormId::RemoteDirectory),
                        })
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::PasswordBlurDown) => {
                assert!(
                    self.app
                        .active(match self.remote_input_mask() {
                            InputMask::Localhost => unreachable!(),
                            InputMask::Generic => &Id::Remote(AuthFormId::RemoteDirectory),
                            #[cfg(posix)]
                            InputMask::Smb => &Id::Remote(AuthFormId::SmbWorkgroup),
                            #[cfg(win)]
                            InputMask::Smb => &Id::Remote(AuthFormId::RemoteDirectory),
                            InputMask::AwsS3 =>
                                unreachable!("this shouldn't happen (password on s3)"),
                            InputMask::Kube =>
                                unreachable!("this shouldn't happen (password on kube)"),
                            InputMask::WebDAV => &Id::Remote(AuthFormId::RemoteDirectory),
                        })
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::PasswordBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::Username))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::PasswordBlurUp) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::Username)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::PortBlurDown) => {
                assert!(
                    self.app
                        .active(match self.host_bridge_input_mask() {
                            InputMask::Generic => &Id::HostBridge(AuthFormId::Username),
                            InputMask::Smb => &Id::HostBridge(AuthFormId::SmbShare),
                            InputMask::Localhost
                            | InputMask::AwsS3
                            | InputMask::Kube
                            | InputMask::WebDAV =>
                                unreachable!("this shouldn't happen (port on s3/kube/webdav)"),
                        })
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::PortBlurDown) => {
                assert!(
                    self.app
                        .active(match self.remote_input_mask() {
                            InputMask::Generic => &Id::Remote(AuthFormId::Username),
                            InputMask::Smb => &Id::Remote(AuthFormId::SmbShare),
                            InputMask::Localhost
                            | InputMask::AwsS3
                            | InputMask::Kube
                            | InputMask::WebDAV =>
                                unreachable!("this shouldn't happen (port on s3/kube/webdav)"),
                        })
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::PortBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::Address))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::PortBlurUp) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::Address)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::ProtocolBlurDown) => {
                assert!(
                    self.app
                        .active(match self.host_bridge_input_mask() {
                            InputMask::Localhost => &Id::HostBridge(AuthFormId::LocalDirectory),
                            InputMask::Generic => &Id::HostBridge(AuthFormId::Address),
                            InputMask::Smb => &Id::HostBridge(AuthFormId::Address),
                            InputMask::AwsS3 => &Id::HostBridge(AuthFormId::S3Bucket),
                            InputMask::Kube => &Id::HostBridge(AuthFormId::KubeNamespace),
                            InputMask::WebDAV => &Id::HostBridge(AuthFormId::WebDAVUri),
                        })
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::ProtocolBlurDown) => {
                assert!(
                    self.app
                        .active(match self.remote_input_mask() {
                            InputMask::Localhost => &Id::Remote(AuthFormId::LocalDirectory),
                            InputMask::Generic => &Id::Remote(AuthFormId::Address),
                            InputMask::Smb => &Id::Remote(AuthFormId::Address),
                            InputMask::AwsS3 => &Id::Remote(AuthFormId::S3Bucket),
                            InputMask::Kube => &Id::Remote(AuthFormId::KubeNamespace),
                            InputMask::WebDAV => &Id::Remote(AuthFormId::WebDAVUri),
                        })
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::ProtocolBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::LocalDirectory))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::ProtocolBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::LocalDirectory))
                        .is_ok()
                );
            }
            UiMsg::RececentsListBlur => {
                assert!(self.app.active(&Id::BookmarksList).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::RemoteDirectoryBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::LocalDirectory))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::RemoteDirectoryBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::LocalDirectory))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::RemoteDirectoryBlurUp) => {
                assert!(
                    self.app
                        .active(match self.host_bridge_input_mask() {
                            InputMask::Localhost => unreachable!(),
                            InputMask::Generic => &Id::HostBridge(AuthFormId::Password),
                            #[cfg(posix)]
                            InputMask::Smb => &Id::HostBridge(AuthFormId::SmbWorkgroup),
                            #[cfg(win)]
                            InputMask::Smb => &Id::HostBridge(AuthFormId::Password),
                            InputMask::Kube => &Id::HostBridge(AuthFormId::KubeClientKey),
                            InputMask::AwsS3 => &Id::HostBridge(AuthFormId::S3NewPathStyle),
                            InputMask::WebDAV => &Id::HostBridge(AuthFormId::Password),
                        })
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::RemoteDirectoryBlurUp) => {
                assert!(
                    self.app
                        .active(match self.remote_input_mask() {
                            InputMask::Localhost => unreachable!(),
                            InputMask::Generic => &Id::Remote(AuthFormId::Password),
                            #[cfg(posix)]
                            InputMask::Smb => &Id::Remote(AuthFormId::SmbWorkgroup),
                            #[cfg(win)]
                            InputMask::Smb => &Id::Remote(AuthFormId::Password),
                            InputMask::Kube => &Id::Remote(AuthFormId::KubeClientKey),
                            InputMask::AwsS3 => &Id::Remote(AuthFormId::S3NewPathStyle),
                            InputMask::WebDAV => &Id::Remote(AuthFormId::Password),
                        })
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3BucketBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3Region))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3BucketBlurDown) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::S3Region)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3BucketBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::Protocol))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3BucketBlurUp) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::Protocol)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3RegionBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3Endpoint))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3RegionBlurDown) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::S3Endpoint)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3RegionBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3Bucket))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3RegionBlurUp) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::S3Bucket)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3EndpointBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3Profile))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3EndpointBlurDown) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::S3Profile)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3EndpointBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3Region))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3EndpointBlurUp) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::S3Region)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3ProfileBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3AccessKey))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3ProfileBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::S3AccessKey))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3ProfileBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3Endpoint))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3ProfileBlurUp) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::S3Endpoint)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3AccessKeyBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3SecretAccessKey))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3AccessKeyBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::S3SecretAccessKey))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3AccessKeyBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3Profile))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3AccessKeyBlurUp) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::S3Profile)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3SecretAccessKeyBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3SecurityToken))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3SecretAccessKeyBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::S3SecurityToken))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3SecretAccessKeyBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3AccessKey))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3SecretAccessKeyBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::S3AccessKey))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3SecurityTokenBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3SessionToken))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3SecurityTokenBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::S3SessionToken))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3SecurityTokenBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3SecretAccessKey))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3SecurityTokenBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::S3SecretAccessKey))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3SessionTokenBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3NewPathStyle))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3SessionTokenBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::S3NewPathStyle))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3SessionTokenBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3SecurityToken))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3SessionTokenBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::S3SecurityToken))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3NewPathStyleBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::RemoteDirectory))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3NewPathStyleBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::RemoteDirectory))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3NewPathStyleBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::S3SessionToken))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::S3NewPathStyleBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::S3SessionToken))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeClientCertBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::KubeClientKey))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::KubeClientCertBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::KubeClientKey))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeClientCertBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::KubeUsername))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::KubeClientCertBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::KubeUsername))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeClientKeyBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::RemoteDirectory))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::KubeClientKeyBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::RemoteDirectory))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeClientKeyBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::KubeClientCert))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::KubeClientKeyBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::KubeClientCert))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeNamespaceBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::KubeClusterUrl))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::KubeNamespaceBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::KubeClusterUrl))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeNamespaceBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::Protocol))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::KubeNamespaceBlurUp) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::Protocol)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeClusterUrlBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::KubeUsername))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::KubeClusterUrlBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::KubeUsername))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeClusterUrlBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::KubeNamespace))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::KubeClusterUrlBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::KubeNamespace))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeUsernameBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::KubeClientCert))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::KubeUsernameBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::KubeClientCert))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeUsernameBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::KubeClusterUrl))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::KubeUsernameBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::KubeClusterUrl))
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::SmbShareBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::Username))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::SmbShareBlurDown) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::Username)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::SmbShareBlurUp) => {
                let id = if cfg!(windows) && self.host_bridge_input_mask() == InputMask::Smb {
                    &Id::HostBridge(AuthFormId::Address)
                } else {
                    &Id::HostBridge(AuthFormId::Port)
                };
                assert!(self.app.active(id).is_ok());
            }
            UiMsg::Remote(UiAuthFormMsg::SmbShareBlurUp) => {
                let id = if cfg!(windows) && self.remote_input_mask() == InputMask::Smb {
                    &Id::Remote(AuthFormId::Address)
                } else {
                    &Id::Remote(AuthFormId::Port)
                };
                assert!(self.app.active(id).is_ok());
            }
            #[cfg(posix)]
            UiMsg::HostBridge(UiAuthFormMsg::SmbWorkgroupDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::RemoteDirectory))
                        .is_ok()
                );
            }
            #[cfg(posix)]
            UiMsg::Remote(UiAuthFormMsg::SmbWorkgroupDown) => {
                assert!(
                    self.app
                        .active(&Id::Remote(AuthFormId::RemoteDirectory))
                        .is_ok()
                );
            }
            #[cfg(posix)]
            UiMsg::HostBridge(UiAuthFormMsg::SmbWorkgroupUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::Password))
                        .is_ok()
                );
            }
            #[cfg(posix)]
            UiMsg::Remote(UiAuthFormMsg::SmbWorkgroupUp) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::Password)).is_ok());
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
                self.mount_bookmark_save_dialog(self.get_current_form_tab());
            }
            UiMsg::HostBridge(UiAuthFormMsg::UsernameBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::Password))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::UsernameBlurDown) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::Password)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::UsernameBlurUp) => {
                assert!(
                    self.app
                        .active(match self.host_bridge_input_mask() {
                            InputMask::Localhost => unreachable!(),
                            InputMask::Generic => &Id::HostBridge(AuthFormId::Port),
                            InputMask::Smb => &Id::HostBridge(AuthFormId::SmbShare),
                            InputMask::Kube =>
                                unreachable!("this shouldn't happen (username on kube)"),
                            InputMask::AwsS3 =>
                                unreachable!("this shouldn't happen (username on s3)"),
                            InputMask::WebDAV => &Id::HostBridge(AuthFormId::WebDAVUri),
                        })
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::UsernameBlurUp) => {
                assert!(
                    self.app
                        .active(match self.remote_input_mask() {
                            InputMask::Localhost => unreachable!(),
                            InputMask::Generic => &Id::Remote(AuthFormId::Port),
                            InputMask::Smb => &Id::Remote(AuthFormId::SmbShare),
                            InputMask::Kube =>
                                unreachable!("this shouldn't happen (username on kube)"),
                            InputMask::AwsS3 =>
                                unreachable!("this shouldn't happen (username on s3)"),
                            InputMask::WebDAV => &Id::Remote(AuthFormId::WebDAVUri),
                        })
                        .is_ok()
                );
            }
            UiMsg::HostBridge(UiAuthFormMsg::WebDAVUriBlurDown) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::Username))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::WebDAVUriBlurDown) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::Username)).is_ok());
            }
            UiMsg::HostBridge(UiAuthFormMsg::WebDAVUriBlurUp) => {
                assert!(
                    self.app
                        .active(&Id::HostBridge(AuthFormId::Protocol))
                        .is_ok()
                );
            }
            UiMsg::Remote(UiAuthFormMsg::WebDAVUriBlurUp) => {
                assert!(self.app.active(&Id::Remote(AuthFormId::Protocol)).is_ok());
            }
            UiMsg::WindowResized => {
                self.redraw = true;
            }
        }

        None
    }
}
