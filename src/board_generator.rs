use crate::board::Board;
use crate::card::{create, Card};
pub use rand::{Rng, SeedableRng};

pub struct MsLcg {
    state: u32,
}

impl Rng for MsLcg {
    // Similarly, this outputs in the range [0, 32767] and should output a `u8`.  Concatenate
    // four `next_u8`s for serious usage.
    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(214_013).wrapping_add(2_531_011);
        self.state %= 1 << 31;
        self.state >> 16 // rand_n = state_n / 2^16
    }
}

impl SeedableRng<u32> for MsLcg {
    fn from_seed(seed: u32) -> Self {
        Self { state: seed }
    }
    fn reseed(&mut self, seed: u32) {
        self.state = seed;
    }
}

fn shuffle<T>(rng: &mut MsLcg, deck: &mut [T]) {
    let len = deck.len() as u32;
    for i in (1..len).rev() {
        let j = rng.next_u32() % (i + 1);
        deck.swap(i as usize, j as usize);
    }
}

fn gen_deck() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for i in 0..52 {
        let (suit_old, rank) = (i % 4, i / 4);
        let suit = match suit_old {
            1 => 0,
            0 => 3,
            3 => 1,
            other => other,
        };
        let card: Card = create(rank as u8, suit as u8);
        cards.push(card);
    }
    cards
}

fn deal_ms_fc_board(seed: u32) -> Vec<Card> {
    let mut rng = MsLcg::from_seed(seed);
    let mut deck: Vec<Card> = gen_deck();

    shuffle(&mut rng, &mut deck);
    deck.reverse();
    deck
}

pub fn generate(seed: u32) -> Board {
    let deck: Vec<Card> = deal_ms_fc_board(seed);
    let mut board = Board::new();
    for (i, card) in deck.iter().enumerate() {
        let cascade = i % 8;
        board.cascades[cascade].push(*card);
    }
    board
}
