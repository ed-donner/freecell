use crate::state::State;
use priority_queue::PriorityQueue;
use rustc_hash::FxHashSet;
use std::collections::HashSet;

pub fn solve(initial: State) -> Result<State, State> {
    let mut queue: PriorityQueue<State, u8> = PriorityQueue::new();
    let mut history = HashSet::new();
    let mut count: u64 = 0;
    let mut history_hit: u64 = 0;
    let priority: u8 = initial.board.eval();
    queue.push(initial, priority);
    while queue.len() > 0 {
        let (next, _) = queue.pop().unwrap();
        count = count + 1;
        if count % 500000 == 0 {
            //next.display();
            println!(
                "Reached {} iterations with {} history hit",
                count, history_hit
            );
        }
        if !history.contains(&next.board) {
            if next.is_solved() {
                return Ok(next);
            }
            for more in next.next_states() {
                let priority = more.eval();
                queue.push(more, priority);
            }
            history.insert(next.board);
        } else {
            history_hit += 1;
        }
    }
    return Err(State::with_board_number(1));
}
