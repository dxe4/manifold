use crate::math::miller_rabin_single;
use rayon::prelude::*;
use rug::{Complete, Integer, Rational};
use std::{collections::HashMap, ops::Rem};

use super::traits::IntegerLike;

pub fn miller_rabin_single_witness<T: IntegerLike>(n: &T, a: &T) -> bool {
    if n < &T::from_i64(2) || n.divisible_by_two() {
        return n == &T::from_i64(2);
    }
    if a >= n {
        return true;
    }

    let mut s = 0u32;

    let mut d = n.sub_u32(1);

    while d.divisible_by_two() {
        s += 1;
        d = d.shr(1);
    }

    let mut x = T::pow_mod_t(a, &d, n).unwrap_or(T::from_i64(0));

    if x == T::from_i64(1) || x == (n.sub_u32(1)) {
        return true;
    }
    for _ in 0..s - 1 {
        x = (x.clone() * x) % n.clone();
        if x == (n.clone() - T::from_i64(1)) {
            return true;
        }
    }
    false
}

pub fn track_witness_accuracy<T: IntegerLike + Send + Sync>(start: u32, end: u32)
where
    T: std::fmt::Display,
{
    let start = T::from_i64(start as i64);
    let end = T::from_i64(end as i64);

    let numbers: Vec<T> = (start.to_i64()..=end.to_i64())
        .map(|n| T::from_i64(n as i64))
        .collect();

    let primes: HashMap<T, bool> = numbers
        .iter()
        .map(|n| (n.clone(), miller_rabin_single(n))) // Assuming miller_rabin_single is adapted for IntegerLike
        .collect();

    let composites: Vec<T> = numbers.iter().filter(|n| !primes[n]).cloned().collect();

    let prime_nums: Vec<T> = numbers.iter().filter(|n| primes[n]).cloned().collect();

    let max_n = numbers.last().unwrap();
    let witnesses: Vec<T> = (2..max_n.to_i64() - 1)
        .map(|n| T::from_i64(n as i64))
        .collect();

    let mut stats: HashMap<T, (u32, u32, u32, u32)> = witnesses
        .iter()
        .map(|a| (a.clone(), (0, 0, 0, 0)))
        .collect();

    for n in &numbers {
        for a in &witnesses {
            if a >= n {
                continue;
            }
            let is_composite = !primes[n];
            let result = miller_rabin_single_witness(n, a);
            let (mut correct_comp, mut tested_comp, mut correct_prime, mut tested_prime) =
                stats.get(a).unwrap().clone();

            if is_composite {
                tested_comp += 1;
                if !result {
                    correct_comp += 1;
                }
            } else {
                tested_prime += 1;
                if result {
                    correct_prime += 1;
                }
            }
            stats.insert(
                a.clone(),
                (correct_comp, tested_comp, correct_prime, tested_prime),
            );
        }
    }

    // println!("\nWitness accuracy for numbers {} to {}:", start, end);
    // println!(
    //     "Total composites: {}, Total primes: {}",
    //     composites.len(),
    //     prime_nums.len()
    // );
    // println!(
    //     "Witness | Comp Correct | Comp Tested | Comp Acc | Prime Correct | Prime Tested | Prime Acc | Combined Acc"
    // );

    for witness in witnesses {
        let (correct_comp, tested_comp, correct_prime, tested_prime) = stats[&witness];
        let comp_acc = if tested_comp > 0 {
            correct_comp as f64 / tested_comp as f64
        } else {
            0.0
        };
        let prime_acc = if tested_prime > 0 {
            correct_prime as f64 / tested_prime as f64
        } else {
            0.0
        };
        let combined_acc = if tested_comp + tested_prime > 0 {
            (correct_comp + correct_prime) as f64 / (tested_comp + tested_prime) as f64
        } else {
            0.0
        };

        // println!(
        //     "a={:3} | {:12} | {:11} | {:8.2}% | {:13} | {:12} | {:9.2}% | {:12.2}%",
        //     witness,
        //     correct_comp,
        //     tested_comp,
        //     comp_acc * 100.0,
        //     correct_prime,
        //     tested_prime,
        //     prime_acc * 100.0,
        //     combined_acc * 100.0
        // );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miller_rabin_witness() {
        // track_witness_accuracy::<Integer>(10_u32.pow(3), 10_u32.pow(4));
        // track_witness_accuracy(10_u32.pow(3), 10_u32.pow(4));
        track_witness_accuracy::<Integer>(10_u32.pow(2), 10_u32.pow(3));
    }
}
