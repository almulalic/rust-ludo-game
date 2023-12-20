use crate::app::CurrentScreen;
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

// Game screen enkapsulira 3 glavna ekrana/faze igrice, inicijalizaciju, glavnu igru i onaj end screen,
// (ekran u kojem pise pobjedio je igrac bla bla)
// Igrac kad tek pokrene igricu sa New Game prvo ulazi u ovaj ekran gdje ga doceka initialization
// screen
pub struct GameScreen<'a> {
    pub should_quit: bool,         // da li treba zavrsiti glavni dio igrice?
    pub previous_phase: GamePhase, // ja sam ovo nazvao phase ili faza igrice, u sustini ovo pamti
    // koja je bila zadnja faza igrice kako bi se mogao vratiti na nju po potrebi
    pub phase: GamePhase, // ovo predstavlja trenutnu fazu igrice kako bi znao kojoj komponenti da
    // prosljedi klikove koje se dese
    pub game_initialization_screen: GameInitializationScreen<'a>, // ovo je instanca ekrana kojeg
    // sam ja zavrsio za inicijalizaciju, on je prvi step i dok god on ne vrati da se moze
    // nastaviti dalje korisnik ostaje na tom ekranu
    pub game_main_screen: GameMainScreen, // ovo je ekran na kojem ti radis, on dolazi nakon sto
                                          // moj ekran kaze da je igra spremna, kad igra bude spremna, varijabla game_main_screen.game ce
                                          // biti inicijalizovana sa informacijama koje su dosle sa initialization screena
                                          //
                                          // ovde fali jos game_end_screen al moze na kraju
}

impl<'a> GameScreen<'a> {
    pub fn new() -> GameScreen<'a> {
        GameScreen {
            should_quit: false,
            previous_phase: GamePhase::INITIALIZATION,
            phase: GamePhase::MAIN, // inicijalno stanje, ono koje se dobije kad igrac
            // tek krene igru
            game_initialization_screen: GameInitializationScreen::new(),
            game_main_screen: GameMainScreen::new(),
        }
    }

    // Ovde dolaze svi eventi/klikovi koje korisnik klikne dok je na GameScreen ekranu (ekran koji
    // enkapsulira inicijalizaciju, igricu i end screen), Zavisno od faze u kojoj se igrica nalazi,
    // toj klasi se prosljede eventi jer ona jedina zna sta da sa njima raadi
    pub fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) {
        match key_event.code {
            // izlaz iz igrice
            KeyCode::Char('q') => {
                self.should_quit = true;
                app.should_quit = true
            }
            // za testiranje su oba ispod, ignorisi
            KeyCode::Char('w') => self.phase = GamePhase::MAIN,
            KeyCode::Char('m') => {
                self.should_quit = true;
                app.current_screen = CurrentScreen::MainMenu
            }
            // ovo znaci, svaki drugi char koji nije nijedan od ovih iznad prosljedi dole (ovi
            // iznad su dodani kako bi se neke akcije handlale prije nego sto uopste dodje do
            // klase)
            _ => match self.phase {
                // dok god traje initialization phase, tjst ovo na cemu sam ja radio, moja klasa
                // uzima i parsira sve klikove koje se dese jer su meni jedino bitno, kad ova klasa
                // vrati da je igrica uspjesno inicijalizovana trenutna faza se mjenja na Main i
                // onda tvoja klasa preuzima evente
                GamePhase::INITIALIZATION => {
                    self.game_initialization_screen.handle_key_event(key_event);

                    // Ako je igrica uspjesno inicijalizovana
                    if self.game_initialization_screen.is_game_initialized {
                        // Stavi da je staro stanje bila inicijalizacija ako se igrac bude htio
                        // vracati
                        self.previous_phase = GamePhase::INITIALIZATION;
                        // I postavi trenutno stanje na main sto znaci da ce se svaki naredni klik
                        // slati u tvoju klasu jer je sad ona zaduzena za prikaz
                        self.phase = GamePhase::MAIN;
                    }
                }
                GamePhase::MAIN => {
                    // Tvoja klasa sad handla evente i ako ti postavis da je igirca gotova
                    // prikazuje se ekran ko je pobjedio
                    self.game_main_screen.handle_key_event(key_event);

                    if self.game_main_screen.is_game_finished {
                        self.previous_phase = GamePhase::MAIN;
                        self.phase = GamePhase::ENDING;
                    }
                }
                _ => {}
            },
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
