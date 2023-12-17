/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;

pub mod custom_widgets;

pub mod screens;

pub mod entities;

pub mod utils;

use app::{App, CurrentScreen};
use tui::Tui;
use color_eyre::Result;
use event::{ Event, EventHandler };
use ratatui::{ backend::CrosstermBackend, Terminal };

use screens::main_menu::MainMenu;
use screens::game_screen::GameScreen;

fn main() -> Result<()> {
    let mut app = App::new();
    
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;

    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let mut main_menu: Option<MainMenu> = None;
    let mut game_screen: Option<GameScreen> = None;

    while !app.should_quit {
        match app.current_screen {
            CurrentScreen::MainMenu => {
                match main_menu {
                    Some(ref mut mm) => {
                        while !mm.should_quit {
                            tui.draw_main_menu(mm);

                            match tui.events.next()? {
                                Event::Tick => {}
                                Event::Key(key_event) => mm.handle_key_event(key_event, &mut app),
                                Event::Mouse(_mouse_event) => {},
                                _ => {}
                            }
                        }

                        main_menu = None;
                    }
                    None => {
                        main_menu = Some(MainMenu::new());
                    }
                }
            },
            CurrentScreen::GameScene => {
                match game_screen {
                    Some(ref mut gs) => {
                        while !gs.should_quit {
                            gs.draw_ui(&mut tui);
                        
                            match tui.events.next()? {
                                Event::Tick => {}
                                Event::Key(key_event) => gs.handle_key_event(key_event, &mut app),
                                Event::Mouse(_mouse_event) => {}
                                Event::Resize(_, _) => {}
                            };
                        }

                        game_screen = None
                    }
                    None => {
                        game_screen = Some(GameScreen::new());
                    }
                }
            },
            _ => {}
            //CurrentScreen::EndScreen => tui.draw(&mut app)?
        }
    }

    tui.exit()?;
    Ok(())
}
