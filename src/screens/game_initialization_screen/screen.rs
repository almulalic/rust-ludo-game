use super::player_count_state::PlayerCountState;
use super::player_order_state::{PlayerOrderState, RollState};
use super::player_pawn_color_state::PlayerPawnColorState;
use crate::custom_widgets::button::{Button, ButtonState};
use crate::entities::pawn::PawnColor;
use crate::entities::player::Player;
use crate::screens::game_main_screen::screen::GameState;
use crate::tui::Tui;
use crate::utils::{has_duplicate_values, next_with_wrap, previous_with_wrap, roll_dice};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum GameInitializationStep {
    PlayerNumberSelection,
    PlayerPawnColorSelection,
    PlayerOrderSelection,
    Confirmation,
}

pub struct GameInitializationScreen<'a> {
    pub state: GameState,
    pub players: Vec<Player>,
    pub step: GameInitializationStep,
    pub player_count_state: PlayerCountState<'a>,
    pub player_order_state: PlayerOrderState,
    pub pawn_color_state: PlayerPawnColorState<'a>,
    pub is_game_initialized: bool,
}

impl<'a> GameInitializationScreen<'a> {
    pub fn new() -> GameInitializationScreen<'a> {
        GameInitializationScreen {
            state: GameState::RUNNING,
            players: Vec::new(),
            step: GameInitializationStep::PlayerNumberSelection,
            player_count_state: PlayerCountState::new(),
            pawn_color_state: PlayerPawnColorState::new(),
            player_order_state: PlayerOrderState::new(),
            is_game_initialized: false,
        }
    }

    pub fn get_players(&mut self) -> &Vec<Player> {
        return &self.players;
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                if self.state == GameState::RUNNING {
                    self.state = GameState::PAUSED;
                } else {
                    self.state = GameState::RUNNING;
                }
            }
            KeyCode::Left => match self.step {
                GameInitializationStep::PlayerNumberSelection => {
                    self.handle_player_count_select_change(previous_with_wrap(
                        self.player_count_state.curr_id,
                        &self.player_count_state.options,
                    ));
                }
                GameInitializationStep::PlayerPawnColorSelection => {
                    let previous = self.find_first_available_button::<Button<'a, PawnColor>>(
                        previous_with_wrap(
                            self.pawn_color_state.curr_id,
                            &self.pawn_color_state.options,
                        ),
                        previous_with_wrap,
                    );
                    self.handle_player_color_select_change(previous);
                }
                _ => {}
            },
            KeyCode::Right => match self.step {
                GameInitializationStep::PlayerNumberSelection => self
                    .handle_player_count_select_change(
                        (self.player_count_state.curr_id + 1)
                            % self.player_count_state.options.len(),
                    ),
                GameInitializationStep::PlayerPawnColorSelection => {
                    let next = self.find_first_available_button::<Button<'a, PawnColor>>(
                        next_with_wrap(
                            self.pawn_color_state.curr_id,
                            &self.pawn_color_state.options,
                        ),
                        next_with_wrap,
                    );
                    self.handle_player_color_select_change(next);
                }
                _ => {}
            },
            KeyCode::Enter => match self.step {
                GameInitializationStep::PlayerNumberSelection => {
                    if let Some(button) = self
                        .player_count_state
                        .options
                        .get_mut(self.player_count_state.curr_id)
                    {
                        button.set_state(ButtonState::Active);

                        self.player_count_state.selected_player_count = button.value;
                        self.step = GameInitializationStep::PlayerPawnColorSelection;
                    }
                }
                GameInitializationStep::PlayerPawnColorSelection => {
                    if let Ok(_) = self.handle_player_pawn_color_confirmation() {
                        self.step = GameInitializationStep::PlayerOrderSelection
                    } else {
                        let next = self.find_first_available_button::<Button<'a, PawnColor>>(
                            next_with_wrap(
                                self.pawn_color_state.curr_id,
                                &self.pawn_color_state.options,
                            ),
                            next_with_wrap,
                        );

                        self.handle_player_color_select_change(next);
                    }
                }
                GameInitializationStep::Confirmation => {
                    self.is_game_initialized = true;
                }
                _ => {}
            },
            KeyCode::Char(' ') => {
                if self.step == GameInitializationStep::PlayerOrderSelection {
                    self.handle_player_order_roll();
                }
            }
            KeyCode::Backspace => {
                if self.step == GameInitializationStep::PlayerPawnColorSelection {
                    self.players = Vec::new();
                    self.pawn_color_state = PlayerPawnColorState::new();
                    self.player_count_state = PlayerCountState::new();

                    self.step = GameInitializationStep::PlayerNumberSelection;
                }

                if self.step == GameInitializationStep::PlayerOrderSelection {
                    self.player_order_state = PlayerOrderState::new();
                    self.pawn_color_state = PlayerPawnColorState::new();
                    self.players = Vec::new();

                    self.step = GameInitializationStep::PlayerPawnColorSelection;
                }

                if self.step == GameInitializationStep::Confirmation {
                    self.player_order_state = PlayerOrderState::new();

                    self.step = GameInitializationStep::PlayerOrderSelection;
                }
            }
            _ => {}
        }
    }

    fn find_first_available_button<T>(
        &mut self,
        curr: usize,
        next: fn(usize, &Vec<Button<'a, PawnColor>>) -> usize,
    ) -> usize {
        if self.pawn_color_state.taken.len() == 4 {
            return 0;
        }

        if let Some(button) = self.pawn_color_state.options.get_mut(curr) {
            if self.pawn_color_state.taken.contains(&button.value) {
                return self.find_first_available_button::<Button<PawnColor>>(
                    next(curr, &self.pawn_color_state.options),
                    next,
                );
            } else {
                return curr;
            }
        } else {
            return 0;
        }
    }

    fn handle_player_count_select_change(&mut self, new_button_id: usize) {
        if let Some(old_button) = self
            .player_count_state
            .options
            .get_mut(self.player_count_state.curr_id)
        {
            old_button.set_state(ButtonState::Normal);
        }

        if let Some(new_button) = self.player_count_state.options.get_mut(new_button_id) {
            new_button.set_state(ButtonState::Selected);
            self.player_count_state.curr_id = new_button_id;
        }
    }

    fn handle_player_color_select_change(&mut self, new_button_id: usize) {
        if let Some(old_button) = self
            .pawn_color_state
            .options
            .get_mut(self.pawn_color_state.curr_id)
        {
            if old_button.state != ButtonState::Active {
                old_button.set_state(ButtonState::Normal)
            }
        }

        if let Some(new_button) = self.pawn_color_state.options.get_mut(new_button_id) {
            new_button.set_state(ButtonState::Selected);
            self.pawn_color_state.curr_id = new_button_id;
        }
    }

    fn handle_player_pawn_color_confirmation(&mut self) -> Result<&'static str, &'static str> {
        if let Some(selected_button) = self
            .pawn_color_state
            .options
            .get_mut(self.pawn_color_state.curr_id)
        {
            if !self.pawn_color_state.taken.contains(&selected_button.value) {
                self.pawn_color_state.taken.push(selected_button.value);
                self.players
                    .push(Player::new(self.players.len(), 0, selected_button.value));

                selected_button.set_state(ButtonState::Active);
                selected_button.set_label(format!(
                    "Player {}",
                    self.pawn_color_state.curr_player_id + 1
                ));

                if self.pawn_color_state.curr_player_id < self.pawn_color_state.options.len() - 1 {
                    self.pawn_color_state.curr_player_id += 1;
                }

                if self.pawn_color_state.label < self.player_count_state.selected_player_count {
                    self.pawn_color_state.label += 1;
                }
            }

            if self.pawn_color_state.taken.len() > 0
                && self.pawn_color_state.taken.len()
                    == self.player_count_state.selected_player_count
            {
                return Ok("");
            }

            return Err("All players must pick a color!");
        } else {
            return Err("Button not found!");
        }
    }

    fn handle_player_order_roll(&mut self) {
        match self.player_order_state.roll_state {
            RollState::Initial => {
                if self.player_order_state.curr_id
                    <= self.player_count_state.selected_player_count - 1
                {
                    self.player_order_state.rolled_numbers.insert(
                        self.player_order_state.curr_id,
                        roll_dice().try_into().unwrap(),
                    );
                    self.player_order_state.curr_id += 1;
                }

                if self.player_order_state.curr_id == self.player_count_state.selected_player_count
                {
                    if has_duplicate_values(&self.player_order_state.rolled_numbers) {
                        self.player_order_state.roll_state = RollState::Rethrow;
                    } else {
                        self.step = GameInitializationStep::Confirmation;
                    }
                }
            }
            RollState::Rethrow => {
                self.handle_rethrow(0);
            }
            RollState::RethrowFinished => {}
        }
    }

    fn handle_rethrow(&mut self, retry: usize) {
        if retry > 0
            || self.player_order_state.curr_id == self.player_count_state.selected_player_count
        {
            self.setup_rethrow();
            return;
        }

        self.player_order_state.rolled_numbers.insert(
            self.player_order_state.curr_id,
            roll_dice().try_into().unwrap(),
        );

        if let Some(player_id) = self.player_order_state.reroll_buffer.pop() {
            self.player_order_state.curr_id = player_id;
            self.player_order_state.rolled_numbers.remove(&player_id);
        } else {
            if has_duplicate_values(&self.player_order_state.rolled_numbers) {
                self.handle_rethrow(retry + 1);
            } else {
                self.player_order_state.roll_state = RollState::RethrowFinished;
                self.step = GameInitializationStep::Confirmation;
            }
        }
    }

    fn setup_rethrow(&mut self) {
        self.player_order_state.reroll_buffer.clear();

        for (player_id, roll) in self.player_order_state.rolled_numbers.iter() {
            if self
                .player_order_state
                .rolled_numbers
                .values()
                .filter(|&value| value == roll)
                .count()
                > 1
            {
                self.player_order_state.reroll_buffer.push(*player_id);
            }
        }

        self.player_order_state.reroll_buffer.reverse();
        if let Some(first_duplicate) = self.player_order_state.reroll_buffer.pop() {
            self.player_order_state.curr_id = first_duplicate;
            self.player_order_state
                .rolled_numbers
                .remove(&first_duplicate);
        }
    }

    pub fn draw_ui(&mut self, tui: &mut Tui) {
        let _ = tui.draw_game_initialization_screen(self);
    }
}
