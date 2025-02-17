use rug::Integer;

pub fn pow_large(base: &Integer, exponent: &Integer) -> Integer {
    let mut result = Integer::from(1);
    let mut base = base.clone();
    let mut exponent = exponent.clone();

    while exponent > 0 {
        if exponent.is_odd() {
            result *= &base;
        }
        let temp = base.clone();
        base *= &temp;
        exponent >>= 1;
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;
    use rug::Integer;

    #[test]
    fn test_pow_small_numbers() {
        let result = pow_large(&Integer::from(2), &Integer::from(3));
        assert_eq!(result, Integer::from(8));
    }

    #[test]
    fn test_pow_zero_exponent() {
        let result = pow_large(&Integer::from(5), &Integer::from(0));
        assert_eq!(result, Integer::from(1));
    }

    #[test]
    fn test_pow_one_exponent() {
        let result = pow_large(&Integer::from(7), &Integer::from(1));
        assert_eq!(result, Integer::from(7));
    }

    #[test]
    fn test_pow_large_exponent() {
        let result = pow_large(&Integer::from(2), &Integer::from(10));
        assert_eq!(result, Integer::from(1024));
    }

    #[test]
    fn test_pow_negative_base() {
        let result = pow_large(&Integer::from(-2), &Integer::from(3));
        assert_eq!(result, Integer::from(-8));
    }

    #[test]
    fn test_pow_large_base() {
        let result = pow_large(&Integer::from(1000), &Integer::from(2));
        assert_eq!(result, Integer::from(1000000));
    }
}
