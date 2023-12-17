use crate::custom_widgets::button::{ Button, ButtonState, GREEN, YELLOW };

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerCountState<'a> {
    pub curr_id: usize,
    pub selected_player_count: usize,
    pub options: Vec<Button<'a, usize>>,
}

impl<'a> PlayerCountState<'a> {
    pub fn new() -> PlayerCountState<'a> {
        PlayerCountState {
            curr_id: 0,
            selected_player_count: 0,
            options: vec![
                Button::new("2").value(2).theme(GREEN).state(ButtonState::Active),
                Button::new("3").value(3).theme(YELLOW).state(ButtonState::Active),
                Button::new("4").value(4).theme(YELLOW).state(ButtonState::Active),
            ]
        }
    }
 }
