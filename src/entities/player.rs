use crate::entities::pawn::PawnColor;

#[derive(Debug, Copy, Clone)]
pub struct Player { 
    pub pawn_color: PawnColor,
    pub pieces_on_board: usize,
    pub safehouses: [ i8; 4 ]
}

impl Player {
    pub fn new(color: PawnColor) -> Player {
        Player {
            pawn_color: color,
            pieces_on_board: 0,
            safehouses: [ 0; 4 ]
        }
    }
}

