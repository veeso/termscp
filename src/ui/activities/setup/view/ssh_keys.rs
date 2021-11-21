//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// Locals
use super::{components, Context, Id, IdCommon, IdSsh, SetupActivity, ViewLayout};
use crate::utils::ui::draw_area_in;

// Ext
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::tui::widgets::Clear;

impl SetupActivity {
    // -- view

    /// ### init_ssh_keys
    ///
    /// Initialize ssh keys view
    pub(super) fn init_ssh_keys(&mut self) {
        // Init view (and mount commons)
        self.new_app(ViewLayout::SshKeys);
        // Load keys
        self.reload_ssh_keys();
        // Give focus
        assert!(self.app.active(&Id::Ssh(IdSsh::SshKeys)).is_ok());
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
                        Constraint::Length(3),      // Current tab
                        Constraint::Percentage(90), // Main body
                        Constraint::Length(3),      // Help footer
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

    /// ### mount_del_ssh_key
    ///
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

    /// ### umount_del_ssh_key
    ///
    /// Umount delete ssh key
    pub(crate) fn umount_del_ssh_key(&mut self) {
        let _ = self.app.umount(&Id::Ssh(IdSsh::DelSshKeyPopup));
    }

    /// ### mount_new_ssh_key
    ///
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

    /// ### umount_new_ssh_key
    ///
    /// Umount new ssh key prompt
    pub(crate) fn umount_new_ssh_key(&mut self) {
        let _ = self.app.umount(&Id::Ssh(IdSsh::SshUsername));
        let _ = self.app.umount(&Id::Ssh(IdSsh::SshHost));
    }

    /// ### reload_ssh_keys
    ///
    /// Reload ssh keys
    pub(crate) fn reload_ssh_keys(&mut self) {
        let keys: Vec<String> = self
            .config()
            .iter_ssh_keys()
            .map(|x| {
                let (addr, username, _) = self.config().get_ssh_key(x).ok().unwrap().unwrap();
                format!("{} at {}", addr, username)
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
    }
}
