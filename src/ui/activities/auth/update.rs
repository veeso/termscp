//! ## Update
//!
//! Update impl

use tuirealm::state::{State, StateValue};

use super::{
    AuthActivity, AuthFormId, ExitReason, FormMsg, FormTab, HostBridgeProtocol, Id, InputMask, Msg,
    UiAuthFormMsg, UiMsg,
};

impl AuthActivity {
    pub(super) fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
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

                debug!("Remote params: {:?}", remote_params);
                debug!("Host bridge params: {:?}", host_bridge_params);

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
                if let Ok(State::Single(StateValue::Usize(idx))) =
                    self.app.state(&Id::BookmarksList)
                {
                    // Umount dialog
                    self.umount_bookmark_del_dialog();
                    // Delete bookmark
                    self.del_bookmark(idx);
                    // Update bookmarks
                    self.view_bookmarks()
                }
            }
            FormMsg::DeleteRecent => {
                if let Ok(State::Single(StateValue::Usize(idx))) = self.app.state(&Id::RecentsList)
                {
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

                if let Err(err) = self.app.active(focus) {
                    error!("Failed to activate component: {err}");
                }
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

                if let Err(err) = self.app.active(focus) {
                    error!("Failed to activate component: {err}");
                }
            }
            FormMsg::HostBridgeProtocolChanged(protocol) => {
                self.host_bridge_protocol = protocol;
                // Update port
                let port: u16 = self.get_input_port(FormTab::HostBridge);
                if let HostBridgeProtocol::Remote(remote_protocol) = protocol
                    && Self::is_port_standard(port)
                {
                    self.mount_port(
                        FormTab::HostBridge,
                        Self::get_default_port_for_protocol(remote_protocol),
                    );
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
            UiMsg::HostBridge(msg) => self.update_host_bridge_ui(msg),
            UiMsg::Remote(msg) => self.update_remote_ui(msg),
            UiMsg::BookmarksListBlur => {
                self.activate_component(Id::RecentsList);
            }
            UiMsg::BookmarkNameBlur => {
                self.activate_component(Id::BookmarkSavePassword);
            }
            UiMsg::BookmarksTabBlur => {
                self.activate_component(Id::HostBridge(AuthFormId::Protocol));
            }
            UiMsg::CloseDeleteBookmark => {
                self.umount_component(Id::DeleteBookmarkPopup);
            }
            UiMsg::CloseDeleteRecent => {
                self.umount_component(Id::DeleteRecentPopup);
            }
            UiMsg::CloseErrorPopup => {
                self.umount_error();
            }
            UiMsg::CloseInfoPopup => {
                self.umount_info();
            }
            UiMsg::CloseInstallUpdatePopup => {
                self.umount_component(Id::NewVersionChangelog);
                self.umount_component(Id::InstallUpdatePopup);
            }
            UiMsg::CloseKeybindingsPopup => {
                self.umount_help();
            }
            UiMsg::CloseQuitPopup => self.umount_quit(),
            UiMsg::CloseSaveBookmark => {
                self.umount_component(Id::BookmarkName);
                self.umount_component(Id::BookmarkSavePassword);
            }
            UiMsg::RececentsListBlur => {
                self.activate_component(Id::BookmarksList);
            }
            UiMsg::SaveBookmarkPasswordBlur => {
                self.activate_component(Id::BookmarkName);
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
            UiMsg::WindowResized => {
                self.redraw = true;
            }
        }

        None
    }

    fn update_host_bridge_ui(&mut self, msg: UiAuthFormMsg) {
        match msg {
            UiAuthFormMsg::AddressBlurDown => {
                let id = if cfg!(windows) && self.host_bridge_input_mask() == InputMask::Smb {
                    Id::HostBridge(AuthFormId::SmbShare)
                } else {
                    Id::HostBridge(AuthFormId::Port)
                };
                self.activate_component(id);
            }
            UiAuthFormMsg::AddressBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::Protocol));
            }
            UiAuthFormMsg::ChangeFormTab => {
                self.last_form_tab = FormTab::Remote;
                self.activate_component(Id::Remote(AuthFormId::Protocol));
            }
            UiAuthFormMsg::LocalDirectoryBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::Protocol));
            }
            UiAuthFormMsg::LocalDirectoryBlurUp => {
                let id = match self.host_bridge_input_mask() {
                    InputMask::Localhost => Id::HostBridge(AuthFormId::Protocol),
                    _ => Id::HostBridge(AuthFormId::RemoteDirectory),
                };
                self.activate_component(id);
            }
            UiAuthFormMsg::ParamsFormBlur => {
                self.activate_component(Id::BookmarksList);
            }
            UiAuthFormMsg::PasswordBlurDown => {
                let id = match self.host_bridge_input_mask() {
                    InputMask::Localhost => unreachable!(),
                    InputMask::Generic => Id::HostBridge(AuthFormId::RemoteDirectory),
                    #[cfg(posix)]
                    InputMask::Smb => Id::HostBridge(AuthFormId::SmbWorkgroup),
                    #[cfg(win)]
                    InputMask::Smb => Id::HostBridge(AuthFormId::RemoteDirectory),
                    InputMask::AwsS3 => unreachable!("this shouldn't happen (password on s3)"),
                    InputMask::Kube => unreachable!("this shouldn't happen (password on kube)"),
                    InputMask::WebDAV => Id::HostBridge(AuthFormId::RemoteDirectory),
                };
                self.activate_component(id);
            }
            UiAuthFormMsg::PasswordBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::Username));
            }
            UiAuthFormMsg::PortBlurDown => {
                let id = match self.host_bridge_input_mask() {
                    InputMask::Generic => Id::HostBridge(AuthFormId::Username),
                    InputMask::Smb => Id::HostBridge(AuthFormId::SmbShare),
                    InputMask::Localhost
                    | InputMask::AwsS3
                    | InputMask::Kube
                    | InputMask::WebDAV => {
                        unreachable!("this shouldn't happen (port on s3/kube/webdav)")
                    }
                };
                self.activate_component(id);
            }
            UiAuthFormMsg::PortBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::Address));
            }
            UiAuthFormMsg::ProtocolBlurDown => {
                let id = match self.host_bridge_input_mask() {
                    InputMask::Localhost => Id::HostBridge(AuthFormId::LocalDirectory),
                    InputMask::Generic => Id::HostBridge(AuthFormId::Address),
                    InputMask::Smb => Id::HostBridge(AuthFormId::Address),
                    InputMask::AwsS3 => Id::HostBridge(AuthFormId::S3Bucket),
                    InputMask::Kube => Id::HostBridge(AuthFormId::KubeNamespace),
                    InputMask::WebDAV => Id::HostBridge(AuthFormId::WebDAVUri),
                };
                self.activate_component(id);
            }
            UiAuthFormMsg::ProtocolBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::LocalDirectory));
            }
            UiAuthFormMsg::RemoteDirectoryBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::LocalDirectory));
            }
            UiAuthFormMsg::RemoteDirectoryBlurUp => {
                let id = match self.host_bridge_input_mask() {
                    InputMask::Localhost => unreachable!(),
                    InputMask::Generic => Id::HostBridge(AuthFormId::Password),
                    #[cfg(posix)]
                    InputMask::Smb => Id::HostBridge(AuthFormId::SmbWorkgroup),
                    #[cfg(win)]
                    InputMask::Smb => Id::HostBridge(AuthFormId::Password),
                    InputMask::Kube => Id::HostBridge(AuthFormId::KubeClientKey),
                    InputMask::AwsS3 => Id::HostBridge(AuthFormId::S3NewPathStyle),
                    InputMask::WebDAV => Id::HostBridge(AuthFormId::Password),
                };
                self.activate_component(id);
            }
            UiAuthFormMsg::S3BucketBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::S3Region))
            }
            UiAuthFormMsg::S3BucketBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::Protocol))
            }
            UiAuthFormMsg::S3RegionBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::S3Endpoint))
            }
            UiAuthFormMsg::S3RegionBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::S3Bucket))
            }
            UiAuthFormMsg::S3EndpointBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::S3Profile))
            }
            UiAuthFormMsg::S3EndpointBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::S3Region))
            }
            UiAuthFormMsg::S3ProfileBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::S3AccessKey))
            }
            UiAuthFormMsg::S3ProfileBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::S3Endpoint))
            }
            UiAuthFormMsg::S3AccessKeyBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::S3SecretAccessKey))
            }
            UiAuthFormMsg::S3AccessKeyBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::S3Profile))
            }
            UiAuthFormMsg::S3SecretAccessKeyBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::S3SecurityToken))
            }
            UiAuthFormMsg::S3SecretAccessKeyBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::S3AccessKey))
            }
            UiAuthFormMsg::S3SecurityTokenBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::S3SessionToken))
            }
            UiAuthFormMsg::S3SecurityTokenBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::S3SecretAccessKey))
            }
            UiAuthFormMsg::S3SessionTokenBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::S3NewPathStyle))
            }
            UiAuthFormMsg::S3SessionTokenBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::S3SecurityToken))
            }
            UiAuthFormMsg::S3NewPathStyleBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::RemoteDirectory))
            }
            UiAuthFormMsg::S3NewPathStyleBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::S3SessionToken))
            }
            UiAuthFormMsg::KubeClientCertBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::KubeClientKey))
            }
            UiAuthFormMsg::KubeClientCertBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::KubeUsername))
            }
            UiAuthFormMsg::KubeClientKeyBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::RemoteDirectory))
            }
            UiAuthFormMsg::KubeClientKeyBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::KubeClientCert))
            }
            UiAuthFormMsg::KubeNamespaceBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::KubeClusterUrl))
            }
            UiAuthFormMsg::KubeNamespaceBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::Protocol))
            }
            UiAuthFormMsg::KubeClusterUrlBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::KubeUsername))
            }
            UiAuthFormMsg::KubeClusterUrlBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::KubeNamespace))
            }
            UiAuthFormMsg::KubeUsernameBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::KubeClientCert))
            }
            UiAuthFormMsg::KubeUsernameBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::KubeClusterUrl))
            }
            UiAuthFormMsg::SmbShareBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::Username))
            }
            UiAuthFormMsg::SmbShareBlurUp => {
                let id = if cfg!(windows) && self.host_bridge_input_mask() == InputMask::Smb {
                    Id::HostBridge(AuthFormId::Address)
                } else {
                    Id::HostBridge(AuthFormId::Port)
                };
                self.activate_component(id);
            }
            #[cfg(posix)]
            UiAuthFormMsg::SmbWorkgroupDown => {
                self.activate_component(Id::HostBridge(AuthFormId::RemoteDirectory))
            }
            #[cfg(posix)]
            UiAuthFormMsg::SmbWorkgroupUp => {
                self.activate_component(Id::HostBridge(AuthFormId::Password))
            }
            UiAuthFormMsg::UsernameBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::Password))
            }
            UiAuthFormMsg::UsernameBlurUp => {
                let id = match self.host_bridge_input_mask() {
                    InputMask::Localhost => unreachable!(),
                    InputMask::Generic => Id::HostBridge(AuthFormId::Port),
                    InputMask::Smb => Id::HostBridge(AuthFormId::SmbShare),
                    InputMask::Kube => unreachable!("this shouldn't happen (username on kube)"),
                    InputMask::AwsS3 => unreachable!("this shouldn't happen (username on s3)"),
                    InputMask::WebDAV => Id::HostBridge(AuthFormId::WebDAVUri),
                };
                self.activate_component(id);
            }
            UiAuthFormMsg::WebDAVUriBlurDown => {
                self.activate_component(Id::HostBridge(AuthFormId::Username))
            }
            UiAuthFormMsg::WebDAVUriBlurUp => {
                self.activate_component(Id::HostBridge(AuthFormId::Protocol))
            }
        }
    }

    fn update_remote_ui(&mut self, msg: UiAuthFormMsg) {
        match msg {
            UiAuthFormMsg::AddressBlurDown => {
                let id = if cfg!(windows) && self.remote_input_mask() == InputMask::Smb {
                    Id::Remote(AuthFormId::SmbShare)
                } else {
                    Id::Remote(AuthFormId::Port)
                };
                self.activate_component(id);
            }
            UiAuthFormMsg::AddressBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::Protocol));
            }
            UiAuthFormMsg::ChangeFormTab => {
                self.last_form_tab = FormTab::HostBridge;
                self.activate_component(Id::HostBridge(AuthFormId::Protocol));
            }
            UiAuthFormMsg::LocalDirectoryBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::Protocol));
            }
            UiAuthFormMsg::LocalDirectoryBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::RemoteDirectory));
            }
            UiAuthFormMsg::ParamsFormBlur => {
                self.activate_component(Id::BookmarksList);
            }
            UiAuthFormMsg::PasswordBlurDown => {
                let id = match self.remote_input_mask() {
                    InputMask::Localhost => unreachable!(),
                    InputMask::Generic => Id::Remote(AuthFormId::RemoteDirectory),
                    #[cfg(posix)]
                    InputMask::Smb => Id::Remote(AuthFormId::SmbWorkgroup),
                    #[cfg(win)]
                    InputMask::Smb => Id::Remote(AuthFormId::RemoteDirectory),
                    InputMask::AwsS3 => unreachable!("this shouldn't happen (password on s3)"),
                    InputMask::Kube => unreachable!("this shouldn't happen (password on kube)"),
                    InputMask::WebDAV => Id::Remote(AuthFormId::RemoteDirectory),
                };
                self.activate_component(id);
            }
            UiAuthFormMsg::PasswordBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::Username));
            }
            UiAuthFormMsg::PortBlurDown => {
                let id = match self.remote_input_mask() {
                    InputMask::Generic => Id::Remote(AuthFormId::Username),
                    InputMask::Smb => Id::Remote(AuthFormId::SmbShare),
                    InputMask::Localhost
                    | InputMask::AwsS3
                    | InputMask::Kube
                    | InputMask::WebDAV => {
                        unreachable!("this shouldn't happen (port on s3/kube/webdav)")
                    }
                };
                self.activate_component(id);
            }
            UiAuthFormMsg::PortBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::Address));
            }
            UiAuthFormMsg::ProtocolBlurDown => {
                let id = match self.remote_input_mask() {
                    InputMask::Localhost => Id::Remote(AuthFormId::LocalDirectory),
                    InputMask::Generic => Id::Remote(AuthFormId::Address),
                    InputMask::Smb => Id::Remote(AuthFormId::Address),
                    InputMask::AwsS3 => Id::Remote(AuthFormId::S3Bucket),
                    InputMask::Kube => Id::Remote(AuthFormId::KubeNamespace),
                    InputMask::WebDAV => Id::Remote(AuthFormId::WebDAVUri),
                };
                self.activate_component(id);
            }
            UiAuthFormMsg::ProtocolBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::LocalDirectory));
            }
            UiAuthFormMsg::RemoteDirectoryBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::LocalDirectory));
            }
            UiAuthFormMsg::RemoteDirectoryBlurUp => {
                let id = match self.remote_input_mask() {
                    InputMask::Localhost => unreachable!(),
                    InputMask::Generic => Id::Remote(AuthFormId::Password),
                    #[cfg(posix)]
                    InputMask::Smb => Id::Remote(AuthFormId::SmbWorkgroup),
                    #[cfg(win)]
                    InputMask::Smb => Id::Remote(AuthFormId::Password),
                    InputMask::Kube => Id::Remote(AuthFormId::KubeClientKey),
                    InputMask::AwsS3 => Id::Remote(AuthFormId::S3NewPathStyle),
                    InputMask::WebDAV => Id::Remote(AuthFormId::Password),
                };
                self.activate_component(id);
            }
            UiAuthFormMsg::S3BucketBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::S3Region))
            }
            UiAuthFormMsg::S3BucketBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::Protocol))
            }
            UiAuthFormMsg::S3RegionBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::S3Endpoint))
            }
            UiAuthFormMsg::S3RegionBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::S3Bucket))
            }
            UiAuthFormMsg::S3EndpointBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::S3Profile))
            }
            UiAuthFormMsg::S3EndpointBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::S3Region))
            }
            UiAuthFormMsg::S3ProfileBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::S3AccessKey))
            }
            UiAuthFormMsg::S3ProfileBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::S3Endpoint))
            }
            UiAuthFormMsg::S3AccessKeyBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::S3SecretAccessKey))
            }
            UiAuthFormMsg::S3AccessKeyBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::S3Profile))
            }
            UiAuthFormMsg::S3SecretAccessKeyBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::S3SecurityToken))
            }
            UiAuthFormMsg::S3SecretAccessKeyBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::S3AccessKey))
            }
            UiAuthFormMsg::S3SecurityTokenBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::S3SessionToken))
            }
            UiAuthFormMsg::S3SecurityTokenBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::S3SecretAccessKey))
            }
            UiAuthFormMsg::S3SessionTokenBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::S3NewPathStyle))
            }
            UiAuthFormMsg::S3SessionTokenBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::S3SecurityToken))
            }
            UiAuthFormMsg::S3NewPathStyleBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::RemoteDirectory))
            }
            UiAuthFormMsg::S3NewPathStyleBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::S3SessionToken))
            }
            UiAuthFormMsg::KubeClientCertBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::KubeClientKey))
            }
            UiAuthFormMsg::KubeClientCertBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::KubeUsername))
            }
            UiAuthFormMsg::KubeClientKeyBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::RemoteDirectory))
            }
            UiAuthFormMsg::KubeClientKeyBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::KubeClientCert))
            }
            UiAuthFormMsg::KubeNamespaceBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::KubeClusterUrl))
            }
            UiAuthFormMsg::KubeNamespaceBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::Protocol))
            }
            UiAuthFormMsg::KubeClusterUrlBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::KubeUsername))
            }
            UiAuthFormMsg::KubeClusterUrlBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::KubeNamespace))
            }
            UiAuthFormMsg::KubeUsernameBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::KubeClientCert))
            }
            UiAuthFormMsg::KubeUsernameBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::KubeClusterUrl))
            }
            UiAuthFormMsg::SmbShareBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::Username))
            }
            UiAuthFormMsg::SmbShareBlurUp => {
                let id = if cfg!(windows) && self.remote_input_mask() == InputMask::Smb {
                    Id::Remote(AuthFormId::Address)
                } else {
                    Id::Remote(AuthFormId::Port)
                };
                self.activate_component(id);
            }
            #[cfg(posix)]
            UiAuthFormMsg::SmbWorkgroupDown => {
                self.activate_component(Id::Remote(AuthFormId::RemoteDirectory))
            }
            #[cfg(posix)]
            UiAuthFormMsg::SmbWorkgroupUp => {
                self.activate_component(Id::Remote(AuthFormId::Password))
            }
            UiAuthFormMsg::UsernameBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::Password))
            }
            UiAuthFormMsg::UsernameBlurUp => {
                let id = match self.remote_input_mask() {
                    InputMask::Localhost => unreachable!(),
                    InputMask::Generic => Id::Remote(AuthFormId::Port),
                    InputMask::Smb => Id::Remote(AuthFormId::SmbShare),
                    InputMask::Kube => unreachable!("this shouldn't happen (username on kube)"),
                    InputMask::AwsS3 => unreachable!("this shouldn't happen (username on s3)"),
                    InputMask::WebDAV => Id::Remote(AuthFormId::WebDAVUri),
                };
                self.activate_component(id);
            }
            UiAuthFormMsg::WebDAVUriBlurDown => {
                self.activate_component(Id::Remote(AuthFormId::Username))
            }
            UiAuthFormMsg::WebDAVUriBlurUp => {
                self.activate_component(Id::Remote(AuthFormId::Protocol))
            }
        }
    }

    fn activate_component(&mut self, id: Id) {
        if let Err(err) = self.app.active(&id) {
            error!("Failed to activate component: {err}");
        }
    }

    fn umount_component(&mut self, id: Id) {
        if let Err(err) = self.app.umount(&id) {
            error!("Failed to umount component: {err}");
        }
    }
}
