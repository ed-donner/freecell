mod board;
mod board_generator;
mod card;

fn main() {
    let board = board_generator::generate(1);
    board.display();
}

// fn main() {
//     println!("Enter card 1");
//     let mut card1_str = String::new();
//     io::stdin()
//         .read_line(&mut card1_str)
//         .expect("Failed to read line");
//     let card1: Card = from_str(&card1_str);
//     let c1_str = show(card1);

//     println!("Enter card 2");
//     let mut card2_str = String::new();
//     io::stdin()
//         .read_line(&mut card2_str)
//         .expect("Failed to read line");
//     let card2: Card = from_str(&card2_str);
//     let c2_str = show(card2);

//     let stackable = can_stack(card1, card2);

//     println!("First card is {c1_str}, and second card is {c2_str}");
//     if stackable {
//         println!("You can stack them");
//     } else {
//         println!("You can't stack them");
//     }
// }
