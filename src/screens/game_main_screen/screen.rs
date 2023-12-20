use crate::entities::pawn::Pawn;
use crate::entities::pawn::PawnColor;
use crate::entities::player::Player;
use crate::tui::Tui;
use crate::utils::roll_dice;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

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
    pub is_game_finished: bool, // indikator da li je igra zavrsena kako bi se mogao pokazati end
    // screen
    pub fields: [Option<Pawn>; 40], // 40 polja gdje svaki moze biti ili None ili Pawn
}

// Klasa koja enkapsulira logiku od momenta kad igra pocne do momenta kad igra zavrsi
impl GameMainScreen {
    pub fn new() -> GameMainScreen {
        GameMainScreen {
            state: GameState::RUNNING,
            current_player_id: 0,
            current_player_roll: None,
            is_game_finished: false,
            fields: [None; 40],
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
                //

                // Dodano kao demo da vidis kako se ispisuje, svaki put kad kliknes doda se jedan
                // pijun pored crvene boje, polja se popunjavaju od srednjeg gornjeg polja u smjeru kazaljke na
                // satu (0 => 40)
                self.fields[self.current_player_id] =
                    Some(Pawn::new(Player::new(PawnColor::RED), PawnColor::RED));

                self.current_player_id = self.current_player_id + 1 % 4;
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
