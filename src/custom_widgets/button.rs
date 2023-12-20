use core::fmt;
use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Normal,
    Selected,
    Active,
}

impl fmt::Display for ButtonState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    text: Color,
    background: Color,
    border_selected: Color,
    border_normal: Color,
    border_active: Color,
}

pub const RED: Theme = Theme {
    text: Color::Rgb(48, 16, 16),
    background: Color::Rgb(0, 0, 0),
    border_selected: Color::Rgb(220, 70, 70),
    border_active: Color::Rgb(220, 20, 20),
    border_normal: Color::Rgb(120, 20, 20),
};

pub const GREEN: Theme = Theme {
    text: Color::Rgb(16, 48, 16),
    background: Color::Rgb(0, 0, 0),
    border_selected: Color::Rgb(70, 220, 70),
    border_active: Color::Rgb(20, 220, 20),
    border_normal: Color::Rgb(20, 120, 20),
};

pub const BLUE: Theme = Theme {
    text: Color::Rgb(16, 24, 48),
    background: Color::Rgb(0, 0, 0),
    border_selected: Color::Rgb(50, 165, 245),
    border_active: Color::Rgb(20, 130, 230),
    border_normal: Color::Rgb(20, 70, 160),
};

pub const YELLOW: Theme = Theme {
    text: Color::Rgb(16, 48, 16),
    background: Color::Rgb(0, 0, 0),
    border_selected: Color::Rgb(240, 195, 40),
    border_active: Color::Rgb(255, 185, 0),
    border_normal: Color::Rgb(140, 110, 20),
};

pub const MATRIX_GREEN: Theme = Theme {
    text: Color::Rgb(16, 48, 16),
    background: Color::Rgb(0, 0, 0),
    border_active: Color::Rgb(0, 143, 17),
    border_selected: Color::Rgb(0, 255, 65),
    border_normal: Color::Rgb(0, 59, 0),
};

pub const GRAY: Theme = Theme {
    text: Color::Rgb(16, 48, 16),
    background: Color::Rgb(0, 0, 0),
    border_active: Color::Rgb(36, 68, 36),
    border_selected: Color::Rgb(56, 88, 56),
    border_normal: Color::Rgb(16, 48, 16),
};

/// A custom widget that renders a button with a label, theme and state.
#[derive(Debug, Clone, PartialEq)]
pub struct Button<'a, TValue: Default + Clone> {
    pub label: Line<'a>,
    pub value: TValue,
    pub theme: Theme,
    pub state: ButtonState,
}

/// A button with a label that can be themed.
impl<'a, TValue: Default + Clone> Button<'a, TValue> {
    pub fn new<TLabel>(label: TLabel) -> Button<'a, TValue>
    where
        TLabel: Into<Line<'a>>,
    {
        Button {
            label: label.into(),
            theme: BLUE,
            value: TValue::default(),
            state: ButtonState::Normal,
        }
    }

    pub fn value(mut self, value: TValue) -> Button<'a, TValue> {
        self.value = value;
        self
    }

    pub fn theme(mut self, theme: Theme) -> Button<'a, TValue> {
        self.theme = theme;
        self
    }

    pub fn state(mut self, state: ButtonState) -> Button<'a, TValue> {
        self.state = state;
        self
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label.into()
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme
    }

    pub fn set_state(&mut self, state: ButtonState) {
        self.state = state;
    }

    pub fn get_value(&mut self) -> TValue {
        self.value.clone()
    }
}

impl<'a, TValue: Default + Clone> Widget for Button<'a, TValue> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (background, border, _) = self.colors();

        // Top line
        buf.set_string(area.x, area.y, "╭", Style::new().fg(border).bg(background));

        buf.set_string(
            area.x + 1,
            area.y,
            "─".repeat((area.width - 2) as usize),
            Style::new().fg(border).bg(background),
        );

        buf.set_string(
            area.right() - 1,
            area.y,
            "╮",
            Style::new().fg(border).bg(background),
        );

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

            // render label centered
            if area.width > 4 {
                let label_width = self.label.width() as u16;
                let label_start_x = area.x + (area.width - label_width) / 2;
                let label_start_y = area.y + (area.height - 1) / 2;
                buf.set_line(label_start_x, label_start_y, &self.label, label_width);
            }
        }
    }
}

impl<'a, TValue: Default + Clone> Button<'a, TValue> {
    fn colors(&self) -> (Color, Color, Color) {
        let theme = self.theme;
        match self.state {
            ButtonState::Normal => (theme.background, theme.border_normal, theme.text),
            ButtonState::Selected => (theme.background, theme.border_selected, theme.text),
            ButtonState::Active => (theme.background, theme.border_active, theme.text),
        }
    }
}
