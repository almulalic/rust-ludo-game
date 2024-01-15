use core::fmt;
use serde::{Deserialize, Serialize};

use crate::entities::pawn::PawnColor;

use super::{field::FieldKind, pawn::Pawn};

#[derive(Debug, Default, Serialize, Deserialize, Copy, Clone)]
pub struct Player {
    pub id: usize,
    pub order: usize,
    pub pawns: [Pawn; 4],
    pub pawn_color: PawnColor,
    pub home_pos: [(usize, usize); 4],
    pub safehouse_pos: [(usize, usize); 4],
    pub safehouse_kind: FieldKind,
    pub home_field_kind: FieldKind,
    pub start_field_kind: FieldKind,
    pub start_pos: (usize, usize),
    pub pawns_on_board: usize,
}

impl fmt::Display for PawnColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Player {
    pub fn new(id: usize, order: usize, color: PawnColor) -> Player {
        let mut pawns: [Pawn; 4] = Default::default();
        let start_pos: (usize, usize);
        let start_field_kind: FieldKind;
        let safehouse_kind: FieldKind;
        let home_field_kind: FieldKind;
        let home_pos: [(usize, usize); 4];
        let safehouse_pos: [(usize, usize); 4];

        match color {
            PawnColor::RED => {
                start_pos = (4, 0);
                home_field_kind = FieldKind::RedHome;
                start_field_kind = FieldKind::RedStart;
                safehouse_kind = FieldKind::RedSafehouse;
                home_pos = [(0, 0), (0, 1), (1, 0), (1, 1)];
                safehouse_pos = [(6, 1), (6, 2), (6, 3), (6, 4)];
            }
            PawnColor::GREEN => {
                start_pos = (0, 8);
                start_field_kind = FieldKind::GreenHome;
                home_field_kind = FieldKind::RedHome;
                safehouse_kind = FieldKind::GreenSafehouse;
                home_pos = [(0, 11), (0, 12), (1, 11), (1, 12)];
                safehouse_pos = [(1, 6), (2, 6), (3, 6), (4, 6)];
            }
            PawnColor::BLUE => {
                start_pos = (8, 12);
                home_field_kind = FieldKind::BlueHome;
                start_field_kind = FieldKind::BlueStart;
                safehouse_kind = FieldKind::GreenSafehouse;
                home_pos = [(11, 11), (11, 12), (12, 11), (12, 12)];
                safehouse_pos = [(6, 11), (6, 10), (6, 9), (6, 8)];
            }
            PawnColor::YELLOW => {
                start_pos = (12, 4);
                home_field_kind = FieldKind::YellowHome;
                start_field_kind = FieldKind::YellowStart;
                safehouse_kind = FieldKind::GreenSafehouse;
                home_pos = [(11, 0), (11, 1), (12, 0), (12, 1)];
                safehouse_pos = [(11, 6), (10, 6), (9, 6), (8, 6)];
            }
        }

        for (i, pawn) in pawns.iter_mut().enumerate() {
            *pawn = Pawn::new(i, color, order, home_pos[i]);
        }

        let player = Player {
            id,
            pawn_color: color,
            order,
            pawns,
            pawns_on_board: Default::default(),
            home_pos,
            home_field_kind,
            start_field_kind,
            safehouse_kind,
            start_pos,
            safehouse_pos,
        };

        player
    }

    pub fn set_order(&mut self, order: usize) {
        self.order = order
    }

    pub fn get_pawn(&mut self, pawn_id: usize) -> &Pawn {
        return &self.pawns[pawn_id];
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
