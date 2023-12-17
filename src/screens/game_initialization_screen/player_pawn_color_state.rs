use crate::entities::pawn::PawnColor;
use crate::custom_widgets::button::{ Button, ButtonState, RED, GREEN, BLUE, YELLOW };

pub fn get_default_player_color_buttons_state<'a>() -> Vec<Button<'a, PawnColor>> {
    vec![
        Button::new("Red").value(PawnColor::RED).theme(RED).state(ButtonState::Selected),
        Button::new("Green").value(PawnColor::GREEN).theme(GREEN).state(ButtonState::Normal),
        Button::new("Blue").value(PawnColor::BLUE).theme(BLUE).state(ButtonState::Normal),
        Button::new("Yellow").value(PawnColor::YELLOW).theme(YELLOW).state(ButtonState::Normal), 
    ]
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerPawnColorState<'a> {
    pub label: usize,
    pub curr_id: usize,
    pub curr_player_id: usize,
    pub taken: Vec<PawnColor>,
    pub options: Vec<Button<'a, PawnColor>>,
}

impl<'a> PlayerPawnColorState<'a> {
    pub fn new() -> PlayerPawnColorState<'a> {
        PlayerPawnColorState {
            label: 1,
            curr_id: 0,
            curr_player_id: 0,
            taken: Vec::new(),
            options: get_default_player_color_buttons_state() 
        }
    }

    pub fn reset_options(&mut self) {
        self.options = get_default_player_color_buttons_state();
    }

    pub fn get_default_options(&mut self) -> Vec<Button<'a, PawnColor>> {
        return get_default_player_color_buttons_state();
    }
}


