use crate::entities::pawn::PawnColor;
use ratatui::{prelude::*, widgets::*};

/// A custom widget that renders a button with a label, theme and state.
#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub color: PawnColor,
    pub width: usize,
    pub height: usize,
}

/// A button with a label that can be themed.
impl Circle {
    pub fn new(color: PawnColor, width: usize, height: usize) -> Circle {
        Circle {
            color,
            width,
            height,
        }
    }
}

impl Widget for Circle {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let background = Color::Rgb(0, 0, 0);
        let border = Color::LightRed;

        if area.height > 2 {
            // Left and right borders, corners, and label
            for y in (area.y + 1)..(area.y + area.height - 1) {
                buf.set_string(area.x, y, "│", Style::new().fg(border).bg(background));

                buf.set_string(
                    area.right() - 1,
                    y,
                    "│",
                    Style::new().fg(border).bg(background),
                );
            }

            // Bottom line
            buf.set_string(
                area.x,
                area.bottom() - 1,
                "╰",
                Style::new().fg(border).bg(background),
            );

            buf.set_string(
                area.x + 1,
                area.bottom() - 1,
                "─".repeat((area.width - 2) as usize),
                Style::new().fg(border).bg(background),
            );

            buf.set_string(
                area.right() - 1,
                area.bottom() - 1,
                "╯",
                Style::new().fg(border).bg(background),
            );
        }
    }
}
