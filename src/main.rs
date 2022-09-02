use engine::solve;
use state::State;

mod board;
mod board_generator;
mod card;
mod engine;
mod moves;
mod state;

fn main() {
    let initial: State = State::with_board_number(7);
    let final_state: State = solve(initial);
    final_state.display();
}
