use lazy_static::lazy_static;
use rug::Integer;

use super::traits::IntegerLike;

fn create_small_trailing() -> [u32; 256] {
    let mut small_trailing = [0u32; 256];
    small_trailing[0] = 0;
    for i in 1..256 {
        let mut val = 0;
        let mut num = i;
        while (num & 1) == 0 && val < 8 {
            val += 1;
            num >>= 1;
        }
        small_trailing[i] = val;
    }

    small_trailing
}

lazy_static! {
    static ref SMALL_TRAILING: [u32; 256] = create_small_trailing();
}

pub fn bit_scan1<T: IntegerLike>(x: &T, n: u32) -> Option<u32> {
    if x == &T::from_i64(0) {
        return None;
    }
    let mut x = x.clone();
    x = x.shr(n);
    x = x.abs();

    let low_byte = (x.bitwise_and(&T::from_i64(0xFF_i64))).to_i64();
    if low_byte != 0 {
        return Some(SMALL_TRAILING[low_byte as usize] + n);
    }

    let mut t = 8 + n;
    x = x.shr(8);

    let z = x.get_significant_bits() as u32 - 1;

    if x == T::from_i64(1).shr(z) {
        return Some(z + t);
    }

    if z < 300 {
        while (x.bitwise_and(&T::from_i64(0xFF_i64))) == T::from_i64(0) {
            x = x.shr(8);
            t += 8;
        }
    } else {
        let mut p = z >> 1;
        while (x.bitwise_and(&T::from_i64(0xFF_i64))) == T::from_i64(0) {
            while x.bitwise_and(&T::from_i64(((1 << p) - 1) as i64)) != T::from_i64(0) {
                p >>= 1;
            }
            x = x.shr(p);
            // x >>= p;
            t += p;
        }
    }

    let final_byte = (x.bitwise_and(&T::from_i64(0xFF_i64))).to_i64();
    Some(t + SMALL_TRAILING[final_byte as usize])
}
