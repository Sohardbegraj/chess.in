use crate::utils::*;
use bitflags::bitflags;
use std::collections::VecDeque;

type PiecePosition = u64;

pub fn bit_to_position(bit: PiecePosition) -> Result<String, String> {
    if bit == 0 {
        return Err("No piece present!".to_string());
    } else {
        let onebit_index = bit_scan(bit);
        return Ok(index_to_position(onebit_index));
    }
}

pub fn position_to_index(position: &str) -> Result<usize, String> {
    let bitboard = position_to_bit(position)?;

    Ok(bit_scan(bitboard))
}

pub fn position_to_bit(position: &str) -> Result<PiecePosition, String> {
    if position.len() != 2 {
        return Err(format!(
            "Invalid length: {}, string: '{}'",
            position.len(),
            position
        ));
    }

    let bytes = position.as_bytes();
    let byte0 = bytes[0];
    if byte0 < 97 || byte0 >= 97 + 8 {
        return Err(format!(
            "Invalid column character: {}, string: '{}'",
            byte0 as char, position
        ));
    }

    let column = (byte0 - 97) as u32;

    let byte1 = bytes[1];
    let row;

    match (byte1 as char).to_digit(10) {
        Some(number) => {
            if number < 1 || number > 8 {
                return Err(format!(
                    "Invalid row character: {}, string: '{}'",
                    byte1 as char, position
                ));
            } else {
                row = number - 1;
            }
        }
        None => {
            return Err(format!(
                "Invalid row character: {}, string '{}'",
                byte1 as char, position
            ))
        }
    }

    let square_number = row * 8 + column;
    let bit = (1 as u64) << square_number;

    Ok(bit)
}

static COL_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

pub fn index_to_position(index: usize) -> String {
    let column = index % 8;
    let row = index / 8 + 1;
    return format!("{}{}", COL_MAP[column], row);
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}
use Color::*;

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Piece {
    pub position: PiecePosition,
    pub color: Color,
    pub piece_type: PieceType,
    pub alive: bool,
}

impl Piece {
    fn to_string(&self) -> String {
        let mut result = match self.piece_type {
            PieceType::Pawn => "p ",
            PieceType::Rook => "r ",
            PieceType::Knight => "n ",
            PieceType::Bishop => "b ",
            PieceType::Queen => "q ",
            PieceType::King => "k ",
        }
        .to_string();

        if self.color == Color::White {
            result.make_ascii_uppercase();
        }

        result
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Square {
    Empty,
    Occupied(usize),
}
use Square::*;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct CastlingRights: u8 {
        const NONE = 0;
        const WHITEKINGSIDE = 1 << 0;
        const WHITEQUEENSIDE = 1 << 1;
        const BLACKKINGSIDE = 1 << 2;
        const BLACKQUEENSIDE = 1 << 3;
        const ALL =
            Self::WHITEKINGSIDE.bits()
            | Self::WHITEQUEENSIDE.bits()
            | Self::BLACKKINGSIDE.bits()
            | Self::BLACKQUEENSIDE.bits();
    }
}

// Position type to own the data
#[derive(Clone)]
pub struct Position {
    
    pub pieces: Vec<Piece>,
    pub squares: Vec<Square>,
    pub active_color: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<PiecePosition>,
    pub halfmove_clock: usize,
    pub fullmove_number: usize,

    pub white_occupancy: Bitboard,
    pub black_occupancy: Bitboard,
}

impl Position {
    fn push_piece_and_square(
        &mut self,
        position: usize,
        color: Color,
        piece_type: PieceType,
        index: &mut usize,
    ) {
        self.pieces.push(Piece {
            position: (1 as u64) << position,
            color: color,
            piece_type: piece_type,
            alive: true,
        });
        self.squares.push(Square::Occupied(*index));

        let bitboard = 1 << position;
        match color {
            Black => self.black_occupancy |= bitboard,
            White => self.white_occupancy |= bitboard,
        }

        *index += 1;
    }

    fn push_empty_square(&mut self) {
        self.squares.push(Square::Empty);
    }

    pub fn new() -> Position {
        Position::read_FEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn to_string(&self) -> String {
        let mut board = "".to_owned();
        let mut temp = "".to_owned();

        for (i, square) in self.squares.iter().enumerate() {
            match square {
                Square::Empty => temp.push_str(". "), //temp.push_str(&index_to_position(i)),
                Square::Occupied(idx) => temp.push_str(&self.pieces[*idx].to_string()),
            }

            if (i + 1) % 8 == 0 {
                temp.push_str("\n");
                board.insert_str(0, &temp);
                temp.clear();
            }
        }
        board.insert_str(0, &temp);

        board
    }

    pub fn empty() -> Self {
        Position::read_FEN("8/8/8/8/8/8/8/8 w - - 0 1")
    }

    #[allow(non_snake_case)]
    pub fn read_FEN(fen: &str) -> Position {
        let mut game = Position {
            pieces: vec![],
            squares: vec![],
            active_color: Color::White,
            castling_rights: CastlingRights::ALL,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            white_occupancy: 0,
            black_occupancy: 0,
        };

        let (position, rest) = split_on(fen, ' ');

        let mut deque_squares = VecDeque::new();
        let mut piece_index = 0;
        let mut piece_position = 64;

        for row in position.splitn(8, |ch| ch == '/') {
            piece_position -= 8;
            let (pieces, squares) = parse_row(&row, piece_index, piece_position);

            for p in pieces {
                let position = p.position;
                match p.color {
                    Black => game.black_occupancy |= position,
                    White => game.white_occupancy |= position,
                }
                game.pieces.push(p);
                piece_index += 1;
            }
            for s in squares {
                deque_squares.push_front(s);
            }
        }

        game.squares = Vec::from(deque_squares);

        let (color_to_move, rest) = split_on(rest, ' ');
        game.active_color = match color_to_move {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Unknown color designator: '{}'", color_to_move),
        };

        let (castling_rights, rest) = split_on(rest, ' ');
        let mut castling = CastlingRights::NONE;
        for ch in castling_rights.chars() {
            match ch {
                'K' => castling |= CastlingRights::WHITEKINGSIDE,
                'Q' => castling |= CastlingRights::WHITEQUEENSIDE,
                'k' => castling |= CastlingRights::BLACKKINGSIDE,
                'q' => castling |= CastlingRights::BLACKQUEENSIDE,
                '-' => (),
                other => panic!("Invalid character in castling rights: '{}'", other),
            }
        }
        game.castling_rights = castling;

        let (en_passant, rest) = split_on(rest, ' ');
        match en_passant {
            "-" => game.en_passant = None,
            s => match position_to_bit(s) {
                Err(msg) => panic!("{}", msg),
                Ok(bit) => game.en_passant = Some(bit),
            },
        };

        let (halfmove_clock, rest) = split_on(rest, ' ');
        match halfmove_clock.parse() {
            Ok(number) => game.halfmove_clock = number,
            Err(_) => panic!("Invalid halfmove: {}", halfmove_clock),
        }

        let (fullmove_number, rest) = split_on(rest, ' ');
        match fullmove_number.parse() {
            Ok(number) => game.fullmove_number = number,
            Err(_) => panic!("Invalid halfmove: {}", fullmove_number),
        }

        game
    }

    pub fn count_pieces(&self) -> usize {
        self.pieces.iter().filter(|p| p.alive).count()
    }

    pub fn move_piece(self: &mut Self, piece_position: PiecePosition, new_position: usize) {
        let square_index = bit_scan(piece_position);
        // Index into piece list
        let square = self.squares[square_index];
        let piece_index = match square {
            Empty => panic!("Tried to move a piece from an empty square"),
            Occupied(idx) => idx,
        };
        self.pieces[piece_index].position = 1 << new_position;
        self.squares[square_index] = Empty;

        // Check if destination square is occupied
        match self.squares[new_position] {
            Empty => self.squares[new_position] = Occupied(piece_index),
            Occupied(other_idx) => {
                // Check that it's not occupied by a piece of the same color as piece
                let other_piece = &self.pieces[other_idx];
                if self.pieces[piece_index].color == other_piece.color {
                    panic!("Cannot move a piece onto a square occupied by one of it's own color");
                }

                // Remove other piece from piece list
                self.pieces[other_idx].alive = false;
                // Update the square
                self.squares[new_position] = Occupied(piece_index);
            }
        }

        match self.pieces[piece_index].piece_type {
            PieceType::Pawn => {
                let (old_row, col) = rowcol(square_index);
                let (new_row, _) = rowcol(new_position);
                if (old_row - new_row).abs() == 2 {
                    let direction = (new_row - old_row).signum();
                    let pawn_left = self.has_pawn(old_row + 2 * direction, col + 1);
                    let pawn_right = self.has_pawn(old_row + 2 * direction, col - 1);

                    if pawn_left || pawn_right {
                        let idx = index(old_row + 1 * direction, col).unwrap() as usize;
                        self.en_passant = Some(1 << idx);
                    } else {
                        self.en_passant = None;
                    }
                }
            }
            _ => self.en_passant = None,
        }
    }

    fn has_pawn(&self, row: i32, col: i32) -> bool {
        if let Some(idx) = index(row, col) {
            let square = self.squares[idx];
            match square {
                Occupied(pidx) => self.pieces[pidx].piece_type == PieceType::Pawn,
                Empty => false,
            }
        } else {
            false
        }
    }

    pub fn take_en_passant(
        self: &mut Self,
        piece_position: PiecePosition,
        new_position: PiecePosition,
    ) {
        // Move the pawn
        // Take the other pawn
        // Clear en passant field
        let square_index = bit_scan(piece_position);
        // Index into piece list
        let square = self.squares[square_index];
        let piece_index = match square {
            Empty => panic!("Tried to move a piece from an empty square"),
            Occupied(idx) => idx,
        };
        self.pieces[piece_index].position = new_position;
        self.squares[square_index] = Empty;

        // Check if destination square is occupied
        if let Occupied(_) = self.squares[bit_scan(new_position)] {
            panic!("Tried to take en passant onto an occupied square");
        }

        let (old_row, old_col) = rowcol(square_index);
        let (new_row, new_col) = rowcol(bit_scan(new_position));
        let direction = (new_row - old_row).signum();

        let taken_square = (new_row - direction, new_col);
        let taken_index = index(taken_square.0, taken_square.1).unwrap();
        match self.squares[taken_index] {
            Occupied(pidx) => {
                self.pieces[pidx].alive = false;
                self.squares[taken_index] = Empty;
            }
            Empty => panic!("Tried to en passant but there was no pawn in expected square"),
        }

        self.en_passant = None;
    }

    pub fn perform_promotion(
        &mut self,
        piece_position: PiecePosition,
        new_index: usize,
        promotion_type: PieceType,
    ) {
        let square_index = bit_scan(piece_position);
        // Index into piece list
        let square = self.squares[square_index];
        let piece_index = match square {
            Empty => panic!("Tried to move a piece from an empty square"),
            Occupied(idx) => idx,
        };
        let current_piece = self.pieces[piece_index];

        let new_position = 1 << new_index as u64;
        self.pieces[piece_index].alive = false;
        self.squares[square_index] = Empty;
        // Check if destination square is occupied
        if let Occupied(_) = self.squares[new_index] {
            panic!("Tried to promote onto an occupied square");
        }

        macro_rules! promote {
            ($type:ident) => {{
                let promoted_piece = Piece {
                    position: new_position,
                    color: current_piece.color,
                    piece_type: PieceType::$type,
                    alive: true,
                };
                self.pieces.push(promoted_piece);
                self.squares[new_index] = Occupied(self.pieces.len() - 1);
            }};
        }

        match promotion_type {
            PieceType::Queen => promote!(Queen),
            PieceType::Bishop => promote!(Bishop),
            PieceType::Knight => promote!(Knight),
            PieceType::Rook => promote!(Rook),
            other => panic!("{}", format!("Cannot promote to {other:?}!")),
        }
    }

    pub fn add(self: &mut Self, piece_color: Color, piece_type: PieceType, square: &str) {
        let index = square_to_index(square);

        let piece = Piece {
            position: 1 << index,
            color: piece_color,
            piece_type: piece_type,
            alive: true,
        };

        self.pieces.push(piece);
        match self.squares[index] {
            Empty => self.squares[index] = Occupied(self.pieces.len() - 1),
            _ => panic!("Square {} is already occupied", index),
        }

        match piece_color {
            White => self.white_occupancy |= 1 << index,
            Black => self.black_occupancy |= 1 << index,
        }
    }
}

pub fn square_to_index(square: &str) -> usize {
    assert_eq!(square.as_bytes().len(), 2);
    // square "e3" -> 'e'
    // 'a' -> 63 -> 0
    // 'b' -> 64 -> 1
    // ...
    // '1'
    let col: u8 = square.as_bytes()[0] - ('a' as u8);
    let row: u8 = square.as_bytes()[1] - ('1' as u8);

    (row * 8 + col) as usize
}

fn parse_row(
    row: &str,
    mut piece_index: usize,
    mut piece_position: usize,
) -> (Vec<Piece>, VecDeque<Square>) {
    let mut pieces = Vec::new();
    let mut squares = VecDeque::new();

    let mut color;

    macro_rules! add_piece {
        ($piece_type:ident) => {{
            let piece = Piece {
                color: color,
                position: (1 as u64) << piece_position,
                piece_type: PieceType::$piece_type,
                alive: true,
            };
            let square = Square::Occupied(piece_index);
            pieces.push(piece);
            squares.push_front(square);
            piece_position += 1;
            piece_index += 1;
        }};
    }

    for ch in row.chars() {
        let is_upper = ch.is_ascii_uppercase();
        color = if is_upper { Color::White } else { Color::Black };
        match ch.to_ascii_lowercase() {
            'r' => add_piece!(Rook),
            'n' => add_piece!(Knight),
            'b' => add_piece!(Bishop),
            'q' => add_piece!(Queen),
            'k' => add_piece!(King),
            'p' => add_piece!(Pawn),
            num => match num.to_digit(10) {
                None => panic!("Invalid input: {}", num),
                Some(number) => {
                    for i in 0..number {
                        squares.push_front(Square::Empty);
                        piece_position += 1;
                    }
                }
            },
        }
    }

    (pieces, squares)
}

#[cfg(test)]
mod tests {
    use super::PieceType::*;
    use super::*;

    fn get_initial_position() -> Position {
        let mut game = Position {
            pieces: vec![],
            squares: vec![],
            active_color: Color::White,
            castling_rights: CastlingRights::ALL,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            white_occupancy: 0,
            black_occupancy: 0,
        };
        let mut piece_index = 0;

        let color = Color::White;

        game.push_piece_and_square(0, color, PieceType::Rook, &mut piece_index);
        game.push_piece_and_square(1, color, PieceType::Knight, &mut piece_index);
        game.push_piece_and_square(2, color, PieceType::Bishop, &mut piece_index);
        game.push_piece_and_square(3, color, PieceType::Queen, &mut piece_index);
        game.push_piece_and_square(4, color, PieceType::King, &mut piece_index);
        game.push_piece_and_square(5, color, PieceType::Bishop, &mut piece_index);
        game.push_piece_and_square(6, color, PieceType::Knight, &mut piece_index);
        game.push_piece_and_square(7, color, PieceType::Rook, &mut piece_index);

        for i in 8..16 {
            game.push_piece_and_square(i, color, PieceType::Pawn, &mut piece_index);
        }

        for i in 16..48 {
            game.push_empty_square();
        }

        let color = Color::Black;
        for i in 48..56 {
            game.push_piece_and_square(i, color, PieceType::Pawn, &mut piece_index);
        }

        let offset = 56;
        game.push_piece_and_square(0 + offset, color, PieceType::Rook, &mut piece_index);
        game.push_piece_and_square(1 + offset, color, PieceType::Knight, &mut piece_index);
        game.push_piece_and_square(2 + offset, color, PieceType::Bishop, &mut piece_index);
        game.push_piece_and_square(3 + offset, color, PieceType::Queen, &mut piece_index);
        game.push_piece_and_square(4 + offset, color, PieceType::King, &mut piece_index);
        game.push_piece_and_square(5 + offset, color, PieceType::Bishop, &mut piece_index);
        game.push_piece_and_square(6 + offset, color, PieceType::Knight, &mut piece_index);
        game.push_piece_and_square(7 + offset, color, PieceType::Rook, &mut piece_index);

        game
    }

    #[test]
    fn read_initial_position() {
        let game = Position::new();
        let default = get_initial_position();
        assert_eq!(game.active_color, Color::White);
        assert_eq!(game.castling_rights, CastlingRights::ALL);
        assert_eq!(game.en_passant, None);
        assert_eq!(game.halfmove_clock, 0);
        assert_eq!(game.fullmove_number, 1);
        for i in 0..64 {
            match (game.squares[i], default.squares[i]) {
                (Square::Empty, Square::Empty) => (),
                (Square::Occupied(idx1), Square::Occupied(idx2)) => {
                    assert_eq!(game.pieces[idx1], default.pieces[idx2])
                }
                _ => panic!("Wrong square at index {}", i),
            }
        }
    }

    #[test]
    fn read_fen_black_active() {
        let game =
            Position::read_FEN("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b - - 1 2");
        assert_eq!(game.active_color, Color::Black);
    }

    #[test]
    fn read_fen_no_castling() {
        let game =
            Position::read_FEN("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b - - 1 2");
        assert_eq!(game.castling_rights, CastlingRights::NONE);
    }

    #[test]
    fn read_fen_en_passant_allowed() {
        let en_passant_square = "g7";
        let game = Position::read_FEN(&format!(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq {} 1 2",
            en_passant_square
        ));
        assert_eq!(
            game.en_passant,
            Some(position_to_bit(en_passant_square).unwrap())
        );
    }

    #[test]
    fn read_fen_moveclocks() {
        let game =
            Position::read_FEN("rnbqkbnr/pp1ppppp/7P/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b - g7 1 2");
        assert_eq!(game.halfmove_clock, 1);
        assert_eq!(game.fullmove_number, 2);
    }

    #[test]
    fn read_all_possible_castling_rights() {
        let mut rights = "".to_owned();
        let right_chars = ["K", "Q", "k", "q"];
        for i in 0..(2 ^ 4) {
            let bitflag_rights = CastlingRights::from_bits(i).unwrap();
            for j in 0..4 {
                if (i >> j) & 1 != 0 {
                    rights.push_str(right_chars[j]);
                }
            }
            let fen = format!(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w {} - 0 1",
                rights
            );
            let game = Position::read_FEN(&fen);
            assert_eq!(
                game.castling_rights, bitflag_rights,
                "FEN: {}\n\n i: {}",
                fen, i
            );
            rights.clear();
        }
    }

    #[test]
    fn test_occupancy_start_position() {
        let start = Position::new();
        let mut white_occupancy = 0;
        for i in 0..16 {
            white_occupancy |= 1 << i;
        }
        let mut black_occupancy = 0;
        for i in 48..64 {
            black_occupancy |= 1 << i;
        }

        assert_eq!(start.white_occupancy, white_occupancy);
        assert_eq!(start.black_occupancy, black_occupancy);
    }

    #[test]
    fn test_move_piece() {
        let mut game = Position::new();
        game.move_piece(1 << 0, 16);

        assert_eq!(game.pieces[24].position, 1 << 16);
        assert_eq!(game.squares[0], Empty);
        assert_eq!(game.squares[16], Occupied(24));
        assert_eq!(game.en_passant, None);
    }

    #[test]
    fn test_en_passant_is_set() {
        let mut game = Position::empty();
        game.add(White, Pawn, "d2");
        game.add(Black, Pawn, "c4");

        game.move_piece(game.pieces[0].position, position_to_index("d4").unwrap());
        assert_eq!(game.en_passant, Some(position_to_bit("d3").unwrap()));
    }

    #[test]
    fn test_make_empty() {
        let position = Position::empty();
    }

    #[test]
    fn test_add_to_empty() {
        let mut position = Position::empty();
        position.add(White, Rook, "e3");
        assert_eq!(position.pieces.len(), 1);
        assert_eq!(position.pieces[0].position, 1 << 20);
    }

    #[test]
    #[should_panic]
    fn test_cannot_add_to_occupied_square() {
        let mut position = Position::empty();
        position.add(White, Rook, "e3");
        position.add(White, Rook, "e3");
    }

    #[test]
    fn test_add_has_correct_occupancy() {
        let mut position = Position::empty();
        position.add(White, Rook, "e3");
        assert_eq!(position.white_occupancy, 1 << 20);
        assert_eq!(position.black_occupancy, 0);
    }

    #[test]
    fn test_has_pawn() {
        let mut pos = Position::empty();
        pos.add(White, Pawn, "f7");
        pos.add(White, King, "g2");
        assert!(pos.has_pawn(7, 6));
        assert!(!pos.has_pawn(2, 7));
    }

    #[test]
    fn test_take_en_passant() {
        let mut game = Position::empty();
        game.add(White, Pawn, "d2");
        game.add(Black, Pawn, "c4");
        game.move_piece(game.pieces[0].position, position_to_index("d4").unwrap());

        let c4_index = position_to_index("c4").unwrap();
        let d3_index = position_to_index("d3").unwrap();
        game.take_en_passant(1 << c4_index as u64, 1 << d3_index as u64);

        let alive_count = game.pieces.iter().filter(|p| p.alive).count();
        assert_eq!(alive_count, 1);
    }

    #[test]
    #[should_panic]
    fn test_cannot_take_en_passant() {
        let mut game = Position::empty();
        game.add(White, Pawn, "d2");
        game.add(Black, Pawn, "c4");
        game.move_piece(game.pieces[0].position, position_to_index("d4").unwrap());
        let c4_index = position_to_index("c4").unwrap();
        let b3_index = position_to_index("b3").unwrap();
        game.take_en_passant(1 << c4_index as u64, 1 << b3_index as u64);
    }
}
