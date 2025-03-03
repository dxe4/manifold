use rug::{Assign, Complete, Integer};
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Rem, Sub};

pub struct IntegerIterator {
    current: Integer,
    end: Integer,
    inclusive: bool,
}

impl IntegerIterator {
    fn new(start: Integer, end: Integer, inclusive: bool) -> Self {
        Self {
            current: start,
            end,
            inclusive,
        }
    }
}

impl Iterator for IntegerIterator {
    type Item = Integer;

    fn next(&mut self) -> Option<Self::Item> {
        let in_bounds = if self.inclusive {
            self.current <= self.end
        } else {
            self.current < self.end
        };
        if in_bounds {
            let result = self.current.clone();
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

pub trait IntegerGenerator {
    fn range_to(self) -> IntegerIterator;
    fn range_to_inclusive(self) -> IntegerIterator;
}

impl IntegerGenerator for Integer {
    fn range_to(self) -> IntegerIterator {
        IntegerIterator::new(Integer::from(0), self, false)
    }
    fn range_to_inclusive(self) -> IntegerIterator {
        IntegerIterator::new(Integer::from(0), self, true)
    }
}

pub trait IntegerLike:
    /*
    Ths hsould support i32, u64, u32 etc
    maybe via a macro
    the first step is to support not using GMP to speed performance
    also need to be careful with overflow
    */
    Clone
    + Display
    + PartialEq
    + Eq
    + Hash
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
{
    fn from_i64(i: i64) -> Self;
    fn to_i64(&self) -> i64;
    fn add_u32(&self, other: u32) -> Self;
    fn sub_u32(&self, other: u32) -> Self;
    fn to_u32(&self) -> Option<u32>;
    fn shl(&self, shift: u32) -> Self;
    fn shr(&self, shift: u32) -> Self;
    fn abs(self) -> Self;
    fn rem_euclid(&self, other: &Self) -> Self;
    fn bitwise_and(&self, other: &Self) -> Self;
    fn to_integer(&self) -> Integer;
    fn is_power_of_2(&self) -> bool;
    fn is_mersenne_number(&self) -> bool;
    fn get_significant_bits(&self) -> u32;
    fn trailing_zeros(&self) -> u32;
    fn modulo(&self, other: &Self) -> Self;
    fn rem(&self, other: &Self) -> Self;
    fn divisible_by_two(&self) -> bool;


    // fn modulo(self, other: Self) -> Self {
    //     self % other
    // }
    fn pow_mod_t(base: &Self, exp: &Self, modulus: &Self) -> Option<Self>;
}

impl IntegerLike for i64 {
    fn divisible_by_two(&self) -> bool {
        self % 2 == 0
    }
    fn pow_mod_t(base: &Self, exp: &Self, modulus: &Self) -> Option<Self> {
        if modulus <= &0 {
            return None;
        }
        if modulus == &1 {
            return Some(0);
        }
        if exp == &0 {
            return Some(1);
        }

        let mut base = base.rem_euclid(modulus);
        let mut exp = exp.clone().abs();
        let mut result = 1;

        while exp > 0 {
            if exp.bitwise_and(&1) == 1 {
                result = (result * base.clone()) % modulus.clone();
            }
            base = (base.clone() * base) % modulus.clone();
            exp = exp.shr(1);
        }

        Some(result)
    }
    fn rem(&self, other: &Self) -> Self {
        self.modulo(other)
    }

    fn modulo(&self, other: &Self) -> Self {
        self % other
    }
    fn get_significant_bits(&self) -> u32 {
        if self == &0 {
            return 0;
        }
        64 - self.leading_zeros()
    }

    fn trailing_zeros(&self) -> u32 {
        // TODO unit test that we are not off by 1
        // (compared with rust impl)
        if self == &0 {
            return 64;
        }
        self.trailing_zeros()
    }

    fn is_power_of_2(&self) -> bool {
        if self <= &0 {
            return false;
        }
        (self & (self - 1)) == 0
    }

    fn is_mersenne_number(&self) -> bool {
        if self <= &0 {
            return false;
        }
        let mut p: i64 = 1;
        while &((1i64 << p) - 1) <= self {
            if &((1i64 << p) - 1) == self {
                return true;
            }
            p += 1;
        }
        false
    }

    fn to_integer(&self) -> Integer {
        Integer::from(self.clone())
    }
    fn bitwise_and(&self, other: &Self) -> Self {
        *self & *other
    }

    fn rem_euclid(&self, other: &Self) -> Self {
        self.clone().rem_euclid(*other)
    }
    fn abs(self) -> Self {
        self.abs()
    }
    fn from_i64(i: i64) -> Self {
        i
    }

    fn to_i64(&self) -> i64 {
        *self
    }

    fn add_u32(&self, other: u32) -> Self {
        *self + other as i64
    }
    fn sub_u32(&self, other: u32) -> Self {
        *self - other as i64
    }

    fn to_u32(&self) -> Option<u32> {
        if self > &(u32::MAX as i64) {
            return None;
        }
        Some(*self as u32)
    }

    fn shl(&self, shift: u32) -> Self {
        *self << shift
    }

    fn shr(&self, shift: u32) -> Self {
        *self >> shift
    }
}

impl IntegerLike for Integer {
    fn divisible_by_two(&self) -> bool {
        self.is_even()
    }
    fn pow_mod_t(base: &Self, exp: &Self, modulus: &Self) -> Option<Self> {
        // base.pow_mod
        Some(base.clone().pow_mod(exp, modulus).unwrap())
    }
    fn rem(&self, other: &Self) -> Self {
        self.modulo(other)
    }

    fn modulo(&self, other: &Self) -> Self {
        (self % other).complete()
    }

    fn get_significant_bits(&self) -> u32 {
        self.significant_bits()
    }
    fn trailing_zeros(&self) -> u32 {
        let significant_bits = self.significant_bits() - 1;

        for i in 0..significant_bits {
            if self.get_bit(i) {
                return i;
            }
        }
        significant_bits
    }

    fn is_power_of_2(&self) -> bool {
        let significant_bits = self.significant_bits() - 1;

        for i in 0..significant_bits {
            if self.get_bit(i) {
                return false;
            }
        }
        true
    }

    fn is_mersenne_number(&self) -> bool {
        /*
        2^n -> 10000000
        2^n -1 -> 11111111
        so this can be detected from bitwise shifts only
        */
        let significant_bits = self.significant_bits() - 1;

        for i in 0..significant_bits {
            if !self.get_bit(i) {
                return false;
            }
        }
        true
    }
    fn to_integer(&self) -> Integer {
        self.clone()
    }
    fn bitwise_and(&self, other: &Self) -> Self {
        (self & other).complete()
    }
    fn rem_euclid(&self, other: &Self) -> Self {
        let mut result = (self % other).complete();
        if result < Integer::from(0) {
            result += other.clone().abs();
        }
        result
    }

    fn abs(self) -> Self {
        self.abs()
    }
    fn from_i64(i: i64) -> Self {
        Integer::from(i)
    }

    fn to_i64(&self) -> i64 {
        self.to_i64().unwrap()
    }

    fn add_u32(&self, other: u32) -> Self {
        self.add(Integer::from(other))
    }
    fn sub_u32(&self, other: u32) -> Self {
        self.sub(Integer::from(other))
    }

    fn to_u32(&self) -> Option<u32> {
        self.to_u32()
    }

    fn shl(&self, shift: u32) -> Self {
        (self << shift).complete()
    }

    fn shr(&self, shift: u32) -> Self {
        (self >> shift).complete()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rug_int_vec;

    #[test]
    fn test_pow_mod_integer() {
        let result = Integer::pow_mod_t(
            &Integer::from_i64(2),
            &Integer::from_i64(3),
            &Integer::from_i64(3),
        )
        .unwrap();
        assert_eq!(result, Integer::from_i64(2));
    }

    #[test]
    fn test_pow_mod_i64() {
        let result = i64::pow_mod_t(&2, &3, &3).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_range_to_small() {
        let end = Integer::from(5);
        let result: Vec<Integer> = end.range_to().collect();
        let expected: Vec<Integer> = rug_int_vec![0, 1, 2, 3, 4];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_range_to_inclusive() {
        let end = Integer::from(5);
        let result: Vec<Integer> = end.range_to_inclusive().collect();
        let expected: Vec<Integer> = rug_int_vec![0, 1, 2, 3, 4, 5];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_range_to_zero() {
        let end = Integer::from(0);
        let result: Vec<Integer> = end.range_to().collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_range_to_large() {
        let end = Integer::from(100);
        let count = end.range_to().count();
        assert_eq!(count, 100);
    }

    #[test]
    fn test_range_to_negative() {
        let end = Integer::from(-5);
        let result: Vec<Integer> = end.range_to().collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_power_of_2() {
        assert_eq!(true, Integer::from(2).is_power_of_two());
        assert_eq!(true, Integer::from(4).is_power_of_two());
        assert_eq!(true, Integer::from(8).is_power_of_two());
        assert_eq!(true, Integer::from(16).is_power_of_two());
        assert_eq!(true, Integer::from(32).is_power_of_two());
        assert_eq!(true, Integer::from(64).is_power_of_two());
        assert_eq!(true, Integer::from(128).is_power_of_two());
        assert_eq!(true, Integer::from(256).is_power_of_two());
        assert_eq!(true, Integer::from(512).is_power_of_two());
    }

    #[test]
    fn test_trailing_zeros() {
        assert_eq!(Integer::from(2).trailing_zeros(), Integer::from(1));
        assert_eq!(Integer::from(4).trailing_zeros(), Integer::from(2));
        assert_eq!(Integer::from(8).trailing_zeros(), Integer::from(3));
        assert_eq!(Integer::from(16).trailing_zeros(), Integer::from(4));
        assert_eq!(Integer::from(32).trailing_zeros(), Integer::from(5));
        assert_eq!(Integer::from(64).trailing_zeros(), Integer::from(6));
        assert_eq!(Integer::from(128).trailing_zeros(), Integer::from(7));
        assert_eq!(Integer::from(256).trailing_zeros(), Integer::from(8));
        assert_eq!(Integer::from(512).trailing_zeros(), Integer::from(9));
    }
    #[test]
    fn test_mersennse() {
        assert_eq!(Integer::from(3).is_mersenne_number(), true);
        assert_eq!(Integer::from(7).is_mersenne_number(), true);
        assert_eq!(Integer::from(11).is_mersenne_number(), false);
        assert_eq!(Integer::from(17).is_mersenne_number(), false);
        assert_eq!(Integer::from(31).is_mersenne_number(), true);
        assert_eq!(Integer::from(8191).is_mersenne_number(), true);

        assert_eq!(Integer::from(8).is_mersenne_number(), false);
        assert_eq!(Integer::from(16).is_mersenne_number(), false);
        assert_eq!(Integer::from(32).is_mersenne_number(), false);

        assert_eq!(Integer::from(8 - 1).is_mersenne_number(), true);
        assert_eq!(Integer::from(16 - 1).is_mersenne_number(), true);
        assert_eq!(Integer::from(32 - 1).is_mersenne_number(), true);

        // this check for 110000
        assert_eq!(Integer::from(8 + (8 >> 1)).is_mersenne_number(), false);
        assert_eq!(Integer::from(16 + (16 >> 1)).is_mersenne_number(), false);
        assert_eq!(Integer::from(32 + (32 >> 1)).is_mersenne_number(), false);

        assert_eq!(Integer::from(8 + (8 >> 2)).is_mersenne_number(), false);
        assert_eq!(Integer::from(8 + (8 >> 3)).is_mersenne_number(), false);
    }

    #[test]
    fn test_is_power_of_w() {
        assert_eq!(Integer::from(8).is_power_of_2(), true);
        assert_eq!(Integer::from(16).is_power_of_2(), true);
        assert_eq!(Integer::from(32).is_power_of_2(), true);

        assert_eq!(Integer::from(8 - 1).is_power_of_2(), false);
        assert_eq!(Integer::from(16 - 1).is_power_of_2(), false);
        assert_eq!(Integer::from(32 - 1).is_power_of_2(), false);

        assert_eq!(Integer::from(8 + 1).is_power_of_2(), false);
        assert_eq!(Integer::from(16 + 1).is_power_of_2(), false);
        assert_eq!(Integer::from(32 + 1).is_power_of_2(), false);
    }
}
