use core::fmt;

use ratatui::{
    style::{Color, Stylize},
    widgets::Paragraph,
};

use crate::debug_log;

use super::field::{Field, FieldKind};

#[derive(Default, Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PawnColor {
    #[default]
    RED,
    GREEN,
    BLUE,
    YELLOW,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct PawnColorPallet {
    primary: Color,
    hovered: Color,
    disabled: Color,
}

impl PawnColorPallet {
    pub fn new(primary: Color, hovered: Color, disabled: Color) -> PawnColorPallet {
        PawnColorPallet {
            primary,
            hovered,
            disabled,
        }
    }
}

fn get_color_pallete(color: &PawnColor) -> PawnColorPallet {
    match color {
        PawnColor::RED => PawnColorPallet {
            primary: Color::Red,
            hovered: Color::Rgb(139, 0, 0),
            disabled: Color::Rgb(139, 0, 0),
        },
        PawnColor::GREEN => PawnColorPallet {
            primary: Color::Green,
            hovered: Color::Rgb(1, 50, 32),
            disabled: Color::Rgb(1, 50, 32),
        },
        PawnColor::BLUE => PawnColorPallet {
            primary: Color::Blue,
            hovered: Color::Rgb(0, 0, 139),
            disabled: Color::Rgb(0, 0, 139),
        },
        PawnColor::YELLOW => PawnColorPallet {
            primary: Color::Yellow,
            hovered: Color::Rgb(246, 190, 0),
            disabled: Color::Rgb(246, 190, 0),
        },
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pawn {
    pub id: usize,
    pub color: PawnColor,
    pub color_pallete: PawnColorPallet,
    pub player_id: usize,
    pub position: (usize, usize),
}

impl Pawn {
    pub fn new(id: usize, color: PawnColor, player: usize, position: (usize, usize)) -> Pawn {
        Pawn {
            id,
            color,
            player_id: player,
            color_pallete: get_color_pallete(&color),
            position,
        }
    }

    pub fn render(&mut self, field: &Field) -> Paragraph {
        if field.is_hovered {
            if field.kind == FieldKind::RedSafehouse {
                debug_log!(format!("{:?}", self));
            }
            return Paragraph::new(format!(" ████ \n██ {} ██\n ████ ", self.id + 1))
                .fg(self.color_pallete.hovered);
        } else {
            return Paragraph::new(format!(" ████ \n██ {} ██\n ████ ", self.id + 1))
                .fg(self.color_pallete.primary);
        }
    }
}

impl fmt::Display for Pawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Pawn {
    fn default() -> Self {
        Pawn {
            id: Default::default(),
            color: Default::default(),
            player_id: Default::default(),
            position: Default::default(),
            color_pallete: Default::default(),
        }
    }
}
