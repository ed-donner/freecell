pub type Card = u8;

const RANKS: [&str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K",
];
const SUITS: [&str; 4] = ["♦️", "♠️", "♥️", "♣️"];

pub const EMPTY: Card = 255;

// pub fn from_str(card_given: &str) -> Card {
//     let rank_given_str: &str = &card_given[0..1];
//     let suit_given_str: &str = &card_given[1..2];

//     let rank_given: u8 = {
//         match rank_given_str {
//             "A" => 1,
//             "T" => 10,
//             "J" => 11,
//             "Q" => 12,
//             "K" => 13,
//             _ => rank_given_str.trim().parse().expect("Invalid rank"),
//         }
//     };

//     let rank_given = rank_given - 1;

//     let suit_given: u8 = {
//         match suit_given_str {
//             "D" => 0,
//             "S" => 1,
//             "H" => 2,
//             "C" => 3,
//             _ => 0,
//         }
//     };

//     create(rank_given, suit_given)
// }

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
