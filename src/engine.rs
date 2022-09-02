use crate::state::State;

pub fn solve(from: State) -> State {
    for next_state in from.next_states() {
        if next_state.board.is_solved() {
            return next_state;
        } else {
            return solve(next_state);
        }
    }
    return from;
}
