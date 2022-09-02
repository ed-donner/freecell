use state::State;

mod board;
mod board_generator;
mod card;
mod moves;
mod state;

fn main() {
    let state: State = State::with_board_number(7);
    state.board.display();
    let states: Vec<State> = state.next_states();
    for state in states {
        state.display();
    }
}
