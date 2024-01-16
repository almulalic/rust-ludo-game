use std::{io, panic};

use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{
    screens::{
        game_ending_screen::{screen::GameEndingScreen, ui::render_game_ending_screen},
        game_initialization_screen::{
            screen::GameInitializationScreen, ui::render_game_initialization_screen,
        },
        game_main_screen::screen::GameMainScreen,
        pause_menu::{screen::PauseMenu, ui::render_pause_menu},
    },
    ui::render_game_main_screen,
};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

use crate::event::EventHandler;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
pub struct Tui {
    /// Interface to the Terminal.
    terminal: CrosstermTerminal,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl Tui {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // Define a custom panic hook to reset the terminal properties.
        // This way, you won't have your terminal messed up if an unexpected error happens.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw_pause_menu(&mut self, pause_menu: &mut PauseMenu) -> Result<()> {
        self.terminal
            .draw(|frame| render_pause_menu(pause_menu, frame))?;
        Ok(())
    }

    pub fn draw_game_initialization_screen(
        &mut self,
        game_initialization_screen: &mut GameInitializationScreen,
    ) -> Result<()> {
        let _ = self
            .terminal
            .draw(|frame| render_game_initialization_screen(game_initialization_screen, frame));
        Ok(())
    }

    pub fn draw_game_main_screen(&mut self, game_main_screen: &mut GameMainScreen) -> Result<()> {
        let _ = self
            .terminal
            .draw(|frame| render_game_main_screen(game_main_screen, frame));
        Ok(())
    }

    pub fn draw_game_ending_screen(
        &mut self,
        game_ending_screen: &mut GameEndingScreen,
    ) -> Result<()> {
        let _ = self
            .terminal
            .draw(|frame| render_game_ending_screen(game_ending_screen, frame));
        Ok(())
    }

    /// Resets the terminal interface.
    ///
    /// This function is also used for the panic hook to revert
    /// the terminal properties if unexpected errors occur.
    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
