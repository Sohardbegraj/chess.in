#[allow(unused_imports)]
use crate::knightattacks::*;
use crate::position::PieceType::*;
use crate::position::*;
use crate::rayattack::*;
use crate::utils::{bit_scan, extract_bits, index, rowcol};
use crate::Game;

fn generate_moves(game: &Game) -> Vec<Position> {
    let mut new_positions = vec![];

    for piece in &game.position.pieces {
        if piece.color == game.position.active_color {
            let positions;
            match &piece.piece_type {
                Knight => {
                    positions = generate_knight_moves(&piece, &game);
                }
                Bishop => {
                    positions = generate_bishop_moves(&piece, &game);
                }
                Rook => {
                    positions = generate_rook_moves(&piece, &game);
                }
                Queen => {
                    positions = generate_queen_moves(&piece, &game);
                }
                King => {
                    positions = generate_king_moves(&piece, &game);
                }
                Pawn => {
                    positions = generate_pawn_moves(&piece, &game);
                }
            }

            for pos in positions {
                if king_is_in_check(
                    &pos,
                    game.position.active_color,
                    &game.ray_attacks,
                    &game.knight_attacks,
                ) {
                    continue;
                }
                new_positions.push(pos);
            }
        }
    }

    new_positions
}

macro_rules! check_if_attacked {
    ($rays:expr, $forward:expr, $king:ident, $position:ident, $types:expr) => {{
        let (own_occupancy, enemy_occupancy) = match $king.color {
            Color::White => ($position.white_occupancy, $position.black_occupancy),
            Color::Black => ($position.black_occupancy, $position.white_occupancy),
        };

        let ray_attacks = blocked_ray_attack(
            $rays[bit_scan($king.position)],
            &$rays,
            $forward,
            own_occupancy,
            enemy_occupancy,
        );

        let overlap = ray_attacks & enemy_occupancy;
        if overlap == 0 {
            false
        } else {
            let locations = extract_bits(overlap);
            let mut is_attacked = false;
            for loc in locations {
                let square = $position.squares[loc];
                match square {
                    Square::Empty => panic!("Something has gone wrong"),
                    Square::Occupied(pidx) => {
                        let attacker = $position.pieces[pidx];
                        assert!(attacker.alive);
                        for atype in $types {
                            is_attacked |= attacker.piece_type == atype;
                            if is_attacked {
                                break;
                            }
                        }
                        if is_attacked {
                            break;
                        }
                    }
                }
            }
            is_attacked
        }
    }};
}

fn king_is_in_check(
    position: &Position,
    color: Color,
    ray_attacks: &Rays,
    knight_attacks: &KnightAttacks,
) -> bool {
    // Check rays
    let enemy_color = color.opposite();
    // let ray_attacks = &game.ray_attacks;

    let king = position
        .pieces
        .iter()
        .find(|p| p.piece_type == King && p.color == color)
        .expect("The king was missing");

    macro_rules! check {
        ($rays:expr, $forward:expr, $types:expr) => {
            check_if_attacked!($rays, $forward, king, position, $types)
        };
    }

    let is_ray_attacked = check!(ray_attacks.n_rays, true, vec![Queen, Rook])
        || check!(ray_attacks.ne_rays, true, vec![Queen, Bishop])
        || check!(ray_attacks.e_rays, true, vec![Queen, Rook])
        || check!(ray_attacks.se_rays, false, vec![Queen, Bishop])
        || check!(ray_attacks.s_rays, false, vec![Queen, Rook])
        || check!(ray_attacks.sw_rays, false, vec![Queen, Bishop])
        || check!(ray_attacks.w_rays, false, vec![Queen, Rook])
        || check!(ray_attacks.nw_rays, true, vec![Queen, Bishop]);
    if is_ray_attacked {
        return true;
    }

    // Check knight attacks
    let knight_attacks = knight_attacks.0[bit_scan(king.position)];
    let enemy_occupancy = match king.color {
        Color::White => position.black_occupancy,
        Color::Black => position.white_occupancy,
    };

    let overlap = knight_attacks & enemy_occupancy;
    if overlap != 0 {
        let positions = extract_bits(overlap);
        for pos in positions {
            let square = position.squares[pos];
            match square {
                Square::Empty => panic!("Empty square, but expected occupied"),
                Square::Occupied(pidx) => {
                    let attacker = position.pieces[pidx];
                    if attacker.piece_type == Knight {
                        return true;
                    }
                }
            }
        }
    }

    // Check for whether the enemy king is attacking our king
    let (row, col) = rowcol(bit_scan(king.position));
    // Direction in which we need to check for enemy pawns
    let direction = match king.color {
        Color::White => 1,
        Color::Black => -1,
    };

    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            let new_row = row + row_offset;
            let new_col = col + col_offset;
            if let Some(idx) = index(new_row, new_col) {
                match position.squares[idx] {
                    Square::Empty => (),
                    Square::Occupied(pidx) => {
                        let piece = position.pieces[pidx];
                        if piece.color != king.color {
                            if piece.piece_type == King {
                                return true;
                            }
                            if (new_col - col).abs() == 1 && (new_row - row) == direction {
                                if piece.piece_type == Pawn {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    false
}

fn generate_knight_moves(piece: &Piece, game: &Game) -> Vec<Position> {
    let mut attacks = game.knight_attacks.0[bit_scan(piece.position)];
    let position = &game.position;

    // Check if any squares are occupied by our own pieces
    // Remove those squares
    let own_occupancy = match piece.color {
        Color::White => position.white_occupancy,
        Color::Black => position.black_occupancy,
    };
    attacks &= !own_occupancy;

    let potential_moves = extract_bits(attacks);
    let mut new_positions = vec![];
    // Remove those pieces if the square is selected as the move
    for pmove in potential_moves {
        // Start by making a new Position where the knight is moved
        let mut new_position = (*game).position.clone();
        new_position.move_piece(piece.position, pmove);
        new_positions.push(new_position);
    }

    new_positions
}

macro_rules! get_attacks {
    ($rays:expr, $forward:expr, $positions:ident, $piece:ident, $game:ident) => {
        let (own_occupancy, enemy_occupancy) = match $piece.color {
            Color::White => (
                $game.position.white_occupancy,
                $game.position.black_occupancy,
            ),
            Color::Black => (
                $game.position.black_occupancy,
                $game.position.white_occupancy,
            ),
        };

        let ray_attacks = blocked_ray_attack(
            $rays[bit_scan($piece.position)],
            &$rays,
            $forward,
            own_occupancy,
            enemy_occupancy,
        );
        let potential_moves = extract_bits(ray_attacks);

        for pmove in potential_moves {
            let mut new_position = (*$game).position.clone();
            new_position.move_piece($piece.position, pmove);
            $positions.push(new_position);
        }
    };
}

fn generate_bishop_moves(piece: &Piece, game: &Game) -> Vec<Position> {
    let attacks = &game.ray_attacks;
    let position = &game.position;
    let mut new_positions = vec![];

    get_attacks!(attacks.nw_rays, true, new_positions, piece, game);
    get_attacks!(attacks.sw_rays, false, new_positions, piece, game);
    get_attacks!(attacks.ne_rays, true, new_positions, piece, game);
    get_attacks!(attacks.se_rays, false, new_positions, piece, game);

    new_positions
}

fn generate_rook_moves(piece: &Piece, game: &Game) -> Vec<Position> {
    let attacks = &game.ray_attacks;
    let position = &game.position;

    let (own_occupancy, enemy_occupancy) = match piece.color {
        Color::White => (position.white_occupancy, position.black_occupancy),
        Color::Black => (position.black_occupancy, position.white_occupancy),
    };
    let mut new_positions = vec![];

    get_attacks!(attacks.n_rays, true, new_positions, piece, game);
    get_attacks!(attacks.s_rays, false, new_positions, piece, game);
    get_attacks!(attacks.e_rays, true, new_positions, piece, game);
    get_attacks!(attacks.w_rays, false, new_positions, piece, game);

    new_positions
}

fn generate_queen_moves(piece: &Piece, game: &Game) -> Vec<Position> {
    let bishop_moves = generate_bishop_moves(piece, game);
    let mut rook_moves = generate_rook_moves(piece, game);

    rook_moves.extend(bishop_moves);

    rook_moves
}

fn generate_king_moves(piece: &Piece, game: &Game) -> Vec<Position> {
    let position_index = bit_scan(piece.position);
    let row = (position_index / 8) as i32;
    let col = (position_index % 8) as i32;

    let own_occupancy = match piece.color {
        Color::White => game.position.white_occupancy,
        Color::Black => game.position.black_occupancy,
    };

    let mut potential_moves = vec![];
    let mut moves_looked_at = 0;
    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            if row_offset == 0 && col_offset == 0 {
                continue;
            }
            let new_row = row + row_offset;
            let new_col = col + col_offset;
            moves_looked_at += 1;

            if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                let position_index = (new_row * 8 + new_col) as usize;
                let position_bit = 1 << position_index;

                if position_bit & own_occupancy == 0 {
                    potential_moves.push(position_index);
                }
            }
        }
    }

    let mut positions = vec![];
    for pmove in potential_moves {
        let mut new_position = (*game).position.clone();
        new_position.move_piece(piece.position, pmove);
        positions.push(new_position);
    }

    positions
}

fn generate_pawn_moves(piece: &Piece, game: &Game) -> Vec<Position> {
    // Consider all possible moves (en passant is possible if the en passant field in the position is set)
    // After making a move we need to decide if the en passant field should be updated
    // The en passant field should be cleared after almost any move
    let position_index = bit_scan(piece.position);
    let (row, col) = rowcol(position_index);

    let (own_occupancy, enemy_occupancy) = match piece.color {
        Color::White => (game.position.white_occupancy, game.position.black_occupancy),
        Color::Black => (game.position.black_occupancy, game.position.white_occupancy),
    };

    let (direction, last_row) = match piece.color {
        Color::White => (1, 7),
        Color::Black => (-1, 2),
    };

    let mut new_squares = vec![];
    let idx = index(row + 1 * direction, col)
        .expect("Pawn was somehow at the edge of the board and trying to move off");
    let pos = 1 << idx;
    if pos & enemy_occupancy == 0 && row != last_row {
        new_squares.push(index(row + 1 * direction, col).unwrap());
    }

    if piece.color == Color::White && row == 2 {
        let idx = index(row + 2, col).unwrap();
        let pos = 1 << idx;
        let idx = index(row + 1, col).unwrap();
        let pos2 = 1 << idx;
        if pos & enemy_occupancy == 0 && pos2 & enemy_occupancy == 0 {
            new_squares.push(index(row + 2, col).unwrap());
        }
    }

    if piece.color == Color::Black && row == 7 {
        let idx = index(row - 2, col).unwrap();
        let pos = 1 << idx;
        let idx = index(row - 1, col).unwrap();
        let pos2 = 1 << idx;
        if pos & enemy_occupancy == 0 && pos2 & enemy_occupancy == 0 {
            new_squares.push(index(row - 2, col).unwrap());
        }
    }

    if let Some(idx) = index(row + 1 * direction, col + 1) {
        if (1 << idx as u64) & enemy_occupancy != 0 {
            new_squares.push(idx);
        }
    }
    if let Some(idx) = index(row + 1 * direction, col - 1) {
        if (1 << idx as u64) & enemy_occupancy != 0 {
            new_squares.push(idx);
        }
    }

    let mut positions = vec![];
    for pmove in new_squares {
        let mut new_position = (*game).position.clone();
        new_position.move_piece(piece.position, pmove);
        positions.push(new_position);
    }

    if row == last_row {
        let new_pos_index = index(row + direction, col).expect("row+-1/col should be valid");
        for tpe in [Queen, Bishop, Knight, Rook] {
            let mut new_position = (*game).position.clone();
            new_position.perform_promotion(piece.position, new_pos_index, tpe);
            positions.push(new_position);
        }
    }

    if let Some(square) = game.position.en_passant {
        let (ep_row, ep_col) = rowcol(bit_scan(square));
        if (row + direction) == ep_row && (col - ep_col).abs() == 1 {
            let mut new_position = (*game).position.clone();
            new_position.take_en_passant(piece.position, square);
            positions.push(new_position);
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Color::*;

    #[test]
    fn test_generate_knight_moves() {
        let not_alot = "8/8/8/4N3/2N5/8/8/8 w - - 0 1";
        let game = Game::read_FEN(not_alot);

        let positions = generate_knight_moves(&game.position.pieces[0], &game);
        let new_positions = [19, 21, 30, 42, 46, 51, 53];
        assert_eq!(positions.len(), 7);
        for pos in positions {
            assert_eq!(pos.pieces.len(), 2);
            let piece = &pos.pieces[0];
            let index = bit_scan(piece.position);
            assert!(new_positions.contains(&index));
        }
    }

    #[test]
    fn test_generate_knight_moves_three_knights() {
        let not_alot = "8/5N2/8/4N3/2N5/8/8/8 w - - 0 1";
        let game = Game::read_FEN(not_alot);

        let positions = generate_knight_moves(&game.position.pieces[1], &game);
        let new_positions = [19, 21, 30, 42, 46, 51];
        assert_eq!(positions.len(), 6);
        for pos in positions {
            assert_eq!(pos.pieces.len(), 3);
            let piece = &pos.pieces[1];
            let index = bit_scan(piece.position);
            assert!(new_positions.contains(&index));
        }
    }

    #[test]
    fn test_generate_knight_moves_three_knights_oneblack() {
        let not_alot = "8/5n2/8/4N3/2N5/8/8/8 w - - 0 1";
        let game = Game::read_FEN(not_alot);

        let positions = generate_knight_moves(&game.position.pieces[1], &game);
        let new_positions = [19, 21, 30, 42, 46, 51, 53];
        assert_eq!(positions.len(), 7);
        for pos in positions {
            let piece;
            if pos.pieces.len() == 3 {
                piece = &pos.pieces[1];
            } else if pos.pieces.len() == 2 {
                piece = &pos.pieces[0];
            } else {
                panic!("Invalid number of pieces: {}", pos.pieces.len());
            }
            assert_eq!(piece.color, Color::White);
            let index = bit_scan(piece.position);
            assert!(new_positions.contains(&index));
        }
    }

    #[test]
    fn test_generate_knight_moves_three_knights_twoblack() {
        let not_alot = "8/5n2/8/4N3/2n5/8/8/8 w - - 0 1";
        let game = Game::read_FEN(not_alot);

        let positions = generate_knight_moves(&game.position.pieces[1], &game);
        let new_positions = [19, 21, 26, 30, 42, 46, 51, 53];
        assert_eq!(positions.len(), 8);
        for pos in positions {
            let mut piece = &pos.pieces[0];
            for pie in &pos.pieces {
                if pie.color == Color::White {
                    piece = pie;
                    break;
                }
            }
            assert!(pos.pieces.len() == 2 || pos.pieces.len() == 3);

            let index = bit_scan(piece.position);
            assert!(new_positions.contains(&index));
        }
    }

    #[test]
    fn test_generate_bishop_moves_one_bishop() {
        let fen = "7B/8/8/8/8/8/8/8 w - - 0 1";
        let game = Game::read_FEN(fen);

        let positions = generate_bishop_moves(&game.position.pieces[0], &game);
        let new_positions = [0, 9, 18, 27, 36, 45, 54];
        assert_eq!(positions.len(), new_positions.len());
        for pos in positions {
            let mut piece = &pos.pieces[0];
            for pie in &pos.pieces {
                if pie.color == Color::White {
                    piece = pie;
                    break;
                }
            }
            assert!(pos.pieces.len() == 1);

            let index = bit_scan(piece.position);
            assert!(new_positions.contains(&index));
        }
    }

    #[test]
    fn test_generate_bishop_moves_one_bishop_middle() {
        let fen = "8/8/8/4B3/8/8/8/8 w - - 0 1";
        let game = Game::read_FEN(fen);

        let positions = generate_bishop_moves(&game.position.pieces[0], &game);
        let new_positions = [45, 54, 63, 27, 18, 9, 0, 43, 50, 57, 29, 22, 15];

        // assert_eq!(positions.len(), new_positions.len());
        for pos in positions {
            let mut piece = &pos.pieces[0];
            for pie in &pos.pieces {
                if pie.color == Color::White {
                    piece = pie;
                    break;
                }
            }
            assert!(pos.pieces.len() == 1);

            let index = bit_scan(piece.position);
            assert!(new_positions.contains(&index));
        }
    }

    #[test]
    fn test_generate_bishop_moves_two_bishops_middle() {
        let fen = "8/8/5B2/4B3/8/8/8/8 w - - 0 1";
        let game = Game::read_FEN(fen);

        let positions = generate_bishop_moves(&game.position.pieces[1], &game);
        let new_positions = [27, 18, 9, 0, 43, 50, 57, 29, 22, 15];

        // assert_eq!(positions.len(), new_positions.len());
        for pos in positions {
            let piece = &pos.pieces[1];
            assert!(pos.pieces.len() == 2);

            let index = bit_scan(piece.position);
            assert!(new_positions.contains(&index));
        }
    }

    #[test]
    fn test_generate_bishop_moves_two_bishops_middle_black() {
        let fen = "8/8/5b2/4b3/8/8/8/8 w - - 0 1";
        let game = Game::read_FEN(fen);

        let positions = generate_bishop_moves(&game.position.pieces[1], &game);
        let new_positions = [27, 18, 9, 0, 43, 50, 57, 29, 22, 15];

        // assert_eq!(positions.len(), new_positions.len());
        for pos in positions {
            let piece = &pos.pieces[1];
            assert!(pos.pieces.len() == 2);

            let index = bit_scan(piece.position);
            assert!(new_positions.contains(&index));
        }
    }

    #[test]
    fn test_generate_bishop_moves_one_friendly_one_enemy() {
        let fen = "8/8/3B4/4B3/3b4/8/8/8 w - - 0 1";
        let game = Game::read_FEN(fen);

        let positions = generate_bishop_moves(&game.position.pieces[1], &game);
        let new_positions = [45, 54, 63, 27, 29, 22, 15];

        assert_eq!(positions.len(), new_positions.len());
        'outer: for pos in positions {
            assert!(pos.pieces.len() == 2 || pos.pieces.len() == 3);

            for piece in &pos.pieces {
                if piece.color == Color::White && new_positions.contains(&bit_scan(piece.position))
                {
                    continue 'outer;
                }
            }
            assert!(false);
        }
    }

    #[test]
    fn test_generate_rook_moves() {
        let game = Game::empty().add(Color::White, Rook, "e3");

        let positions = generate_rook_moves(&game.position.pieces[0], &game);
        let new_positions = [
            "e2", "e1", "a3", "b3", "c3", "d3", "f3", "g3", "h3", "e4", "e5", "e6", "e7", "e8",
        ]
        .iter()
        .map(|string| square_to_index(string))
        .collect::<Vec<usize>>();

        assert_eq!(positions.len(), new_positions.len());
        for pos in positions {
            assert!(pos.pieces.len() == 1);
            assert!(new_positions.contains(&bit_scan(pos.pieces[0].position)));
            assert_eq!(pos.pieces[0].piece_type, Rook);
            assert_eq!(pos.pieces[0].color, White);
        }
    }

    #[test]
    fn test_generate_rook_moves_one_enemy() {
        let game = Game::empty().add(White, Rook, "e3").add(Black, Rook, "e5");

        assert_eq!(game.position.pieces.len(), 2);

        let new_positions = [
            "e1", "e2", "e4", "e5", "a3", "b3", "c3", "d3", "f3", "g3", "h3",
        ]
        .iter()
        .map(|s| square_to_index(s))
        .collect::<Vec<usize>>();

        let positions = generate_rook_moves(&game.position.pieces[0], &game);

        assert_eq!(positions.len(), new_positions.len());
        for pos in positions {
            assert!(pos.pieces.len() == 1 || pos.pieces.len() == 2);
            assert!(new_positions.contains(&bit_scan(pos.pieces[0].position)));
            assert_eq!(pos.pieces[0].piece_type, Rook);
            assert_eq!(pos.pieces[0].color, White);
        }
    }

    #[test]
    fn test_generate_rook_moves_one_enemy_one_friendly() {
        let game = Game::empty()
            .add(White, Rook, "d5")
            .add(White, Rook, "d2")
            .add(Black, Rook, "g5");

        let new_positions = [
            "d3", "d4", "d6", "d7", "d8", "a5", "b5", "c5", "e5", "f5", "g5",
        ]
        .iter()
        .map(|s| square_to_index(s))
        .collect::<Vec<usize>>();

        let positions = generate_rook_moves(&game.position.pieces[0], &game);

        assert_eq!(positions.len(), new_positions.len());

        for pos in positions {
            assert!(pos.pieces.len() == 3 || pos.pieces.len() == 2);
            assert!(new_positions.contains(&bit_scan(pos.pieces[0].position)));
            assert_eq!(pos.pieces[0].piece_type, Rook);
            assert_eq!(pos.pieces[0].color, White);
        }
    }

    #[test]
    fn test_generate_queen_moves() {
        let game = Game::empty().add(White, Queen, "d5");

        let expected_positions = [
            "a5", "b5", "c5", "e5", "f5", "g5", "h5", "d1", "d2", "d3", "d4", "d6", "d7", "d8",
            "c4", "b3", "a2", "e6", "f7", "g8", "c6", "b7", "a8", "e4", "f3", "g2", "h1",
        ]
        .iter()
        .map(|s| square_to_index(s))
        .collect::<Vec<usize>>();

        let positions = generate_queen_moves(&game.position.pieces[0], &game);

        assert_eq!(positions.len(), expected_positions.len());
        for pos in positions {
            assert_eq!(pos.pieces.len(), 1);
            assert!(expected_positions.contains(&bit_scan(pos.pieces[0].position)));
            assert_eq!(pos.pieces[0].piece_type, Queen);
            assert_eq!(pos.pieces[0].color, White);
        }
    }

    #[test]
    fn test_generate_queen_moves_one_enemy() {
        let game = Game::empty().add(White, Queen, "d5").add(Black, Pawn, "c6");

        let expected_positions = [
            "a5", "b5", "c5", "e5", "f5", "g5", "h5", "d1", "d2", "d3", "d4", "d6", "d7", "d8",
            "c4", "b3", "a2", "e6", "f7", "g8", "c6", "e4", "f3", "g2", "h1",
        ]
        .iter()
        .map(|s| square_to_index(s))
        .collect::<Vec<usize>>();

        let positions = generate_queen_moves(&game.position.pieces[0], &game);

        assert_eq!(positions.len(), expected_positions.len());
        for pos in positions {
            assert!(pos.pieces.len() == 1 || pos.pieces.len() == 2);
            assert!(expected_positions.contains(&bit_scan(pos.pieces[0].position)));
            assert_eq!(pos.pieces[0].piece_type, Queen);
            assert_eq!(pos.pieces[0].color, White);
        }
    }

    #[test]
    fn test_generate_queen_moves_one_enemy_one_friendly() {
        let game = Game::empty()
            .add(White, Queen, "d5")
            .add(Black, Pawn, "d6")
            .add(White, Pawn, "c5");

        let expected_positions = [
            "e5", "f5", "g5", "h5", "d1", "d2", "d3", "d4", "d6", "c4", "b3", "a2", "e6", "f7",
            "g8", "c6", "b7", "a8", "e4", "f3", "g2", "h1",
        ]
        .iter()
        .map(|s| square_to_index(s))
        .collect::<Vec<usize>>();

        let positions = generate_queen_moves(&game.position.pieces[0], &game);

        assert_eq!(positions.len(), expected_positions.len());
        for pos in positions {
            assert!(pos.pieces.len() == 2 || pos.pieces.len() == 3);
            assert!(expected_positions.contains(&bit_scan(pos.pieces[0].position)));
            assert_eq!(pos.pieces[0].piece_type, Queen);
            assert_eq!(pos.pieces[0].color, White);
        }
    }

    #[test]
    fn test_generate_king_moves() {
        let game = Game::empty().add(White, King, "d5");

        let expected_positions = ["c5", "e5", "d4", "d6", "c4", "e6", "c6", "e4"]
            .iter()
            .map(|s| square_to_index(s))
            .collect::<Vec<usize>>();

        let positions = generate_king_moves(&game.position.pieces[0], &game);

        assert_eq!(positions.len(), expected_positions.len());
        for pos in positions {
            assert_eq!(pos.pieces.len(), 1);
            assert!(expected_positions.contains(&bit_scan(pos.pieces[0].position)));
            assert_eq!(pos.pieces[0].piece_type, King);
            assert_eq!(pos.pieces[0].color, White);
        }
    }

    #[test]
    fn test_generate_king_moves_one_enemy() {
        let game = Game::empty().add(White, King, "d5").add(Black, King, "d6");

        let expected_positions = ["c5", "e5", "d4", "d6", "c4", "e6", "c6", "e4"]
            .iter()
            .map(|s| square_to_index(s))
            .collect::<Vec<usize>>();

        let positions = generate_king_moves(&game.position.pieces[0], &game);

        assert_eq!(positions.len(), expected_positions.len());
        for pos in positions {
            let alive_count = pos.pieces.iter().filter(|p| p.alive).count();
            assert!(alive_count == 1 || alive_count == 2);
            assert_eq!(pos.pieces.len(), 2);
            assert!(expected_positions.contains(&bit_scan(pos.pieces[0].position)));
            assert_eq!(pos.pieces[0].piece_type, King);
            assert_eq!(pos.pieces[0].color, White);
        }
    }

    #[test]
    fn test_generate_king_moves_one_enemy_one_friendly() {
        let game = Game::empty()
            .add(White, King, "d5")
            .add(Black, King, "d6")
            .add(White, King, "d4");

        let expected_positions = ["c5", "e5", "d6", "c4", "e6", "c6", "e4"]
            .iter()
            .map(|s| square_to_index(s))
            .collect::<Vec<usize>>();

        let positions = generate_king_moves(&game.position.pieces[0], &game);

        assert_eq!(positions.len(), expected_positions.len());
        for pos in positions {
            assert!(pos.pieces.len() == 3 || pos.pieces.len() == 2);
            assert!(expected_positions.contains(&bit_scan(pos.pieces[0].position)));
            assert_eq!(pos.pieces[0].piece_type, King);
            assert_eq!(pos.pieces[0].color, White);
        }
    }

    #[test]
    fn test_generate_pawn_moves() {
        let game = Game::empty().add(White, Pawn, "d2");

        let expected_positions = ["d3", "d4"]
            .iter()
            .map(|s| square_to_index(s))
            .collect::<Vec<usize>>();

        let positions = generate_pawn_moves(&game.position.pieces[0], &game);

        assert_eq!(positions.len(), 2);

        for pos in positions {
            assert!(expected_positions.contains(&bit_scan(pos.pieces[0].position)));
        }
    }

    #[test]
    fn test_generate_pawn_moves_en_passant() {
        let mut game = Game::empty().add(White, Pawn, "d2").add(Black, Pawn, "c4");

        game.position.move_piece(
            game.position.pieces[0].position,
            position_to_index("d4").unwrap(),
        );

        let expected_positions = ["d3", "c3"]
            .iter()
            .map(|s| square_to_index(s))
            .collect::<Vec<usize>>();

        let positions = generate_pawn_moves(&game.position.pieces[1], &game);

        assert_eq!(positions.len(), 2);

        for pos in positions {
            assert!(expected_positions.contains(&bit_scan(pos.pieces[1].position)));
        }
    }

    #[test]
    fn test_generate_pawn_moves_middle() {
        let game = Game::empty().add(White, Pawn, "d4");

        let expected_positions = ["d5"]
            .iter()
            .map(|s| square_to_index(s))
            .collect::<Vec<usize>>();

        let positions = generate_pawn_moves(&game.position.pieces[0], &game);

        assert_eq!(positions.len(), 1);

        for pos in positions {
            assert!(expected_positions.contains(&bit_scan(pos.pieces[0].position)));
        }
    }

    #[test]
    fn test_pawn_blocked() {
        let game = Game::empty().add(White, Pawn, "d4").add(Black, Pawn, "d5");

        let positions = generate_pawn_moves(&game.position.pieces[0], &game);

        assert_eq!(positions.len(), 0);
    }

    #[test]
    fn test_king_is_in_check() {
        let game = Game::empty().add(White, King, "e1").add(Black, Queen, "e5");

        assert!(king_is_in_check(
            &game.position,
            White,
            &game.ray_attacks,
            &game.knight_attacks
        ));
    }

    #[test]
    fn test_king_is_not_in_check() {
        let game = Game::empty().add(White, King, "e1").add(White, Queen, "e5");

        assert!(!king_is_in_check(
            &game.position,
            White,
            &game.ray_attacks,
            &game.knight_attacks
        ));
    }

    #[test]
    fn test_king_is_in_knight_check() {
        let game = Game::empty()
            .add(White, King, "e1")
            .add(Black, Knight, "f3");

        assert!(king_is_in_check(
            &game.position,
            White,
            &game.ray_attacks,
            &game.knight_attacks
        ));
    }

    #[test]
    fn test_king_is_not_in_check_bishop() {
        let game = Game::empty()
            .add(White, King, "e1")
            .add(Black, Bishop, "h4")
            .add(White, Pawn, "f2");

        assert!(!king_is_in_check(
            &game.position,
            White,
            &game.ray_attacks,
            &game.knight_attacks
        ));
    }

    #[test]
    fn test_move_generator_doesnt_generate_in_check() {
        let game = Game::empty()
            .add(White, King, "e1")
            .add(White, Pawn, "e2")
            .add(Black, Rook, "e3")
            .add(Black, Queen, "d7");

        let positions = generate_moves(&game);

        assert_eq!(positions.len(), 2);

        let expected_positions = ["f1", "f2"]
            .iter()
            .map(|s| square_to_index(s))
            .collect::<Vec<usize>>();

        for pos in positions {
            assert!(expected_positions.contains(&bit_scan(pos.pieces[0].position)));
        }
    }

    #[test]
    fn test_promote_single_pawn() {
        let game = Game::empty().add(White, Pawn, "e7");

        let positions = generate_pawn_moves(&game.position.pieces[0], &game);

        assert_eq!(positions.len(), 4);

        let allowed_types = [Queen, Bishop, Knight, Rook];

        for pos in positions {
            assert_eq!(pos.count_pieces(), 1);
            assert!(allowed_types.contains(&pos.pieces[1].piece_type));
        }
    }
}
