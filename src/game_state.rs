mod state_enum;
use state_enum::StateTrait;

pub struct GameState {}

impl StateTrait for GameState {
    fn update(&self) {}
    fn draw(&self) {}
}
