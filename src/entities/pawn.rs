use crate::entities::player::Player;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum PawnColor {
    #[default]
    RED,
    GREEN,
    BLUE,
    YELLOW
}

#[derive(Debug, Copy, Clone)]
pub struct Pawn {
    pub owner: Player
}

impl Pawn {
    pub fn new(player: Player) -> Pawn {
        Pawn {
            owner: player
        }
    }
}

