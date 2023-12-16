use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Normal,
    Selected,
    Active,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    text: Color,
    background: Color,
    highlight: Color,
    shadow: Color,
}

pub const BLUE: Theme = Theme {
    text: Color::Rgb(16, 24, 48),
    background: Color::Rgb(48, 72, 144),
    highlight: Color::Rgb(64, 96, 192),
    shadow: Color::Rgb(32, 48, 96),
};

pub const RED: Theme = Theme {
    text: Color::Rgb(48, 16, 16),
    background: Color::Rgb(144, 48, 48),
    highlight: Color::Rgb(192, 64, 64),
    shadow: Color::Rgb(96, 32, 32),
};

pub const GREEN: Theme = Theme {
    text: Color::Rgb(16, 48, 16),
    background: Color::Rgb(48, 144, 48),
    highlight: Color::Rgb(64, 192, 64),
    shadow: Color::Rgb(32, 96, 32),
};

pub const GRAY: Theme = Theme {
    text: Color::Rgb(16, 48, 16),
    background: Color::Rgb(160, 160, 160),
    highlight: Color::Rgb(200, 200, 200),
    shadow: Color::Rgb(160, 160, 160),
};

pub const YELLOW: Theme = Theme {
    text: Color::Rgb(16, 48, 16),
    background: Color::Rgb(255, 255, 0),
    highlight: Color::Rgb(255, 255, 0),
    shadow: Color::Rgb(160, 160, 160),
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
        let (background, text, shadow, highlight) = self.colors();
         
        buf.set_style(
            area, 
            Style::new()
                .bg(background)
                .fg(text)
        );

        // render top line if there's enough space
        if area.height > 2 {
            buf.set_string(
                area.x,
                area.y,
                "▔".repeat(area.width as usize),
                Style::new().fg(highlight).bg(background),
            );
        }
        
        // render bottom line if there's enough space
        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                "▁".repeat(area.width as usize),
                Style::new().fg(shadow).bg(background),
            );
        }

        // render label centered
        buf.set_line(
            area.x + (area.width.saturating_sub(self.label.width() as u16)) / 2,
            area.y + (area.height.saturating_sub(1)) / 2,
            &self.label,
            area.width,
        );
    }
}

impl<'a, TValue: Default + Clone> Button<'a, TValue> {
    fn colors(&self) -> (Color, Color, Color, Color) {
        let theme = self.theme;
        match self.state {
            ButtonState::Normal => (theme.background, theme.text, theme.shadow, theme.highlight),
            ButtonState::Selected => (theme.highlight, theme.text, theme.shadow, theme.highlight),
            ButtonState::Active => (theme.background, theme.text, theme.highlight, theme.shadow),
        }
    }
}


