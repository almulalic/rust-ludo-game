use core::fmt;

use crate::{debug_log, entities::player::Player};

use super::screen::PlayerAction;

#[derive(Debug)]
pub struct CurrentPlayer {
    pub id: usize,
    pub player_action: PlayerAction,
    pub selected_pawn_id: Option<usize>,
    pub prev_hover_flat: usize,
    pub prev_hover_position: (usize, usize),
    pub curr_hover_flat: usize,
    pub curr_hover_position: (usize, usize),
    pub roll: Option<usize>,
}

impl CurrentPlayer {
    pub fn new() -> CurrentPlayer {
        CurrentPlayer {
            id: 0,
            player_action: PlayerAction::WaitingRoll,
            selected_pawn_id: None,
            prev_hover_flat: Default::default(),
            prev_hover_position: Default::default(),
            curr_hover_flat: Default::default(),
            curr_hover_position: Default::default(),
            roll: None,
        }
    }

    pub fn can_hover(&mut self) -> bool {
        return self.roll.is_some();
    }

    pub fn next(curr_player: &CurrentPlayer, player: Player) -> CurrentPlayer {
        debug_log!(format!(
            "switching players - next() \n    - player {:?}\n",
            player
        ));

        CurrentPlayer {
            id: curr_player.id,
            player_action: PlayerAction::WaitingRoll,
            selected_pawn_id: None,
            prev_hover_flat: curr_player.prev_hover_flat,
            prev_hover_position: curr_player.prev_hover_position,
            curr_hover_flat: curr_player.curr_hover_flat,
            curr_hover_position: curr_player.curr_hover_position,
            roll: None,
        }
    }

    pub fn repeat_turn(&mut self) {
        debug_log!(format!(
            "switching players - repeat_move() \n    - curr_player {:?}\n",
            self
        ));

        self.player_action = PlayerAction::WaitingRoll;
        self.selected_pawn_id = None;
        self.roll = None;
    }

    pub fn diff_from_roll(&self, num: usize) -> usize {
        if let Some(roll) = self.roll {
            return roll.abs_diff(num) % 44;
        }

        return 0;
    }
}

impl fmt::Display for CurrentPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
