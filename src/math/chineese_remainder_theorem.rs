use rug::{Complete, Integer};

pub fn extended_gcd(a: &Integer, b: &Integer) -> (Integer, Integer, Integer) {
    if b == &0 {
        return (a.clone(), Integer::from(1), Integer::from(0));
    }

    let (gcd, x1, y1) = extended_gcd(b, &(a % b).complete());
    let x = y1.clone();
    let y = x1 - (a / b).complete() * &y1;

    (gcd, x, y)
}

pub fn chinese_remainder_theorem_impl(
    a_list: &[Integer],
    n_list: &[Integer],
) -> Result<Integer, String> {
    let k = a_list.len();

    if k != n_list.len() {
        return Err("Lists must have equal length".to_string());
    }

    let mut n = Integer::from(1);
    for ni in n_list {
        n *= ni;
    }

    let mut result = Integer::from(0);

    for i in 0..k {
        let ai = &a_list[i];
        let ni = &n_list[i];

        let ni_partial = (&n / ni).complete();

        let (gcd, inv, _) = extended_gcd(&ni_partial, ni);
        if gcd != 1 {
            return Err(format!(
                "Modulo {} and {} are not coprime, no solution exists.",
                ni, ni_partial
            ));
        }

        result += (ai * &ni_partial).complete() * inv;
    }

    Ok(result % n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crt() {
        let a_list = vec![Integer::from(2), Integer::from(3), Integer::from(2)];
        let n_list = vec![Integer::from(3), Integer::from(5), Integer::from(7)];

        let result = chinese_remainder_theorem_impl(&a_list, &n_list).unwrap();
        assert_eq!(result, Integer::from(23));
        for (a, n) in a_list.iter().zip(n_list.iter()) {
            assert_eq!((&result % n).complete(), (a % n).complete());
        }
    }
}
