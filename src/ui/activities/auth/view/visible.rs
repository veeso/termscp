use tuirealm::{Sub, SubClause, SubEventClause};

use super::*;

impl AuthActivity {
    pub(in crate::ui::activities::auth) fn get_host_bridge_generic_params_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::HostBridge(AuthFormId::RemoteDirectory)) => [
                Id::HostBridge(AuthFormId::Port),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::HostBridge(AuthFormId::LocalDirectory)) => [
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::RemoteDirectory),
                Id::HostBridge(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::HostBridge(AuthFormId::Address),
                Id::HostBridge(AuthFormId::Port),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
            ],
        }
    }

    pub(in crate::ui::activities::auth) fn get_remote_generic_params_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::Remote(AuthFormId::RemoteDirectory)) => [
                Id::Remote(AuthFormId::Port),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::Remote(AuthFormId::LocalDirectory)) => [
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::RemoteDirectory),
                Id::Remote(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::Remote(AuthFormId::Address),
                Id::Remote(AuthFormId::Port),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
            ],
        }
    }

    pub(in crate::ui::activities::auth) fn get_host_bridge_localhost_view(&self) -> [Id; 1] {
        [Id::HostBridge(AuthFormId::LocalDirectory)]
    }

    pub(in crate::ui::activities::auth) fn get_host_bridge_s3_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::HostBridge(AuthFormId::S3AccessKey)) => [
                Id::HostBridge(AuthFormId::S3Region),
                Id::HostBridge(AuthFormId::S3Endpoint),
                Id::HostBridge(AuthFormId::S3Profile),
                Id::HostBridge(AuthFormId::S3AccessKey),
            ],
            Some(&Id::HostBridge(AuthFormId::S3SecretAccessKey)) => [
                Id::HostBridge(AuthFormId::S3Endpoint),
                Id::HostBridge(AuthFormId::S3Profile),
                Id::HostBridge(AuthFormId::S3AccessKey),
                Id::HostBridge(AuthFormId::S3SecretAccessKey),
            ],
            Some(&Id::HostBridge(AuthFormId::S3SecurityToken)) => [
                Id::HostBridge(AuthFormId::S3Profile),
                Id::HostBridge(AuthFormId::S3AccessKey),
                Id::HostBridge(AuthFormId::S3SecretAccessKey),
                Id::HostBridge(AuthFormId::S3SecurityToken),
            ],
            Some(&Id::HostBridge(AuthFormId::S3SessionToken)) => [
                Id::HostBridge(AuthFormId::S3AccessKey),
                Id::HostBridge(AuthFormId::S3SecretAccessKey),
                Id::HostBridge(AuthFormId::S3SecurityToken),
                Id::HostBridge(AuthFormId::S3SessionToken),
            ],
            Some(&Id::HostBridge(AuthFormId::S3NewPathStyle)) => [
                Id::HostBridge(AuthFormId::S3SecretAccessKey),
                Id::HostBridge(AuthFormId::S3SecurityToken),
                Id::HostBridge(AuthFormId::S3SessionToken),
                Id::HostBridge(AuthFormId::S3NewPathStyle),
            ],
            Some(&Id::HostBridge(AuthFormId::RemoteDirectory)) => [
                Id::HostBridge(AuthFormId::S3SecurityToken),
                Id::HostBridge(AuthFormId::S3SessionToken),
                Id::HostBridge(AuthFormId::S3NewPathStyle),
                Id::HostBridge(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::HostBridge(AuthFormId::LocalDirectory)) => [
                Id::HostBridge(AuthFormId::S3SessionToken),
                Id::HostBridge(AuthFormId::S3NewPathStyle),
                Id::HostBridge(AuthFormId::RemoteDirectory),
                Id::HostBridge(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::HostBridge(AuthFormId::S3Bucket),
                Id::HostBridge(AuthFormId::S3Region),
                Id::HostBridge(AuthFormId::S3Endpoint),
                Id::HostBridge(AuthFormId::S3Profile),
            ],
        }
    }

    pub(in crate::ui::activities::auth) fn get_remote_s3_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::Remote(AuthFormId::S3AccessKey)) => [
                Id::Remote(AuthFormId::S3Region),
                Id::Remote(AuthFormId::S3Endpoint),
                Id::Remote(AuthFormId::S3Profile),
                Id::Remote(AuthFormId::S3AccessKey),
            ],
            Some(&Id::Remote(AuthFormId::S3SecretAccessKey)) => [
                Id::Remote(AuthFormId::S3Endpoint),
                Id::Remote(AuthFormId::S3Profile),
                Id::Remote(AuthFormId::S3AccessKey),
                Id::Remote(AuthFormId::S3SecretAccessKey),
            ],
            Some(&Id::Remote(AuthFormId::S3SecurityToken)) => [
                Id::Remote(AuthFormId::S3Profile),
                Id::Remote(AuthFormId::S3AccessKey),
                Id::Remote(AuthFormId::S3SecretAccessKey),
                Id::Remote(AuthFormId::S3SecurityToken),
            ],
            Some(&Id::Remote(AuthFormId::S3SessionToken)) => [
                Id::Remote(AuthFormId::S3AccessKey),
                Id::Remote(AuthFormId::S3SecretAccessKey),
                Id::Remote(AuthFormId::S3SecurityToken),
                Id::Remote(AuthFormId::S3SessionToken),
            ],
            Some(&Id::Remote(AuthFormId::S3NewPathStyle)) => [
                Id::Remote(AuthFormId::S3SecretAccessKey),
                Id::Remote(AuthFormId::S3SecurityToken),
                Id::Remote(AuthFormId::S3SessionToken),
                Id::Remote(AuthFormId::S3NewPathStyle),
            ],
            Some(&Id::Remote(AuthFormId::RemoteDirectory)) => [
                Id::Remote(AuthFormId::S3SecurityToken),
                Id::Remote(AuthFormId::S3SessionToken),
                Id::Remote(AuthFormId::S3NewPathStyle),
                Id::Remote(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::Remote(AuthFormId::LocalDirectory)) => [
                Id::Remote(AuthFormId::S3SessionToken),
                Id::Remote(AuthFormId::S3NewPathStyle),
                Id::Remote(AuthFormId::RemoteDirectory),
                Id::Remote(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::Remote(AuthFormId::S3Bucket),
                Id::Remote(AuthFormId::S3Region),
                Id::Remote(AuthFormId::S3Endpoint),
                Id::Remote(AuthFormId::S3Profile),
            ],
        }
    }

    pub(in crate::ui::activities::auth) fn get_host_bridge_kube_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::HostBridge(AuthFormId::KubeClientCert)) => [
                Id::HostBridge(AuthFormId::KubeNamespace),
                Id::HostBridge(AuthFormId::KubeClusterUrl),
                Id::HostBridge(AuthFormId::KubeUsername),
                Id::HostBridge(AuthFormId::KubeClientCert),
            ],
            Some(&Id::HostBridge(AuthFormId::KubeClientKey)) => [
                Id::HostBridge(AuthFormId::KubeClusterUrl),
                Id::HostBridge(AuthFormId::KubeUsername),
                Id::HostBridge(AuthFormId::KubeClientCert),
                Id::HostBridge(AuthFormId::KubeClientKey),
            ],
            Some(&Id::HostBridge(AuthFormId::RemoteDirectory)) => [
                Id::HostBridge(AuthFormId::KubeUsername),
                Id::HostBridge(AuthFormId::KubeClientCert),
                Id::HostBridge(AuthFormId::KubeClientKey),
                Id::HostBridge(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::HostBridge(AuthFormId::LocalDirectory)) => [
                Id::HostBridge(AuthFormId::KubeClientCert),
                Id::HostBridge(AuthFormId::KubeClientKey),
                Id::HostBridge(AuthFormId::RemoteDirectory),
                Id::HostBridge(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::HostBridge(AuthFormId::KubeNamespace),
                Id::HostBridge(AuthFormId::KubeClusterUrl),
                Id::HostBridge(AuthFormId::KubeUsername),
                Id::HostBridge(AuthFormId::KubeClientCert),
            ],
        }
    }

    pub(in crate::ui::activities::auth) fn get_remote_kube_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::Remote(AuthFormId::KubeClientCert)) => [
                Id::Remote(AuthFormId::KubeNamespace),
                Id::Remote(AuthFormId::KubeClusterUrl),
                Id::Remote(AuthFormId::KubeUsername),
                Id::Remote(AuthFormId::KubeClientCert),
            ],
            Some(&Id::Remote(AuthFormId::KubeClientKey)) => [
                Id::Remote(AuthFormId::KubeClusterUrl),
                Id::Remote(AuthFormId::KubeUsername),
                Id::Remote(AuthFormId::KubeClientCert),
                Id::Remote(AuthFormId::KubeClientKey),
            ],
            Some(&Id::Remote(AuthFormId::RemoteDirectory)) => [
                Id::Remote(AuthFormId::KubeUsername),
                Id::Remote(AuthFormId::KubeClientCert),
                Id::Remote(AuthFormId::KubeClientKey),
                Id::Remote(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::Remote(AuthFormId::LocalDirectory)) => [
                Id::Remote(AuthFormId::KubeClientCert),
                Id::Remote(AuthFormId::KubeClientKey),
                Id::Remote(AuthFormId::RemoteDirectory),
                Id::Remote(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::Remote(AuthFormId::KubeNamespace),
                Id::Remote(AuthFormId::KubeClusterUrl),
                Id::Remote(AuthFormId::KubeUsername),
                Id::Remote(AuthFormId::KubeClientCert),
            ],
        }
    }

    #[cfg(posix)]
    pub(in crate::ui::activities::auth) fn get_host_bridge_smb_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(
                &Id::HostBridge(AuthFormId::Address)
                | &Id::HostBridge(AuthFormId::Port)
                | &Id::HostBridge(AuthFormId::SmbShare)
                | &Id::HostBridge(AuthFormId::Username),
            ) => [
                Id::HostBridge(AuthFormId::Address),
                Id::HostBridge(AuthFormId::Port),
                Id::HostBridge(AuthFormId::SmbShare),
                Id::HostBridge(AuthFormId::Username),
            ],
            Some(&Id::HostBridge(AuthFormId::Password)) => [
                Id::HostBridge(AuthFormId::Port),
                Id::HostBridge(AuthFormId::SmbShare),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
            ],
            Some(&Id::HostBridge(AuthFormId::SmbWorkgroup)) => [
                Id::HostBridge(AuthFormId::SmbShare),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::SmbWorkgroup),
            ],
            Some(&Id::HostBridge(AuthFormId::RemoteDirectory)) => [
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::SmbWorkgroup),
                Id::HostBridge(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::HostBridge(AuthFormId::LocalDirectory)) => [
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::SmbWorkgroup),
                Id::HostBridge(AuthFormId::RemoteDirectory),
                Id::HostBridge(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::HostBridge(AuthFormId::Address),
                Id::HostBridge(AuthFormId::Port),
                Id::HostBridge(AuthFormId::SmbShare),
                Id::HostBridge(AuthFormId::Username),
            ],
        }
    }

    #[cfg(posix)]
    pub(in crate::ui::activities::auth) fn get_remote_smb_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(
                &Id::Remote(AuthFormId::Address)
                | &Id::Remote(AuthFormId::Port)
                | &Id::Remote(AuthFormId::SmbShare)
                | &Id::Remote(AuthFormId::Username),
            ) => [
                Id::Remote(AuthFormId::Address),
                Id::Remote(AuthFormId::Port),
                Id::Remote(AuthFormId::SmbShare),
                Id::Remote(AuthFormId::Username),
            ],
            Some(&Id::Remote(AuthFormId::Password)) => [
                Id::Remote(AuthFormId::Port),
                Id::Remote(AuthFormId::SmbShare),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
            ],
            Some(&Id::Remote(AuthFormId::SmbWorkgroup)) => [
                Id::Remote(AuthFormId::SmbShare),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::SmbWorkgroup),
            ],
            Some(&Id::Remote(AuthFormId::RemoteDirectory)) => [
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::SmbWorkgroup),
                Id::Remote(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::Remote(AuthFormId::LocalDirectory)) => [
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::SmbWorkgroup),
                Id::Remote(AuthFormId::RemoteDirectory),
                Id::Remote(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::Remote(AuthFormId::Address),
                Id::Remote(AuthFormId::Port),
                Id::Remote(AuthFormId::SmbShare),
                Id::Remote(AuthFormId::Username),
            ],
        }
    }

    #[cfg(win)]
    pub(in crate::ui::activities::auth) fn get_host_bridge_smb_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(
                &Id::HostBridge(AuthFormId::Address)
                | &Id::HostBridge(AuthFormId::Password)
                | &Id::HostBridge(AuthFormId::SmbShare)
                | &Id::HostBridge(AuthFormId::Username),
            ) => [
                Id::HostBridge(AuthFormId::Address),
                Id::HostBridge(AuthFormId::SmbShare),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
            ],
            Some(&Id::HostBridge(AuthFormId::RemoteDirectory)) => [
                Id::HostBridge(AuthFormId::SmbShare),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::HostBridge(AuthFormId::LocalDirectory)) => [
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::RemoteDirectory),
                Id::HostBridge(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::HostBridge(AuthFormId::Address),
                Id::HostBridge(AuthFormId::SmbShare),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
            ],
        }
    }

    #[cfg(win)]
    pub(in crate::ui::activities::auth) fn get_remote_smb_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(
                &Id::Remote(AuthFormId::Address)
                | &Id::Remote(AuthFormId::Password)
                | &Id::Remote(AuthFormId::SmbShare)
                | &Id::Remote(AuthFormId::Username),
            ) => [
                Id::Remote(AuthFormId::Address),
                Id::Remote(AuthFormId::SmbShare),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
            ],
            Some(&Id::Remote(AuthFormId::RemoteDirectory)) => [
                Id::Remote(AuthFormId::SmbShare),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::RemoteDirectory),
            ],
            Some(&Id::Remote(AuthFormId::LocalDirectory)) => [
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::RemoteDirectory),
                Id::Remote(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::Remote(AuthFormId::Address),
                Id::Remote(AuthFormId::SmbShare),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
            ],
        }
    }

    pub(in crate::ui::activities::auth) fn get_host_bridge_webdav_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::HostBridge(AuthFormId::LocalDirectory)) => [
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::RemoteDirectory),
                Id::HostBridge(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::HostBridge(AuthFormId::WebDAVUri),
                Id::HostBridge(AuthFormId::Username),
                Id::HostBridge(AuthFormId::Password),
                Id::HostBridge(AuthFormId::RemoteDirectory),
            ],
        }
    }

    pub(in crate::ui::activities::auth) fn get_remote_webdav_view(&self) -> [Id; 4] {
        match self.app.focus() {
            Some(&Id::Remote(AuthFormId::LocalDirectory)) => [
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::RemoteDirectory),
                Id::Remote(AuthFormId::LocalDirectory),
            ],
            _ => [
                Id::Remote(AuthFormId::WebDAVUri),
                Id::Remote(AuthFormId::Username),
                Id::Remote(AuthFormId::Password),
                Id::Remote(AuthFormId::RemoteDirectory),
            ],
        }
    }

    pub(in crate::ui::activities::auth) fn init_global_listener(&mut self) {
        use tuirealm::event::{Key, KeyEvent, KeyModifiers};

        if let Err(err) = self.app.mount(
            Id::GlobalListener,
            Box::<components::GlobalListener>::default(),
            vec![
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Esc,
                        modifiers: KeyModifiers::NONE,
                    }),
                    Self::no_popup_mounted_clause(),
                ),
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Function(10),
                        modifiers: KeyModifiers::NONE,
                    }),
                    Self::no_popup_mounted_clause(),
                ),
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                    }),
                    Self::no_popup_mounted_clause(),
                ),
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Char('h'),
                        modifiers: KeyModifiers::CONTROL,
                    }),
                    Self::no_popup_mounted_clause(),
                ),
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Function(1),
                        modifiers: KeyModifiers::NONE,
                    }),
                    Self::no_popup_mounted_clause(),
                ),
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Char('r'),
                        modifiers: KeyModifiers::CONTROL,
                    }),
                    Self::no_popup_mounted_clause(),
                ),
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Char('s'),
                        modifiers: KeyModifiers::CONTROL,
                    }),
                    Self::no_popup_mounted_clause(),
                ),
                Sub::new(SubEventClause::WindowResize, SubClause::Always),
            ],
        ) {
            error!("Failed to mount component: {err}");
        }
    }

    pub(in crate::ui::activities::auth) fn get_current_form_tab(&self) -> FormTab {
        match self.app.focus() {
            Some(&Id::HostBridge(_)) => FormTab::HostBridge,
            _ => FormTab::Remote,
        }
    }

    fn no_popup_mounted_clause() -> SubClause<Id> {
        tuirealm::subclause_and_not!(
            Id::ErrorPopup,
            Id::InfoPopup,
            Id::Keybindings,
            Id::DeleteBookmarkPopup,
            Id::DeleteRecentPopup,
            Id::InstallUpdatePopup,
            Id::BookmarkSavePassword,
            Id::WaitPopup
        )
    }
}
