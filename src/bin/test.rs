#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
struct Pawn {
    id: usize,
}

impl Pawn {
    fn new(id: usize) -> Pawn {
        Pawn { id }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
struct Field {
    pawn: Pawn,
}

impl Field {
    fn new(id: usize) -> Field {
        Field {
            pawn: Pawn::new(id),
        }
    }
}

struct GameMainScreen<'a> {
    curr_field: Option<&'a mut Field>,
    board: [[Field; 2]; 2],
    flat: Vec<&'a Field>,
}

impl<'a> GameMainScreen<'a> {
    fn new() -> GameMainScreen<'a> {
        GameMainScreen {
            curr_field: None,
            board: [
                [Field::new(0), Field::new(1)],
                [Field::new(2), Field::new(3)],
            ],
            flat: Vec::new(),
        }
    }

    fn add_fields(&mut self) {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                self.flat.push(&self.board[i][j]);
            }
        }
    }

    fn modify_curr_field(&mut self) {}
}

fn main() {
    let mut gms = GameMainScreen::new();
    gms.add_fields();

    // Access and modify the current field
    if let Some(curr_field) = gms.curr_field {
        // Modify curr_field
    }
}
