use lazy_static::lazy_static;
use rug::{Complete, Integer};

fn create_small_trailing() -> [u32; 256] {
    let mut small_trailing = [0u32; 256];

    for j in 1..8 {
        let step = 1 << (j + 1);
        let val = j;

        for i in (1 << j..256).step_by(step) {
            for k in 0..(1 << (7 - j)) {
                if i + k < 256 {
                    small_trailing[(i + k) as usize] = val;
                }
            }
        }
    }

    small_trailing
}

lazy_static! {
    static ref SMALL_TRAILING: [u32; 256] = create_small_trailing();
}

pub fn bit_scan1(x: &Integer, n: u32) -> Option<u32> {
    if x.is_zero() {
        return None;
    }
    let mut x = x.clone();
    x >>= n;
    x = x.abs();

    let low_byte = (&x & Integer::from(0xFF_u32)).to_u32().unwrap();
    if low_byte != 0 {
        return Some(SMALL_TRAILING[low_byte as usize] + n);
    }

    let mut t = 8 + n;
    x >>= 8;

    let z = x.significant_bits() as u32 - 1;

    if x == Integer::from(1) << z {
        return Some(z + t);
    }

    if z < 300 {
        while (x.clone() & Integer::from(0xFF_u32)) == 0 {
            x >>= 8;
            t += 8;
        }
    } else {
        let mut p = z >> 1;
        while (&x & Integer::from(0xFF_u32)) == 0 {
            while &x & ((1 << p) - Integer::from(1)) != 0 {
                p >>= 1;
            }
            x >>= p;
            t += p;
        }
    }

    let final_byte = (&x & Integer::from(0xFF_u32)).to_u32().unwrap();
    Some(t + SMALL_TRAILING[final_byte as usize])
}
