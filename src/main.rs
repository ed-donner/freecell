use engine::solve;
use state::State;
use std::time::{Duration, Instant};

mod board;
mod board_generator;
mod card;
mod engine;
mod moves;
mod state;

fn main() {
    let mut total: Duration = Duration::from_secs(0);
    let first_board = 1;
    let last_board = 1000;
    for i in first_board..(last_board + 1) {
        let initial: State = State::with_board_number(i);
        let start = Instant::now();
        let final_state: State = solve(initial).expect("Failure");
        let duration = start.elapsed();
        total += duration;
        //final_state.display();
        println!(
            "Solved board number {} in {:?} with {:} moves",
            i,
            duration,
            final_state.moves.len()
        );
    }
    println!("Total time {:?}", total);
    println!("Average time {:?}", total / (last_board - first_board + 1));
}
