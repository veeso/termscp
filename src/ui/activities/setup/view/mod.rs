//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

pub mod setup;
pub mod ssh_keys;
pub mod theme;

use super::*;
use crate::utils::ui::draw_area_in;
pub use setup::*;
pub use ssh_keys::*;
pub use theme::*;

use tuirealm::tui::widgets::Clear;
use tuirealm::{
    event::{Key, KeyEvent, KeyModifiers},
    Frame, Sub, SubClause, SubEventClause,
};

impl SetupActivity {
    // -- view

    pub(super) fn init(&mut self, layout: ViewLayout) {
        self.layout = layout;
        match self.layout {
            ViewLayout::SetupForm => self.init_setup(),
            ViewLayout::SshKeys => self.init_ssh_keys(),
            ViewLayout::Theme => self.init_theme(),
        }
    }

    /// View gui
    pub(super) fn view(&mut self) {
        self.redraw = false;
        match self.layout {
            ViewLayout::SetupForm => self.view_setup(),
            ViewLayout::SshKeys => self.view_ssh_keys(),
            ViewLayout::Theme => self.view_theme(),
        }
    }

    // -- mount

    /// Mount error box
    pub(super) fn mount_error<S: AsRef<str>>(&mut self, text: S) {
        assert!(self
            .app
            .remount(
                Id::Common(IdCommon::ErrorPopup),
                Box::new(components::ErrorPopup::new(text)),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::Common(IdCommon::ErrorPopup)).is_ok());
    }

    /// Umount error message
    pub(super) fn umount_error(&mut self) {
        let _ = self.app.umount(&Id::Common(IdCommon::ErrorPopup));
    }

    /// Mount quit popup
    pub(super) fn mount_quit(&mut self) {
        assert!(self
            .app
            .remount(
                Id::Common(IdCommon::QuitPopup),
                Box::<components::QuitPopup>::default(),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::Common(IdCommon::QuitPopup)).is_ok());
    }

    /// Umount quit
    pub(super) fn umount_quit(&mut self) {
        let _ = self.app.umount(&Id::Common(IdCommon::QuitPopup));
    }

    /// Mount save popup
    pub(super) fn mount_save_popup(&mut self) {
        assert!(self
            .app
            .remount(
                Id::Common(IdCommon::SavePopup),
                Box::<components::SavePopup>::default(),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::Common(IdCommon::SavePopup)).is_ok());
    }

    /// Umount quit
    pub(super) fn umount_save_popup(&mut self) {
        let _ = self.app.umount(&Id::Common(IdCommon::SavePopup));
    }

    /// Mount help
    pub(super) fn mount_help(&mut self) {
        assert!(self
            .app
            .remount(
                Id::Common(IdCommon::Keybindings),
                Box::<components::Keybindings>::default(),
                vec![],
            )
            .is_ok());
        assert!(self.app.active(&Id::Common(IdCommon::Keybindings)).is_ok());
    }

    /// Umount help
    pub(super) fn umount_help(&mut self) {
        let _ = self.app.umount(&Id::Common(IdCommon::Keybindings));
    }

    pub(super) fn view_popups(&mut self, f: &mut Frame) {
        if self.app.mounted(&Id::Common(IdCommon::ErrorPopup)) {
            let popup = draw_area_in(f.size(), 50, 10);
            f.render_widget(Clear, popup);
            // make popup
            self.app.view(&Id::Common(IdCommon::ErrorPopup), f, popup);
        } else if self.app.mounted(&Id::Common(IdCommon::QuitPopup)) {
            // make popup
            let popup = draw_area_in(f.size(), 40, 10);
            f.render_widget(Clear, popup);
            self.app.view(&Id::Common(IdCommon::QuitPopup), f, popup);
        } else if self.app.mounted(&Id::Common(IdCommon::Keybindings)) {
            // make popup
            let popup = draw_area_in(f.size(), 50, 70);
            f.render_widget(Clear, popup);
            self.app.view(&Id::Common(IdCommon::Keybindings), f, popup);
        } else if self.app.mounted(&Id::Common(IdCommon::SavePopup)) {
            // make popup
            let popup = draw_area_in(f.size(), 30, 10);
            f.render_widget(Clear, popup);
            self.app.view(&Id::Common(IdCommon::SavePopup), f, popup);
        }
    }

    /// Clean app up and remount common components and global listener
    fn new_app(&mut self, layout: ViewLayout) {
        self.app.umount_all();
        self.mount_global_listener();
        self.mount_commons(layout);
    }

    /// Mount common components
    fn mount_commons(&mut self, layout: ViewLayout) {
        // Radio tab
        assert!(self
            .app
            .remount(
                Id::Common(IdCommon::Header),
                Box::new(components::Header::new(layout)),
                vec![],
            )
            .is_ok());
        // Footer
        assert!(self
            .app
            .remount(
                Id::Common(IdCommon::Footer),
                Box::<components::Footer>::default(),
                vec![],
            )
            .is_ok());
    }

    /// Mount global listener
    fn mount_global_listener(&mut self) {
        assert!(self
            .app
            .mount(
                Id::Common(IdCommon::GlobalListener),
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
                            code: Key::Tab,
                            modifiers: KeyModifiers::NONE,
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
                    Sub::new(
                        SubEventClause::Keyboard(KeyEvent {
                            code: Key::Function(4),
                            modifiers: KeyModifiers::NONE,
                        }),
                        Self::no_popup_mounted_clause(),
                    ),
                    Sub::new(SubEventClause::WindowResize, SubClause::Always)
                ]
            )
            .is_ok());
    }

    /// Returns a sub clause which requires that no popup is mounted in order to be satisfied
    fn no_popup_mounted_clause() -> SubClause<Id> {
        SubClause::And(
            Box::new(SubClause::Not(Box::new(SubClause::IsMounted(Id::Common(
                IdCommon::ErrorPopup,
            ))))),
            Box::new(SubClause::And(
                Box::new(SubClause::Not(Box::new(SubClause::IsMounted(Id::Common(
                    IdCommon::Keybindings,
                ))))),
                Box::new(SubClause::And(
                    Box::new(SubClause::Not(Box::new(SubClause::IsMounted(Id::Common(
                        IdCommon::QuitPopup,
                    ))))),
                    Box::new(SubClause::And(
                        Box::new(SubClause::Not(Box::new(SubClause::IsMounted(Id::Common(
                            IdCommon::SavePopup,
                        ))))),
                        Box::new(SubClause::And(
                            Box::new(SubClause::Not(Box::new(SubClause::IsMounted(Id::Ssh(
                                IdSsh::DelSshKeyPopup,
                            ))))),
                            Box::new(SubClause::Not(Box::new(SubClause::IsMounted(Id::Ssh(
                                IdSsh::SshHost,
                            ))))),
                        )),
                    )),
                )),
            )),
        )
    }
}
