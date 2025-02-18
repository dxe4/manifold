use lazy_static::lazy_static;
use num_cpus;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;
use rug::{Complete, Integer};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use super::bitscan::bit_scan1;
use super::miller_rabin_bases::get_miller_rabin_bases;
use super::threading::get_large_pool;

fn _miller_rabin_test(n: &Integer, base: &Integer, s: u32, t: &Integer) -> bool {
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
    if number >= &Integer::from_str("3317044064679887385961981").unwrap() {
        // i dont want to return boolean on something that has 0.99999999% chance of being a prime
        // we have to be explicit, theres a big difference between 1 and 0.99999999
        panic!("you are using the boolean function for the non deterministic part of miller rabin. above 3317044064679887385961981 is probabilistic");
    }

    if number <= &Integer::from(1) {
        return false;
    }
    if number == &Integer::from(2) {
        return true;
    }
    if number.is_even() {
        return false;
    }
    if number == &Integer::from(3) {
        return true;
    }

    let n_minus_one = number - Integer::from(1);
    let bit_scan_result = bit_scan1(&n_minus_one, 0);
    let s = bit_scan_result.expect("TODO - we assume no 0 passed in");
    let t = Integer::from(number >> s);

    let bases = get_miller_rabin_bases(number);

    for base in bases.iter() {
        let base_mod = if base >= number {
            (base % number).complete()
        } else {
            base.clone()
        };

        if base_mod >= Integer::from(2) && !_miller_rabin_test(number, &base_mod, s, &t) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miller_rabin_multiple() {
        let low = &Integer::from_str("341550071728321").unwrap();

        let high = (low + 1e7 as u32).complete();
        let _ = miller_rabin_impl(low, &high);
    }

    #[test]
    fn test_valid_range() {
        let low = Integer::from(2);
        let high = Integer::from(10);
        let result = miller_rabin_impl(&low, &high);

        assert_eq!(result.len(), 9);
    }

    #[test]
    fn test_single_prime() {
        let low = Integer::from(7);
        let high = Integer::from(7);
        let result = miller_rabin_impl(&low, &high);

        assert_eq!(result, vec![true]);
    }

    #[test]
    fn test_few_bigger_primes() {
        let low = Integer::from(1000000007);
        let high = Integer::from(1000000010);
        let result = miller_rabin_impl(&low, &high);
        assert_eq!(result, vec![true, false, true, false]);
    }

    #[test]
    fn test_single_composite() {
        let low = Integer::from(8);
        let high = Integer::from(8);
        let result = miller_rabin_impl(&low, &high);

        assert_eq!(result, vec![false]);
    }
}
