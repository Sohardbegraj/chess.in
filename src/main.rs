mod knightattacks;
mod movegeneration;
mod pawnattack;
mod position;
mod rayattack;
mod utils;

use knightattacks::KnightAttacks;
use position::*;
use rayattack::Rays;

pub struct Game {
    position: Position,
    knight_attacks: KnightAttacks,
    ray_attacks: Rays,
}

impl Game {
    fn new() -> Self {
        Self {
            position: Position::new(),
            knight_attacks: KnightAttacks::new(),
            ray_attacks: Rays::new(),
        }
    }

    #[allow(non_snake_case)]
    fn read_FEN(fen: &str) -> Self {
        Self {
            position: Position::read_FEN(fen),
            knight_attacks: KnightAttacks::new(),
            ray_attacks: Rays::new(),
        }
    }

    fn empty() -> Self {
        Self {
            position: Position::empty(),
            knight_attacks: KnightAttacks::new(),
            ray_attacks: Rays::new(),
        }
    }

    fn add(mut self, piece_color: Color, piece_type: PieceType, square: &str) -> Self {
        self.position.add(piece_color, piece_type, square);
        self
    }
}

fn main() {
    let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let not_alot = "8/8/8/4N3/8/8/8/8 w - - 0 1";
    let not_alot2 = "8/8/8/4N3/2N5/8/8/8 w - - 0 1";
    let game = Position::read_FEN(not_alot2);

    // let (first_row, rest) = split_on(fen_str, ' ');
    // println!("First: {}, second: {}", first_row, rest);
    println!("{}", game.to_string());
    println!(
        "{:?}, {:?}, {}",
        game.active_color, game.en_passant, game.fullmove_number
    );
}
