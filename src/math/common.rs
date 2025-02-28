use rug::{ops::Pow, Complete, Integer};
use std::collections::HashSet;

use super::num_utils::pow_large;
use super::traits::IntegerLike;
use super::{primes::sieve, traits::IntegerGenerator};

pub fn binary_gcd<T: IntegerLike>(mut a: T, mut b: T) -> T {
    if a.is_zero() {
        return b;
    }
    if b.is_zero() {
        return a;
    }

    let shift = (&a.bitwise_or(&b)).trailing_zeros();

    a = a.shr(a.trailing_zeros());
    b = b.shr(b.trailing_zeros());

    while !b.is_zero() {
        while b.divisible_by_two() {
            b = b.shr(1);
        }

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        b = b - a.clone();
    }

    a.shl(shift)
}

pub fn euclidean_gcd<T: IntegerLike>(mut a: T, mut b: T) -> T {
    while !b.is_zero() {
        let temp = a % b.clone();
        a = b;
        b = temp;
    }
    a
}

pub fn is_coprime<T: IntegerLike>(a: T, b: T) -> bool {
    return binary_gcd(a, b) == T::from_i64(1);
}

pub fn lucas_lehmer_q<T: IntegerLike>(prime_q: &T) -> bool {
    if prime_q < &T::from_i64(2) {
        return false;
    }

    let m_q = pow_large(&Integer::from(2), &(prime_q.to_integer())) - Integer::from(1);

    let mut s = Integer::from(4);

    let q_loop = prime_q.clone() - T::from_i64(2);
    let mut cnt = T::from_i64(0);

    while cnt < q_loop {
        s = (s.square() - Integer::from(2)) % &m_q;
        cnt = cnt + T::from_i64(1);
    }

    s.is_zero()
}

pub fn quadratic_residues<T: IntegerLike>(p: &T) -> Vec<Integer> {
    let mut residues = HashSet::new();

    let half_p = p.clone() / T::from_i64(2) + T::from_i64(1);

    // todo fix this
    for current in half_p.to_integer().range_to() {
        let current_t = T::from_integer(current).unwrap();
        let residue = T::pow_mod_t(&current_t, &T::from_i64(2), p).unwrap();
        residues.insert(residue);
    }
    let mut residues_vec: Vec<Integer> = residues.into_iter().map(|x| x.to_integer()).collect();
    residues_vec.sort();
    residues_vec
}

pub fn mobius(n: &Integer) -> i32 {
    if n == &Integer::from(1) {
        return 1;
    }
    let n_u32 = n.to_u32();
    let primes = sieve::<Integer>(n_u32.unwrap() as usize);
    let mut n = n.clone();
    let mut count = 0;
    for prime in primes {
        let squared = prime.clone() << 1;

        if &squared > &n {
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
        let res = sieve::<Integer>(10);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_quadradic_residue() {
        let residues = quadratic_residues(&Integer::from(7));
        let epxected = rug_int_vec![0, 1, 2, 4];
        assert_eq!(residues, epxected);
    }

    #[test]
    fn test_euclidean_gcd() {
        assert_eq!(
            euclidean_gcd(Integer::from(12), Integer::from(18)),
            Integer::from(6)
        );
        assert_eq!(
            euclidean_gcd(Integer::from(24), Integer::from(36)),
            Integer::from(12)
        );
        assert_eq!(
            euclidean_gcd(Integer::from(35), Integer::from(49)),
            Integer::from(7)
        );
        assert_eq!(
            euclidean_gcd(Integer::from(56), Integer::from(98)),
            Integer::from(14)
        );
        assert_eq!(
            euclidean_gcd(Integer::from(101), Integer::from(103)),
            Integer::from(1)
        );
    }

    #[test]
    fn test_binary_gcd() {
        assert_eq!(
            binary_gcd(Integer::from(12), Integer::from(18)),
            Integer::from(6)
        );
        assert_eq!(
            binary_gcd(Integer::from(24), Integer::from(36)),
            Integer::from(12)
        );
        assert_eq!(
            binary_gcd(Integer::from(35), Integer::from(49)),
            Integer::from(7)
        );
        assert_eq!(
            binary_gcd(Integer::from(56), Integer::from(98)),
            Integer::from(14)
        );
        assert_eq!(
            binary_gcd(Integer::from(101), Integer::from(103)),
            Integer::from(1)
        );
    }
}
