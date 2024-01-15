pub mod app;
pub mod constants;
pub mod custom_widgets;
pub mod entities;
pub mod event;
pub mod macros;
pub mod screens;
pub mod tui;
pub mod ui;
pub mod utils;

use app::{App, CurrentScreen};
use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use screens::game_screen::GameScreen;
use screens::main_menu::MainMenu;
use tui::Tui;

fn main() -> Result<()> {
    let mut app = App::new();
    prepare_debug_log!();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;

    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let mut main_menu: Option<MainMenu> = None;
    let mut game_screen: Option<GameScreen> = None;

    while !app.should_quit {
        match app.current_screen {
            CurrentScreen::MainMenu => match main_menu {
                Some(ref mut mm) => {
                    while !mm.should_quit {
                        let _ = tui.draw_main_menu(mm);

                        match tui.events.next()? {
                            Event::Key(key_event) => mm.handle_key_event(key_event, &mut app),
                            _ => {}
                        }
                    }

                    main_menu = None;
                }
                None => {
                    main_menu = Some(MainMenu::new());
                }
            },
            CurrentScreen::GameScene => match game_screen {
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
            },
            _ => {} //CurrentScreen::EndScreen => tui.draw(&mut app)?
        }
    }

    tui.exit()?;
    Ok(())
}
