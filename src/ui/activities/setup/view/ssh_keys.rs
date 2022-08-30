//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

// Locals
use super::{components, Context, Id, IdCommon, IdSsh, SetupActivity, ViewLayout};
use crate::utils::ui::draw_area_in;

// Ext
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::tui::widgets::Clear;

impl SetupActivity {
    // -- view

    /// Initialize ssh keys view
    pub(super) fn init_ssh_keys(&mut self) {
        // Init view (and mount commons)
        self.new_app(ViewLayout::SshKeys);
        // Load keys
        self.reload_ssh_keys();
    }

    pub(crate) fn view_ssh_keys(&mut self) {
        let mut ctx: Context = self.context.take().unwrap();
        let _ = ctx.terminal().raw_mut().draw(|f| {
            // Prepare main chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3), // Current tab
                        Constraint::Min(5),    // Main body
                        Constraint::Length(1), // Help footer
                    ]
                    .as_ref(),
                )
                .split(f.size());
            // Render common widget
            self.app.view(&Id::Common(IdCommon::Header), f, chunks[0]);
            self.app.view(&Id::Common(IdCommon::Footer), f, chunks[2]);
            self.app.view(&Id::Ssh(IdSsh::SshKeys), f, chunks[1]);
            // Popups
            self.view_popups(f);
            if self.app.mounted(&Id::Ssh(IdSsh::DelSshKeyPopup)) {
                let popup = draw_area_in(f.size(), 30, 10);
                f.render_widget(Clear, popup);
                self.app.view(&Id::Ssh(IdSsh::DelSshKeyPopup), f, popup);
            } else if self.app.mounted(&Id::Ssh(IdSsh::SshHost)) {
                let popup = draw_area_in(f.size(), 50, 20);
                f.render_widget(Clear, popup);
                let popup_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(3), // Host
                            Constraint::Length(3), // Username
                        ]
                        .as_ref(),
                    )
                    .split(popup);
                self.app.view(&Id::Ssh(IdSsh::SshHost), f, popup_chunks[0]);
                self.app
                    .view(&Id::Ssh(IdSsh::SshUsername), f, popup_chunks[1]);
            }
        });
        // Put context back to context
        self.context = Some(ctx);
    }

    // -- mount

    /// Mount delete ssh key component
    pub(crate) fn mount_del_ssh_key(&mut self) {
        assert!(self
            .app
            .remount(
                Id::Ssh(IdSsh::DelSshKeyPopup),
                Box::new(components::DelSshKeyPopup::default()),
                vec![]
            )
            .is_ok());
        assert!(self.app.active(&Id::Ssh(IdSsh::DelSshKeyPopup)).is_ok());
    }

    /// Umount delete ssh key
    pub(crate) fn umount_del_ssh_key(&mut self) {
        let _ = self.app.umount(&Id::Ssh(IdSsh::DelSshKeyPopup));
    }

    /// Mount new ssh key prompt
    pub(crate) fn mount_new_ssh_key(&mut self) {
        assert!(self
            .app
            .remount(
                Id::Ssh(IdSsh::SshHost),
                Box::new(components::SshHost::default()),
                vec![]
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::Ssh(IdSsh::SshUsername),
                Box::new(components::SshUsername::default()),
                vec![]
            )
            .is_ok());
        assert!(self.app.active(&Id::Ssh(IdSsh::SshHost)).is_ok());
    }

    /// Umount new ssh key prompt
    pub(crate) fn umount_new_ssh_key(&mut self) {
        let _ = self.app.umount(&Id::Ssh(IdSsh::SshUsername));
        let _ = self.app.umount(&Id::Ssh(IdSsh::SshHost));
    }

    /// Reload ssh keys
    pub(crate) fn reload_ssh_keys(&mut self) {
        let keys: Vec<String> = self
            .config()
            .iter_ssh_keys()
            .map(|x| {
                let (addr, username, _) = self.config().get_ssh_key(x).ok().unwrap().unwrap();
                format!("{} at {}", username, addr)
            })
            .collect();
        assert!(self
            .app
            .remount(
                Id::Ssh(IdSsh::SshKeys),
                Box::new(components::SshKeys::new(&keys)),
                vec![]
            )
            .is_ok());
        assert!(self.app.active(&Id::Ssh(IdSsh::SshKeys)).is_ok());
    }
}
