//! ## Update
//!
//! Update impl

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
use super::{AuthActivity, ExitReason, Id, InputMask, Msg, Update};

use tuirealm::{State, StateValue};

impl Update<Msg> for AuthActivity {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.redraw = true;
        match msg.unwrap_or(Msg::None) {
            Msg::AddressBlurDown => {
                assert!(self.app.active(&Id::Port).is_ok());
            }
            Msg::AddressBlurUp => {
                assert!(self.app.active(&Id::Protocol).is_ok());
            }
            Msg::BookmarksListBlur => {
                assert!(self.app.active(&Id::RecentsList).is_ok());
            }
            Msg::BookmarkNameBlur => {
                assert!(self.app.active(&Id::BookmarkSavePassword).is_ok());
            }
            Msg::BookmarksTabBlur => {
                assert!(self.app.active(&Id::Protocol).is_ok());
            }
            Msg::CloseDeleteBookmark => {
                assert!(self.app.umount(&Id::DeleteBookmarkPopup).is_ok());
            }
            Msg::CloseDeleteRecent => {
                assert!(self.app.umount(&Id::DeleteRecentPopup).is_ok());
            }
            Msg::CloseErrorPopup => {
                self.umount_error();
            }
            Msg::CloseInfoPopup => {
                self.umount_info();
            }
            Msg::CloseInstallUpdatePopup => {
                assert!(self.app.umount(&Id::NewVersionChangelog).is_ok());
                assert!(self.app.umount(&Id::InstallUpdatePopup).is_ok());
            }
            Msg::CloseKeybindingsPopup => {
                self.umount_help();
            }
            Msg::CloseQuitPopup => self.umount_quit(),
            Msg::CloseSaveBookmark => {
                assert!(self.app.umount(&Id::BookmarkName).is_ok());
                assert!(self.app.umount(&Id::BookmarkSavePassword).is_ok());
            }
            Msg::Connect => {
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
            Msg::DeleteBookmark => {
                if let Ok(State::One(StateValue::Usize(idx))) = self.app.state(&Id::BookmarksList) {
                    // Umount dialog
                    self.umount_bookmark_del_dialog();
                    // Delete bookmark
                    self.del_bookmark(idx);
                    // Update bookmarks
                    self.view_bookmarks()
                }
            }
            Msg::DeleteRecent => {
                if let Ok(State::One(StateValue::Usize(idx))) = self.app.state(&Id::RecentsList) {
                    // Umount dialog
                    self.umount_recent_del_dialog();
                    // Delete recent
                    self.del_recent(idx);
                    // Update recents
                    self.view_recent_connections();
                }
            }
            Msg::EnterSetup => {
                self.exit_reason = Some(ExitReason::EnterSetup);
            }
            Msg::InstallUpdate => {
                self.install_update();
            }
            Msg::LoadBookmark(i) => {
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
            Msg::LoadRecent(i) => {
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
            Msg::ParamsFormBlur => {
                assert!(self.app.active(&Id::BookmarksList).is_ok());
            }
            Msg::PasswordBlurDown => {
                assert!(self.app.active(&Id::Protocol).is_ok());
            }
            Msg::PasswordBlurUp => {
                assert!(self.app.active(&Id::Username).is_ok());
            }
            Msg::PortBlurDown => {
                assert!(self.app.active(&Id::Username).is_ok());
            }
            Msg::PortBlurUp => {
                assert!(self.app.active(&Id::Address).is_ok());
            }
            Msg::ProtocolBlurDown => {
                assert!(self
                    .app
                    .active(match self.input_mask() {
                        InputMask::Generic => &Id::Address,
                        InputMask::AwsS3 => &Id::S3Bucket,
                    })
                    .is_ok());
            }
            Msg::ProtocolBlurUp => {
                assert!(self
                    .app
                    .active(match self.input_mask() {
                        InputMask::Generic => &Id::Password,
                        InputMask::AwsS3 => &Id::S3Profile,
                    })
                    .is_ok());
            }
            Msg::ProtocolChanged(protocol) => {
                self.protocol = protocol;
                // Update port
                let port: u16 = self.get_input_port();
                if Self::is_port_standard(port) {
                    self.mount_port(Self::get_default_port_for_protocol(protocol));
                }
            }
            Msg::Quit => {
                self.exit_reason = Some(ExitReason::Quit);
            }
            Msg::RececentsListBlur => {
                assert!(self.app.active(&Id::BookmarksList).is_ok());
            }
            Msg::S3BucketBlurDown => {
                assert!(self.app.active(&Id::S3Region).is_ok());
            }
            Msg::S3BucketBlurUp => {
                assert!(self.app.active(&Id::Protocol).is_ok());
            }
            Msg::S3RegionBlurDown => {
                assert!(self.app.active(&Id::S3Profile).is_ok());
            }
            Msg::S3RegionBlurUp => {
                assert!(self.app.active(&Id::S3Bucket).is_ok());
            }
            Msg::S3ProfileBlurDown => {
                assert!(self.app.active(&Id::Protocol).is_ok());
            }
            Msg::S3ProfileBlurUp => {
                assert!(self.app.active(&Id::S3Region).is_ok());
            }
            Msg::SaveBookmark => {
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
            Msg::SaveBookmarkPasswordBlur => {
                assert!(self.app.active(&Id::BookmarkName).is_ok());
            }
            Msg::ShowDeleteBookmarkPopup => {
                self.mount_bookmark_del_dialog();
            }
            Msg::ShowDeleteRecentPopup => {
                self.mount_recent_del_dialog();
            }
            Msg::ShowKeybindingsPopup => {
                self.mount_keybindings();
            }
            Msg::ShowQuitPopup => {
                self.mount_quit();
            }
            Msg::ShowReleaseNotes => {
                self.mount_release_notes();
            }
            Msg::ShowSaveBookmarkPopup => {
                self.mount_bookmark_save_dialog();
            }
            Msg::UsernameBlurDown => {
                assert!(self.app.active(&Id::Password).is_ok());
            }
            Msg::UsernameBlurUp => {
                assert!(self.app.active(&Id::Port).is_ok());
            }
            Msg::None => {}
        }
        None
    }
}
