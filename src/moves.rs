#[derive(Copy, Clone, Hash, Debug)]
pub enum Move {
    CascadeToCascade { from: u8, to: u8, count: u8 },
    CascadeToCell { from: u8, cell: u8 },
    CascadeToFoundation { from: u8 },
    CellToFoundation { cell: u8 },
    CellToCascade { cell: u8, to: u8 },
}

impl Move {
    pub fn display(&self) {
        match *self {
            Move::CascadeToCascade { from, to, count } => println!(
                "Move {} cards from cascade {} to cascade {}",
                count,
                from + 1,
                to + 1
            ),
            Move::CascadeToCell { from, cell } => {
                println!("Move a card from cascade {} to cell {}", from + 1, cell + 1)
            }
            Move::CascadeToFoundation { from } => {
                println!("Move a card from cascade {} to the foundation", from + 1)
            }
            Move::CellToFoundation { cell } => {
                println!("Move a card from cell {} to the foundation", cell + 1)
            }
            Move::CellToCascade { cell, to } => {
                println!("Move a card from cell {} to cascade {}", cell + 1, to + 1)
            }
        };
    }
}
