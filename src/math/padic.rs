extern crate rug;
extern crate rayon;

use rug::Integer;
use rug::ops::Pow;
use rayon::prelude::*;
use super::threading::get_large_pool;
use super::num_utils::pow_large;

const DIGIT_LIMIT: u32 = 250;

pub fn two_pow_10_pow_n_parallel(start: usize, end: usize) -> Vec<String> {
    /*
        x = 10^n
        result = 2^x
    */
    let pool = get_large_pool();

    let mut results = vec![String::new(); (end - start + 1) as usize];

    pool.install(|| {
        results
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, result)| {
                let n = start + i;

                let modulus = Integer::from(10).pow(DIGIT_LIMIT);
                let exponent = pow_large(&Integer::from(10), &Integer::from(n));

                // Calculate 2^(10^n) mod 10^150
                let res = Integer::from(2).pow_mod(&exponent, &modulus).unwrap();
                *result = res.to_string();
            });
    });

    results
}
