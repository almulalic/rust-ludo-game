use crate::entities::player::Player;

#[derive(Debug, Copy, Clone)]
pub enum PawnColor {
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

