use super::*;
use crate::ui::activities::auth::STORE_KEY_RELEASE_NOTES;

impl AuthActivity {
    /// Make text span from bookmarks
    pub(in crate::ui::activities::auth) fn view_bookmarks(&mut self) {
        let bookmarks: Vec<String> = self
            .bookmarks_list
            .iter()
            .map(|x| {
                Self::fmt_bookmark(x, self.bookmarks_client().unwrap().get_bookmark(x).unwrap())
            })
            .collect();
        let bookmarks_color = self.theme().auth_bookmarks;
        if let Err(err) = self.app.remount(
            Id::BookmarksList,
            Box::new(components::BookmarksList::new(&bookmarks, bookmarks_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    /// View recent connections
    pub(in crate::ui::activities::auth) fn view_recent_connections(&mut self) {
        let bookmarks: Vec<String> = self
            .recents_list
            .iter()
            .map(|x| Self::fmt_recent(self.bookmarks_client().unwrap().get_recent(x).unwrap()))
            .collect();
        let recents_color = self.theme().auth_recents;
        if let Err(err) = self.app.remount(
            Id::RecentsList,
            Box::new(components::RecentsList::new(&bookmarks, recents_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_error<S: AsRef<str>>(&mut self, text: S) {
        let err_color = self.theme().misc_error_dialog;
        if let Err(err) = self.app.remount(
            Id::ErrorPopup,
            Box::new(components::ErrorPopup::new(text, err_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
        if let Err(err) = self.app.active(&Id::ErrorPopup) {
            error!("Failed to activate component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn umount_error(&mut self) {
        let _ = self.app.umount(&Id::ErrorPopup);
    }

    pub(in crate::ui::activities::auth) fn mount_info<S: AsRef<str>>(&mut self, text: S) {
        let color = self.theme().misc_info_dialog;
        if let Err(err) = self.app.remount(
            Id::InfoPopup,
            Box::new(components::InfoPopup::new(text, color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
        if let Err(err) = self.app.active(&Id::InfoPopup) {
            error!("Failed to activate component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn umount_info(&mut self) {
        let _ = self.app.umount(&Id::InfoPopup);
    }

    pub(in crate::ui::activities::auth) fn mount_wait(&mut self, text: &str) {
        let wait_color = self.theme().misc_info_dialog;
        if let Err(err) = self.app.remount(
            Id::WaitPopup,
            Box::new(components::WaitPopup::new(text, wait_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
        if let Err(err) = self.app.active(&Id::WaitPopup) {
            error!("Failed to activate component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn umount_wait(&mut self) {
        let _ = self.app.umount(&Id::WaitPopup);
    }

    pub(in crate::ui::activities::auth) fn mount_size_err(&mut self) {
        if let Err(err) = self.app.remount(
            Id::WindowSizeError,
            Box::new(components::WindowSizeError::new(Color::Red)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
        if let Err(err) = self.app.active(&Id::WindowSizeError) {
            error!("Failed to activate component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn umount_size_err(&mut self) {
        let _ = self.app.umount(&Id::WindowSizeError);
    }

    pub(in crate::ui::activities::auth) fn mount_quit(&mut self) {
        let quit_color = self.theme().misc_quit_dialog;
        if let Err(err) = self.app.remount(
            Id::QuitPopup,
            Box::new(components::QuitPopup::new(quit_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
        if let Err(err) = self.app.active(&Id::QuitPopup) {
            error!("Failed to activate component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn umount_quit(&mut self) {
        let _ = self.app.umount(&Id::QuitPopup);
    }

    pub(in crate::ui::activities::auth) fn mount_bookmark_del_dialog(&mut self) {
        let warn_color = self.theme().misc_warn_dialog;
        if let Err(err) = self.app.remount(
            Id::DeleteBookmarkPopup,
            Box::new(components::DeleteBookmarkPopup::new(warn_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
        if let Err(err) = self.app.active(&Id::DeleteBookmarkPopup) {
            error!("Failed to activate component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn umount_bookmark_del_dialog(&mut self) {
        let _ = self.app.umount(&Id::DeleteBookmarkPopup);
    }

    pub(in crate::ui::activities::auth) fn mount_recent_del_dialog(&mut self) {
        let warn_color = self.theme().misc_warn_dialog;
        if let Err(err) = self.app.remount(
            Id::DeleteRecentPopup,
            Box::new(components::DeleteRecentPopup::new(warn_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
        if let Err(err) = self.app.active(&Id::DeleteRecentPopup) {
            error!("Failed to activate component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn umount_recent_del_dialog(&mut self) {
        let _ = self.app.umount(&Id::DeleteRecentPopup);
    }

    pub(in crate::ui::activities::auth) fn mount_bookmark_save_dialog(
        &mut self,
        form_tab: FormTab,
    ) {
        let save_color = self.theme().misc_save_dialog;
        let warn_color = self.theme().misc_warn_dialog;
        if let Err(err) = self.app.remount(
            Id::BookmarkName,
            Box::new(components::BookmarkName::new(form_tab, save_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
        if let Err(err) = self.app.remount(
            Id::BookmarkSavePassword,
            Box::new(components::BookmarkSavePassword::new(form_tab, warn_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
        if let Err(err) = self.app.active(&Id::BookmarkName) {
            error!("Failed to activate component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn umount_bookmark_save_dialog(&mut self) {
        let _ = self.app.umount(&Id::BookmarkName);
        let _ = self.app.umount(&Id::BookmarkSavePassword);
    }

    pub(in crate::ui::activities::auth) fn mount_keybindings(&mut self) {
        let key_color = self.theme().misc_keys;
        if let Err(err) = self.app.remount(
            Id::Keybindings,
            Box::new(components::Keybindings::new(key_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
        if let Err(err) = self.app.active(&Id::Keybindings) {
            error!("Failed to activate component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn umount_help(&mut self) {
        let _ = self.app.umount(&Id::Keybindings);
    }

    pub(in crate::ui::activities::auth) fn mount_release_notes(&mut self) {
        if let Some(ctx) = self.context.as_ref()
            && let Some(release_notes) = ctx.store().get_string(STORE_KEY_RELEASE_NOTES)
        {
            let info_color = self.theme().misc_info_dialog;
            if let Err(err) = self.app.remount(
                Id::NewVersionChangelog,
                Box::new(components::ReleaseNotes::new(release_notes, info_color)),
                vec![],
            ) {
                error!("Failed to remount component: {err}");
            }
            if let Err(err) = self.app.remount(
                Id::InstallUpdatePopup,
                Box::new(components::InstallUpdatePopup::new(info_color)),
                vec![],
            ) {
                error!("Failed to remount component: {err}");
            }
            if let Err(err) = self.app.active(&Id::InstallUpdatePopup) {
                error!("Failed to activate component: {err}");
            }
        }
    }

    pub(in crate::ui::activities::auth) fn umount_release_notes(&mut self) {
        let _ = self.app.umount(&Id::NewVersionChangelog);
        let _ = self.app.umount(&Id::InstallUpdatePopup);
    }

    pub(in crate::ui::activities::auth) fn mount_host_bridge_protocol(
        &mut self,
        protocol: HostBridgeProtocol,
    ) {
        let protocol_color = self.theme().auth_protocol;
        if let Err(err) = self.app.remount(
            Id::HostBridge(AuthFormId::Protocol),
            Box::new(components::HostBridgeProtocolRadio::new(
                protocol,
                protocol_color,
            )),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_remote_protocol(
        &mut self,
        protocol: FileTransferProtocol,
    ) {
        let protocol_color = self.theme().auth_protocol;
        if let Err(err) = self.app.remount(
            Id::Remote(AuthFormId::Protocol),
            Box::new(components::RemoteProtocolRadio::new(
                protocol,
                protocol_color,
            )),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_remote_directory<S: AsRef<str>>(
        &mut self,
        form_tab: FormTab,
        remote_path: S,
    ) {
        let id = Self::form_tab_id(form_tab, AuthFormId::RemoteDirectory);
        let protocol_color = self.theme().auth_protocol;
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputRemoteDirectory::new(
                remote_path.as_ref(),
                form_tab,
                protocol_color,
            )),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_local_directory<S: AsRef<str>>(
        &mut self,
        form_tab: FormTab,
        local_path: S,
    ) {
        let id = Self::form_tab_id(form_tab, AuthFormId::LocalDirectory);
        let color = self.theme().auth_username;
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputLocalDirectory::new(
                local_path.as_ref(),
                form_tab,
                color,
            )),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_address(
        &mut self,
        form_tab: FormTab,
        address: &str,
    ) {
        let addr_color = self.theme().auth_address;
        let id = Self::form_tab_id(form_tab, AuthFormId::Address);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputAddress::new(address, form_tab, addr_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_port(&mut self, form_tab: FormTab, port: u16) {
        let port_color = self.theme().auth_port;
        let id = Self::form_tab_id(form_tab, AuthFormId::Port);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputPort::new(port, form_tab, port_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_username(
        &mut self,
        form_tab: FormTab,
        username: &str,
    ) {
        let username_color = self.theme().auth_username;
        let id = Self::form_tab_id(form_tab, AuthFormId::Username);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputUsername::new(
                username,
                form_tab,
                username_color,
            )),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_password(
        &mut self,
        form_tab: FormTab,
        password: &str,
    ) {
        let password_color = self.theme().auth_password;
        let id = Self::form_tab_id(form_tab, AuthFormId::Password);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputPassword::new(
                password,
                form_tab,
                password_color,
            )),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_s3_bucket(
        &mut self,
        form_tab: FormTab,
        bucket: &str,
    ) {
        let addr_color = self.theme().auth_address;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3Bucket);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputS3Bucket::new(bucket, form_tab, addr_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_s3_region(
        &mut self,
        form_tab: FormTab,
        region: &str,
    ) {
        let port_color = self.theme().auth_port;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3Region);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputS3Region::new(region, form_tab, port_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_s3_endpoint(
        &mut self,
        form_tab: FormTab,
        endpoint: &str,
    ) {
        let username_color = self.theme().auth_username;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3Endpoint);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputS3Endpoint::new(
                endpoint,
                form_tab,
                username_color,
            )),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_s3_profile(
        &mut self,
        form_tab: FormTab,
        profile: &str,
    ) {
        let color = self.theme().auth_password;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3Profile);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputS3Profile::new(profile, form_tab, color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_s3_access_key(
        &mut self,
        form_tab: FormTab,
        key: &str,
    ) {
        let color = self.theme().auth_address;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3AccessKey);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputS3AccessKey::new(key, form_tab, color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_s3_secret_access_key(
        &mut self,
        form_tab: FormTab,
        key: &str,
    ) {
        let color = self.theme().auth_port;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3SecretAccessKey);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputS3SecretAccessKey::new(
                key, form_tab, color,
            )),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_s3_security_token(
        &mut self,
        form_tab: FormTab,
        token: &str,
    ) {
        let color = self.theme().auth_username;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3SecurityToken);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputS3SecurityToken::new(
                token, form_tab, color,
            )),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_s3_session_token(
        &mut self,
        form_tab: FormTab,
        token: &str,
    ) {
        let color = self.theme().auth_password;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3SessionToken);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputS3SessionToken::new(token, form_tab, color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_s3_new_path_style(
        &mut self,
        form_tab: FormTab,
        new_path_style: bool,
    ) {
        let color = self.theme().auth_address;
        let id = Self::form_tab_id(form_tab, AuthFormId::S3NewPathStyle);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::RadioS3NewPathStyle::new(
                new_path_style,
                form_tab,
                color,
            )),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_kube_namespace(
        &mut self,
        form_tab: FormTab,
        value: &str,
    ) {
        let color = self.theme().auth_port;
        let id = Self::form_tab_id(form_tab, AuthFormId::KubeNamespace);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputKubeNamespace::new(value, form_tab, color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_kube_cluster_url(
        &mut self,
        form_tab: FormTab,
        value: &str,
    ) {
        let color = self.theme().auth_username;
        let id = Self::form_tab_id(form_tab, AuthFormId::KubeClusterUrl);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputKubeClusterUrl::new(value, form_tab, color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_kube_username(
        &mut self,
        form_tab: FormTab,
        value: &str,
    ) {
        let color = self.theme().auth_password;
        let id = Self::form_tab_id(form_tab, AuthFormId::KubeUsername);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputKubeUsername::new(value, form_tab, color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_kube_client_cert(
        &mut self,
        form_tab: FormTab,
        value: &str,
    ) {
        let color = self.theme().auth_address;
        let id = Self::form_tab_id(form_tab, AuthFormId::KubeClientCert);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputKubeClientCert::new(value, form_tab, color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_kube_client_key(
        &mut self,
        form_tab: FormTab,
        value: &str,
    ) {
        let color = self.theme().auth_port;
        let id = Self::form_tab_id(form_tab, AuthFormId::KubeClientKey);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputKubeClientKey::new(value, form_tab, color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_smb_share(
        &mut self,
        form_tab: FormTab,
        share: &str,
    ) {
        let color = self.theme().auth_password;
        let id = Self::form_tab_id(form_tab, AuthFormId::SmbShare);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputSmbShare::new(share, form_tab, color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    #[cfg(posix)]
    pub(in crate::ui::activities::auth) fn mount_smb_workgroup(
        &mut self,
        form_tab: FormTab,
        workgroup: &str,
    ) {
        let color = self.theme().auth_address;
        let id = Self::form_tab_id(form_tab, AuthFormId::SmbWorkgroup);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputSmbWorkgroup::new(
                workgroup, form_tab, color,
            )),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn mount_webdav_uri(
        &mut self,
        form_tab: FormTab,
        uri: &str,
    ) {
        let addr_color = self.theme().auth_address;
        let id = Self::form_tab_id(form_tab, AuthFormId::WebDAVUri);
        if let Err(err) = self.app.remount(
            id,
            Box::new(components::InputWebDAVUri::new(uri, form_tab, addr_color)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn form_tab_id(form_tab: FormTab, id: AuthFormId) -> Id {
        match form_tab {
            FormTab::HostBridge => Id::HostBridge(id),
            FormTab::Remote => Id::Remote(id),
        }
    }
}
