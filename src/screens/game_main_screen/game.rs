use crate::entities::pawn::Pawn;
use crate::entities::player::Player;

pub struct Game {
    pub players: Vec<Player>, // Vektor svih igraca koji igraju, ako ih igra 2 bit ce niz velicine
    // 2
    pub fields: Vec<Option<Pawn>>, // 40 polja za igru, ili None ili Player
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(),
            fields: vec![None; 40],
        }
    }
}
