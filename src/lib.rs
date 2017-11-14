extern crate failure;
#[macro_use]
extern crate failure_derive;

pub mod utilities;
pub mod gamestate;

pub use gamestate::GameState;
pub use gamestate::Player;
