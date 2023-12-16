use crate::app::App;
use crate::entities::pawn::PawnColor;
use crate::tui::Tui;
use crossterm::event::{ KeyEvent, KeyCode };
use crate::screens::game_screen::GameState;
use crate::custom_widgets::button::{ Button, ButtonState, BLUE, GREEN, RED, GRAY, YELLOW };

use crate::entities::player::{Player, self};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum GameInitializationStep {
    PLAYER_NUMBER_SELECTION,
    PLAYER_PAWN_COLOR_SELECTION,
    CONFIRMATION
}

pub struct GameInitializationScreen<'a> {
    pub state: GameState,
    pub step: GameInitializationStep,
    pub players: Vec<Player>,
    pub taken_colors: Vec<PawnColor>,
    pub selecting_player_id: usize,
    pub curr_selected_player_pawn_color_id: usize,
    pub selected_player_count_choice_id: usize,
    pub player_count_choice_buttons: [ Button<'a, i8>; 3 ],
    pub player_color_choice_buttons: [ Button<'a, PawnColor>; 4 ]
}

impl<'a> GameInitializationScreen<'a> {
    pub fn new() -> GameInitializationScreen<'a> {
        GameInitializationScreen {
            state: GameState::RUNNING,
            step: GameInitializationStep::PLAYER_NUMBER_SELECTION,
            players: Vec::new(),
            taken_colors: Vec::new(),
            curr_selected_player_pawn_color_id: 0,
            selected_player_count_choice_id: 0,
            selecting_player_id: 0,
            player_count_choice_buttons: [
                Button::new("2").value(2).theme(GREEN).state(ButtonState::Active),
                Button::new("3").value(3).theme(YELLOW).state(ButtonState::Active),
                Button::new("4").value(4).theme(YELLOW).state(ButtonState::Active),
            ],
            player_color_choice_buttons: [
                Button::new("Red").value(PawnColor::RED).theme(RED).state(ButtonState::Selected),
                Button::new("Green").value(PawnColor::GREEN).theme(GREEN).state(ButtonState::Normal),
                Button::new("Blue").value(PawnColor::BLUE).theme(BLUE).state(ButtonState::Normal),
                Button::new("Yellow").value(PawnColor::YELLOW).theme(YELLOW).state(ButtonState::Normal), 
            ]
        }
    }

    pub fn get_selected_number_of_players(&mut self) -> usize {
        if let Some(button) = self.player_count_choice_buttons.get_mut(self.selected_player_count_choice_id) {
            button.get_value().try_into().unwrap()
        } else {
            0
        }
    }

    pub fn get_button(&self, index: usize) -> Option<Button<i8>> {
        self.player_count_choice_buttons.get(index).cloned()
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
                        self.handle_player_count_select_change((self.selected_player_count_choice_id + self.player_count_choice_buttons.len() - 1) % self.player_count_choice_buttons.len())
                    }
                    GameInitializationStep::PLAYER_PAWN_COLOR_SELECTION => {
                        self.handle_player_color_select_change((self.curr_selected_player_pawn_color_id + self.player_color_choice_buttons.len() - 1) % self.player_color_choice_buttons.len())
                    }
                    _ => {}
                }
            },
            KeyCode::Right => {
                match self.step {
                    GameInitializationStep::PLAYER_NUMBER_SELECTION => {
                        self.handle_player_count_select_change((self.curr_selected_player_pawn_color_id + 1) % self.player_color_choice_buttons.len())
                    }
                    GameInitializationStep::PLAYER_PAWN_COLOR_SELECTION => {
                        self.handle_player_color_select_change((self.curr_selected_player_pawn_color_id + 1) % self.player_color_choice_buttons.len())
                    }
                    _ => {}
                }
            },
            KeyCode::Enter => {
                match self.step {
                    GameInitializationStep::PLAYER_NUMBER_SELECTION => {
                        self.step = GameInitializationStep::PLAYER_PAWN_COLOR_SELECTION
                    }
                    GameInitializationStep::PLAYER_PAWN_COLOR_SELECTION => {
                        if let Ok(_) = self.handle_player_pawn_color_confirmation() {
                            self.step = GameInitializationStep::CONFIRMATION
                        } 
                    }
                    _ => {}
                }
            },
            KeyCode::Backspace => {
                match self.step {
                    GameInitializationStep::PLAYER_PAWN_COLOR_SELECTION => {
                        self.step = GameInitializationStep::PLAYER_NUMBER_SELECTION
                    },
                    GameInitializationStep::CONFIRMATION => {
                        if let Ok("GO_BACK") = self.handle_player_pawn_color_revert() {
                            self.step = GameInitializationStep::PLAYER_PAWN_COLOR_SELECTION;
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }       
    }

    fn handle_player_count_select_change(&mut self, new_button_id: usize) {
        if let Some(old_button) = self.player_count_choice_buttons.get_mut(self.selected_player_count_choice_id) {
            old_button.set_theme(YELLOW);
            old_button.set_state(ButtonState::Normal)
        }

        if let Some(new_button) = self.player_count_choice_buttons.get_mut(new_button_id) {
            new_button.set_theme(GREEN);
            new_button.set_state(ButtonState::Selected);
            self.selected_player_count_choice_id = new_button_id;
        }
    }

    fn handle_player_color_select_change(&mut self, new_button_id: usize) {
        if let Some(old_button) = self.player_color_choice_buttons.get_mut(self.curr_selected_player_pawn_color_id) {
            old_button.set_state(ButtonState::Normal)
        }

        if let Some(new_button) = self.player_color_choice_buttons.get_mut(new_button_id) {
            new_button.set_state(ButtonState::Selected);
            self.curr_selected_player_pawn_color_id = new_button_id;
        }
    }

    fn handle_player_pawn_color_confirmation(&mut self) -> Result<&'static str, &'static str> {
        if let Some(selected_button) = self.player_color_choice_buttons.get_mut(self.curr_selected_player_pawn_color_id) {
            if !self.taken_colors.contains(&selected_button.value) {
                self.taken_colors.push(selected_button.value);
                self.players.push(Player::new(selected_button.value));
                selected_button.set_label(format!("Player {}", self.selecting_player_id + 1));

                self.selecting_player_id += 1;
                selected_button.set_theme(GRAY);
                
                if let Some(first_button) = self.player_color_choice_buttons.get_mut(0) {
                    //first_button.set_state(ButtonState::Active);
                }
            }

            if self.taken_colors.len() == self.get_selected_number_of_players() {
                return Ok("")
            }

            return Err("All players must pick a color!")
        } else {
            return Err("Button not found!")
        }
    }

    pub fn handle_player_pawn_color_revert(&mut self) -> Result<&'static str, &'static str> {
        if self.selecting_player_id == 0 {
           return Ok("GO_BACK")
        }
        
        self.taken_colors.remove(self.selecting_player_id);
        self.selecting_player_id -= 1;

        return Ok("REVERTED");
    }

    pub fn draw_ui(&mut self, tui: &mut Tui) {
        tui.draw_game_initialization_screen(self)
    }
}


