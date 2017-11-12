use super::Player;
use super::Throw;

pub enum CatchupHandling {
    Disabled, /* Quick,
               * Gradual, */
}

impl Default for CatchupHandling {
    fn default() -> CatchupHandling {
        CatchupHandling::Disabled
    }
}

#[derive(Default)]
pub struct GameState {
    players: Vec<Player>,
    catchup: CatchupHandling,
}

impl GameState {
    pub fn new() -> GameState {
        GameState::default()
    }

    pub fn reset(&mut self) {
        self.players.clear();
    }

    pub fn render_scores(&self) {
        unimplemented!();
    }

    pub fn add_throw(&self, throw: Throw) {
        unimplemented!();
    }

    pub fn get_turn(&self) -> String {
        unimplemented!();
    }
}
