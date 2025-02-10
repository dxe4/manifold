use std::collections::HashMap;

/*
this file is for research
*/
use indexmap::IndexMap;
use lazy_static::lazy_static;
use rug::Integer;

lazy_static! {
    static ref ONE: Integer = Integer::from(1);
    static ref TWO: Integer = Integer::from(2);
    static ref THREE: Integer = Integer::from(3);
}

pub fn pentagonal_number_index(i: &Integer) -> (Integer, Integer) {
    // f(n) n(3n - 1) / 1, n = -1/+1
    (
        ((i * (THREE.clone() * i + ONE.clone())) / TWO.clone()) - ONE.clone(),
        ((i * (THREE.clone() * i - ONE.clone())) / TWO.clone()) - ONE.clone(),
    )
}

pub fn sum_of_factors_from_pentagonal_numbers(target: Integer) -> Vec<Integer> {
    /*

    // https://scholarlycommons.pacific.edu/euler-works/158/
    // https://scholarlycommons.pacific.edu/euler-works/175/
    // https://arxiv.org/pdf/math/0505373

    This finds the sum of all the factors of a number
    For a prime N it will always be n + 1
    euler wrote 2 papers on this
    adding a fib like sum
    but the indices to add are determined by the pentagolan numbers
    list[p0] + list[p1] - list[p2] - list[p3] + list[p4] where Pn is a pentagonal number
    the signs are [+,+,-,-][idx % 4]
    we avoid modulo for performance
    */

    let mut left: Vec<Integer> = vec![Integer::from(1), Integer::from(1)];
    // we may want to use a queue here,
    // but will have to keep a hashmap for the lookup
    let mut right: IndexMap<Integer, i16> = IndexMap::new();

    let mut biggest_pentagonal_number: Integer = Integer::from(-1);
    let mut counter = Integer::from(1);
    let mut new_num;
    let mut multiplier = 1 as i16;
    let mut pos;

    while left.len() < target {
        let (a, b) = pentagonal_number_index(&Integer::from(&counter));

        left[0] = left[0].clone() + 1;

        if a > biggest_pentagonal_number {
            right.insert(b.clone(), multiplier);
            right.insert(a.clone(), multiplier);
            multiplier = multiplier * -1;
            biggest_pentagonal_number = a.clone();
        }
        new_num = Integer::from(0);
        for (k, multiplier_) in &right {
            pos = k.to_usize().unwrap();
            if pos >= left.len() {
                // prevent overflow of usize
                break;
            }
            pos = left.len() - pos - 1;
            if pos >= left.len() {
                break;
            }

            new_num =
                new_num + (left[left.len() - k.to_usize().unwrap() - 1].clone() * multiplier_);
        }
        left.push(new_num.clone());
        counter += 1;
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simpple_partition_number() {
        let num = Integer::from(16);

        let result = sum_of_factors_from_pentagonal_numbers(num);
        println!("{:?}", result);
        assert!(result[11] == 12);
        assert!(result[5] == 6);
        assert!(result[13] == 14);
    }
}
