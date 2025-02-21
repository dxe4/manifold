use rug::{Complete, Integer};

pub fn contains_zero_in_binary(n: &Integer) -> bool {
    let significant_bits = n.significant_bits();

    for i in 0..significant_bits {
        if !n.get_bit(i) {
            return true;
        }
    }
    false
}

pub fn trailing_zeros(n: &Integer) -> u32 {
    /*
     TODO this logic is used in multiple places
     make one central function and use that
    */
    let significant_bits = n.significant_bits();

    for i in 0..significant_bits {
        if n.get_bit(i) {
            return i;
        }
    }
    significant_bits
}

pub fn binary_gcd(mut a: Integer, mut b: Integer) -> Integer {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    let shift = trailing_zeros(&(&a | &b).complete());

    a >>= trailing_zeros(&a);
    b >>= trailing_zeros(&b);

    while b != 0 {
        while b.is_even() {
            b >>= 1;
        }

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        b -= &a;
    }

    a << shift
}

pub fn euclidean_gcd(mut a: Integer, mut b: Integer) -> Integer {
    while b != 0 {
        let temp = a % &b;
        a = b;
        b = temp;
    }
    a
}

pub fn is_coprime(a: Integer, b: Integer) -> bool {
    return binary_gcd(a.clone(), b.clone()) == 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let egcd = euclidean_gcd(Integer::from(3), Integer::from(1));
        let bgcd = euclidean_gcd(Integer::from(3), Integer::from(1));
        assert_eq!(bgcd, egcd);
        assert_eq!(bgcd, Integer::from(1));
    }

    #[test]
    fn test_coprime() {
        let egcd = euclidean_gcd(Integer::from(3), Integer::from(1));
        let bgcd = euclidean_gcd(Integer::from(3), Integer::from(1));
        assert_eq!(bgcd, egcd);
        assert_eq!(bgcd, Integer::from(1));
    }
    #[test]
    fn test_coprime_numbers() {
        assert_eq!(is_coprime(Integer::from(8), Integer::from(15)), true);
        assert_eq!(is_coprime(Integer::from(13), Integer::from(27)), true);
        assert_eq!(is_coprime(Integer::from(35), Integer::from(64)), true);
        assert_eq!(is_coprime(Integer::from(17), Integer::from(19)), true);
        assert_eq!(is_coprime(Integer::from(9), Integer::from(28)), true);
    }

    #[test]
    fn test_non_coprime_numbers() {
        assert_eq!(is_coprime(Integer::from(12), Integer::from(18)), false);
        assert_eq!(is_coprime(Integer::from(24), Integer::from(36)), false);
        assert_eq!(is_coprime(Integer::from(14), Integer::from(21)), false);
        assert_eq!(is_coprime(Integer::from(16), Integer::from(40)), false);
        assert_eq!(is_coprime(Integer::from(30), Integer::from(45)), false);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(is_coprime(Integer::from(1), Integer::from(1)), true);
        assert_eq!(is_coprime(Integer::from(1), Integer::from(100)), true);
        assert_eq!(is_coprime(Integer::from(0), Integer::from(5)), false);
        assert_eq!(is_coprime(Integer::from(7), Integer::from(0)), false);
        assert_eq!(is_coprime(Integer::from(5), Integer::from(5)), false);
    }
}
