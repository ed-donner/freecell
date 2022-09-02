use crate::card::{show, Card, EMPTY, FULL};

#[derive(Copy, Clone)]
pub struct Board {
    pub cells: [Card; 4],
    pub foundation: [Card; 4],
    pub cascades: [[Card; 20]; 8],
    pub lengths: [u8; 8],
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [EMPTY, EMPTY, EMPTY, EMPTY],
            foundation: [EMPTY, EMPTY, EMPTY, EMPTY],
            cascades: [[EMPTY; 20]; 8],
            lengths: [0; 8],
        }
    }

    pub fn longest(&self) -> u8 {
        return (0..8)
            .map(|i| self.lengths[i])
            .max()
            .expect("Problem finding number of rows");
    }

    pub fn first_open_cell(&self) -> usize {
        if self.cells[0] == EMPTY {
            return 0;
        }
        if self.cells[1] == EMPTY {
            return 1;
        }
        if self.cells[2] == EMPTY {
            return 2;
        }
        if self.cells[3] == EMPTY {
            return 3;
        }
        return FULL;
    }

    pub fn push(&mut self, col: usize, card: Card) {
        self.cascades[col][(self.lengths[col] as usize)] = card;
        self.lengths[col] += 1;
    }

    pub fn display(&self) {
        print!("\nBOARD\nF ");
        for c in self.foundation {
            print!("[{} ] ", show(c));
        }
        print!(" C ");
        for c in self.cells {
            print!("[{} ] ", show(c));
        }
        print!("\n\n  ");
        for col in 1..9 {
            print!("   {}  ", col);
        }
        for row in 0..self.longest() {
            print!("\n  ");
            for col in 0..8 {
                if row < self.lengths[col] {
                    print!("[{} ] ", show(self.cascades[col][row as usize]));
                } else {
                    print!("[   ] ");
                }
            }
        }
        println!();
    }
}
