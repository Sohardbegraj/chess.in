type Bitboard = u64;

pub struct Rays {
    n_rays: Vec<Bitboard>,
    e_rays: Vec<Bitboard>,
    nw_rays: Vec<Bitboard>,
    ne_rays: Vec<Bitboard>,
    w_rays: Vec<Bitboard>,
    s_rays: Vec<Bitboard>,
    sw_rays: Vec<Bitboard>,
    se_rays: Vec<Bitboard>,
}

macro_rules! make_rays {
    ($ray_fn:ident) => {{
        let mut rays = vec![];
        for row in 1..=8 {
            for col in 1..=8 {
                rays.push($ray_fn(row, col));
            }
        }

        rays
    }};
}

impl Rays {
    fn initialize() -> Self {
        let n_rays = make_rays!(n_ray);
        let e_rays = make_rays!(e_ray);
        let nw_rays = make_rays!(nw_ray);
        let ne_rays = make_rays!(ne_ray);
        let w_rays = make_rays!(w_ray);
        let s_rays = make_rays!(s_ray);
        let sw_rays = make_rays!(sw_ray);
        let se_rays = make_rays!(se_ray);

        Self {
            n_rays: n_rays,
            e_rays: e_rays,
            nw_rays: nw_rays,
            ne_rays: ne_rays,
            w_rays: w_rays,
            s_rays: s_rays,
            sw_rays: sw_rays,
            se_rays: se_rays,
        }
    }
}

macro_rules! define_ray {
    ($name:ident, $offset_fn:expr) => {
        fn $name(row: i64, col: i64) -> Bitboard {
            let mut bitboard = 0;

            for offset in 1..=8 {
                bitboard = set_bit(bitboard, $offset_fn(row, col, offset));
            }

            bitboard
        }
    };
}
define_ray!(n_ray, |row, col, offset| (row + offset, col));
define_ray!(e_ray, |row, col, offset| (row, col + offset));
define_ray!(nw_ray, |row, col, offset| (row + offset, col - offset));
define_ray!(ne_ray, |row, col, offset| (row + offset, col + offset));
define_ray!(w_ray, |row, col, offset| (row, col - offset));
define_ray!(s_ray, |row, col, offset| (row - offset, col));
define_ray!(sw_ray, |row, col, offset| (row - offset, col - offset));
define_ray!(se_ray, |row, col, offset| (row - offset, col + offset));


fn set_bit(bitboard: Bitboard, row_col: (i64, i64)) -> Bitboard {
    let row = row_col.0;
    let col = row_col.1;
    if row < 1 || row > 8 || col < 1 || col > 8 {
        return bitboard;
    }
    bitboard | (1 << ((col - 1) + (row - 1) * 8))
}

fn bitboard_to_string(bitboard: Bitboard, mark: Option<usize>) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_n_ray() {
        let rays = Rays::initialize();
        let row = 5;
        let col = 4;
        let idx = (row - 1) * 8 + col - 1;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.n_rays[idx], Some(idx))
        );
    }

    #[test]
    fn print_se_ray() {
        let rays = Rays::initialize();
        let idx = 44;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.se_rays[idx], Some(idx))
        );
    }

    #[test]
    fn print_sw_ray() {
        let rays = Rays::initialize();
        let idx = 44;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.sw_rays[idx], Some(idx))
        );
    }

    #[test]
    fn print_nw_ray() {
        let rays = Rays::initialize();
        let idx = 44;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.nw_rays[idx], Some(idx))
        );
    }

    #[test]
    fn print_ne_ray() {
        let rays = Rays::initialize();
        let idx = 44;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.ne_rays[idx], Some(idx))
        );
    }

    #[test]
    fn print_e_ray() {
        let rays = Rays::initialize();
        let idx = 44;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.e_rays[idx], Some(idx))
        );
    }

    #[test]
    fn print_w_ray() {
        let rays = Rays::initialize();
        let idx = 44;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.w_rays[idx], Some(idx))
        );
    }

    #[test]
    fn print_s_ray() {
        let rays = Rays::initialize();
        let idx = 44;
        println!(
            "Here's the bitboard:\n--------------------\n{}\n--------------------",
            bitboard_to_string(rays.s_rays[idx], Some(idx))
        );
    }
}
