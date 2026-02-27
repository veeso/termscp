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
            UiMsg::HostBridge(UiAuthFormMsg::AddressBlurDown) => {
                let id = if cfg!(windows) && self.host_bridge_input_mask() == InputMask::Smb {
                    &Id::HostBridge(AuthFormId::SmbShare)
                } else {
                    &Id::HostBridge(AuthFormId::Port)
                };
                if let Err(err) = self.app.active(id) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::AddressBlurDown) => {
                let id = if cfg!(windows) && self.remote_input_mask() == InputMask::Smb {
                    &Id::Remote(AuthFormId::SmbShare)
                } else {
                    &Id::Remote(AuthFormId::Port)
                };
                if let Err(err) = self.app.active(id) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::AddressBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::Protocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::AddressBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::Protocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::BookmarksListBlur => {
                if let Err(err) = self.app.active(&Id::RecentsList) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::BookmarkNameBlur => {
                if let Err(err) = self.app.active(&Id::BookmarkSavePassword) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::BookmarksTabBlur => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::Protocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::ChangeFormTab) => {
                self.last_form_tab = FormTab::Remote;
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::Protocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::ChangeFormTab) => {
                self.last_form_tab = FormTab::HostBridge;
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::Protocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::CloseDeleteBookmark => {
                if let Err(err) = self.app.umount(&Id::DeleteBookmarkPopup) {
                    error!("Failed to umount component: {err}");
                }
            }
            UiMsg::CloseDeleteRecent => {
                if let Err(err) = self.app.umount(&Id::DeleteRecentPopup) {
                    error!("Failed to umount component: {err}");
                }
            }
            UiMsg::CloseErrorPopup => {
                self.umount_error();
            }
            UiMsg::CloseInfoPopup => {
                self.umount_info();
            }
            UiMsg::CloseInstallUpdatePopup => {
                if let Err(err) = self.app.umount(&Id::NewVersionChangelog) {
                    error!("Failed to umount component: {err}");
                }
                if let Err(err) = self.app.umount(&Id::InstallUpdatePopup) {
                    error!("Failed to umount component: {err}");
                }
            }
            UiMsg::CloseKeybindingsPopup => {
                self.umount_help();
            }
            UiMsg::CloseQuitPopup => self.umount_quit(),
            UiMsg::CloseSaveBookmark => {
                if let Err(err) = self.app.umount(&Id::BookmarkName) {
                    error!("Failed to umount component: {err}");
                }
                if let Err(err) = self.app.umount(&Id::BookmarkSavePassword) {
                    error!("Failed to umount component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::LocalDirectoryBlurDown) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::Protocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::LocalDirectoryBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::Protocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::LocalDirectoryBlurUp) => {
                if let Err(err) = self.app.active(match self.host_bridge_input_mask() {
                    InputMask::Localhost => &Id::HostBridge(AuthFormId::Protocol),
                    _ => &Id::HostBridge(AuthFormId::RemoteDirectory),
                }) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::LocalDirectoryBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::RemoteDirectory)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::ParamsFormBlur) => {
                if let Err(err) = self.app.active(&Id::BookmarksList) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::ParamsFormBlur) => {
                if let Err(err) = self.app.active(&Id::BookmarksList) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::PasswordBlurDown) => {
                if let Err(err) = self.app.active(match self.host_bridge_input_mask() {
                    InputMask::Localhost => unreachable!(),
                    InputMask::Generic => &Id::HostBridge(AuthFormId::RemoteDirectory),
                    #[cfg(posix)]
                    InputMask::Smb => &Id::HostBridge(AuthFormId::SmbWorkgroup),
                    #[cfg(win)]
                    InputMask::Smb => &Id::HostBridge(AuthFormId::RemoteDirectory),
                    InputMask::AwsS3 => unreachable!("this shouldn't happen (password on s3)"),
                    InputMask::Kube => unreachable!("this shouldn't happen (password on kube)"),
                    InputMask::WebDAV => &Id::HostBridge(AuthFormId::RemoteDirectory),
                }) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::PasswordBlurDown) => {
                if let Err(err) = self.app.active(match self.remote_input_mask() {
                    InputMask::Localhost => unreachable!(),
                    InputMask::Generic => &Id::Remote(AuthFormId::RemoteDirectory),
                    #[cfg(posix)]
                    InputMask::Smb => &Id::Remote(AuthFormId::SmbWorkgroup),
                    #[cfg(win)]
                    InputMask::Smb => &Id::Remote(AuthFormId::RemoteDirectory),
                    InputMask::AwsS3 => unreachable!("this shouldn't happen (password on s3)"),
                    InputMask::Kube => unreachable!("this shouldn't happen (password on kube)"),
                    InputMask::WebDAV => &Id::Remote(AuthFormId::RemoteDirectory),
                }) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::PasswordBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::Username)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::PasswordBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::Username)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::PortBlurDown) => {
                if let Err(err) = self.app.active(match self.host_bridge_input_mask() {
                    InputMask::Generic => &Id::HostBridge(AuthFormId::Username),
                    InputMask::Smb => &Id::HostBridge(AuthFormId::SmbShare),
                    InputMask::Localhost
                    | InputMask::AwsS3
                    | InputMask::Kube
                    | InputMask::WebDAV => {
                        unreachable!("this shouldn't happen (port on s3/kube/webdav)")
                    }
                }) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::PortBlurDown) => {
                if let Err(err) = self.app.active(match self.remote_input_mask() {
                    InputMask::Generic => &Id::Remote(AuthFormId::Username),
                    InputMask::Smb => &Id::Remote(AuthFormId::SmbShare),
                    InputMask::Localhost
                    | InputMask::AwsS3
                    | InputMask::Kube
                    | InputMask::WebDAV => {
                        unreachable!("this shouldn't happen (port on s3/kube/webdav)")
                    }
                }) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::PortBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::Address)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::PortBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::Address)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::ProtocolBlurDown) => {
                if let Err(err) = self.app.active(match self.host_bridge_input_mask() {
                    InputMask::Localhost => &Id::HostBridge(AuthFormId::LocalDirectory),
                    InputMask::Generic => &Id::HostBridge(AuthFormId::Address),
                    InputMask::Smb => &Id::HostBridge(AuthFormId::Address),
                    InputMask::AwsS3 => &Id::HostBridge(AuthFormId::S3Bucket),
                    InputMask::Kube => &Id::HostBridge(AuthFormId::KubeNamespace),
                    InputMask::WebDAV => &Id::HostBridge(AuthFormId::WebDAVUri),
                }) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::ProtocolBlurDown) => {
                if let Err(err) = self.app.active(match self.remote_input_mask() {
                    InputMask::Localhost => &Id::Remote(AuthFormId::LocalDirectory),
                    InputMask::Generic => &Id::Remote(AuthFormId::Address),
                    InputMask::Smb => &Id::Remote(AuthFormId::Address),
                    InputMask::AwsS3 => &Id::Remote(AuthFormId::S3Bucket),
                    InputMask::Kube => &Id::Remote(AuthFormId::KubeNamespace),
                    InputMask::WebDAV => &Id::Remote(AuthFormId::WebDAVUri),
                }) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::ProtocolBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::LocalDirectory)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::ProtocolBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::LocalDirectory)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::RececentsListBlur => {
                if let Err(err) = self.app.active(&Id::BookmarksList) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::RemoteDirectoryBlurDown) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::LocalDirectory)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::RemoteDirectoryBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::LocalDirectory)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::RemoteDirectoryBlurUp) => {
                if let Err(err) = self.app.active(match self.host_bridge_input_mask() {
                    InputMask::Localhost => unreachable!(),
                    InputMask::Generic => &Id::HostBridge(AuthFormId::Password),
                    #[cfg(posix)]
                    InputMask::Smb => &Id::HostBridge(AuthFormId::SmbWorkgroup),
                    #[cfg(win)]
                    InputMask::Smb => &Id::HostBridge(AuthFormId::Password),
                    InputMask::Kube => &Id::HostBridge(AuthFormId::KubeClientKey),
                    InputMask::AwsS3 => &Id::HostBridge(AuthFormId::S3NewPathStyle),
                    InputMask::WebDAV => &Id::HostBridge(AuthFormId::Password),
                }) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::RemoteDirectoryBlurUp) => {
                if let Err(err) = self.app.active(match self.remote_input_mask() {
                    InputMask::Localhost => unreachable!(),
                    InputMask::Generic => &Id::Remote(AuthFormId::Password),
                    #[cfg(posix)]
                    InputMask::Smb => &Id::Remote(AuthFormId::SmbWorkgroup),
                    #[cfg(win)]
                    InputMask::Smb => &Id::Remote(AuthFormId::Password),
                    InputMask::Kube => &Id::Remote(AuthFormId::KubeClientKey),
                    InputMask::AwsS3 => &Id::Remote(AuthFormId::S3NewPathStyle),
                    InputMask::WebDAV => &Id::Remote(AuthFormId::Password),
                }) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3BucketBlurDown) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::S3Region)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3BucketBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3Region)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3BucketBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::Protocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3BucketBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::Protocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3RegionBlurDown) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::S3Endpoint)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3RegionBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3Endpoint)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3RegionBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::S3Bucket)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3RegionBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3Bucket)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3EndpointBlurDown) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::S3Profile)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3EndpointBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3Profile)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3EndpointBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::S3Region)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3EndpointBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3Region)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3ProfileBlurDown) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::S3AccessKey)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3ProfileBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3AccessKey)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3ProfileBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::S3Endpoint)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3ProfileBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3Endpoint)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3AccessKeyBlurDown) => {
                if let Err(err) = self
                    .app
                    .active(&Id::HostBridge(AuthFormId::S3SecretAccessKey))
                {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3AccessKeyBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3SecretAccessKey)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3AccessKeyBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::S3Profile)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3AccessKeyBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3Profile)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3SecretAccessKeyBlurDown) => {
                if let Err(err) = self
                    .app
                    .active(&Id::HostBridge(AuthFormId::S3SecurityToken))
                {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3SecretAccessKeyBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3SecurityToken)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3SecretAccessKeyBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::S3AccessKey)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3SecretAccessKeyBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3AccessKey)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3SecurityTokenBlurDown) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::S3SessionToken)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3SecurityTokenBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3SessionToken)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3SecurityTokenBlurUp) => {
                if let Err(err) = self
                    .app
                    .active(&Id::HostBridge(AuthFormId::S3SecretAccessKey))
                {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3SecurityTokenBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3SecretAccessKey)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3SessionTokenBlurDown) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::S3NewPathStyle)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3SessionTokenBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3NewPathStyle)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3SessionTokenBlurUp) => {
                if let Err(err) = self
                    .app
                    .active(&Id::HostBridge(AuthFormId::S3SecurityToken))
                {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3SessionTokenBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3SecurityToken)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3NewPathStyleBlurDown) => {
                if let Err(err) = self
                    .app
                    .active(&Id::HostBridge(AuthFormId::RemoteDirectory))
                {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3NewPathStyleBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::RemoteDirectory)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::S3NewPathStyleBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::S3SessionToken)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::S3NewPathStyleBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::S3SessionToken)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeClientCertBlurDown) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::KubeClientKey)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::KubeClientCertBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::KubeClientKey)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeClientCertBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::KubeUsername)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::KubeClientCertBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::KubeUsername)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeClientKeyBlurDown) => {
                if let Err(err) = self
                    .app
                    .active(&Id::HostBridge(AuthFormId::RemoteDirectory))
                {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::KubeClientKeyBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::RemoteDirectory)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeClientKeyBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::KubeClientCert)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::KubeClientKeyBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::KubeClientCert)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeNamespaceBlurDown) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::KubeClusterUrl)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::KubeNamespaceBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::KubeClusterUrl)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeNamespaceBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::Protocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::KubeNamespaceBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::Protocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeClusterUrlBlurDown) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::KubeUsername)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::KubeClusterUrlBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::KubeUsername)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeClusterUrlBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::KubeNamespace)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::KubeClusterUrlBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::KubeNamespace)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeUsernameBlurDown) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::KubeClientCert)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::KubeUsernameBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::KubeClientCert)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::KubeUsernameBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::KubeClusterUrl)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::KubeUsernameBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::KubeClusterUrl)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::SmbShareBlurDown) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::Username)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::SmbShareBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::Username)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::SmbShareBlurUp) => {
                let id = if cfg!(windows) && self.host_bridge_input_mask() == InputMask::Smb {
                    &Id::HostBridge(AuthFormId::Address)
                } else {
                    &Id::HostBridge(AuthFormId::Port)
                };
                if let Err(err) = self.app.active(id) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::SmbShareBlurUp) => {
                let id = if cfg!(windows) && self.remote_input_mask() == InputMask::Smb {
                    &Id::Remote(AuthFormId::Address)
                } else {
                    &Id::Remote(AuthFormId::Port)
                };
                if let Err(err) = self.app.active(id) {
                    error!("Failed to activate component: {err}");
                }
            }
            #[cfg(posix)]
            UiMsg::HostBridge(UiAuthFormMsg::SmbWorkgroupDown) => {
                if let Err(err) = self
                    .app
                    .active(&Id::HostBridge(AuthFormId::RemoteDirectory))
                {
                    error!("Failed to activate component: {err}");
                }
            }
            #[cfg(posix)]
            UiMsg::Remote(UiAuthFormMsg::SmbWorkgroupDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::RemoteDirectory)) {
                    error!("Failed to activate component: {err}");
                }
            }
            #[cfg(posix)]
            UiMsg::HostBridge(UiAuthFormMsg::SmbWorkgroupUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::Password)) {
                    error!("Failed to activate component: {err}");
                }
            }
            #[cfg(posix)]
            UiMsg::Remote(UiAuthFormMsg::SmbWorkgroupUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::Password)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::SaveBookmarkPasswordBlur => {
                if let Err(err) = self.app.active(&Id::BookmarkName) {
                    error!("Failed to activate component: {err}");
                }
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
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::Password)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::UsernameBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::Password)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::UsernameBlurUp) => {
                if let Err(err) = self.app.active(match self.host_bridge_input_mask() {
                    InputMask::Localhost => unreachable!(),
                    InputMask::Generic => &Id::HostBridge(AuthFormId::Port),
                    InputMask::Smb => &Id::HostBridge(AuthFormId::SmbShare),
                    InputMask::Kube => unreachable!("this shouldn't happen (username on kube)"),
                    InputMask::AwsS3 => unreachable!("this shouldn't happen (username on s3)"),
                    InputMask::WebDAV => &Id::HostBridge(AuthFormId::WebDAVUri),
                }) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::UsernameBlurUp) => {
                if let Err(err) = self.app.active(match self.remote_input_mask() {
                    InputMask::Localhost => unreachable!(),
                    InputMask::Generic => &Id::Remote(AuthFormId::Port),
                    InputMask::Smb => &Id::Remote(AuthFormId::SmbShare),
                    InputMask::Kube => unreachable!("this shouldn't happen (username on kube)"),
                    InputMask::AwsS3 => unreachable!("this shouldn't happen (username on s3)"),
                    InputMask::WebDAV => &Id::Remote(AuthFormId::WebDAVUri),
                }) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::WebDAVUriBlurDown) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::Username)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::WebDAVUriBlurDown) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::Username)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::HostBridge(UiAuthFormMsg::WebDAVUriBlurUp) => {
                if let Err(err) = self.app.active(&Id::HostBridge(AuthFormId::Protocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::Remote(UiAuthFormMsg::WebDAVUriBlurUp) => {
                if let Err(err) = self.app.active(&Id::Remote(AuthFormId::Protocol)) {
                    error!("Failed to activate component: {err}");
                }
            }
            UiMsg::WindowResized => {
                self.redraw = true;
            }
        }

        None
    }
}
