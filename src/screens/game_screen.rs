use crate::entities::pawn::PawnColor;
use crate::entities::player::Player;
use crate::screens::game_initialization_screen::screen::GameInitializationScreen;
use crate::screens::game_main_screen::screen::GameMainScreen;
use crate::{app::App, tui::Tui};
use crossterm::event::KeyEvent;

use super::game_ending_screen::screen::GameEndingScreen;

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum GamePhase {
    INITIALIZATION,
    MAIN,
    ENDING,
}

pub struct GameScreen<'a> {
    pub should_quit: bool,
    pub previous_phase: GamePhase,
    pub phase: GamePhase,
    pub game_initialization_screen: GameInitializationScreen<'a>,
    pub game_main_screen: Option<GameMainScreen<'a>>,
    pub game_ending_screen: Option<GameEndingScreen>,
}

impl<'a> GameScreen<'a> {
    pub fn new() -> GameScreen<'a> {
        GameScreen {
            should_quit: false,
            previous_phase: GamePhase::INITIALIZATION,
            phase: GamePhase::MAIN,
            game_initialization_screen: GameInitializationScreen::new(),
            game_main_screen: None,
            game_ending_screen: None,
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) {
        match key_event.code {
            _ => match self.phase {
                GamePhase::INITIALIZATION => {
                    self.game_initialization_screen.handle_key_event(key_event);

                    if self.game_initialization_screen.is_game_initialized {
                        self.previous_phase = GamePhase::INITIALIZATION;

                        self.game_main_screen = Some(GameMainScreen::new(
                            self.game_initialization_screen.players.clone(),
                        ));

                        self.phase = GamePhase::MAIN;
                    }
                }
                GamePhase::MAIN => {
                    if let Some(game_main_screen) = self.game_main_screen.as_mut() {
                        game_main_screen.handle_key_event(key_event, app);

                        if app.should_quit == true {
                            self.should_quit = true;
                        }

                        if game_main_screen.is_game_finished {
                            self.previous_phase = GamePhase::MAIN;

                            self.game_ending_screen =
                                Some(GameEndingScreen::new(game_main_screen.game_winner.unwrap()));

                            self.phase = GamePhase::ENDING;
                        }
                    } else {
                        self.game_main_screen = Some(GameMainScreen::new(vec![
                            Player::new(0, 1, PawnColor::RED),
                            Player::new(3, 4, PawnColor::YELLOW),
                        ]));
                    }
                }
                GamePhase::ENDING => {
                    if let Some(game_ending_screen) = self.game_ending_screen.as_mut() {
                        game_ending_screen.handle_key_event(key_event);
                    } else {
                        self.game_ending_screen =
                            Some(GameEndingScreen::new(Player::new(0, 1, PawnColor::RED)));
                    }
                }
            },
        }
    }

    pub fn draw_ui(&mut self, tui: &mut Tui) {
        match self.phase {
            GamePhase::INITIALIZATION => self.game_initialization_screen.draw_ui(tui),
            GamePhase::MAIN => {
                if let Some(ref mut game_main_screen) = self.game_main_screen {
                    game_main_screen.draw_ui(tui)
                }
            }
            GamePhase::ENDING => {
                if let Some(ref mut game_ending_screen) = self.game_ending_screen {
                    game_ending_screen.draw_ui(tui)
                }
            }
        }
    }
}
