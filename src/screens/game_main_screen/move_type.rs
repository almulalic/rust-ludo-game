use core::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum GoodMoveType {
    #[default]
    Start,
    Move,
    EatMove,
    Safehouse,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum BadMoveType {
    #[default]
    DidntRoll6,
    StartOccupied,
    WrongStart,
    Generic,
    UnreachableField,
    CantEatOwnPawn,
}

impl fmt::Display for BadMoveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BadMoveType::DidntRoll6 => write!(f, "You selected a home field without getting 6!"),
            BadMoveType::StartOccupied => write!(f, "Start field is occupied!"),
            BadMoveType::WrongStart => write!(
                f,
                "Invalid start! You can only start on your own start position."
            ),
            BadMoveType::UnreachableField => {
                write!(f, "Selected field is unreachable with current roll.")
            }
            BadMoveType::CantEatOwnPawn => {
                write!(f, "You can't move to your own pawn's field!")
            }
            BadMoveType::Generic => write!(f, "Generic error :( I coded something wrong..."),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum NoValidMoveType {
    #[default]
    NoPawnsNot6,
    StarOccupied,
}

impl fmt::Display for NoValidMoveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NoValidMoveType::NoPawnsNot6 => {
                write!(f, "you have no pawns on field")
            }
            NoValidMoveType::StarOccupied => write!(f, "start field is occupied"),
        }
    }
}
