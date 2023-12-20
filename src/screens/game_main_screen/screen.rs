use crate::tui::Tui;
use crate::utils::roll_dice;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum GameState {
    RUNNING,
    PAUSED,
}

pub struct GameMainScreen {
    pub state: GameState, // Ovo sluzi za pause menu, mozes ignorisati mislim da ovo moze bolje
    pub current_player_id: usize, // pokazuje na igraca koji trenutni igr
    pub current_player_roll: Option<usize>, // broj koji je trenutni igrac bacio (bacanje od
    // current_player_id)
    pub board: [[Rect; 11]; 11], // UI ploca koja ce prikazati 11x11 matricu polja (neka sakrivena, neka
    // kucice) od kojih ce 40 biti mapirano iz niza Game.fields
    pub is_game_finished: bool, // indikator da li je igra zavrsena kako bi se mogao pokazati end
                                // screen
}

// Klasa koja enkapsulira logiku od momenta kad igra pocne do momenta kad igra zavrsi
impl GameMainScreen {
    pub fn new() -> GameMainScreen {
        GameMainScreen {
            state: GameState::RUNNING,
            current_player_id: 0,
            current_player_roll: None,
            is_game_finished: false,
            board: [[Rect {
                x: 0,
                y: 0,
                width: 3,
                height: 3,
            }; 11]; 11],
        }
    }

    // key_even koji je dobio od GameScreen klase koja je to dobila od main klase, mislim da ovde
    // mozes vidjet kako to "Delegiranje odgovornosti" funkcionise
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            // Logika za popup pause screen
            KeyCode::Esc => {
                if self.state == GameState::RUNNING {
                    self.state = GameState::PAUSED;
                } else {
                    self.state = GameState::RUNNING;
                }
            }
            KeyCode::Char('1') | KeyCode::Char('2') | KeyCode::Char('3') | KeyCode::Char('4') => {
                let char = key_event.code;

                // Pozove se svaki put kad igrac klikne neki od navedenih brojeva, posto igrica
                // nema boja da li je igrac u ovom momentu smjeo kliknuti ovaj broj ovde prvo treba
                // biti logika da se provjeri da li igrac koji igra ima validnog pijuna broj X i da
                // li sa njim (u odnosu na dobijeni broj) moze igrati, a ako moze igrati onda sta
                // sve smije raditi. Kad prodju svi ti ifovi za provjeru, ako je igrac odigrao
                // dobar potez, povecaj current_player_id za 1 ili daj igracu ponovo da baca ako je
                // dobio 6
                //
                self.current_player_id += 1;
                self.current_player_roll = None;
            }
            KeyCode::Char(' ') => {
                // Bacanje kockice, u sustini bi trebalo biti prije strelica, tako ako igrac nema
                // bacen broj, dozvoli mu da baci ali ako nema nemoj
                //
                if self.current_player_roll.is_none() {
                    self.current_player_roll = Some(roll_dice().try_into().unwrap());
                }
            }
            _ => {}
        }
    }

    pub fn draw_ui(&mut self, tui: &mut Tui) {
        let _ = tui.draw_game_main_screen(self);
    }
}
