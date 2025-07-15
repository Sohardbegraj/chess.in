mod utils;
mod game;

use game::*;

fn main() {
    let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = Game::read_FEN(fen_str);

    // let (first_row, rest) = split_on(fen_str, ' ');
    // println!("First: {}, second: {}", first_row, rest);
    println!("{}", game.to_string());
    println!(
        "{:?}, {:?}, {}",
        game.active_color, game.en_passant, game.fullmove_number
    );
}
