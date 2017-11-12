use super::Player;

pub struct GameState {
    players: Vec<Player>,
}

impl GameState {
    pub fn reset(&mut self) {
        &self.players.clear();
    }
}
