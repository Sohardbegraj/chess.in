use crate::position::Color;
use crate::utils::*;

pub struct PawnAttacks {
    white_forward_moves: Vec<Bitboard>,
    white_diagonal_moves: Vec<Bitboard>,
    black_forward_moves: Vec<Bitboard>,
    black_diagonal_moves: Vec<Bitboard>,
}

impl PawnAttacks {
    fn initialize() -> Self {
        let mut w_forward = vec![];
        let mut w_diagonal = vec![];
        let mut b_forward = vec![];
        let mut b_diagonal = vec![];

        for row in 1..=8 {
            for col in 1..=8 {
                let f = forward_move(row, col, Color::White);
                let d = diagonal_move(row, col, Color::White);

                w_forward.push(f);
                w_diagonal.push(d);

                let f = forward_move(row, col, Color::Black);
                let d = diagonal_move(row, col, Color::Black);

                b_forward.push(f);
                b_diagonal.push(d);
            }
        }

        Self {
            white_forward_moves: w_forward,
            white_diagonal_moves: w_diagonal,
            black_forward_moves: b_forward,
            black_diagonal_moves: b_diagonal,
        }
    }
}

// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0

fn forward_move(row: i32, col: i32, color: Color) -> Bitboard {
    if row == 1 || row == 8 {
        return 0;
    }

    let mut bitboard = 0;
    if color == Color::White {
        if row < 8 {
            bitboard |= set_bit(row + 1, col);
        }
        if row == 2 {
            bitboard |= set_bit(row + 2, col);
        }
    } else {
        if row > 1 {
            bitboard |= set_bit(row - 1, col);
        }
        if row == 7 {
            bitboard |= set_bit(row - 2, col);
        }
    }
    bitboard
}

fn diagonal_move(row: i32, col: i32, color: Color) -> Bitboard {
    if row == 1 || row == 8 {
        return 0;
    }
    let mut bitboard = 0;
    if color == Color::White {
        if row < 8 {
            bitboard |= set_bit(row + 1, col + 1);
            bitboard |= set_bit(row + 1, col - 1);
        }
    } else {
        if row > 1 {
            bitboard |= set_bit(row - 1, col + 1);
            bitboard |= set_bit(row - 1, col - 1);
        }
    }
    bitboard
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_row_white_pawn() {
        let row = 2;
        for col in 1..=8 {
            let bitboard = forward_move(row, col, Color::White);
            let lsb = bit_scan(bitboard);
            let msb = bit_scan_backwards(bitboard);

            let expected_lsb = (col - 1) + (row + 1 - 1) * 8;
            let expected_msb = (col - 1) + (row + 2 - 1) * 8;

            assert_eq!(lsb, expected_lsb as usize);
            assert_eq!(msb, expected_msb as usize);
        }
    }

    #[test]
    fn test_second_row_black_pawn() {
        let row = 2;
        for col in 1..=8 {
            let bitboard = forward_move(row, col, Color::Black);
            let lsb = bit_scan(bitboard);

            let expected_lsb = (col - 1) + (row - 1 - 1) * 8;

            assert_eq!(lsb, expected_lsb as usize);
        }
    }

    #[test]
    fn test_middle_rows_white_pawn() {
        for row in 3..=7 {
            for col in 1..=8 {
                let bitboard = forward_move(row, col, Color::White);
                let lsb = bit_scan(bitboard);

                let expected_lsb = (col - 1) + (row + 1 - 1) * 8;

                assert_eq!(lsb, expected_lsb as usize);
            }
        }
    }

    #[test]
    fn test_middle_rows_black_pawn() {
        for row in 2..=6 {
            for col in 1..=8 {
                let bitboard = forward_move(row, col, Color::Black);
                let lsb = bit_scan(bitboard);

                let expected_lsb = (col - 1) + (row - 1 - 1) * 8;

                assert_eq!(lsb, expected_lsb as usize);
            }
        }
    }

    #[test]
    fn test_edges() {
        for color in [Color::White, Color::Black] {
            for row in [1, 8] {
                for col in 1..=8 {
                    let bitboard = forward_move(row, col, color);
                    assert_eq!(bitboard, 0);
                }
            }
        }
    }

    #[test]
    fn test_diagonal_white() {
        for row in 2..=7 {
            for col in 2..=7 {
                let bitboard = diagonal_move(row, col, Color::White);
                let lsb = bit_scan(bitboard);
                let msb = bit_scan_backwards(bitboard);

                let expected_lsb = (col - 1 - 1) + (row + 1 - 1) * 8;
                let expected_msb = (col + 1 - 1) + (row + 1 - 1) * 8;

                assert_eq!(lsb, expected_lsb as usize);
                assert_eq!(msb, expected_msb as usize);
            }
        }
    }

    #[test]
    fn test_diagonal_black() {
        for row in 2..=7 {
            for col in 2..=7 {
                let bitboard = diagonal_move(row, col, Color::Black);
                let lsb = bit_scan(bitboard);
                let msb = bit_scan_backwards(bitboard);

                let expected_lsb = (col - 1 - 1) + (row - 1 - 1) * 8;
                let expected_msb = (col + 1 - 1) + (row - 1 - 1) * 8;

                assert_eq!(lsb, expected_lsb as usize);
                assert_eq!(msb, expected_msb as usize);
            }
        }
    }

    #[test]
    fn test_diagonal_edge_white() {
        for row in 2..=7 {
            let col = 1;
            let bitboard = diagonal_move(row, col, Color::White);
            let lsb = bit_scan(bitboard);

            let expected_lsb = (col + 1 - 1) + (row - 1 + 1) * 8;

            assert_eq!(lsb, expected_lsb as usize);

            let col = 8;
            let bitboard = diagonal_move(row, col, Color::White);
            let lsb = bit_scan(bitboard);

            let expected_lsb = (col - 1 - 1) + (row - 1 + 1) * 8;

            assert_eq!(lsb, expected_lsb as usize);
        }
    }

    #[test]
    fn test_diagonal_edge_black() {
        for row in 2..=7 {
            let col = 1;
            let bitboard = diagonal_move(row, col, Color::Black);
            let lsb = bit_scan(bitboard);

            let expected_lsb = (col + 1 - 1) + (row - 1 - 1) * 8;

            assert_eq!(lsb, expected_lsb as usize);

            let col = 8;
            let bitboard = diagonal_move(row, col, Color::Black);
            let lsb = bit_scan(bitboard);

            let expected_lsb = (col - 1 - 1) + (row - 1 - 1) * 8;

            assert_eq!(lsb, expected_lsb as usize);
        }
    }

    #[test]
    fn test_pawnattacks_init() {
        let pawnattacks = PawnAttacks::initialize();
    }
}
