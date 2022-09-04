pub type Card = u8;

const RANKS: [&str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K",
];
const SUITS: [&str; 4] = ["♦️", "♠️", "♥️", "♣️"];

pub const EMPTY: Card = 255;
pub const FULL: usize = 254;

#[inline(always)]
pub fn suit(card: Card) -> u8 {
    card / 13
}

#[inline(always)]
pub fn rank(card: Card) -> u8 {
    card % 13
}

#[inline(always)]
pub fn eval(card: Card) -> i32 {
    if card == EMPTY {
        0
    } else {
        ((rank(card) + 1) as i32) * 10
    }
}

#[inline(always)]
pub fn create(rank: u8, suit: u8) -> Card {
    suit * 13 + rank
}

pub fn show(card: Card) -> String {
    if card == EMPTY {
        String::from("   ")
    } else {
        let rank: &str = RANKS[rank(card) as usize];
        let suit: &str = SUITS[suit(card) as usize];
        let result = String::from(rank);
        result + suit
    }
}

#[inline(always)]
pub fn can_stack(card_above: Card, card_below: Card) -> bool {
    (card_below % 13 != 12)
        && (((card_below + 14) % 52 == card_above) || ((card_below + 40) % 52 == card_above))
}
