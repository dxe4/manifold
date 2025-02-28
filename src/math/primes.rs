use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rug::{Complete, Integer};
use std::str::FromStr;

use super::bitscan::bit_scan1;
use super::common::lucas_lehmer_q;
use super::miller_rabin_bases::get_miller_rabin_bases;
use super::static_data::{PRIME_CACHE_LIMIT, SMALL_PRIME_CACHE};
use super::threading::get_large_pool;
use super::traits::IntegerLike;

#[inline]
fn _check_prime_cache<T: IntegerLike>(n: &T) -> Option<bool> {
    match n.to_u32() {
        Some(x) => {
            if x <= PRIME_CACHE_LIMIT {
                return Some(
                    SMALL_PRIME_CACHE
                        .binary_search(&n.to_u32().unwrap())
                        .is_ok(),
                );
            }
        }
        None => {}
    };
    return None;
}

fn _miller_rabin_test<T: IntegerLike>(n: &T, base: &T, s: u32, t: &T) -> bool {
    let mut b = T::pow_mod_t(base, t, n).unwrap_or(T::from_i64(0));

    if b == T::from_i64(1) || b == (n.clone() - T::from_i64(1)) {
        return true;
    }

    if s == 0 {
        return false;
    }

    for _ in 0..s - 1 {
        b = T::pow_mod_t(&b, &T::from_i64(2), n).unwrap_or(T::from_i64(0));
        if b == (n.clone() - T::from_i64(1)) {
            return true;
        }
        if b == T::from_i64(1) {
            return false;
        }
    }
    false
}

pub fn miller_rabin_single<T: IntegerLike>(number: &T) -> bool {
    if number.to_integer() >= Integer::from_str("3317044064679887385961981").unwrap() {
        panic!("using the non deterministic version of miller rabin");
    }

    if number <= &T::from_i64(1) {
        return false;
    }
    if number == &T::from_i64(2) {
        return true;
    }
    if number.rem_euclid(&T::from_i64(2)) == T::from_i64(0) {
        return false;
    }
    if number == &T::from_i64(3) {
        return true;
    }

    match _check_prime_cache(number) {
        Some(val) => return val,
        None => {}
    }

    let n_minus_one = number.clone() - T::from_i64(1);
    let bit_scan_result = bit_scan1(&n_minus_one, 0);
    let s = bit_scan_result.unwrap();
    let t = number.shr(s);

    let bases = get_miller_rabin_bases(&number.to_integer());

    for base in bases.iter() {
        let base_t = T::from_i64(*base as i64);
        let base_mod = if base_t >= number.clone() {
            base_t % number.clone()
        } else {
            base_t
        };

        if base_mod >= T::from_i64(2) && !_miller_rabin_test(number, &base_mod, s, &t) {
            return false;
        }
    }

    true
}

pub fn miller_rabin_impl<T: IntegerLike + Send + Sync>(low: &T, high: &T) -> Vec<bool> {
    if low > high {
        panic!("low > high");
    }
    if high.to_integer() >= Integer::from_str("3317044064679887385961981").unwrap() {
        panic!("using the non deterministic version of miller rabin");
    }

    let mut range_vec: Vec<T> = Vec::new();
    let mut current = if low == &T::from_i64(2) {
        range_vec.push(low.clone());
        low.clone().add_u32(1)
    } else {
        low.clone()
    };

    while current <= *high {
        range_vec.push(current.clone());
        current = current.add_u32(2);
    }
    println!("{}", range_vec.last().unwrap());
    let pool = get_large_pool();
    let result: Vec<bool> = pool.install(|| {
        range_vec
            .par_iter()
            .map(|x| miller_rabin_single(x))
            .collect()
    });

    // TODO fix this
    let mut modified: Vec<bool> = result.iter().flat_map(|&b| vec![b, false]).collect();

    if modified.len() - 1 > (high.clone() - low.clone()).to_u32().unwrap() as usize {
        modified.pop();
    }
    return modified;
}

pub fn is_mersenne_prime<T: IntegerLike>(num: &T) -> bool {
    if !num.is_mersenne_number() {
        return false;
    }

    let significant_bits = num.get_significant_bits();
    let q = Integer::from(significant_bits);
    if !miller_rabin_single(&q) {
        println!("miller rabin \n\n");
        return false;
    }
    lucas_lehmer_q(&q)
}

pub fn sieve<T: IntegerLike>(limit: usize) -> Vec<T> {
    let mut is_prime = vec![true; (limit + 1) >> 1];
    let mut primes = vec![T::from_i64(2)];

    let sqrt_limit = (limit as f64).sqrt() as usize;
    for i in (3..=limit).step_by(2) {
        if i > sqrt_limit && is_prime[i >> 1] {
            primes.push(T::from_i64(i as i64));
            continue;
        }
        if is_prime[i >> 1] {
            primes.push(T::from_i64(i as i64));
            let mut multiple = i * i;
            while multiple <= limit {
                is_prime[multiple >> 1] = false;
                multiple += i * 2;
            }
        }
    }

    primes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_miller_rabin_above_1mil() {
        let a = miller_rabin_single(&Integer::from(1_000_003));
        let b = miller_rabin_single(&Integer::from(1_000_019));
        let c = miller_rabin_single(&Integer::from(1_000_043));
        let d = miller_rabin_single(&Integer::from(1_000_081));

        assert_eq!(a, true);
        assert_eq!(b, false);
        assert_eq!(c, false);
        assert_eq!(d, true);
    }

    #[test]
    fn test_prime_test_cache_hit() {
        let a = &Integer::from(9973);
        let b = &Integer::from(9975);

        assert_eq!(miller_rabin_single(a), true);
        assert_eq!(miller_rabin_single(b), false);
    }

    #[test]
    fn test_miller_rabin_multiple() {
        let low = &Integer::from_str("341550071728321").unwrap();

        let high = (low + 1e2 as u32).complete();
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

    #[test]
    fn test_is_mersenne_prime() {
        assert!(is_mersenne_prime(&Integer::from(7)));
        assert!(is_mersenne_prime(&Integer::from(31)));
        assert!(is_mersenne_prime(&Integer::from(127)));
        assert!(is_mersenne_prime(&Integer::from(8191)));
    }
}
