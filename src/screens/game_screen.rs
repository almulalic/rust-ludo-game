use crate::app::CurrentScreen;
use crate::entities::pawn::PawnColor;
use crate::entities::player::Player;
use crate::screens::game_initialization_screen::screen::GameInitializationScreen;
use crate::screens::game_main_screen::screen::GameMainScreen;
use crate::{app::App, tui::Tui};
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

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
    pub game_main_screen: Option<GameMainScreen>,
}

impl<'a> GameScreen<'a> {
    pub fn new() -> GameScreen<'a> {
        GameScreen {
            should_quit: false,
            previous_phase: GamePhase::INITIALIZATION,
            phase: GamePhase::INITIALIZATION,
            game_initialization_screen: GameInitializationScreen::new(),
            game_main_screen: None,
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) {
        match key_event.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
                app.should_quit = true
            }
            KeyCode::Char('w') => self.phase = GamePhase::MAIN,
            KeyCode::Char('m') => {
                self.should_quit = true;
                app.current_screen = CurrentScreen::MainMenu
            }
            _ => match self.phase {
                GamePhase::INITIALIZATION => {
                    self.game_initialization_screen.handle_key_event(key_event);

                    if self.game_initialization_screen.is_game_initialized {
                        self.previous_phase = GamePhase::INITIALIZATION;

                        //self.game_main_screen = Some(GameMainScreen::new(
                        //    self.game_initialization_screen.players.clone(),
                        //));
                        //
                        //
                        if self.game_main_screen.is_none() {
                            self.game_main_screen = Some(GameMainScreen::new(
                                self.game_initialization_screen.players.clone(),
                            ));

                            self.phase = GamePhase::MAIN;
                        }
                    }
                }
                GamePhase::MAIN => {
                    if let Some(game_main_screen) = self.game_main_screen.as_mut() {
                        game_main_screen.handle_key_event(key_event);

                        if game_main_screen.is_game_finished {
                            self.previous_phase = GamePhase::MAIN;
                            self.phase = GamePhase::ENDING;
                        }
                    } else {
                        self.game_main_screen = Some(GameMainScreen::new(vec![
                            Player::new(0, 1, PawnColor::RED),
                            Player::new(3, 4, PawnColor::YELLOW),
                        ]));
                    }
                }
                _ => {}
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
            _ => {}
        }
    }
}
