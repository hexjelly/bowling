extern crate bowling_scorer;
use bowling_scorer::GameState;

#[test]
fn adding_new_players() {
    let mut state = GameState::new();
    assert_eq!(0, state.player_count());
    state.add_player("Joe");
    assert_eq!(1, state.player_count());
    state.add_player("Chloe");
    assert_eq!(2, state.player_count());
}
