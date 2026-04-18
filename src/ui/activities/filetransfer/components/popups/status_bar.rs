use tui_realm_stdlib::components::Span;
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, NoUserEvent};
use tuirealm::props::{Color, SpanStatic};
use tuirealm::ratatui::style::Stylize;

use crate::explorer::FileSorting;
use crate::ui::activities::filetransfer::Msg;
use crate::ui::activities::filetransfer::lib::browser::Browser;

#[derive(Component)]
pub struct StatusBarLocal {
    component: Span,
}

impl StatusBarLocal {
    pub fn new(browser: &Browser, sorting_color: Color, hidden_color: Color) -> Self {
        let file_sorting = file_sorting_label(browser.host_bridge().file_sorting);
        let hidden_files = hidden_files_label(browser.host_bridge().hidden_files_visible());
        Self {
            component: Span::default().spans([
                SpanStatic::raw("File sorting: ").fg(sorting_color),
                SpanStatic::raw(file_sorting).fg(sorting_color).reversed(),
                SpanStatic::raw(" Hidden files: ").fg(hidden_color),
                SpanStatic::raw(hidden_files).fg(hidden_color).reversed(),
            ]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for StatusBarLocal {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(Component)]
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
                SpanStatic::raw("File sorting: ").fg(sorting_color),
                SpanStatic::raw(file_sorting).fg(sorting_color).reversed(),
                SpanStatic::raw(" Hidden files: ").fg(hidden_color),
                SpanStatic::raw(hidden_files).fg(hidden_color).reversed(),
                SpanStatic::raw(" Sync browsing: ").fg(sync_color),
                SpanStatic::raw(sync_browsing).fg(sync_color).reversed(),
            ]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for StatusBarRemote {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
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
