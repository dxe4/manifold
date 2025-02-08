use lazy_static::lazy_static;
use num_cpus;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::ThreadPool;
use rayon::ThreadPoolBuilder;
use rug::ops::RemRoundingAssign;
use rug::{Complete, Integer};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::iter::successors;
use std::panic::{self, AssertUnwindSafe};
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering as AtomicOrdering;
use std::sync::Once;
use std::sync::{Arc, OnceLock};

static SMALL_POOL: OnceLock<Arc<ThreadPool>> = OnceLock::new();
static LARGE_POOL: OnceLock<Arc<ThreadPool>> = OnceLock::new();

fn get_small_pool() -> Arc<ThreadPool> {
    SMALL_POOL
        .get_or_init(|| {
            Arc::new(
                ThreadPoolBuilder::new()
                    // .stack_size(28 * 1024 * 1024)
                    .num_threads(2)
                    .build()
                    .expect("Failed to create small pool"),
            )
        })
        .clone()
}

fn get_large_pool() -> Arc<ThreadPool> {
    let num_cpus = num_cpus::get() - 1;
    LARGE_POOL
        .get_or_init(|| {
            Arc::new(
                ThreadPoolBuilder::new()
                    // .stack_size(28 * 1024 * 1024)
                    .num_threads((num_cpus).max(1))
                    .build()
                    .expect("Failed to create large pool"),
            )
        })
        .clone()
}

lazy_static! {
    static ref EMPTY_BASES: Vec<Integer> = Vec::new();
    static ref BASES_2: Vec<Integer> = vec![Integer::from(2)];
    static ref BASES_2_3: Vec<Integer> = vec![Integer::from(2), Integer::from(3)];
    static ref BASES_31_73: Vec<Integer> = vec![Integer::from(31), Integer::from(73)];
    static ref BASES_2_3_5: Vec<Integer> =
        vec![Integer::from(2), Integer::from(3), Integer::from(5)];
    static ref BASES_2_3_5_7: Vec<Integer> = vec![
        Integer::from(2),
        Integer::from(3),
        Integer::from(5),
        Integer::from(7),
    ];
    static ref BASES_2_7_61: Vec<Integer> =
        vec![Integer::from(2), Integer::from(7), Integer::from(61)];
    static ref BASES_2_13_23_1662803: Vec<Integer> = vec![
        Integer::from(2),
        Integer::from(13),
        Integer::from(23),
        Integer::from(1662803),
    ];
    static ref BASES_2_3_5_7_11: Vec<Integer> = vec![
        Integer::from(2),
        Integer::from(3),
        Integer::from(5),
        Integer::from(7),
        Integer::from(11),
    ];
    static ref BASES_2_3_5_7_11_13: Vec<Integer> = vec![
        Integer::from(2),
        Integer::from(3),
        Integer::from(5),
        Integer::from(7),
        Integer::from(11),
        Integer::from(13),
    ];
    static ref BASES_2_3_5_7_11_13_17: Vec<Integer> = vec![
        Integer::from(2),
        Integer::from(3),
        Integer::from(5),
        Integer::from(7),
        Integer::from(11),
        Integer::from(13),
        Integer::from(17),
    ];
    static ref BASES_2_3_5_7_11_13_17_19_23: Vec<Integer> = vec![
        Integer::from(2),
        Integer::from(3),
        Integer::from(5),
        Integer::from(7),
        Integer::from(11),
        Integer::from(13),
        Integer::from(17),
        Integer::from(19),
        Integer::from(23),
    ];
    static ref BASES_ALL: Vec<Integer> = vec![
        Integer::from(2),
        Integer::from(3),
        Integer::from(5),
        Integer::from(7),
        Integer::from(11),
        Integer::from(13),
        Integer::from(17),
        Integer::from(19),
        Integer::from(23),
        Integer::from(29),
        Integer::from(31),
        Integer::from(37),
    ];
}

fn miller_rabin_bases(n: &Integer) -> &'static Vec<Integer> {
    match n.cmp0() {
        std::cmp::Ordering::Less => &EMPTY_BASES,
        _ => match n.to_u64().unwrap_or(u64::MAX) {
            0..=2046 => &BASES_2,
            2047..=1373652 => &BASES_2_3,
            1373653..=9080190 => &BASES_31_73,
            9080191..=25326000 => &BASES_2_3_5,
            25326001..=3215031750 => &BASES_2_3_5_7,
            3215031751..=4759123140 => &BASES_2_7_61,
            4759123141..=1122004669632 => &BASES_2_13_23_1662803,
            1122004669633..=2152302898746 => &BASES_2_3_5_7_11,
            2152302898747..=3474749660382 => &BASES_2_3_5_7_11_13,
            3474749660383..=341550071728320 => &BASES_2_3_5_7_11_13_17,
            341550071728321..=3825123056546413050 => &BASES_2_3_5_7_11_13_17_19_23,
            _ => &BASES_ALL,
        },
    }
}

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

    if s == 0 {
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

pub fn miller_rabin_single(number: &Integer) -> bool {
    if number < &Integer::from(2) {
        return false;
    }

    let n_minus_one = number - Integer::from(1);
    let bit_scan_result = bit_scan1(&n_minus_one, 0);
    let s = bit_scan_result.expect("TODO - we assume no 0 passed in");
    let t = Integer::from(number >> s);

    let bases = miller_rabin_bases(number);

    for base in bases.iter() {
        let base_mod = if base >= number {
            (base % number).complete()
        } else {
            base.clone()
        };

        if base_mod >= Integer::from(2) && !_test(number, &base_mod, s, &t) {
            return false;
        }
    }

    true
}

pub fn miller_rabin_impl(low: &Integer, high: &Integer) -> Vec<bool> {
    if low > high {
        panic!("low > high");
    }

    if high >= &Integer::from_str("3317044064679887385961981").unwrap() {
        // i dont want to return boolean on something that has 0.99999999% chance of being a prime
        // we have to be explicit, theres a big difference between 1 and 0.99999999
        panic!("you are using the boolean function for the non deterministic part of miller rabin. above 3317044064679887385961981 is probabilistic");
    }
    let mut range_vec: Vec<Integer> = Vec::new();
    let mut current = low.clone();
    while current <= *high {
        range_vec.push(current.clone());
        current += 1;
    }

    let pool = get_large_pool();
    let pool_map = pool.install(|| range_vec.par_iter().map(|x| miller_rabin_single(x)));
    let res = pool_map.collect::<Vec<bool>>();
    return res;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miller_rabin_multiple() {
        let low = &Integer::from_str("341550071728321").unwrap();

        let high = (low + 1e7 as u32).complete();
        let result = miller_rabin_impl(low, &high);
    }
}
