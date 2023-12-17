use std::fmt;
use crate::entities::pawn::PawnColor;

#[derive(Debug, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Player { 
    pub pawn_color: PawnColor,
    pub order: usize,
    pub safehouses: [ i8; 4 ],
    pub pieces_on_board: usize,
}

impl fmt::Display for PawnColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Player {
    pub fn new(color: PawnColor) -> Player {
        Player {
            pawn_color: color,
            order: 1,
            pieces_on_board: 0,
            safehouses: [ 0; 4 ]
        }
    }

    pub fn set_order(&mut self, order: usize) {
        self.order = order
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

