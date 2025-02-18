extern crate rug;
extern crate rayon;

use rug::Integer;
use rug::ops::Pow;
use rayon::prelude::*;
use super::threading::get_large_pool;
use super::num_utils::pow_large;

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
        results
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, result)| {
                let n = number_config.start + i;

                let modulus = Integer::from(number_config.exponent).pow(number_config.digit_limit);
                let exponent = pow_large(&Integer::from(number_config.exponent), &Integer::from(n));

                // Calculate 2^(10^n) mod 10^DIGIT_LIMIT
                let res = Integer::from(number_config.base).pow_mod(&exponent, &modulus).unwrap();
                *result = res.to_string();
            });
    });

    results
}
