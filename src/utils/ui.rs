//! ## Utils
//!
//! `Utils` implements utilities functions to work with layouts

use tuirealm::tui::layout::{Constraint, Direction, Layout, Rect};

/// ### draw_area_in
///
/// Draw an area (WxH / 3) in the middle of the parent area
pub fn draw_area_in(parent: Rect, width: u16, height: u16) -> Rect {
    let new_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - height) / 2),
                Constraint::Percentage(height),
                Constraint::Percentage((100 - height) / 2),
            ]
            .as_ref(),
        )
        .split(parent);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - width) / 2),
                Constraint::Percentage(width),
                Constraint::Percentage((100 - width) / 2),
            ]
            .as_ref(),
        )
        .split(new_area[1])[1]
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_utils_ui_draw_area_in() {
        let area: Rect = Rect::new(0, 0, 1024, 512);
        let child: Rect = draw_area_in(area, 75, 30);
        assert_eq!(child.x, 43);
        assert_eq!(child.y, 63);
        assert_eq!(child.width, 271);
        assert_eq!(child.height, 54);
    }
}
