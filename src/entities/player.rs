use crate::entities::pawn::PawnColor;

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub color: PawnColor,
    pub pieces_on_board: usize,
    pub safehouses: [ i8; 4 ]
}

impl Player {
    pub fn new(color: PawnColor, safehouses: [ i8; 4 ]) -> Player {
        Player {
            color,
            pieces_on_board: 0,
            safehouses
        }
    }
}

