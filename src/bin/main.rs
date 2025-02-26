use manifold_rs::math::primes::miller_rabin_impl;
use rug::Integer;

fn print_prime_cache() {
    let prime_bool_vec = miller_rabin_impl(&Integer::from(2), &Integer::from(10_000));
    let prime_res: Vec<(u32, bool)> = prime_bool_vec
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| if b { Some((i as u32, b)) } else { None }) // Keep only true values
        .collect();
    println!("const PRIME_CACHE: [u32; {}] = [", prime_res.len());
    let last_idx = prime_res.len();
    for (enumerate_idx, (idx, is_prime)) in prime_res.iter().enumerate() {
        if enumerate_idx == last_idx - 1 {
            print!("{}", idx + 2);
        } else if *is_prime {
            print!("{},", idx + 2);
        }
    }
    println!("];");
}

fn main() {
    print_prime_cache();
}
