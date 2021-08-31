mod state;
fn main() {
    let mut state = state::State::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let state_unchanged = state;
    assert_eq!(state.perft(6), 119060324);
    assert_eq!(state_unchanged, state);
}
