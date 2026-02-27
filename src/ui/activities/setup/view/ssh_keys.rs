//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

// Locals
// Ext
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::ratatui::widgets::Clear;

use super::{Context, Id, IdCommon, IdSsh, SetupActivity, ViewLayout, components};
use crate::utils::ui::{Popup, Size};

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
                .split(f.area());
            // Render common widget
            self.app.view(&Id::Common(IdCommon::Header), f, chunks[0]);
            self.app.view(&Id::Common(IdCommon::Footer), f, chunks[2]);
            self.app.view(&Id::Ssh(IdSsh::SshKeys), f, chunks[1]);
            // Popups
            self.view_popups(f);
            if self.app.mounted(&Id::Ssh(IdSsh::DelSshKeyPopup)) {
                let popup = Popup(Size::Percentage(30), Size::Unit(3)).draw_in(f.area());
                f.render_widget(Clear, popup);
                self.app.view(&Id::Ssh(IdSsh::DelSshKeyPopup), f, popup);
            } else if self.app.mounted(&Id::Ssh(IdSsh::SshHost)) {
                let popup = Popup(Size::Percentage(50), Size::Percentage(20)).draw_in(f.area());
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
        if let Err(err) = self.app.remount(
            Id::Ssh(IdSsh::DelSshKeyPopup),
            Box::<components::DelSshKeyPopup>::default(),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
        if let Err(err) = self.app.active(&Id::Ssh(IdSsh::DelSshKeyPopup)) {
            error!("Failed to activate component: {err}");
        }
    }

    /// Umount delete ssh key
    pub(crate) fn umount_del_ssh_key(&mut self) {
        let _ = self.app.umount(&Id::Ssh(IdSsh::DelSshKeyPopup));
    }

    /// Mount new ssh key prompt
    pub(crate) fn mount_new_ssh_key(&mut self) {
        if let Err(err) = self.app.remount(
            Id::Ssh(IdSsh::SshHost),
            Box::<components::SshHost>::default(),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
        if let Err(err) = self.app.remount(
            Id::Ssh(IdSsh::SshUsername),
            Box::<components::SshUsername>::default(),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
        if let Err(err) = self.app.active(&Id::Ssh(IdSsh::SshHost)) {
            error!("Failed to activate component: {err}");
        }
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
                let (addr, username, _) = self.config().get_ssh_key(x).unwrap();
                format!("{username} at {addr}")
            })
            .collect();
        if let Err(err) = self.app.remount(
            Id::Ssh(IdSsh::SshKeys),
            Box::new(components::SshKeys::new(&keys)),
            vec![],
        ) {
            error!("Failed to remount component: {err}");
        }
        if let Err(err) = self.app.active(&Id::Ssh(IdSsh::SshKeys)) {
            error!("Failed to activate component: {err}");
        }
    }
}
