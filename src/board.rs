use crate::card::{eval, show, Card, EMPTY, FULL};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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

    pub fn is_solved(&self) -> bool {
        self.foundation[0] == 12
            && self.foundation[1] == 25
            && self.foundation[2] == 38
            && self.foundation[3] == 51
    }

    // pub fn cascade_starting_with(&self, c: Card) -> [Card; 20] {
    //     for i in 0..8 {
    //         let cascade = self.cascades[i];
    //         if cascade[0] == c {
    //             return cascade;
    //         }
    //     }
    //     return self.cascades[0];
    // }

    // pub fn normalize(&self) {
    //     let mut headers = [EMPTY; 8];
    //     for i in 0..8 {
    //         headers[i] = self.cascades[i][0];
    //     }
    //     headers.sort();
    //     let mut new_cascades = [[EMPTY; 20]; 8];
    //     for i in 0..8 {
    //         new_cascades[i] =
    //     }
    // }

    pub fn count_empty_cells(&self) -> u8 {
        let mut count = 0;
        if self.cells[0] == EMPTY {
            count += 1
        };
        if self.cells[1] == EMPTY {
            count += 1
        };
        if self.cells[2] == EMPTY {
            count += 1
        };
        if self.cells[3] == EMPTY {
            count += 1
        };
        count
    }

    pub fn count_empty_cascades(&self) -> u8 {
        let mut count: u8 = 0;
        if self.lengths[0] == 0 {
            count += 1;
        }
        if self.lengths[1] == 0 {
            count += 1;
        }
        if self.lengths[2] == 0 {
            count += 1;
        }
        if self.lengths[3] == 0 {
            count += 1;
        }
        if self.lengths[4] == 0 {
            count += 1;
        }
        if self.lengths[5] == 0 {
            count += 1;
        }
        if self.lengths[6] == 0 {
            count += 1;
        }
        if self.lengths[7] == 0 {
            count += 1;
        }
        count
    }

    pub fn eval(&self) -> u8 {
        (4 - self.count_empty_cells())
            // + (8 - self.count_empty_cascades())
            + eval(self.foundation[0])
            + eval(self.foundation[1])
            + eval(self.foundation[2])
            + eval(self.foundation[3])
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
