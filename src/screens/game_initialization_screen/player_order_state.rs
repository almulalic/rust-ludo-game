use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollState {
    Initial,
    Rethrow
}

pub struct PlayerOrderState {
    pub curr_id: usize,
    pub rolled_numbers: BTreeMap<usize, usize>,
    pub roll_state: RollState
}

impl PlayerOrderState<> {
    
    pub fn new() -> PlayerOrderState {
        PlayerOrderState {
            curr_id: 0,
            rolled_numbers: BTreeMap::new(),
            roll_state: RollState::Initial
        }
    }

}
