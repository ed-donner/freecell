use crate::board::Board;
use crate::board_generator;
use crate::card;
use crate::card::{can_stack, rank, suit, Card, EMPTY, FULL};
use crate::moves::Move;

#[derive(Clone, Hash, Debug)]
pub struct State {
    pub board: Board,
    pub moves: Vec<Move>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl Eq for State {}

impl State {
    pub fn with_board_number(board_number: u32) -> State {
        let board: Board = board_generator::generate(board_number);
        State {
            board: board,
            moves: Vec::new(),
        }
    }

    pub fn display(&self) {
        println!("STATE");
        for m in &self.moves {
            m.display();
        }
        self.board.display();
        println!("__________________");
    }

    pub fn is_solved(&self) -> bool {
        self.board.is_solved()
    }

    pub fn eval(&self) -> u8 {
        let mut penalty = 4;
        let depth = self.moves.len();
        if depth > 100 {
            penalty -= 1;
        }
        if depth > 200 {
            penalty -= 1;
        }
        if depth > 300 {
            penalty -= 1;
        }
        if depth > 400 {
            penalty -= 1;
        }
        penalty = 1;
        penalty + self.board.eval()
    }

    pub fn do_c2c(
        &self,
        states: &mut Vec<State>,
        from: usize,
        from_len: usize,
        to: usize,
        to_len: usize,
        count: usize,
    ) {
        let mut next: State = self.clone();
        for i in 0..count {
            next.board.cascades[to][to_len + i] = next.board.cascades[from][from_len - 1 - i];
            next.board.cascades[from][from_len - 1 - i] = EMPTY;
        }
        next.board.lengths[to] += count as u8;
        next.board.lengths[from] -= count as u8;
        let m = Move::CascadeToCascade {
            from: from as u8,
            to: to as u8,
            count: count as u8,
        };
        next.moves.push(m);
        states.push(next);
    }

    pub fn do_cell2c(
        &self,
        states: &mut Vec<State>,
        cell: usize,
        from_card: Card,
        to: usize,
        to_len: usize,
    ) {
        let mut next: State = self.clone();
        next.board.cascades[to][to_len] = from_card;
        next.board.lengths[to] += 1;
        next.board.cells[cell] = EMPTY;
        let m = Move::CellToCascade {
            cell: cell as u8,
            to: to as u8,
        };
        next.moves.push(m);
        states.push(next);
    }

    pub fn do_c2f(
        &self,
        states: &mut Vec<State>,
        from: usize,
        from_len: usize,
        from_card: Card,
        suit: u8,
    ) {
        let mut next: State = self.clone();
        next.board.foundation[suit as usize] = from_card;
        next.board.cascades[from][from_len - 1] = EMPTY;
        next.board.lengths[from] -= 1;
        let m: Move = Move::CascadeToFoundation { from: from as u8 };
        next.moves.push(m);
        states.push(next);
    }

    pub fn do_cell2f(&self, states: &mut Vec<State>, cell: usize, card: Card, suit: u8) {
        let mut next: State = self.clone();
        next.board.foundation[suit as usize] = card;
        next.board.cells[cell] = EMPTY;
        let m: Move = Move::CellToFoundation { cell: cell as u8 };
        next.moves.push(m);
        states.push(next);
    }

    pub fn do_c2cell(
        &self,
        states: &mut Vec<State>,
        from: usize,
        from_len: usize,
        from_card: Card,
        cell: usize,
    ) {
        let mut next: State = self.clone();
        next.board.cells[cell] = from_card;
        next.board.cascades[from][from_len - 1] = EMPTY;
        next.board.lengths[from] -= 1;
        let m: Move = Move::CascadeToCell {
            from: from as u8,
            cell: cell as u8,
        };
        next.moves.push(m);
        states.push(next);
    }

    pub fn c2c(
        &self,
        states: &mut Vec<State>,
        from: usize,
        from_len: usize,
        to: usize,
        count: usize,
    ) {
        let to_len = self.board.lengths[to] as usize;
        if to_len == 0 {
            self.do_c2c(states, from, from_len, to, 0, count);
        } else {
            let to_card = self.board.cascades[to][to_len - 1];
            let from_card: Card = self.board.cascades[from][from_len - count];
            if can_stack(to_card, from_card) {
                self.do_c2c(states, from, from_len, to, to_len, count);
            }
        }
    }

    pub fn c2c_inner(
        &self,
        states: &mut Vec<State>,
        from: usize,
        from_len: usize,
        to: usize,
        combo_max: usize,
    ) {
        let mut finished = false;
        let mut count: usize = 1;
        while !finished {
            finished = true;
            self.c2c(states, from, from_len, to, count);
            if (count < combo_max) && ((count + 1) <= from_len) {
                let card_above = self.board.cascades[from][from_len - count - 1];
                let card_below = self.board.cascades[from][from_len - count];
                if can_stack(card_above, card_below) {
                    finished = false;
                    count += 1;
                }
            }
        }
    }

    pub fn c2c_loop(&self, states: &mut Vec<State>) {
        let combo_max = self.combo_max();
        for from in 0..8 {
            let from_len = self.board.lengths[from] as usize;
            if from_len != 0 {
                for to in 0..8 {
                    if from != to {
                        self.c2c_inner(states, from, from_len, to, combo_max);
                    }
                }
            }
        }
    }

    pub fn c2f_loop(&self, states: &mut Vec<State>) {
        for from in 0..8 {
            let from_len = self.board.lengths[from] as usize;
            if from_len != 0 {
                let from_card = self.board.cascades[from][from_len - 1];
                let suit = suit(from_card);
                let rank = rank(from_card);
                let foundation = self.board.foundation[suit as usize];
                let f_rank = card::rank(foundation);
                if ((rank == 0) && (foundation == EMPTY))
                    || ((foundation != EMPTY) && (rank == f_rank + 1))
                {
                    self.do_c2f(states, from, from_len, from_card, suit);
                }
            }
        }
    }

    pub fn c2cell_loop(&self, states: &mut Vec<State>) {
        let cell: usize = self.board.first_open_cell();
        if cell != FULL {
            for from in 0..8 {
                let from_len = self.board.lengths[from] as usize;
                if from_len != 0 {
                    let from_card = self.board.cascades[from][from_len - 1];
                    self.do_c2cell(states, from, from_len, from_card, cell);
                }
            }
        }
    }

    pub fn cell2f_loop(&self, states: &mut Vec<State>) {
        for cell in 0..4 {
            let card: Card = self.board.cells[cell];
            if card != EMPTY {
                let suit = suit(card);
                let rank = card::rank(card);
                let foundation = self.board.foundation[suit as usize];
                let f_rank = card::rank(foundation);
                if ((rank == 0) && (foundation == EMPTY))
                    || ((foundation != EMPTY) && (rank == f_rank + 1))
                {
                    self.do_cell2f(states, cell, card, suit);
                }
            }
        }
    }

    pub fn combo_max(&self) -> usize {
        let two: u32 = 2;
        let m = self.board.count_empty_cascades();
        let n = self.board.count_empty_cells();
        (two.pow(m as u32) * (n as u32 + 1)) as usize
    }

    pub fn cell2c_loop(&self, states: &mut Vec<State>) {
        for cell in 0..4 {
            let from_card: Card = self.board.cells[cell];
            if from_card != EMPTY {
                for to in 0..8 {
                    let to_len = self.board.lengths[to] as usize;
                    if to_len == 0 {
                        self.do_cell2c(states, cell, from_card, to, 0);
                    } else {
                        let to_card = self.board.cascades[to][to_len - 1];
                        if can_stack(to_card, from_card) {
                            self.do_cell2c(states, cell, from_card, to, to_len);
                        }
                    }
                }
            }
        }
    }

    pub fn next_states(&self) -> Vec<State> {
        let mut states: Vec<State> = Vec::new();
        self.cell2f_loop(&mut states);
        self.c2f_loop(&mut states);
        self.cell2c_loop(&mut states);
        self.c2c_loop(&mut states);
        self.c2cell_loop(&mut states);

        states
    }
}
