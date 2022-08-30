mod board;
mod board_generator;
mod card;

fn main() {
    let board = board_generator::generate(1);
    board.display();
}
