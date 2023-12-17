use ratatui::layout::Rect;
use crate::app::CurrentScreen;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crate::entities::pawn::Pawn;
use crate::{ app::App, tui::Tui };
use crate::entities::player::Player;
use crate::screens::game_initialization_screen::screen::{ GameInitializationScreen };

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum GameState {
    RUNNING,
    PAUSED
}

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum GamePhase {
    INITIALIZATION,
    MAIN,
    ENDING
}

pub struct Game {
    pub number_of_players: i8,
    pub players: Vec<Player>,
    pub fields: [ Option<Pawn>; 40 ],
}

pub struct GameMainScreen {
    pub state: GameState
}

impl GameMainScreen {
    pub fn new() -> GameMainScreen {
        GameMainScreen {
            state: GameState::RUNNING
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
            _ => {}
        }
    }

    pub fn draw_ui(&mut self, tui: &mut Tui) {
        tui.draw_game_main_screen(self)
    }
}

pub struct GameScreen<'a> {
    pub board: [ [Rect; 11]; 11],
    pub should_quit: bool,
    pub previous_phase: GamePhase,
    pub phase: GamePhase,
    pub game_initialization_screen: GameInitializationScreen<'a>,
    pub game_main_screen: GameMainScreen
}

impl<'a> GameScreen<'a> {
    pub fn new() -> GameScreen<'a> {
        GameScreen {
            should_quit: false,
            board: [[Rect { x: 0, y: 0, width: 0, height: 0 }; 11]; 11],
            previous_phase: GamePhase::INITIALIZATION,
            phase: GamePhase::INITIALIZATION,
            game_initialization_screen: GameInitializationScreen::new(),
            game_main_screen: GameMainScreen::new()
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) {
        match key_event.code {
            KeyCode::Char('q') => { self.should_quit = true; app.should_quit = true },
            KeyCode::Char('w') => { self.phase = GamePhase::MAIN },
            KeyCode::Char('m') => { self.should_quit = true; app.current_screen = CurrentScreen::MainMenu },
            _ => {
               match self.phase {
                    GamePhase::INITIALIZATION => self.game_initialization_screen.handle_key_event(key_event, app),
                    GamePhase::MAIN => self.game_main_screen.handle_key_event(key_event, app),
                    _ => {}
               } 
            }
        }
    }

    pub fn draw_ui(&mut self, tui: &mut Tui) {
        match self.phase {
            GamePhase::INITIALIZATION => self.game_initialization_screen.draw_ui(tui),
            GamePhase::MAIN => self.game_main_screen.draw_ui(tui),
            _ => {}
        }
    }
}


