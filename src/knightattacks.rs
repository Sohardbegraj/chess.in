use crate::utils::*;

type Bitboard = u64;

pub struct KnightAttacks(pub Vec<Bitboard>);

impl KnightAttacks {
    pub fn new() -> Self {
        let mut attacks = vec![];

        for row in 1..=8 {
            for col in 1..=8 {
                let attacks_from_this_square = knight_attacks(row, col);
                attacks.push(attacks_from_this_square);
            }
        }

        Self(attacks)
    }
}

fn knight_attacks(row: i32, col: i32) -> Bitboard {
    // row - 2, col - 1
    // row - 2, col + 1
    // row - 1, col - 2
    // row - 1, col + 2,
    // ..

    let attack_pairs = [
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
    ];

    let mut bitboard = 0;

    for (r, c) in attack_pairs {
        bitboard |= set_bit(row + r, col + c);
    }

    bitboard
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_attacks_can_initialize() {
        let knight_attacks = KnightAttacks::new();
    }

    #[test]
    fn check_knight_attacks() {
        let knight_attacks = KnightAttacks::new();
        let expected_0 = 1 << 10 | 1 << 17;
        assert_eq!(knight_attacks.0[0], expected_0);
        let expected_40 = 1 << 25 | 1 << 34 | 1 << 50 | 1 << 57;
        assert_eq!(knight_attacks.0[40], expected_40);
        let expected_18 =
            1 << 1 | 1 << 3 | 1 << 12 | 1 << 8 | 1 << 28 | 1 << 24 | 1 << 35 | 1 << 33;
        assert_eq!(knight_attacks.0[18], expected_18);
    }
}
