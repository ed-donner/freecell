use state::State;

mod board;
mod board_generator;
mod card;
mod state;

fn main() {
    let state: State = State::with_board_number(1);
    state.board.display();
}
