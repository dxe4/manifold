use rayon::prelude::*;
use rug::{Complete, Integer, Rational};
use std::collections::HashMap;

use crate::math::primes::miller_rabin_single;

fn miller_rabin_single_witness(n: &Integer, a: &Integer) -> bool {
    if n < &Integer::from(2) || n.is_even() {
        return n == &Integer::from(2);
    }
    if a >= n {
        // panic here?
        // we shouldnt reach this point or its a bug outside of the function
        return true;
    }

    let mut s = 0;
    let mut d = n - Integer::from(1);
    while d.is_even() {
        s += 1;
        d >>= 1;
    }

    let mut x = a.clone().pow_mod(&d, &n).unwrap();
    if x == Integer::from(1) || x == n - Integer::from(1) {
        return true;
    }
    for _ in 0..s - 1 {
        x = (&x * &x).complete() % n;
        if x == n - Integer::from(1) {
            return true;
        }
    }
    false
}

fn track_witness_accuracy(start: u32, end: u32) {
    /*
    TODO the combinatorics here explode
    we want to gather statistics on how accurate the witness numbers are
    this is very intense, even a full parallel version is very slow
    how was miller rabin tested?
    there must be some academic paper to optimise this
    */
    let start = Integer::from(start);
    let end = Integer::from(end);

    let numbers: Vec<Integer> = (start.to_u32().unwrap()..=end.to_u32().unwrap())
        .map(Integer::from)
        .collect();
    let primes: HashMap<Integer, bool> = numbers
        .iter()
        .map(|n| (n.clone(), miller_rabin_single(n)))
        .collect();
    let composites: Vec<Integer> = numbers.iter().filter(|n| !primes[n]).cloned().collect();
    let prime_nums: Vec<Integer> = numbers.iter().filter(|n| primes[n]).cloned().collect();

    let max_n = numbers.last().unwrap();
    let witnesses: Vec<Integer> = (2..max_n.to_u32().unwrap() - 1)
        .map(Integer::from)
        .collect();

    let mut stats: HashMap<Integer, (u32, u32, u32, u32)> = witnesses
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

    let mut accuracies: HashMap<Integer, (Rational, Rational, Rational)> = HashMap::new();
    for (a, (correct_comp, tested_comp, correct_prime, tested_prime)) in &stats {
        let comp_acc = if *tested_comp > 0 {
            Rational::from((Integer::from(*correct_comp), Integer::from(*tested_comp)))
        } else {
            Rational::from(0)
        };
        let prime_acc = if *tested_prime > 0 {
            Rational::from((Integer::from(*correct_prime), Integer::from(*tested_prime)))
        } else {
            Rational::from(0)
        };
        let combined_acc = if *tested_comp + *tested_prime > 0 {
            Rational::from((
                Integer::from(*correct_comp + *correct_prime),
                Integer::from(*tested_comp + *tested_prime),
            ))
        } else {
            Rational::from(0)
        };
        accuracies.insert(a.clone(), (comp_acc, prime_acc, combined_acc));
    }

    println!("\nWitness accuracy for numbers {} to {}:", start, end);
    println!(
        "Total composites: {}, Total primes: {}",
        composites.len(),
        prime_nums.len()
    );
    println!(
        "Witness | Comp Correct | Comp Tested | Comp Acc | Prime Correct | Prime Tested | Prime Acc | Combined Acc"
    );
    for witness in witnesses {
        let (correct_comp, tested_comp, correct_prime, tested_prime) = stats[&witness];
        let (comp_acc, prime_acc, combined_acc) = &accuracies[&witness];
        let comp_percent = comp_acc.to_f64() * 100.0;
        let prime_percent = prime_acc.to_f64() * 100.0;
        let combined_percent = combined_acc.to_f64() * 100.0;
        println!(
            "a={:3} | {:12} | {:11} | {:8} = {:6.2}% | {:13} | {:12} | {:9} = {:6.2}% | {:12} = {:6.2}%",
            witness, correct_comp, tested_comp, comp_acc, comp_percent,
            correct_prime, tested_prime, prime_acc, prime_percent,
            combined_acc, combined_percent
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miller_rabin_witness() {
        track_witness_accuracy(10_u32.pow(2), 10_u32.pow(3));
    }
}
