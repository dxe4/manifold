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
