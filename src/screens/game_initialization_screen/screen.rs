use std::collections::BTreeMap;

use crate::app::App;
use crate::tui::Tui;
use crate::entities::player::{Player, self};
use crate::entities::pawn::PawnColor;
use crate::screens::game_screen::GameState;
use crossterm::event::{ KeyEvent, KeyCode };
use super::player_count_state::PlayerCountState;
use super::player_pawn_color_state::PlayerPawnColorState;
use super::player_order_state::{PlayerOrderState, RollState};
use crate::custom_widgets::button::{ Button, ButtonState, GREEN,YELLOW };
use crate::utils::{ previous_with_wrap, next_with_wrap, roll_dice, has_duplicates, has_duplicate_values };

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum GameInitializationStep {
    PLAYER_NUMBER_SELECTION,
    PLAYER_PAWN_COLOR_SELECTION,
    PLAYER_ORDER_SELECTION,
    CONFIRMATION
}

pub struct GameInitializationScreen<'a> {
    pub state: GameState,
    pub players: Vec<Player>,
    pub step: GameInitializationStep,
    pub count_state: PlayerCountState<'a>,
    pub player_order_state: PlayerOrderState,
    pub pawn_color_state: PlayerPawnColorState<'a>,
}

impl<'a> GameInitializationScreen<'a> {
    pub fn new() -> GameInitializationScreen<'a> {
        GameInitializationScreen {
            state: GameState::RUNNING,
            players: Vec::new(),
            step: GameInitializationStep::PLAYER_NUMBER_SELECTION,
            count_state: PlayerCountState::new(),
            pawn_color_state: PlayerPawnColorState::new(),
            player_order_state: PlayerOrderState::new()
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) {
         match key_event.code {
            KeyCode::Esc => {
                if self.state == GameState::RUNNING {
                    self.state = GameState::PAUSED;
                } else {
                    self.state = GameState::RUNNING;
                }
            },
            KeyCode::Left => {
                match self.step {
                    GameInitializationStep::PLAYER_NUMBER_SELECTION => { 
                        self.handle_player_count_select_change(previous_with_wrap(self.count_state.curr_id, &self.count_state.options));
                    }
                    GameInitializationStep::PLAYER_PAWN_COLOR_SELECTION => {
                        let previous = self.find_first_available_button::<Button<'a, PawnColor>>(
                            previous_with_wrap::<>(self.pawn_color_state.curr_id, &self.pawn_color_state.options), 
                            previous_with_wrap
                        );
                        self.handle_player_color_select_change(previous);
                    }
                    _ => {}
                }
            },
            KeyCode::Right => {
                match self.step {
                    GameInitializationStep::PLAYER_NUMBER_SELECTION => {
                        self.handle_player_count_select_change((self.count_state.curr_id + 1) % self.count_state.options.len())
                    }
                    GameInitializationStep::PLAYER_PAWN_COLOR_SELECTION => {
                        let next = self.find_first_available_button::<Button<'a, PawnColor>>(
                            next_with_wrap::<>(self.pawn_color_state.curr_id, &self.pawn_color_state.options), 
                            next_with_wrap
                        );
                        self.handle_player_color_select_change(next);
                    }
                    _ => {}
                }
            },
            KeyCode::Enter => {
                match self.step {
                    GameInitializationStep::PLAYER_NUMBER_SELECTION => {
                        if let Some(button) = self.count_state.options.get(self.count_state.curr_id) {
                            self.count_state.selected_player_count = button.value;
                            self.step = GameInitializationStep::PLAYER_PAWN_COLOR_SELECTION;
                        }
                    }
                    GameInitializationStep::PLAYER_PAWN_COLOR_SELECTION => {
                        if let Ok(_) = self.handle_player_pawn_color_confirmation() {
                            self.step = GameInitializationStep::PLAYER_ORDER_SELECTION
                        } else {
                            let next = self.find_first_available_button::<Button<'a, PawnColor>>(
                                next_with_wrap::<>(self.pawn_color_state.curr_id, &self.pawn_color_state.options), 
                                next_with_wrap
                            );
                            
                            self.handle_player_color_select_change(next);
                        }
                    }
                    _ => {}
                }
            },
            KeyCode::Char(' ') => {
                if self.step == GameInitializationStep::PLAYER_ORDER_SELECTION {
                    self.handle_player_order_roll();
                }
            }
            KeyCode::Backspace => {
                match self.step {
                    GameInitializationStep::PLAYER_PAWN_COLOR_SELECTION => {
                        if let Ok("GO_BACK") = self.handle_player_pawn_color_revert() {
                            self.step = GameInitializationStep::PLAYER_NUMBER_SELECTION;
                            self.pawn_color_state.reset_options();
                        }
                    },
                    GameInitializationStep::CONFIRMATION => {
                        self.step = GameInitializationStep::PLAYER_PAWN_COLOR_SELECTION;
                        
                        self.players = Vec::new();
                        self.pawn_color_state = PlayerPawnColorState::new()
                    },
                    _ => {}
                }
            },
            _ => {}
        }       
    }

    fn find_first_available_button<T>(&mut self, curr: usize, next: fn(usize, &Vec<Button<'a, PawnColor>>) -> usize) -> usize {
        if self.pawn_color_state.taken.len() == 4 {
            return 0;
        }
        
        if let Some(button) = self.pawn_color_state.options.get_mut(curr) {
            if self.pawn_color_state.taken.contains(&button.value) {
                return self.find_first_available_button::<Button<PawnColor>>(next(curr, &self.pawn_color_state.options), next);
            } else {
                return curr;
            }
        } else {
            return 0;
        }
    }

    fn handle_player_count_select_change(&mut self, new_button_id: usize) {
        if let Some(old_button) = self.count_state.options.get_mut(self.count_state.curr_id) {
            old_button.set_theme(YELLOW);
            old_button.set_state(ButtonState::Normal)
        }

        if let Some(new_button) = self.count_state.options.get_mut(new_button_id) {
            new_button.set_theme(GREEN);
            new_button.set_state(ButtonState::Selected);
            self.count_state.curr_id = new_button_id;
        }
    }

    fn handle_player_color_select_change(&mut self, new_button_id: usize) {
        if let Some(old_button) = self.pawn_color_state.options.get_mut(self.pawn_color_state.curr_id) {
            old_button.set_state(ButtonState::Normal)
        }

        if let Some(new_button) = self.pawn_color_state.options.get_mut(new_button_id) {
            new_button.set_state(ButtonState::Selected);
            self.pawn_color_state.curr_id = new_button_id;
        }
    }

    fn handle_player_pawn_color_confirmation(&mut self) -> Result<&'static str, &'static str> {
        if let Some(selected_button) = self.pawn_color_state.options.get_mut(self.pawn_color_state.curr_id) {
            if !self.pawn_color_state.taken.contains(&selected_button.value) {
                self.pawn_color_state.taken.push(selected_button.value);
                self.players.push(Player::new(selected_button.value));
                
                selected_button.set_label(format!("Player {}", self.pawn_color_state.curr_player_id + 1));

                if self.pawn_color_state.curr_player_id < self.pawn_color_state.options.len() - 1 {
                    self.pawn_color_state.curr_player_id += 1;
                }

                if self.pawn_color_state.label < self.count_state.selected_player_count {
                    self.pawn_color_state.label += 1;
                }
            }

            if self.pawn_color_state.taken.len() == self.count_state.selected_player_count {
                return Ok("")
            }

            return Err("All players must pick a color!")
        } else {
            return Err("Button not found!")
        }
    }

    pub fn handle_player_pawn_color_revert(&mut self) -> Result<&'static str, &'static str> {
        if self.pawn_color_state.taken.len() == 0 {
           return Ok("GO_BACK")
        }
        
        if let Some(button) = self.pawn_color_state.options.get_mut(self.pawn_color_state.curr_id) {
            if self.pawn_color_state.taken.contains(&button.value) {
                self.pawn_color_state.taken.retain(|&pawn_color| pawn_color != button.value);
                self.players.retain(|&player| player.pawn_color == button.value);

                button.set_state(ButtonState::Normal);

                self.pawn_color_state.curr_player_id -= 1;

                if self.pawn_color_state.label > 1 {
                    self.pawn_color_state.label -= 1;

                }
            }

            return Ok("REVERTED");
        }

        return Err("BUTTON_NOT_VALID");
    }

    fn handle_player_order_roll(&mut self) {
        match self.player_order_state.roll_state {
            RollState::Initial => {
                if self.player_order_state.curr_id <= self.count_state.selected_player_count - 1 {
                    self.player_order_state.rolled_numbers.insert(self.player_order_state.curr_id, roll_dice().try_into().unwrap());
                    self.player_order_state.curr_id += 1;
                }

                if self.player_order_state.curr_id == self.count_state.selected_player_count {

                    if has_duplicate_values(&self.player_order_state.rolled_numbers) {
                        self.player_order_state.curr_id = 0;
                        self.player_order_state.roll_state = RollState::Rethrow
                    } else {
                        //for (i, (_player, order)) in self.player_order_state.rolled_numbers.iter().enumerate() {
                        //    if let Some(player) = self.players.get_mut(i) {
                        //        player.set_order(*order);
                        //    }
                        // }
                        //
                            
                        self.step = GameInitializationStep::CONFIRMATION
                    }
                }
            }
            RollState::Rethrow => {
                //if self.player_order_state.curr_id <= self.count_state.selected_player_count - 1 {
                //    if 
                //}
            }
        }
    }

    fn get_first_duplicate_value() {}
    

    pub fn draw_ui(&mut self, tui: &mut Tui) {
        tui.draw_game_initialization_screen(self)
    }
}

