//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

// Locals
// Ext
use std::path::PathBuf;

use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::{State, StateValue};

use super::{components, Context, Id, IdCommon, IdConfig, SetupActivity, ViewLayout};
use crate::explorer::GroupDirs;
use crate::filetransfer::FileTransferProtocol;
use crate::utils::fmt::fmt_bytes;

impl SetupActivity {
    // -- view

    /// Initialize setup view
    pub(super) fn init_setup(&mut self) {
        // Init view (and mount commons)
        self.new_app(ViewLayout::SetupForm);
        // Load values
        self.load_input_values();
        // Active text editor
        assert!(self.app.active(&Id::Config(IdConfig::TextEditor)).is_ok());
    }

    pub(super) fn view_setup(&mut self) {
        let mut ctx: Context = self.context.take().unwrap();
        let _ = ctx.terminal().raw_mut().draw(|f| {
            // Prepare main chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3), // Current tab
                        Constraint::Min(18),   // Main body
                        Constraint::Length(1), // Help footer
                    ]
                    .as_ref(),
                )
                .split(f.size());
            // Render common widget
            self.app.view(&Id::Common(IdCommon::Header), f, chunks[0]);
            self.app.view(&Id::Common(IdCommon::Footer), f, chunks[2]);
            // Make chunks (two columns)
            let ui_cfg_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);
            // Column 1
            let ui_cfg_chunks_col1 = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3), // Text editor
                        Constraint::Length(3), // Protocol tab
                        Constraint::Length(3), // Hidden files
                        Constraint::Length(3), // Updates tab
                        Constraint::Length(3), // Prompt file replace
                        Constraint::Length(3), // Group dirs
                        Constraint::Length(1), // Prevent overflow
                    ]
                    .as_ref(),
                )
                .split(ui_cfg_chunks[0]);
            self.app
                .view(&Id::Config(IdConfig::TextEditor), f, ui_cfg_chunks_col1[0]);
            self.app.view(
                &Id::Config(IdConfig::DefaultProtocol),
                f,
                ui_cfg_chunks_col1[1],
            );
            self.app
                .view(&Id::Config(IdConfig::HiddenFiles), f, ui_cfg_chunks_col1[2]);
            self.app.view(
                &Id::Config(IdConfig::CheckUpdates),
                f,
                ui_cfg_chunks_col1[3],
            );
            self.app.view(
                &Id::Config(IdConfig::PromptOnFileReplace),
                f,
                ui_cfg_chunks_col1[4],
            );
            self.app
                .view(&Id::Config(IdConfig::GroupDirs), f, ui_cfg_chunks_col1[5]);
            // Column 2
            let ui_cfg_chunks_col2 = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3), // Local Format input
                        Constraint::Length(3), // Remote Format input
                        Constraint::Length(3), // Notifications enabled
                        Constraint::Length(3), // Notifications threshold
                        Constraint::Length(3), // Ssh config
                        Constraint::Length(1), // Prevent overflow
                    ]
                    .as_ref(),
                )
                .split(ui_cfg_chunks[1]);
            self.app.view(
                &Id::Config(IdConfig::LocalFileFmt),
                f,
                ui_cfg_chunks_col2[0],
            );
            self.app.view(
                &Id::Config(IdConfig::RemoteFileFmt),
                f,
                ui_cfg_chunks_col2[1],
            );
            self.app.view(
                &Id::Config(IdConfig::NotificationsEnabled),
                f,
                ui_cfg_chunks_col2[2],
            );
            self.app.view(
                &Id::Config(IdConfig::NotificationsThreshold),
                f,
                ui_cfg_chunks_col2[3],
            );
            self.app
                .view(&Id::Config(IdConfig::SshConfig), f, ui_cfg_chunks_col2[4]);
            // Popups
            self.view_popups(f);
        });
        // Put context back to context
        self.context = Some(ctx);
    }

    /// Load values from configuration into input fields
    pub(crate) fn load_input_values(&mut self) {
        // Text editor
        let text_editor: String =
            String::from(self.config().get_text_editor().as_path().to_string_lossy());
        assert!(self
            .app
            .remount(
                Id::Config(IdConfig::TextEditor),
                Box::new(components::TextEditor::new(text_editor.as_str())),
                vec![]
            )
            .is_ok());
        // Protocol
        assert!(self
            .app
            .remount(
                Id::Config(IdConfig::DefaultProtocol),
                Box::new(components::DefaultProtocol::new(
                    self.config().get_default_protocol()
                )),
                vec![]
            )
            .is_ok());
        // Hidden files
        assert!(self
            .app
            .remount(
                Id::Config(IdConfig::HiddenFiles),
                Box::new(components::HiddenFiles::new(
                    self.config().get_show_hidden_files()
                )),
                vec![]
            )
            .is_ok());
        // Updates
        assert!(self
            .app
            .remount(
                Id::Config(IdConfig::CheckUpdates),
                Box::new(components::CheckUpdates::new(
                    self.config().get_check_for_updates()
                )),
                vec![]
            )
            .is_ok());
        // File replace
        assert!(self
            .app
            .remount(
                Id::Config(IdConfig::PromptOnFileReplace),
                Box::new(components::PromptOnFileReplace::new(
                    self.config().get_prompt_on_file_replace()
                )),
                vec![]
            )
            .is_ok());
        // Group dirs
        assert!(self
            .app
            .remount(
                Id::Config(IdConfig::GroupDirs),
                Box::new(components::GroupDirs::new(self.config().get_group_dirs())),
                vec![]
            )
            .is_ok());
        // Local File Fmt
        assert!(self
            .app
            .remount(
                Id::Config(IdConfig::LocalFileFmt),
                Box::new(components::LocalFileFmt::new(
                    &self.config().get_local_file_fmt().unwrap_or_default()
                )),
                vec![]
            )
            .is_ok());
        // Remote File Fmt
        assert!(self
            .app
            .remount(
                Id::Config(IdConfig::RemoteFileFmt),
                Box::new(components::RemoteFileFmt::new(
                    &self.config().get_remote_file_fmt().unwrap_or_default()
                )),
                vec![]
            )
            .is_ok());
        // Notifications enabled
        assert!(self
            .app
            .remount(
                Id::Config(IdConfig::NotificationsEnabled),
                Box::new(components::NotificationsEnabled::new(
                    self.config().get_notifications()
                )),
                vec![]
            )
            .is_ok());
        // Notifications threshold
        assert!(self
            .app
            .remount(
                Id::Config(IdConfig::NotificationsThreshold),
                Box::new(components::NotificationsThreshold::new(&fmt_bytes(
                    self.config().get_notification_threshold()
                ))),
                vec![]
            )
            .is_ok());
        // Ssh config
        assert!(self
            .app
            .remount(
                Id::Config(IdConfig::SshConfig),
                Box::new(components::SshConfig::new(
                    self.config().get_ssh_config().unwrap_or("")
                )),
                vec![]
            )
            .is_ok());
    }

    /// Collect values from input and put them into the configuration
    pub(crate) fn collect_input_values(&mut self) {
        if let Ok(State::One(StateValue::String(editor))) =
            self.app.state(&Id::Config(IdConfig::TextEditor))
        {
            self.config_mut()
                .set_text_editor(PathBuf::from(editor.as_str()));
        }
        if let Ok(State::One(StateValue::Usize(protocol))) =
            self.app.state(&Id::Config(IdConfig::DefaultProtocol))
        {
            let protocol: FileTransferProtocol = match protocol {
                1 => FileTransferProtocol::Scp,
                2 => FileTransferProtocol::Ftp(false),
                3 => FileTransferProtocol::Ftp(true),
                4 => FileTransferProtocol::AwsS3,
                _ => FileTransferProtocol::Sftp,
            };
            self.config_mut().set_default_protocol(protocol);
        }
        if let Ok(State::One(StateValue::Usize(opt))) =
            self.app.state(&Id::Config(IdConfig::HiddenFiles))
        {
            let show: bool = matches!(opt, 0);
            self.config_mut().set_show_hidden_files(show);
        }
        if let Ok(State::One(StateValue::Usize(opt))) =
            self.app.state(&Id::Config(IdConfig::CheckUpdates))
        {
            let check: bool = matches!(opt, 0);
            self.config_mut().set_check_for_updates(check);
        }
        if let Ok(State::One(StateValue::Usize(opt))) =
            self.app.state(&Id::Config(IdConfig::PromptOnFileReplace))
        {
            let check: bool = matches!(opt, 0);
            self.config_mut().set_prompt_on_file_replace(check);
        }
        if let Ok(State::One(StateValue::String(fmt))) =
            self.app.state(&Id::Config(IdConfig::LocalFileFmt))
        {
            self.config_mut().set_local_file_fmt(fmt);
        }
        if let Ok(State::One(StateValue::String(fmt))) =
            self.app.state(&Id::Config(IdConfig::RemoteFileFmt))
        {
            self.config_mut().set_remote_file_fmt(fmt);
        }
        if let Ok(State::One(StateValue::Usize(opt))) =
            self.app.state(&Id::Config(IdConfig::GroupDirs))
        {
            let dirs: Option<GroupDirs> = match opt {
                0 => Some(GroupDirs::First),
                1 => Some(GroupDirs::Last),
                _ => None,
            };
            self.config_mut().set_group_dirs(dirs);
        }
        if let Ok(State::One(StateValue::Usize(opt))) =
            self.app.state(&Id::Config(IdConfig::NotificationsEnabled))
        {
            self.config_mut().set_notifications(opt == 0);
        }
        if let Ok(State::One(StateValue::U64(bytes))) = self
            .app
            .state(&Id::Config(IdConfig::NotificationsThreshold))
        {
            self.config_mut().set_notification_threshold(bytes);
        }
        if let Ok(State::One(StateValue::String(mut path))) =
            self.app.state(&Id::Config(IdConfig::SshConfig))
        {
            if path.is_empty() {
                self.config_mut().set_ssh_config(None);
            } else {
                // Replace '~' with home path
                if path.starts_with('~') {
                    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/root"));
                    path = path.replacen('~', &home_dir.to_string_lossy(), 1);
                }
                self.config_mut().set_ssh_config(Some(path));
            }
        }
    }
}
