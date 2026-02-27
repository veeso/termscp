use tui_realm_stdlib::Span;
use tuirealm::props::{Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

use crate::explorer::FileSorting;
use crate::ui::activities::filetransfer::Msg;
use crate::ui::activities::filetransfer::lib::browser::Browser;

#[derive(MockComponent)]
pub struct StatusBarLocal {
    component: Span,
}

impl StatusBarLocal {
    pub fn new(browser: &Browser, sorting_color: Color, hidden_color: Color) -> Self {
        let file_sorting = file_sorting_label(browser.host_bridge().file_sorting);
        let hidden_files = hidden_files_label(browser.host_bridge().hidden_files_visible());
        Self {
            component: Span::default().spans([
                TextSpan::new("File sorting: ").fg(sorting_color),
                TextSpan::new(file_sorting).fg(sorting_color).reversed(),
                TextSpan::new(" Hidden files: ").fg(hidden_color),
                TextSpan::new(hidden_files).fg(hidden_color).reversed(),
            ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for StatusBarLocal {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct StatusBarRemote {
    component: Span,
}

impl StatusBarRemote {
    pub fn new(
        browser: &Browser,
        sorting_color: Color,
        hidden_color: Color,
        sync_color: Color,
    ) -> Self {
        let file_sorting = file_sorting_label(browser.remote().file_sorting);
        let hidden_files = hidden_files_label(browser.remote().hidden_files_visible());
        let sync_browsing = match browser.sync_browsing {
            true => "ON ",
            false => "OFF",
        };
        Self {
            component: Span::default().spans([
                TextSpan::new("File sorting: ").fg(sorting_color),
                TextSpan::new(file_sorting).fg(sorting_color).reversed(),
                TextSpan::new(" Hidden files: ").fg(hidden_color),
                TextSpan::new(hidden_files).fg(hidden_color).reversed(),
                TextSpan::new(" Sync browsing: ").fg(sync_color),
                TextSpan::new(sync_browsing).fg(sync_color).reversed(),
            ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for StatusBarRemote {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

fn file_sorting_label(sorting: FileSorting) -> &'static str {
    match sorting {
        FileSorting::CreationTime => "By creation time",
        FileSorting::ModifyTime => "By modify time",
        FileSorting::Name => "By name",
        FileSorting::Size => "By size",
        FileSorting::None => "",
    }
}

fn hidden_files_label(visible: bool) -> &'static str {
    match visible {
        true => "Show",
        false => "Hide",
    }
}
