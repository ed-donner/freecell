use crate::card::{show, Card, EMPTY};

pub struct Board {
    pub cells: [Card; 4],
    pub foundation: [Card; 4],
    pub cascades: [Vec<Card>; 8],
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [EMPTY, EMPTY, EMPTY, EMPTY],
            foundation: [EMPTY, EMPTY, EMPTY, EMPTY],
            cascades: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
        }
    }

    pub fn rows(&self) -> usize {
        return (0..8)
            .map(|i| self.cascades[i].len())
            .max()
            .expect("Problem finding number of rows");
    }

    pub fn display(&self) {
        print!("\nBOARD\nF ");
        for c in self.foundation {
            print!("[{}] ", show(c));
        }
        print!(" C ");
        for c in self.cells {
            print!("[{}] ", show(c));
        }
        print!("\n\n  ");
        for col in 1..9 {
            print!("   {}  ", col);
        }
        for row in 0..self.rows() {
            print!("\n  ");
            for col in 0..8 {
                let cascade_length = self.cascades[col].len();
                if row < cascade_length {
                    print!("[{} ] ", show(self.cascades[col][row]));
                } else {
                    print!("[   ] ");
                }
            }
        }
        println!();
    }
}
