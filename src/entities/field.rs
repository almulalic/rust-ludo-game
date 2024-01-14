use core::fmt;

use super::pawn::Pawn;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum FieldKind {
    #[default]
    Gap,
    Path,
    RedHome,
    GreenHome,
    BlueHome,
    YellowHome,
    RedStart,
    GreenStart,
    BlueStart,
    YellowStart,
    RedSafehouse,
    GreenSafehouse,
    BlueSafehouse,
    YellowSafehouse,
}

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub is_visible: bool,
    pub is_hovered: bool,
    pub kind: FieldKind,
    pub pawn: Option<Pawn>,
    pub position: (usize, usize),
}

impl Field {
    pub fn new(
        field_type: FieldKind,
        is_visible: bool,
        is_hovered: bool,
        pawn: Option<Pawn>,
    ) -> Field {
        Field {
            is_visible,
            is_hovered,
            kind: field_type,
            pawn,
            position: Default::default(),
        }
    }

    pub fn set_position(&mut self, (pi, pj): (usize, usize)) {
        self.position = (pi, pj);
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
