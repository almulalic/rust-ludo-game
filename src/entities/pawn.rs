use ratatui::style::Color;

use crate::entities::player::Player;

#[derive(Default, Debug, Copy, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum PawnColor {
    #[default]
    RED,
    GREEN,
    BLUE,
    YELLOW,
}

#[derive(Debug, Copy, Clone)]
pub struct Pawn {
    pub player: Player,
    pub color: PawnColor,
}

impl Pawn {
    pub fn new(player: Player, color: PawnColor) -> Pawn {
        Pawn { player, color }
    }

    pub fn get_rgb_color(&mut self) -> Color {
        match self.color {
            PawnColor::RED => Color::Red,
            PawnColor::GREEN => Color::Green,
            PawnColor::BLUE => Color::Blue,
            PawnColor::YELLOW => Color::Yellow,
        }
    }
}
