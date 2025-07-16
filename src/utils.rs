static MOD67TABLE: [usize; 67] = [
    64, 0, 1, 39, 2, 15, 40, 23, 3, 12, 16, 59, 41, 19, 24, 54, 4, 64, 13, 10, 17, 62, 60, 28, 42,
    30, 20, 51, 25, 44, 55, 47, 5, 32, 64, 38, 14, 22, 11, 58, 18, 53, 63, 9, 61, 27, 29, 50, 43,
    46, 31, 37, 21, 57, 52, 8, 26, 49, 45, 36, 56, 7, 48, 35, 6, 34, 33,
];

pub fn bit_scan(bit: u64) -> usize {
    assert!(bit != 0);
    let one_bit = (bit ^ (bit - 1)) ^ (!bit & (bit - 1));
    let remainder = (one_bit % 67) as usize;
    return MOD67TABLE[remainder];
}

pub fn bit_scan_backwards(bit: u64) -> usize {
    // 0010000 = 16 = 2^4 -> index of 1 = log_2(0010000)
    // 0010010 = 18 = 2^4.2 -> log_2(0010010) = 4.2 = between 4-5
    // 0100000
    (bit as f64).log2().floor() as usize
}

pub fn extract_bits(mut bits: u64) -> Vec<usize> {
    // 00101 -> [0, 2]
    let mut result = vec![];
    while bits != 0 {
        let next_bit = bit_scan(bits);
        result.push(next_bit);
        bits ^= 1 << next_bit;
    }

    result
}

pub fn index(row: i32, col: i32) -> Option<usize> {
    if row < 1 || row > 8 || col < 1 || col > 8 {
        return None;
    }
    
    Some(((row - 1) * 8 + col - 1) as usize)
}

pub fn rowcol(index: usize) -> (i32, i32) {
    let row = index / 8 + 1;
    let col = index % 8 + 1;

    (row as i32, col as i32)
}

pub fn split_on(s: &str, sep: char) -> (&str, &str) {
    for (i, item) in s.chars().enumerate() {
        if item == sep {
            return (&s[0..i], &s[i + 1..]);
        }
    }

    (&s[..], "")
}

pub type Bitboard = u64;
pub fn bitboard_to_string(bitboard: Bitboard, mark: Option<usize>) -> String {
    let mut row = "".to_owned();
    let mut board = "".to_owned();

    for i in 0..64 {
        let value = (bitboard >> i) & 1;

        let s = if value == 0 {
            ".".to_owned()
        } else {
            value.to_string()
        };

        match mark {
            Some(idx) => {
                if i == idx {
                    row.push_str("X");
                } else {
                    row.push_str(&s);
                }
            }
            None => row.push_str(&s),
        }

        if (i + 1) % 8 == 0 {
            row.push_str("\n");
            board.insert_str(0, &row);
            row.clear();
        }
    }
    board
}

pub fn print_bitboard(bitboard: Bitboard, marker: Option<usize>) {
    println!("{}", bitboard_to_string(bitboard, marker));
}

pub fn set_bit(row: i32, col: i32) -> Bitboard {
    if row < 1 || row > 8 || col < 1 || col > 8 {
        return 0;
    }

    1 << ((col - 1) + (row - 1) * 8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_on_space_works() {
        let test_string = "A B C D";
        let (should_be_a, rest) = split_on(test_string, ' ');
        assert_eq!(should_be_a, "A");
        assert_eq!(rest, "B C D");
    }

    #[test]
    fn split_on_ascii_works() {
        for i in 0..128 {
            let ch = char::from(i);
            if ch == 'A' {
                continue;
            }
            let test_string = format!("AA{}BB{}CC{}DD", ch, ch, ch);
            let (should_be_a, rest) = split_on(&test_string, ch);
            assert_eq!(should_be_a, "AA", "{},{}, {}", test_string, ch, i);
            assert_eq!(rest, &format!("BB{}CC{}DD", ch, ch));
        }
    }

    #[test]
    fn bit_scan_works() {
        for i in 0..64 {
            let bit = (1 as u64) << i;
            let index = bit_scan(bit);

            assert_eq!(i, index);
        }
    }

    #[test]
    fn test_bit_scan_with_multiple_bits() {
        for lowest_bit in 0..64 {
            let mut bit = 1 << lowest_bit;

            for other_bit in (lowest_bit + 1)..64 {
                if (other_bit + 37) % 3 != 0 {
                    bit |= 1 << other_bit;
                }
            }

            let bit_scan_result = bit_scan(bit);
            assert_eq!(lowest_bit, bit_scan_result);
        }
    }

    #[test]
    fn test_bit_scan_backward_with_multiple_bits() {
        for highest_bit in 0..64 {
            let mut bit = 1 << highest_bit;

            for other_bit in 0..highest_bit {
                if (other_bit + 37) % 3 != 0 {
                    bit |= 1 << other_bit;
                }
            }

            let bit_scan_result = bit_scan_backwards(bit);
            assert_eq!(highest_bit, bit_scan_result);
        }
    }

    #[test]
    fn test_extract_bits() {
        let input: u64 = 1 << 2 | 1 << 5 | 1 << 55;
        let output = extract_bits(input);

        assert_eq!(output, vec![2, 5, 55]);
    }

    #[test]
    fn test_index_correct() {
        let result = index(5, 4);
        assert_eq!(result, Some(4 * 8 + 3));
    }

    #[test]
    fn test_index_out_of_bounds() {
        let result = index(-1, 4);
        assert_eq!(result, None);

        let result = index(9, 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_rowcol_to_index() {
        let row = 4;
        let col = 7;

        let index = index(row, col).unwrap();
        let (new_row, new_col) = rowcol(index);

        assert_eq!(row, new_row);
        assert_eq!(col, new_col);
    }
}
