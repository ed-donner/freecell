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
    for i in 1..8 {
        let initial: State = State::with_board_number(i);
        let start = Instant::now();
        let final_state: State = solve(initial).expect("Failure");
        let duration = start.elapsed();
        total += duration;
        //final_state.display();
        println!("Solved board number {} in {:?}", i, duration);
    }
    println!("Total time {:?}", total)
}
