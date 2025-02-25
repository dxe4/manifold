use std::collections::HashSet;

use super::{primes::sieve, traits::IntegerGenerator};
use rug::{ops::Pow, Complete, Integer};

use super::{
    num_utils::pow_large,
    primes::{is_mersenne_number, miller_rabin_single},
};

pub fn is_power_of_2(n: &Integer) -> bool {
    let significant_bits = n.significant_bits() - 1;

    for i in 0..significant_bits {
        if n.get_bit(i) {
            return false;
        }
    }
    true
}

pub fn contains_zero_in_binary(n: &Integer) -> bool {
    /*
     TODO
     shall we rename this to is_mersenne number?
     anme is misleading
    */
    let significant_bits = n.significant_bits();

    for i in 0..significant_bits - 1 {
        if !n.get_bit(i) {
            return true;
        }
    }
    false
}

pub fn trailing_zeros(n: &Integer) -> u32 {
    /*
     TODO this logic is used in multiple places
     make one central function and use that
    */
    let significant_bits = n.significant_bits();

    for i in 0..significant_bits {
        if n.get_bit(i) {
            return i;
        }
    }
    significant_bits
}

pub fn binary_gcd(mut a: Integer, mut b: Integer) -> Integer {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    let shift = trailing_zeros(&(&a | &b).complete());

    a >>= trailing_zeros(&a);
    b >>= trailing_zeros(&b);

    while b != 0 {
        while b.is_even() {
            b >>= 1;
        }

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        b -= &a;
    }

    a << shift
}

pub fn euclidean_gcd(mut a: Integer, mut b: Integer) -> Integer {
    while b != 0 {
        let temp = a % &b;
        a = b;
        b = temp;
    }
    a
}

pub fn is_coprime(a: Integer, b: Integer) -> bool {
    return binary_gcd(a.clone(), b.clone()) == 1;
}

pub fn lucas_lehmer_q(prime_q: &Integer) -> bool {
    if prime_q < &2 {
        return false;
    }

    let m_q = pow_large(&Integer::from(2), &prime_q) - Integer::from(1);

    let mut s = Integer::from(4);

    let q_loop = prime_q - Integer::from(2);
    let mut cnt = Integer::from(0);

    while &cnt < &q_loop {
        s = (s.square() - Integer::from(2)) % &m_q;
        cnt += 1;
    }

    s.is_zero()
}

pub fn quadratic_residues(p: &Integer) -> Vec<Integer> {
    let mut residues = HashSet::new();

    let half_p = p.clone() / Integer::from(2) + Integer::from(1);

    for current in half_p.range_to() {
        let residue = current.pow_mod(&Integer::from(2), p).unwrap();
        residues.insert(residue);
    }
    let mut residues_vec: Vec<Integer> = residues.into_iter().collect();
    residues_vec.sort();
    residues_vec
}

pub fn mobius(n: &Integer) -> i32 {
    if n == &Integer::from(1) {
        return 1;
    }
    let n_u32 = n.to_u32();
    let primes = sieve(n_u32.unwrap() as usize);
    let mut n = n.clone();
    let mut count = 0;
    for prime in primes {
        if &(&prime * &prime).complete() > &n {
            break;
        }
        if n.is_divisible(&prime) {
            n = n / &prime;
            if n.is_divisible(&prime) {
                return 0;
            }
            count += 1;
        }
    }
    if n > 1 {
        count += 1;
    }
    if count % 2 == 0 {
        1
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::rug_int_vec;

    use super::*;
    #[test]
    fn test_lucas_lehmer_primes() {
        assert!(lucas_lehmer_q(&Integer::from(3)));
        assert!(lucas_lehmer_q(&Integer::from(5)));
        assert!(lucas_lehmer_q(&Integer::from(7)));
        assert!(lucas_lehmer_q(&Integer::from(13)));
    }

    #[test]
    fn test_gcd() {
        let egcd = euclidean_gcd(Integer::from(3), Integer::from(1));
        let bgcd = euclidean_gcd(Integer::from(3), Integer::from(1));
        assert_eq!(bgcd, egcd);
        assert_eq!(bgcd, Integer::from(1));
    }

    #[test]
    fn test_coprime() {
        let egcd = euclidean_gcd(Integer::from(3), Integer::from(1));
        let bgcd = euclidean_gcd(Integer::from(3), Integer::from(1));
        assert_eq!(bgcd, egcd);
        assert_eq!(bgcd, Integer::from(1));
    }
    #[test]
    fn test_coprime_numbers() {
        assert_eq!(is_coprime(Integer::from(8), Integer::from(15)), true);
        assert_eq!(is_coprime(Integer::from(13), Integer::from(27)), true);
        assert_eq!(is_coprime(Integer::from(35), Integer::from(64)), true);
        assert_eq!(is_coprime(Integer::from(17), Integer::from(19)), true);
        assert_eq!(is_coprime(Integer::from(9), Integer::from(28)), true);
    }

    #[test]
    fn test_non_coprime_numbers() {
        assert_eq!(is_coprime(Integer::from(12), Integer::from(18)), false);
        assert_eq!(is_coprime(Integer::from(24), Integer::from(36)), false);
        assert_eq!(is_coprime(Integer::from(14), Integer::from(21)), false);
        assert_eq!(is_coprime(Integer::from(16), Integer::from(40)), false);
        assert_eq!(is_coprime(Integer::from(30), Integer::from(45)), false);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(is_coprime(Integer::from(1), Integer::from(1)), true);
        assert_eq!(is_coprime(Integer::from(1), Integer::from(100)), true);
        assert_eq!(is_coprime(Integer::from(0), Integer::from(5)), false);
        assert_eq!(is_coprime(Integer::from(7), Integer::from(0)), false);
        assert_eq!(is_coprime(Integer::from(5), Integer::from(5)), false);
    }

    #[test]
    fn test_power_of_2() {
        assert!(is_power_of_2(&Integer::from(2)));
        assert!(is_power_of_2(&Integer::from(4)));
        assert!(is_power_of_2(&Integer::from(8)));
        assert!(is_power_of_2(&Integer::from(16)));
        assert!(is_power_of_2(&Integer::from(32)));
        assert!(is_power_of_2(&Integer::from(64)));
        assert!(is_power_of_2(&Integer::from(128)));
        assert!(is_power_of_2(&Integer::from(256)));
        assert!(is_power_of_2(&Integer::from(512)));
    }

    #[test]
    fn test_trailing_zeros() {
        assert_eq!(trailing_zeros(&Integer::from(2)), Integer::from(1));
        assert_eq!(trailing_zeros(&Integer::from(4)), Integer::from(2));
        assert_eq!(trailing_zeros(&Integer::from(8)), Integer::from(3));
        assert_eq!(trailing_zeros(&Integer::from(16)), Integer::from(4));
        assert_eq!(trailing_zeros(&Integer::from(32)), Integer::from(5));
        assert_eq!(trailing_zeros(&Integer::from(64)), Integer::from(6));
        assert_eq!(trailing_zeros(&Integer::from(128)), Integer::from(7));
        assert_eq!(trailing_zeros(&Integer::from(256)), Integer::from(8));
        assert_eq!(trailing_zeros(&Integer::from(512)), Integer::from(9));
    }

    #[test]
    fn test_mobius() {
        assert_eq!(mobius(&Integer::from(1)), 1);
        assert_eq!(mobius(&Integer::from(2)), -1);
        assert_eq!(mobius(&Integer::from(3)), -1);
        assert_eq!(mobius(&Integer::from(4)), 0);
        assert_eq!(mobius(&Integer::from(5)), -1);
        assert_eq!(mobius(&Integer::from(6)), 1);
        assert_eq!(mobius(&Integer::from(10)), 1);
        assert_eq!(mobius(&Integer::from(12)), 0);
        assert_eq!(mobius(&Integer::from(30)), -1);
    }
    #[test]
    fn test_sieve_limit_10() {
        let expected = rug_int_vec![2, 3, 5, 7];
        let res = sieve(10);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_quadradic_residue() {
        let residues = quadratic_residues(&Integer::from(7));
        let epxected = rug_int_vec![0, 1, 2, 4];
        assert_eq!(residues, epxected);
    }

    #[test]
    fn test_contains_zero_in_binary() {
        assert_eq!(contains_zero_in_binary(&Integer::from(8)), true);
        assert_eq!(contains_zero_in_binary(&Integer::from(16)), true);
        assert_eq!(contains_zero_in_binary(&Integer::from(32)), true);

        assert_eq!(contains_zero_in_binary(&Integer::from(8 - 1)), false);
        assert_eq!(contains_zero_in_binary(&Integer::from(16 - 1)), false);
        assert_eq!(contains_zero_in_binary(&Integer::from(32 - 1)), false);
    }
}
