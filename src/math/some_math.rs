use pyo3::prelude::*;
use pyo3::types::PyAny;
use rug::Float;
use rug::{Assign, Integer};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::sync::OnceLock;

use lazy_static::lazy_static;

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

fn _test(n: &Integer, base: &Integer, s: u32, t: &Integer) -> bool {
    let mut b = Integer::from(base.pow_mod_ref(t, n).unwrap());

    if b == 1 || b == n - Integer::from(1) {
        return true;
    }

    if (s == 0) {
        return false;
    }

    for _ in 0..s - 1 {
        b = Integer::from(b.pow_mod_ref(&Integer::from(2), n).unwrap());
        if b == n - Integer::from(1) {
            return true;
        }
        if b == 1 {
            return false;
        }
    }
    false
}

pub fn miller_rabin_impl(n: &Integer) -> bool {
    if n > &Integer::from_str("3317044064679887385961981").unwrap() {
        // i dont want to return boolean on something that has 0.99999999% chance of being a prime
        // we have to be explicit, theres a big difference between 1 and 0.99999999
        panic!("you are using the boolean function for the non deterministic part of miller rabin. above 3317044064679887385961981 is probabilistic");
    }
    // TODO bases can be optimised https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
    // we have to pick it from there, sympy is only using miller rabin for the numbers which is optimal for
    let bases = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 47, 53, 59, 67, 71, 73, 79, 97,
    ];
    if n < &Integer::from(2) {
        return false;
    }

    let n_minus_one = n - Integer::from(1);
    let bit_scan_result = bit_scan1(&n_minus_one, 0 as u32);
    let s = bit_scan_result.expect("TODO - we assume no 0 passed in");
    let t = Integer::from(n >> s);

    for base in bases {
        let mut base_int = Integer::from(base);
        if base_int >= *n {
            base_int %= n;
        }

        if base_int >= Integer::from(2) {
            if !_test(n, &base_int, s, &t) {
                return false;
            }
        }
    }
    true
}

fn bit_scan1(x: &Integer, n: u32) -> Option<u32> {
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

#[derive(Debug)]
pub struct IntegerRing {
    zero: Integer,
    one: Integer,
}

impl IntegerRing {
    pub fn new() -> Self {
        IntegerRing {
            zero: Integer::from(0),
            one: Integer::from(1),
        }
    }

    pub fn zero(&self) -> Integer {
        self.zero.clone()
    }

    pub fn one(&self) -> Integer {
        self.one.clone()
    }

    pub fn gcd(&self, a: &Integer, b: &Integer) -> Integer {
        a.clone().gcd(&b)
    }

    fn gcdex(&self, a: &Integer, b: &Integer) -> (Integer, Integer, Integer) {
        // todo double check logic and unit test
        let mut x0 = Integer::from(1);
        let mut x1 = Integer::from(0);
        let mut y0 = Integer::from(0);
        let mut y1 = Integer::from(1);

        let mut a = a.clone();
        let mut b = b.clone();

        while b != 0 {
            let (q, r) = a.div_rem(b.clone());
            a = b.clone();
            b = r.clone();

            let temp_x = x0 - &q * &x1;
            let temp_y = y0 - &q * &y1;
            x0 = x1;
            x1 = temp_x;
            y0 = y1;
            y1 = temp_y;
        }

        (a, x0, y0)
    }

    fn lcm(a: &Integer, b: &Integer) -> Integer {
        let gcd = a.clone().gcd(b);
        let product = a.clone() * b.clone();

        product / gcd
    }

    pub fn sqrt(&self, a: &Integer) -> Integer {
        a.clone().sqrt()
    }

    pub fn is_square(&self, a: &Integer) -> bool {
        if a.cmp0() == Ordering::Less {
            return false;
        }
        let root = self.sqrt(a);
        let square = root.clone() * &root;
        square == *a
    }

    pub fn factorial(&self, n: &Integer) -> Option<Integer> {
        if n.cmp0() == Ordering::Less {
            return None;
        }

        let mut result = Integer::from(1);
        let mut i = Integer::from(2);

        while &i <= n {
            result *= &i;
            i += 1;
        }

        Some(result)
    }

    pub fn log(&self, a: &Integer, b: &Integer) -> Option<Integer> {
        if a.cmp0() <= Ordering::Equal || b.cmp0() <= Ordering::Equal || *b == Integer::from(1) {
            return None;
        }

        // This function uses ``math.log`` which is based on ``float`` so it will
        // fail for large integer arguments.
        let a_f64 = a.to_f64();
        let b_f64 = b.to_f64();

        Some(Integer::from(a_f64.log(b_f64).floor() as i64))
    }
}

impl PartialEq for IntegerRing {
    fn eq(&self, _other: &Self) -> bool {
        true // There's only one integer ring
    }
}

impl Eq for IntegerRing {}

impl Hash for IntegerRing {
    fn hash<H: Hasher>(&self, state: &mut H) {
        "ZZ".hash(state);
    }
}

static ZZ: OnceLock<IntegerRing> = OnceLock::new();
pub fn get_zz() -> &'static IntegerRing {
    ZZ.get_or_init(|| IntegerRing::new())
}
