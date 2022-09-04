use crate::board::BoardRecord;
use crate::state::State;
use std::collections::BinaryHeap;
use std::collections::HashSet;

pub fn solve(initial: State) -> Result<State, State> {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut history: HashSet<BoardRecord> = HashSet::new();
    let mut count: u64 = 0;
    let mut history_hit: u64 = 0;
    heap.push(initial);
    while heap.len() > 0 {
        let next = heap.pop().unwrap();
        count = count + 1;
        if count % 1000000 == 0 {
            //next.display();
            println!(
                ">> {}M iterations with {}K history hit and {}K heap and {}K history",
                count / 1000000,
                history_hit / 1000,
                heap.len() / 1000,
                history.len() / 1000
            );
        }
        let record: BoardRecord = next.board.as_record();
        if !history.contains(&record) {
            for more in next.next_states() {
                if more.is_solved() {
                    return Ok(more);
                }
                if !history.contains(&more.board.as_record()) {
                    heap.push(more);
                } else {
                    history_hit += 1
                }
            }
            history.insert(record);
        } else {
            history_hit += 1;
        }
    }
    return Err(State::with_board_number(1));
}
