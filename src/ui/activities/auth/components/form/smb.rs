#[cfg(posix)]
use tui_realm_stdlib::components::Span;
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::NoUserEvent;
#[cfg(posix)]
use tuirealm::props::SpanStatic;

use super::*;
#[cfg(posix)]
use crate::filetransfer::params::SmbDialect;

#[derive(Component)]
pub struct InputSmbShare {
    component: Input,
    form_tab: FormTab,
}

impl InputSmbShare {
    pub fn new(host: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .title(Title::from("Share").alignment(HorizontalAlignment::Left))
                .input_type(InputType::Text)
                .value(host),
            form_tab,
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputSmbShare {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::SmbShareBlurDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::SmbShareBlurDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::SmbShareBlurUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::SmbShareBlurUp)),
        };

        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

#[cfg(posix)]
#[derive(Component)]
pub struct InputSmbWorkgroup {
    component: Input,
    form_tab: FormTab,
}

#[cfg(posix)]
impl InputSmbWorkgroup {
    pub fn new(host: &str, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .title(Title::from("Workgroup").alignment(HorizontalAlignment::Left))
                .input_type(InputType::Text)
                .value(host),
            form_tab,
        }
    }
}

#[cfg(posix)]
impl AppComponent<Msg, NoUserEvent> for InputSmbWorkgroup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let on_key_down = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::SmbWorkgroupDown)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::SmbWorkgroupDown)),
        };
        let on_key_up = match self.form_tab {
            FormTab::Remote => Msg::Ui(UiMsg::Remote(UiAuthFormMsg::SmbWorkgroupUp)),
            FormTab::HostBridge => Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::SmbWorkgroupUp)),
        };

        let form_tab = self.form_tab;
        handle_input_ev(self, ev, on_key_down, on_key_up, form_tab)
    }
}

#[cfg(all(test, posix))]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::filetransfer::params::SmbDialect;

    #[test]
    fn should_map_radio_options_to_dialect() {
        assert_eq!(RadioSmbDialect::opt_to_dialect(0), SmbDialect::Auto);
        assert_eq!(RadioSmbDialect::opt_to_dialect(1), SmbDialect::Smb1);
        assert_eq!(RadioSmbDialect::opt_to_dialect(2), SmbDialect::Smb2);
        assert_eq!(RadioSmbDialect::opt_to_dialect(3), SmbDialect::Smb3);
        assert_eq!(RadioSmbDialect::opt_to_dialect(99), SmbDialect::Auto);
    }

    #[test]
    fn should_map_dialect_to_radio_options() {
        for dialect in [
            SmbDialect::Auto,
            SmbDialect::Smb1,
            SmbDialect::Smb2,
            SmbDialect::Smb3,
        ] {
            let opt = RadioSmbDialect::dialect_to_opt(dialect);
            assert_eq!(RadioSmbDialect::opt_to_dialect(opt), dialect);
        }
    }
}

#[cfg(posix)]
const RADIO_SMB_DIALECT_AUTO: usize = 0;
#[cfg(posix)]
const RADIO_SMB_DIALECT_SMB1: usize = 1;
#[cfg(posix)]
const RADIO_SMB_DIALECT_SMB2: usize = 2;
#[cfg(posix)]
const RADIO_SMB_DIALECT_SMB3: usize = 3;

/// Radio to select the SMB protocol family.
#[cfg(posix)]
#[derive(Component)]
pub struct RadioSmbDialect {
    component: Radio,
    form_tab: FormTab,
}

#[cfg(posix)]
impl RadioSmbDialect {
    pub fn new(dialect: SmbDialect, form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Radio::default()
                .highlight_style(
                    Style::default()
                        .fg(color)
                        .add_modifier(TextModifiers::REVERSED),
                )
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .choices(["Auto", "SMB1 (insecure)", "SMB2", "SMB3"])
                .rewind(true)
                .title(Title::from("SMB version").alignment(HorizontalAlignment::Left))
                .value(Self::dialect_to_opt(dialect)),
            form_tab,
        }
    }

    /// Converts the radio choice index to a dialect. Unknown indexes map to `Auto`.
    pub fn opt_to_dialect(opt: usize) -> SmbDialect {
        match opt {
            RADIO_SMB_DIALECT_SMB1 => SmbDialect::Smb1,
            RADIO_SMB_DIALECT_SMB2 => SmbDialect::Smb2,
            RADIO_SMB_DIALECT_SMB3 => SmbDialect::Smb3,
            _ => SmbDialect::Auto,
        }
    }

    fn dialect_to_opt(dialect: SmbDialect) -> usize {
        match dialect {
            SmbDialect::Auto => RADIO_SMB_DIALECT_AUTO,
            SmbDialect::Smb1 => RADIO_SMB_DIALECT_SMB1,
            SmbDialect::Smb2 => RADIO_SMB_DIALECT_SMB2,
            SmbDialect::Smb3 => RADIO_SMB_DIALECT_SMB3,
        }
    }
}

#[cfg(posix)]
impl AppComponent<Msg, NoUserEvent> for RadioSmbDialect {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Form(FormMsg::Connect)),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(if self.form_tab == FormTab::Remote {
                Msg::Ui(UiMsg::Remote(UiAuthFormMsg::SmbDialectBlurDown))
            } else {
                Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::SmbDialectBlurDown))
            }),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                Some(if self.form_tab == FormTab::Remote {
                    Msg::Ui(UiMsg::Remote(UiAuthFormMsg::SmbDialectBlurUp))
                } else {
                    Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::SmbDialectBlurUp))
                })
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                Some(if self.form_tab == FormTab::Remote {
                    Msg::Ui(UiMsg::Remote(UiAuthFormMsg::ParamsFormBlur))
                } else {
                    Msg::Ui(UiMsg::HostBridge(UiAuthFormMsg::ParamsFormBlur))
                })
            }
            _ => None,
        }
    }
}

/// One-line warning shown above the dialect radio while SMB1 is selected.
#[cfg(posix)]
#[derive(Component)]
pub struct SmbDialectWarning {
    component: Span,
}

#[cfg(posix)]
impl SmbDialectWarning {
    pub fn new(color: Color) -> Self {
        Self {
            component: Span::default().foreground(color).spans([SpanStatic::from(
                "Warning: SMB1 is deprecated and insecure. Use it only for isolated legacy devices.",
            )]),
        }
    }
}

#[cfg(posix)]
impl AppComponent<Msg, NoUserEvent> for SmbDialectWarning {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
