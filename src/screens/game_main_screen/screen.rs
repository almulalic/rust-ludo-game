use core::fmt;
use std::collections::{BTreeMap, HashMap};

use crate::debug_log;
use crate::entities::board::extend_safehouses;
use crate::entities::board::get_path_map;
use crate::entities::board::initialize_board;
use crate::entities::board::reorder_path_map;
use crate::entities::field::Field;
use crate::entities::field::FieldKind;
use crate::entities::pawn::Pawn;
use crate::entities::player::Player;
use crate::screens::game_main_screen::current_player::CurrentPlayer;
use crate::tui::Tui;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

use super::event_handler::MainEventHandler;
use super::move_type::{BadMoveType, GoodMoveType, NoValidMoveType};

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum GameState {
    RUNNING,
    PAUSED,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum PlayerAction {
    #[default]
    WaitingRoll,
    Rolled,
    Selecting,
    Hovering,
    Playing,
}

impl fmt::Display for PlayerAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum HoverDir {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum RelativeMove {
    #[default]
    Forward,
    Back,
}

pub fn field_diff(num1: usize, num2: usize) -> usize {
    let mut diff = (num1 as isize) - (num2 as isize);
    let max_range = 40 as isize;

    if diff < 0 {
        diff += max_range;
    }

    diff as usize % 44
}

#[derive(Debug)]
pub struct GameMainScreen {
    pub players: Vec<Player>,
    pub state: GameState,
    pub curr_player: CurrentPlayer,
    pub is_game_finished: bool,
    pub board: [[Field; 13]; 13],
    pub path_map: BTreeMap<usize, (usize, usize)>,
    pub message: String,
    pub should_normalize_movement: bool,
}

impl GameMainScreen {
    pub fn new(players: Vec<Player>) -> GameMainScreen {
        let mut game_main_screen = GameMainScreen {
            players: players.clone(),
            curr_player: CurrentPlayer::new(),
            state: GameState::RUNNING,
            is_game_finished: false,
            board: initialize_board(),
            path_map: get_path_map(),
            message: String::from("Press SPACE to roll the dice!"),
            should_normalize_movement: false,
        };

        for player in &game_main_screen.players {
            for (i, home_positions) in player.home_pos.iter().enumerate() {
                let (row, col) = *home_positions;
                game_main_screen.board[row][col].pawn = Some(player.pawns[i].clone());
            }
        }

        game_main_screen.path_map = extend_safehouses(
            &get_path_map(),
            game_main_screen.get_current_player().safehouse_pos,
        );

        Self::focus_field(&mut game_main_screen, players[0].start_pos);

        game_main_screen
    }

    pub fn get_current_player(&self) -> &Player {
        return &self.players[self.curr_player.id];
    }

    pub fn get_current_pawn(&self) -> &Pawn {
        return &self.players[self.curr_player.id].pawns
            [self.curr_player.selected_pawn_id.unwrap()];
    }

    pub fn get_pawn_field(&mut self, pawn_id: usize) -> Field {
        let (pi, pj) = self.players[self.curr_player.id].pawns[pawn_id].position;
        return self.board[pi][pj];
    }

    pub fn update_field(&mut self, field: Field) {
        if let Some(pawn) = field.pawn {
            self.players[self.curr_player.id].pawns[pawn.id] = pawn;
        }

        self.board[field.position.0][field.position.1] = field;
    }

    pub fn flat_from_pos(&self, (i, j): (usize, usize)) -> Option<usize> {
        return self
            .path_map
            .iter()
            .find(|&(_, v)| v == &(i, j))
            .map(|(k, _)| *k);
    }

    pub fn focus_field(&mut self, (si, sj): (usize, usize)) {
        if let Some(key) = self.flat_from_pos((si, sj)) {
            self.curr_player.prev_hover_position = self.curr_player.curr_hover_position;
            self.curr_player.prev_hover_flat = self.curr_player.curr_hover_flat;

            let (ci, cj) = self.curr_player.curr_hover_position;

            self.board[ci][cj].is_hovered = false;
            self.board[si][sj].is_hovered = true;

            self.curr_player.curr_hover_flat = key;
            self.curr_player.curr_hover_position = (si, sj);
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.state = match self.state {
                    GameState::RUNNING => GameState::PAUSED,
                    _ => GameState::RUNNING,
                }
            }
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                MainEventHandler::handle_relative_change(self, key_event);
            }
            KeyCode::F(1..=12) => {
                let raw_num = match key_event.code {
                    KeyCode::F(n) if n >= 1 && n <= 12 => n as usize,
                    _ => 0,
                };

                MainEventHandler::handle_roll(self, Some(raw_num));
            }
            KeyCode::Char('e') => {
                MainEventHandler::handle_roll(self, Some(39));
            }
            KeyCode::Char('1'..='4') => {
                MainEventHandler::handle_pawn_select(self, key_event);
            }
            KeyCode::Enter => match self.curr_player.player_action {
                PlayerAction::Hovering => {
                    let mut selected_pawn_field: Field = self
                        .get_pawn_field(self.curr_player.selected_pawn_id.unwrap())
                        .clone();

                    match self
                        .is_valid_move(&selected_pawn_field, &self.curr_player.curr_hover_position)
                    {
                        Ok(good_move_type) => match good_move_type {
                            GoodMoveType::Start => match self.move_pawn(
                                &mut selected_pawn_field,
                                self.get_current_player().start_pos,
                            ) {
                                Ok(_) => {
                                    self.message =
                                        String::from("Valid move! \n\n Press SPACE to continue.");
                                    self.curr_player.player_action = PlayerAction::Playing;

                                    self.players[self.curr_player.id].pawns_on_board += 1;

                                    debug_log!(format!(
                                        "move_pawn_to_start() - Ok() - \n    - selected_pawn_field: {:?} \n    - curr_pawn: {:?} \n",
                                        selected_pawn_field,
                                        self.players[self.curr_player.id].pawns[self.curr_player.selected_pawn_id.unwrap()]
                                    ));
                                }
                                Err(message) => {
                                    self.message = message.to_string();
                                }
                            },
                            GoodMoveType::Move | GoodMoveType::EatMove => {
                                match self.move_pawn(
                                    &mut selected_pawn_field,
                                    self.curr_player.curr_hover_position,
                                ) {
                                    Ok(good_move_type) => match good_move_type {
                                        GoodMoveType::EatMove => {
                                            self.message = String::from(
                                                "Valid move! \n\n Press SPACE to continue.",
                                            );
                                        }
                                        _ => {
                                            self.curr_player.player_action = PlayerAction::Playing;
                                        }
                                    },
                                    Err(message) => self.message = message.to_string(),
                                }
                            }
                            GoodMoveType::Safehouse => {}
                        },
                        Err(bad_move_type) => match bad_move_type {
                            BadMoveType::StartOccupied => {
                                self.message = format!(
                                    "{} \n\n Press BACKSPCE to select a pawn that is on board.",
                                    bad_move_type
                                )
                            }
                            BadMoveType::CantEatOwnPawn => {
                                self.message =
                                    format!("{} \n\n Move to another field.", bad_move_type)
                            }
                            BadMoveType::DidntRoll6 => {
                                self.message = format!(
                                    "{} \n\n Press BACKSPACE to select another pawn.",
                                    bad_move_type
                                )
                            }
                            BadMoveType::UnreachableField => {
                                let roll: usize = self.curr_player.roll.unwrap();

                                debug_log!(format!(
                                    "move_field() unreachable_field: \n    - curr_player: {:?} \n",
                                    self.curr_player
                                ));

                                let pawn_field_flat: usize = self
                                    .flat_from_pos(self.get_current_pawn().position)
                                    .unwrap();

                                let new_field_flat: usize = self
                                    .flat_from_pos(self.curr_player.curr_hover_position)
                                    .unwrap();

                                self.message = format!(
                                    "{} \n\n You rolled a {} but the field is {} fields away.",
                                    bad_move_type.to_string(),
                                    roll,
                                    field_diff(new_field_flat, pawn_field_flat)
                                );
                            }
                            BadMoveType::WrongStart => {
                                if let Some(selected_pawn_id) = self.curr_player.selected_pawn_id {
                                    self.message = format!(
                                        "{} \n\n Select a new position for pawn {} \n",
                                        bad_move_type.to_string(),
                                        selected_pawn_id + 1
                                    );
                                }
                            }
                            _ => {
                                self.message = bad_move_type.to_string();
                            }
                        },
                    }

                    match self.check_winner() {
                        Ok(player_id) => panic!("{}", player_id),
                        _ => {}
                    }
                }
                _ => {}
            },
            KeyCode::Char(' ') => match self.curr_player.player_action {
                PlayerAction::Playing => {
                    if self.curr_player.roll >= Some(6) {
                        self.message = String::from(
                            "You have rolled a 6! Your turn again \n\n Press SPACE to roll.",
                        );

                        let selected_pawn_field: Field = self
                            .get_pawn_field(self.curr_player.selected_pawn_id.unwrap())
                            .clone();

                        debug_log!(format!(
                            "move_pawn_to_start() - Ok() - \n    - selected_pawn_field: {:?} \n    - player: {:?} \n",
                            selected_pawn_field,
                            self.players[self.curr_player.id]
                        ));

                        self.curr_player.repeat_turn();
                    } else {
                        self.next_player();
                    }
                }
                _ => MainEventHandler::handle_roll(self, None),
            },
            KeyCode::Backspace => {
                if self.curr_player.player_action > PlayerAction::Selecting
                    && self.curr_player.player_action < PlayerAction::Playing
                {
                    MainEventHandler::handle_unselect_pawn(self)
                }
            }
            _ => {}
        }
    }

    pub fn check_winner(&mut self) -> Result<usize, bool> {
        for player in &self.players {
            if player.pawns_on_board == 4 {
                let mut in_safehouse = 0;

                for (i, j) in player.home_pos {
                    if self.board[i][j].pawn.is_some() {
                        in_safehouse += 1;
                    }
                }

                if in_safehouse == 4 {
                    return Ok(player.id);
                }
            }
        }

        return Err(false);
    }

    pub fn next_player(&mut self) {
        debug_log!(format!(
            "Changing player from {} \n    - player: {}\n",
            self.curr_player.id, self.curr_player
        ));

        self.curr_player.id = (self.curr_player.id + 1) % self.players.len();
        self.message = String::from("Press SPACE to roll!");
        self.curr_player =
            CurrentPlayer::next(&self.curr_player, self.players[self.curr_player.id]);

        self.path_map.retain(|&key, _| key < 40);
        self.path_map = reorder_path_map(&self.path_map, self.players.len());
        self.path_map = extend_safehouses(&self.path_map, self.get_current_player().safehouse_pos);

        self.focus_field(self.players[self.curr_player.id].start_pos);

        debug_log!(format!(
            "Changing player to {} ({}) \n    - player: {}\n",
            self.curr_player.id,
            self.get_current_player().pawn_color,
            self.curr_player
        ));
    }

    pub fn has_valid_moves(&mut self) -> Result<&'static str, NoValidMoveType> {
        if let Some(roll) = self.curr_player.roll {
            debug_log!(format!(
                "has_valid_moves: \n    - curr_player: {:?} \n    - roll: {}",
                self.curr_player, roll
            ));

            if roll != 6 {
                if self.get_current_player().pawns_on_board == 0 {
                    return Err(NoValidMoveType::NoPawnsNot6);
                }
            }
        }

        return Ok("");
    }

    pub fn is_valid_move(
        &self,
        selected_pawn_field: &Field,
        (nfi, nfj): &(usize, usize),
    ) -> Result<GoodMoveType, BadMoveType> {
        if let Some(selected_pawn) = selected_pawn_field.pawn {
            debug_log!(format!(
                "is_valid_move() - Checking for :\n    - selected_pawn_field {:?}\n    - new_field_id: {:?} \n",
                selected_pawn_field,
                (nfi,nfj)
            ));

            let selected_new_field: &Field = &self.board[*nfi][*nfj];
            let current_player: &Player = self.get_current_player();

            if selected_pawn_field.kind == current_player.home_field_kind {
                if self.curr_player.roll != Some(6) {
                    return Err(BadMoveType::DidntRoll6);
                } else {
                    let (si, sj): (usize, usize) = current_player.start_pos;

                    if let Some(start_field) = self.board.get(si).and_then(|row| row.get(sj)) {
                        if let Some(start_pawn) = start_field.pawn {
                            if start_pawn.color == current_player.pawn_color {
                                return Err(BadMoveType::StartOccupied);
                            }
                        }
                    }
                    if selected_new_field.kind != current_player.start_field_kind {
                        return Err(BadMoveType::WrongStart);
                    }

                    return Ok(GoodMoveType::Start);
                }
            }

            let pawn_field_flat: usize = self.flat_from_pos(selected_pawn.position).unwrap();
            let new_field_flat: usize = self.flat_from_pos((*nfi, *nfj)).unwrap();

            match (field_diff(new_field_flat, pawn_field_flat)) == self.curr_player.roll.unwrap() {
                true => {
                    if let Some(new_pawn) = selected_new_field.pawn {
                        if new_pawn.color == selected_pawn.color {
                            return Err(BadMoveType::CantEatOwnPawn);
                        }
                    }

                    Ok(GoodMoveType::Move)
                }
                false => Err(BadMoveType::UnreachableField),
            }
        } else {
            debug_log!(format!(
                "is_valid_move() - Pawn not found:\n    - selected_pawn_field: {:?} \n",
                selected_pawn_field
            ));

            Err(BadMoveType::Generic)
        }
    }

    pub fn move_pawn(
        &mut self,
        selected_pawn_field: &mut Field,
        (nfi, nfj): (usize, usize),
    ) -> Result<GoodMoveType, BadMoveType> {
        if let Some(mut selected_pawn) = selected_pawn_field.pawn {
            let mut hovered_field: Field = self.board[nfi][nfj];
            let mut move_type: GoodMoveType = GoodMoveType::Move;

            debug_log!(format!(
                "move_pawn() - Before moving pawn: \n    - selected_pawn_field: {:?} \n    - hovered_field: {:?} \n",
                selected_pawn_field, hovered_field
            ));

            if let Some(hovered_pawn) = hovered_field.pawn {
                if selected_pawn.color != hovered_pawn.color {
                    self.eat_pawn(&mut hovered_field);
                    move_type = GoodMoveType::EatMove;
                }
            }

            selected_pawn.position = (nfi, nfj);
            hovered_field.pawn = Some(selected_pawn);
            hovered_field.is_hovered = false;

            selected_pawn_field.pawn = None;

            self.update_field(*selected_pawn_field);
            self.update_field(hovered_field);

            self.players[self.curr_player.id].pawns[self.curr_player.selected_pawn_id.unwrap()] =
                hovered_field.pawn.unwrap();

            debug_log!(format!(
                "move_pawn() - After moving pawn: \n    - selected_pawn_field: {:?} \n    - hovered_field: {:?} \n",
                selected_pawn_field, hovered_field
            ));

            return Ok(move_type);
        }

        return Err(BadMoveType::Generic);
    }

    pub fn eat_pawn(&mut self, losing_pawn_curr_field: &mut Field) {
        if let Some(mut losing_pawn) = losing_pawn_curr_field.pawn {
            let (hi, hj): (usize, usize) =
                self.players[losing_pawn.player_id].home_pos[losing_pawn.id];

            let mut losing_pawn_home_field: Field = self.board[hi][hj];

            debug_log!(format!(
                "eat_pawn() - After moving pawn: \n    - losing_pawn_curr_field: {:?} \n    - losing_pawn_home: {:?}     - losing_player: {:?} \n",
                losing_pawn_curr_field, losing_pawn_home_field, self.players[losing_pawn.player_id]
            ));

            // Copy the pawn to home
            losing_pawn.position = (hi, hj);
            losing_pawn_home_field.pawn = Some(losing_pawn);
            self.players[losing_pawn.player_id].pawns_on_board -= 1;
            self.players[losing_pawn.player_id].pawns[losing_pawn.id] = losing_pawn;
            self.update_field(losing_pawn_home_field);

            // Delete from current spot
            losing_pawn_curr_field.pawn = None;
            losing_pawn_curr_field.is_hovered = false;
            self.update_field(*losing_pawn_curr_field);

            debug_log!(format!(
                "eat_pawn() - After moving pawn: \n    - losing_pawn_curr_field: {:?} \n    - losing_pawn_home: {:?}     - losing_player: {:?} \n",
                losing_pawn_curr_field, losing_pawn_home_field, self.players[losing_pawn.player_id]
            ));
        }
    }

    pub fn select_pawn(
        &mut self,
        requested_pawn_id: Option<usize>,
    ) -> Result<&'static str, &'static str> {
        if let Some(selected_pawn_id) = requested_pawn_id {
            if let Some(pawn) = self.get_current_player().pawns.get(selected_pawn_id) {
                let (sfi, sfj) = pawn.position;

                if let Some(selected_field) =
                    self.board.get_mut(sfi).and_then(|row| row.get_mut(sfj))
                {
                    self.curr_player.selected_pawn_id = Some(selected_pawn_id);
                    self.curr_player.player_action = PlayerAction::Hovering;

                    debug_log!(format!(
                        "select_pawn: {:?} \n    - selected_pawn_field: {:?}",
                        self.curr_player.selected_pawn_id, selected_field
                    ));

                    return Ok("");
                } else {
                    return Err("No field found");
                }
            } else {
                return Err("No pawn found");
            }
        } else {
            Err("no pawn_id passed")
        }
    }

    pub fn hover_relative(&mut self, curr_hover_dir: HoverDir) {
        if self.curr_player.can_hover() == false
            || self.curr_player.player_action < PlayerAction::Hovering
        {
            return;
        }

        self.curr_player.prev_hover_position = self.curr_player.curr_hover_position;
        self.curr_player.prev_hover_flat = self.curr_player.curr_hover_flat;

        let (ci, cj) = self.curr_player.curr_hover_position;

        if self.normalize_dir(ci, cj, curr_hover_dir) == RelativeMove::Forward {
            self.curr_player.curr_hover_flat = (self.curr_player.curr_hover_flat + 1) % 44
        } else {
            self.curr_player.curr_hover_flat = (self.curr_player.curr_hover_flat + 43) % 44;
        }

        if let Some(&(i, j)) = self.path_map.get(&self.curr_player.prev_hover_flat) {
            self.board[i][j].is_hovered = false;
        }

        if let Some(&(i, j)) = self.path_map.get(&self.curr_player.curr_hover_flat) {
            self.curr_player.curr_hover_position = (i, j);
            self.board[i][j].is_hovered = true;
        }
    }

    pub fn normalize_dir(&self, ci: usize, cj: usize, dir: HoverDir) -> RelativeMove {
        if self.should_normalize_movement {
            if (ci <= 4 && cj == 8) && dir == HoverDir::Up {
                return RelativeMove::Back;
            } else if (ci <= 4 && cj == 8) && dir == HoverDir::Down {
                return RelativeMove::Forward;
            } else if (ci >= 4 && cj == 12) && dir == HoverDir::Up {
                return RelativeMove::Back;
            } else if (ci >= 4 && cj == 12) && dir == HoverDir::Down {
                return RelativeMove::Forward;
            } else if (ci == 8 && cj >= 6) && dir == HoverDir::Left {
                return RelativeMove::Forward;
            } else if (ci == 8 && cj >= 6) && dir == HoverDir::Right {
                return RelativeMove::Back;
            } else if (ci >= 8 && cj == 8) && dir == HoverDir::Up {
                return RelativeMove::Back;
            } else if (ci >= 8 && cj == 8) && dir == HoverDir::Down {
                return RelativeMove::Forward;
            } else if (ci == 12 && cj <= 8) && dir == HoverDir::Left {
                return RelativeMove::Forward;
            } else if (ci == 12 && cj <= 8) && dir == HoverDir::Right {
                return RelativeMove::Back;
            } else if (ci == 8 && cj <= 4) && dir == HoverDir::Left {
                return RelativeMove::Forward;
            } else if (ci == 8 && cj <= 4) && dir == HoverDir::Right {
                return RelativeMove::Back;
            }
        }

        return match dir {
            HoverDir::Right | HoverDir::Up => RelativeMove::Forward,
            HoverDir::Left | HoverDir::Down => RelativeMove::Back,
        };
    }

    pub fn draw_ui(&mut self, tui: &mut Tui) {
        let _ = tui.draw_game_main_screen(self);
    }
}
