use crossterm::event::{KeyCode, KeyEvent};

use crate::{entities::player::Player, utils::roll_dice};

use super::screen::{GameMainScreen, HoverDir, PlayerAction};

pub struct MainEventHandler;

impl MainEventHandler {
    pub fn handle_relative_change(gms: &mut GameMainScreen, key_event: KeyEvent) {
        if gms.curr_player.player_action == PlayerAction::Hovering {
            let direction = match key_event.code {
                KeyCode::Up => HoverDir::Up,
                KeyCode::Down => HoverDir::Down,
                KeyCode::Left => HoverDir::Left,
                KeyCode::Right => HoverDir::Right,
                _ => unreachable!(),
            };

            gms.hover_relative(direction);
        }
    }

    pub fn handle_pawn_select(gms: &mut GameMainScreen, key_event: KeyEvent) {
        let raw_code = match key_event.code {
            KeyCode::Char(ch) if ch.is_digit(10) => ch.to_digit(10).unwrap_or(0) as usize,
            _ => 0,
        };

        if gms.curr_player.player_action == PlayerAction::Selecting {
            match gms.select_pawn(Some(raw_code - 1)) {
                Ok(_) => {
                    gms.message = format!(
                        "Selected pawn {:?}! \n\n Move to desired location and press ENTER.",
                        raw_code
                    );

                    let current_player: &Player = gms.get_current_player();

                    let selected_pawn =
                        current_player.pawns[gms.curr_player.selected_pawn_id.unwrap()];
                    let mut focus_pos: (usize, usize) = selected_pawn.position;

                    if gms.board[selected_pawn.position.0][selected_pawn.position.1].kind
                        == current_player.home_field_kind
                    {
                        focus_pos = gms.get_current_player().start_pos;
                    }

                    gms.focus_field(focus_pos);
                }
                Err(message) => gms.message = message.to_string(),
            }
        }
    }

    pub fn handle_roll(gms: &mut GameMainScreen, dice_roll: Option<usize>) {
        if gms.curr_player.roll.is_none() {
            let roll = dice_roll.unwrap_or_else(roll_dice);
            gms.curr_player.roll = Some(roll);

            match gms.has_valid_moves() {
                Ok(_) => {
                    gms.curr_player.player_action = PlayerAction::Selecting;
                    gms.message = format!(
                        "You rolled a {}!  \n\n Press numbers 1-4 to select your pawn.",
                        roll
                    );
                }
                Err(message) => {
                    gms.curr_player.player_action = PlayerAction::Playing;
                    gms.message = format!(
                        "You rolled a {} but {} \n\n Press ENTER to continue.",
                        roll, message
                    );
                }
            }
        }
    }

    pub fn handle_unselect_pawn(gms: &mut GameMainScreen) {
        if let Some(selected_pawn_id) = gms.curr_player.selected_pawn_id {
            gms.curr_player.player_action = PlayerAction::Selecting;
            gms.message = format!(
                "Unselected pawn {:?}! \n\n Press numbers 1-4 to select new pawn.",
                selected_pawn_id + 1
            );
            gms.curr_player.selected_pawn_id = None;
        }
    }
}
