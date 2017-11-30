use super::Player;
use super::Throw;

#[derive(Clone)]
pub enum CatchupHandling {
    Disabled,
    Quick,
    Gradual,
}

impl Default for CatchupHandling {
    fn default() -> CatchupHandling {
        CatchupHandling::Disabled
    }
}

#[derive(Clone, Default)]
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

    pub fn to_json(&self) {
        unimplemented!();
    }

    pub fn add_player<P: Into<Player>>(&mut self, player: P) {
        self.players.push(player.into());
    }

    pub fn add_throw(&self, _throw: Throw) {
        unimplemented!();
    }

    pub fn turn(&self) -> &str {
        unimplemented!();
    }

    pub fn set_catchup_handling(&mut self, catchup_handling: CatchupHandling) {
        self.catchup = catchup_handling;
    }

    pub fn player_count(&self) -> usize {
        self.players.len()
    }
}
