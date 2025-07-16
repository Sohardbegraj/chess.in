use crate::utils::*;

type Bitboard = u64;

pub struct Rays {
    pub n_rays: Vec<Bitboard>,
    pub e_rays: Vec<Bitboard>,
    pub nw_rays: Vec<Bitboard>,
    pub ne_rays: Vec<Bitboard>,
    pub w_rays: Vec<Bitboard>,
    pub s_rays: Vec<Bitboard>,
    pub sw_rays: Vec<Bitboard>,
    pub se_rays: Vec<Bitboard>,
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
    pub fn new() -> Self {
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

// ........
// ........
// ........
// .X111111
// ........

fn set_bit(bitboard: Bitboard, row_col: (i64, i64)) -> Bitboard {
    let row = row_col.0;
    let col = row_col.1;
    if row < 1 || row > 8 || col < 1 || col > 8 {
        return bitboard;
    }
    bitboard | (1 << ((col - 1) + (row - 1) * 8))
}

fn first_hit(ray: Bitboard, forward_ray: bool, occupancy: Bitboard) -> Option<usize> {
    let intersection = ray & occupancy;
    if intersection == 0 {
        return None;
    } else {
        if forward_ray {
            return Some(bit_scan(intersection));
        } else {
            return Some(bit_scan_backwards(intersection));
        }
    }
}

pub fn blocked_ray_attack(
    ray: Bitboard,
    ray_family: &Vec<Bitboard>,
    forward_ray: bool,
    own_occupancy: Bitboard,
    enemy_occupancy: Bitboard,
) -> Bitboard {
    let enemy_overlap = ray & enemy_occupancy;
    let own_overlap = ray & own_occupancy;
    let first_own_hit = first_hit(ray, forward_ray, own_overlap);
    let first_enemy_hit = first_hit(ray, forward_ray, enemy_overlap);

    match (first_own_hit, first_enemy_hit) {
        (None, None) => ray,
        (None, Some(idx)) => {
            let ray_after = ray_family[idx];
            return ray ^ ray_after;
        }
        (Some(idx), None) => {
            let ray_after = ray_family[idx];
            return ray ^ (ray_after | 1 << idx);
        }
        (Some(own_idx), Some(en_idx)) => {
            let own_after = ray_family[own_idx];
            let en_after = ray_family[en_idx];
            return ray ^ ((own_after | 1 << own_idx) | en_after);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_occupancy_1() -> (Bitboard, Bitboard) {
        let mut own_occupancy = 0;
        for i in 0..16 {
            if i == 5 {
                continue;
            }
            own_occupancy |= 1 << i;
        }
        own_occupancy |= 1 << 22;

        let mut enemy_occupancy = 0;
        for i in 48..64 {
            if i == 57 || i == 49 {
                continue;
            }
            enemy_occupancy |= 1 << i;
        }
        enemy_occupancy |= 1 << 41;
        enemy_occupancy |= 1 << 42;
        (own_occupancy, enemy_occupancy)
    }

    fn get_occupancy_2() -> (Bitboard, Bitboard) {
        let mut own_occupancy = 0;
        for i in 32..48 {
            if i % 3 == 0 || i == 41 {
                continue;
            }
            own_occupancy |= 1 << i;
        }

        let mut enemy_occupancy = 0;
        for i in 48..64 {
            if i == 57 || i == 49 {
                continue;
            }
            enemy_occupancy |= 1 << i;
        }
        enemy_occupancy |= 1 << 41;
        enemy_occupancy |= 1 << 42;
        assert!(own_occupancy & enemy_occupancy == 0);

        (own_occupancy, enemy_occupancy)
    }

    fn get_occupancy_3() -> (Bitboard, Bitboard) {
        let mut own_occupancy = 0;
        for i in 16..32 {
            if i % 3 == 0 || i == 25 {
                continue;
            }
            own_occupancy |= 1 << i;
        }

        let mut enemy_occupancy = 0;
        for i in 32..48 {
            if i == 41 || i == 33 {
                continue;
            }
            enemy_occupancy |= 1 << i;
        }
        enemy_occupancy |= 1 << 25;
        enemy_occupancy |= 1 << 26;
        enemy_occupancy &= !own_occupancy;
        assert!(own_occupancy & enemy_occupancy == 0);

        (own_occupancy, enemy_occupancy)
    }

    #[test]
    fn test_ray() {
        let rays = Rays::new();
        let row = 6;
        let col = 7;
        let idx = (row - 1) * 8 + col - 1;

        let mut expected_sw_6_7: Bitboard = 0;
        for i in 1..=8 {
            if col > i && row > i {
                expected_sw_6_7 |= 1 << ((col - i - 1) + (row - i - 1) * 8);
            }
        }
        assert_eq!(rays.sw_rays[idx], expected_sw_6_7);

        let mut expected_w_6_7: Bitboard = 0;
        for i in 1..=8 {
            if col > i {
                expected_w_6_7 |= 1 << ((col - i - 1) + (row - 1) * 8);
            }
        }
        assert_eq!(rays.w_rays[idx], expected_w_6_7);

        let mut expected_ne_6_7: Bitboard = 0;
        for i in 1..=8 {
            if col + i <= 8 && row + i <= 8 {
                expected_ne_6_7 |= 1 << ((col + i - 1) + (row + i - 1) * 8);
            }
        }
        assert_eq!(rays.ne_rays[idx], expected_ne_6_7);
    }

    #[test]
    fn test_blocked_ray() {
        let (own_occupancy, enemy_occupancy) = get_occupancy_1();
        let rays = Rays::new();
        let row = 5;
        let col = 5;
        let idx = (row - 1) * 8 + col - 1;

        let blocked_attack = blocked_ray_attack(
            rays.nw_rays[idx],
            &rays.nw_rays,
            true,
            own_occupancy,
            enemy_occupancy,
        );
        assert_eq!(blocked_attack, 1 << (idx + 7) | 1 << (idx + 14));

        let blocked_attack = blocked_ray_attack(
            rays.nw_rays[idx],
            &rays.nw_rays,
            true,
            enemy_occupancy,
            own_occupancy,
        );
        assert_eq!(blocked_attack, 1 << (idx + 7));
    }

    #[test]
    fn test_blocked_ray_several_layers() {
        let (own_occupancy, enemy_occupancy) = get_occupancy_2();
        let rays = Rays::new();
        let row = 5;
        let col = 5;
        let idx = (row - 1) * 8 + col - 1;

        let blocked_attack = blocked_ray_attack(
            rays.nw_rays[idx],
            &rays.nw_rays,
            true,
            own_occupancy,
            enemy_occupancy,
        );
        assert_eq!(blocked_attack, 0);

        let blocked_attack = blocked_ray_attack(
            rays.nw_rays[idx],
            &rays.nw_rays,
            true,
            enemy_occupancy,
            own_occupancy,
        );
        assert_eq!(blocked_attack, 1 << (idx + 7));
    }

    #[test]
    fn test_blocked_backward_ray_several_layers() {
        let (own_occupancy, enemy_occupancy) = get_occupancy_3();

        let rays = Rays::new();
        let row = 7;
        let col = 2;
        let idx = (row - 1) * 8 + col - 1;

        let blocked_attack = blocked_ray_attack(
            rays.s_rays[idx],
            &rays.s_rays,
            false,
            own_occupancy,
            enemy_occupancy,
        );
        assert_eq!(
            blocked_attack,
            1 << ((2 - 1) + (4 - 1) * 8)
                | 1 << ((2 - 1) + (5 - 1) * 8)
                | 1 << ((2 - 1) + (6 - 1) * 8)
        );

        let blocked_attack = blocked_ray_attack(
            rays.s_rays[idx],
            &rays.s_rays,
            true,
            enemy_occupancy,
            own_occupancy,
        );
        assert_eq!(
            blocked_attack,
            1 << ((2 - 1) + (5 - 1) * 8) | 1 << ((2 - 1) + (6 - 1) * 8)
        );
    }
}
