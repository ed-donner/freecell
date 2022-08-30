pub type Card = u8;

const RANKS: [&str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K",
];
const SUITS: [&str; 4] = ["♦️", "♠️", "♥️", "♣️"];

pub const EMPTY: Card = 255;

fn suit(card: Card) -> u8 {
    card / 13
}

fn rank(card: Card) -> u8 {
    card % 13
}

pub fn create(rank: u8, suit: u8) -> Card {
    suit * 13 + rank
}

pub fn show(card: Card) -> String {
    if card == EMPTY {
        String::from("---")
    } else {
        let rank: &str = RANKS[rank(card) as usize];
        let suit: &str = SUITS[suit(card) as usize];
        let result = String::from(rank);
        result + suit
    }
}

// pub fn can_stack(card_above: Card, card_below: Card) -> bool {
//     (card_below % 13 == 12)
//         || ((card_below + 14) % 52 == card_above)
//         || ((card_below + 40) % 52 == card_above)
// }
