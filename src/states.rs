mod game_state;
use game_state::GameState;

mod state_enum;
use state_enum::{StateEnum, StateTrait};

use std::collections::HashMap;

pub struct StateManager {
    state_map: HashMap<StateTrait, StateEnum>,
    state_obj: StateTrait,
}

impl Default for StateManager {
    fn default(&self) -> Self {
        StateManager {}
    }
}
