//! ## Utils
//!
//! `Utils` implements utilities functions to work with layouts

use tuirealm::ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Size type for UI renders
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Size {
    Percentage(u16),
    Unit(u16),
}

/// Ui popup dialog (w x h)
pub struct Popup(pub Size, pub Size);

impl Popup {
    /// Draw popup in provided area
    pub fn draw_in(&self, parent: Rect) -> Rect {
        let new_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(self.height(&parent).as_ref())
            .split(parent);
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(self.width(&parent).as_ref())
            .split(new_area[1])[1]
    }

    fn height(&self, parent: &Rect) -> [Constraint; 3] {
        Self::constraints(parent.height, self.1)
    }

    fn width(&self, parent: &Rect) -> [Constraint; 3] {
        Self::constraints(parent.width, self.0)
    }

    fn constraints(area_size: u16, popup_size: Size) -> [Constraint; 3] {
        match popup_size {
            Size::Percentage(popup_size) => [
                Constraint::Percentage((100 - popup_size) / 2),
                Constraint::Percentage(popup_size),
                Constraint::Percentage((100 - popup_size) / 2),
            ],
            Size::Unit(popup_size) => {
                let margin = (area_size - popup_size) / 2;
                [
                    Constraint::Length(margin),
                    Constraint::Length(popup_size),
                    Constraint::Length(margin),
                ]
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_utils_ui_draw_area_in() {
        let area: Rect = Rect::new(0, 0, 1024, 512);
        let child: Rect = Popup(Size::Percentage(75), Size::Percentage(30)).draw_in(area);
        assert_eq!(child.x, 43);
        assert_eq!(child.y, 63);
        assert_eq!(child.width, 272);
        assert_eq!(child.height, 55);
    }
}
