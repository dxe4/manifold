use rayon::prelude::*;
use rug::{Complete, Integer};
use std::collections::HashMap;
use std::env;

use crate::math::static_data::find_in_cache;

use super::bitscan::bit_scan1;
use super::threading::get_small_pool;

// use csv::Writer;
use csv::WriterBuilder;
use std::fs::OpenOptions;

use std::error::Error;
use std::fs::File;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug, serde::Serialize)]
pub struct WitnessResult {
    num: String,
    is_prime: bool,
    base: String,
    is_prime_from_base: bool,
}

fn process_witness_results_to_csv(
    rx: Receiver<WitnessResult>,
    filename: String,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    std::thread::spawn(
        move || -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
            let current_dir = env::current_dir()?;
            let file_path = current_dir.join("data").join(filename);
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(file_path)?;
            let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);

            while let Ok(result) = rx.recv() {
                wtr.serialize(&result)?;
                wtr.flush()?;
            }

            Ok(())
        },
    );

    Ok(())
}

fn _miller_rabin_test(n: &Integer, base: &Integer, s: u32, t: &Integer) -> bool {
    // TODO re-use, allow passing bases in miller rabin
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

fn miller_rabin_single_witness(n: &Integer, base: &Integer) -> bool {
    if n < &Integer::from(2) || n.is_even() {
        return n == &Integer::from(2);
    }
    if base >= n {
        panic!("a ({}) > n ({}) ", base, n);
    }

    let d = n - Integer::from(1);

    let bit_scan_result = bit_scan1(&d, 0);

    let s = bit_scan_result.unwrap();
    let t = Integer::from(n >> s);

    if !_miller_rabin_test(n, &base, s, &t) {
        return false;
    }
    true
}

pub fn track_witness_accuracy(start_u32: u32, end_u32: u32) {
    let start = Integer::from(start_u32);
    let end = Integer::from(end_u32);

    let numbers: Vec<Integer> = (start.to_u32().unwrap()..=end.to_u32().unwrap())
        .map(Integer::from)
        .collect();

    let primes: HashMap<Integer, bool> = numbers
        .iter()
        .map(|n| {
            let key = n.to_u32().unwrap();

            let is_prime = find_in_cache(&key).unwrap();
            (n.clone(), is_prime)
        })
        .collect();

    let max_n = numbers.last().unwrap().to_u32().unwrap();
    let witnesses: Vec<Integer> = (2..max_n).map(Integer::from).collect();
    let (tx, rx): (Sender<WitnessResult>, Receiver<WitnessResult>) = channel::<WitnessResult>();

    process_witness_results_to_csv(rx, "witness.csv".into()).unwrap();

    let pool = get_small_pool();
    pool.install(|| {
        witnesses.par_iter().for_each_with(tx, |tx, witness| {
            for to_test in witness.to_u32().unwrap()..=max_n {
                if witness >= &to_test {
                    continue;
                }
                let n_integer = Integer::from(to_test);
                let is_composite = !primes.get(&n_integer).unwrap_or(&false);
                let result = miller_rabin_single_witness(&n_integer, witness);

                let witness_row = WitnessResult {
                    num: to_test.to_string(),
                    is_prime: !is_composite,
                    base: witness.to_string(),
                    is_prime_from_base: result,
                };

                tx.send(witness_row).unwrap();
            }
        });
    });
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_miller_rabin_witness() {
    //     // track_witness_accuracy(10_u32.pow(2), 10_u32.pow(3));
    //     track_witness_accuracy(2, 5_000);
    // }
}
