use super::num_utils::pow_large;
use super::threading::get_large_pool;
use rayon::prelude::*;
use rug::ops::Pow;
use rug::{Complete, Integer, Rational};
use std::ops::{Add, Mul, Sub};

use std::default::Default;

#[derive(Debug)]
pub struct NumberConfig {
    pub base: usize,
    pub start: usize,
    pub exponent: usize,
    pub end: usize,
    pub digit_limit: u32,
}

impl Default for NumberConfig {
    fn default() -> Self {
        Self {
            base: 2,
            start: 1,
            end: 100,
            exponent: 10,
            digit_limit: 250,
        }
    }
}

pub fn x_pow_y_pow_z_mod_k(number_config: NumberConfig) -> Vec<String> {
    /*
        x = 10^n
        result = 2^x
    */
    let pool = get_large_pool();

    let mut results = vec![String::new(); (number_config.end - number_config.start + 1) as usize];

    pool.install(|| {
        results.par_iter_mut().enumerate().for_each(|(i, result)| {
            let n = number_config.start + i;

            let modulus = Integer::from(number_config.exponent).pow(number_config.digit_limit);
            let exponent = pow_large(&Integer::from(number_config.exponent), &Integer::from(n));

            // Calculate 2^(10^n) mod 10^DIGIT_LIMIT
            let res = Integer::from(number_config.base)
                .pow_mod(&exponent, &modulus)
                .unwrap();
            *result = res.to_string();
        });
    });

    results
}

#[derive(Debug, Clone)]
pub struct TwoAdicInteger {
    /*
     TODO
     the adic package is quite good
     we should use that eventually
     for now we only need valuation and distance so its fine
     https://lib.rs/crates/adic
    */
    pub value: Integer,
}

impl TwoAdicInteger {
    pub fn new(value: Integer) -> Self {
        Self { value }
    }

    pub fn valuation(&self) -> u32 {
        let abs_self = self.value.clone().abs();
        let significant_bits = abs_self.significant_bits();

        for i in 0..significant_bits {
            if abs_self.get_bit(i) {
                return i;
            }
        }
        return significant_bits;
    }

    pub fn distance(&self, other: &Self) -> Rational {
        let diff = (&self.value - &other.value).complete();
        let valuation = TwoAdicInteger::new(diff).valuation();
        let denominator = Integer::from(1) << valuation;
        return Rational::from((1, denominator));
    }
}

impl Add for TwoAdicInteger {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl Mul for TwoAdicInteger {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
}

impl Sub for TwoAdicInteger {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rug::{Integer, Rational};

    #[test]
    fn test_valuation() {
        assert_eq!(TwoAdicInteger::new(Integer::from(8)).valuation(), 3);
        assert_eq!(TwoAdicInteger::new(Integer::from(12)).valuation(), 2);
        assert_eq!(TwoAdicInteger::new(Integer::from(1)).valuation(), 0);
        assert_eq!(TwoAdicInteger::new(Integer::from(0)).valuation(), 0);
        assert_eq!(TwoAdicInteger::new(Integer::from(7)).valuation(), 0);
    }

    #[test]
    fn test_distance() {
        let a = TwoAdicInteger::new(Integer::from(8));
        let b = TwoAdicInteger::new(Integer::from(12));
        let expected_distance = Rational::from((1, Integer::from(4)));

        assert_eq!(a.distance(&b), expected_distance);
    }

    #[test]
    fn test_distance_2() {
        let a = TwoAdicInteger::new(Integer::from(3));
        let b = TwoAdicInteger::new(Integer::from(5));
        let expected_distance = Rational::from((1, Integer::from(2)));
        assert_eq!(a.distance(&b), expected_distance);
    }

    #[test]
    fn test_distance_3() {
        let a = TwoAdicInteger::new(Integer::from(16));
        let b = TwoAdicInteger::new(Integer::from(4));
        let expected_distance = Rational::from((1, Integer::from(4)));
        assert_eq!(a.distance(&b), expected_distance);
    }

    #[test]
    fn test_distance_abs() {
        let a = TwoAdicInteger::new(Integer::from(27));
        let b = TwoAdicInteger::new(Integer::from(81));

        assert_eq!(a.distance(&b), b.distance(&a));
    }
}
