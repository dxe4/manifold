use rug::Integer;

#[derive(Debug)]
pub struct IntegerRing {
    zero: Integer,
    one: Integer,
}

impl IntegerRing {
    pub fn new() -> Self {
        IntegerRing {
            zero: Integer::from(0),
            one: Integer::from(1),
        }
    }

    pub fn zero(&self) -> Integer {
        self.zero.clone()
    }

    pub fn one(&self) -> Integer {
        self.one.clone()
    }

    pub fn gcd(&self, a: &Integer, b: &Integer) -> Integer {
        a.clone().gcd(&b)
    }

    fn gcdex(&self, a: &Integer, b: &Integer) -> (Integer, Integer, Integer) {
        // todo double check logic and unit test
        let mut x0 = Integer::from(1);
        let mut x1 = Integer::from(0);
        let mut y0 = Integer::from(0);
        let mut y1 = Integer::from(1);

        let mut a = a.clone();
        let mut b = b.clone();

        while b != 0 {
            let (q, r) = a.div_rem(b.clone());
            a = b.clone();
            b = r.clone();

            let temp_x = x0 - &q * &x1;
            let temp_y = y0 - &q * &y1;
            x0 = x1;
            x1 = temp_x;
            y0 = y1;
            y1 = temp_y;
        }

        (a, x0, y0)
    }

    fn lcm(a: &Integer, b: &Integer) -> Integer {
        let gcd = a.clone().gcd(b);
        let product = a.clone() * b.clone();

        product / gcd
    }

    pub fn sqrt(&self, a: &Integer) -> Integer {
        a.clone().sqrt()
    }

    pub fn is_square(&self, a: &Integer) -> bool {
        if a.cmp0() == Ordering::Less {
            return false;
        }
        let root = self.sqrt(a);
        let square = root.clone() * &root;
        square == *a
    }

    pub fn factorial(&self, n: &Integer) -> Option<Integer> {
        if n.cmp0() == Ordering::Less {
            return None;
        }

        let mut result = Integer::from(1);
        let mut i = Integer::from(2);

        while &i <= n {
            result *= &i;
            i += 1;
        }

        Some(result)
    }

    pub fn log(&self, a: &Integer, b: &Integer) -> Option<Integer> {
        if a.cmp0() <= Ordering::Equal || b.cmp0() <= Ordering::Equal || *b == Integer::from(1) {
            return None;
        }

        // This function uses ``math.log`` which is based on ``float`` so it will
        // fail for large integer arguments.
        let a_f64 = a.to_f64();
        let b_f64 = b.to_f64();

        Some(Integer::from(a_f64.log(b_f64).floor() as i64))
    }
}

impl PartialEq for IntegerRing {
    fn eq(&self, _other: &Self) -> bool {
        true // There's only one integer ring
    }
}

impl Eq for IntegerRing {}

impl Hash for IntegerRing {
    fn hash<H: Hasher>(&self, state: &mut H) {
        "ZZ".hash(state);
    }
}

static ZZ: OnceLock<IntegerRing> = OnceLock::new();
pub fn get_zz() -> &'static IntegerRing {
    ZZ.get_or_init(|| IntegerRing::new())
}
