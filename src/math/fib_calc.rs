use rug::{Assign, Complete, Integer};

fn multiply_matrices(a: [[Integer; 2]; 2], b: [[Integer; 2]; 2]) -> [[Integer; 2]; 2] {
    [
        [
            Integer::from((&a[0][0] * &b[0][0]).complete() + (&a[0][1] * &b[1][0]).complete()),
            Integer::from((&a[0][0] * &b[0][1]).complete() + (&a[0][1] * &b[1][1]).complete()),
        ],
        [
            Integer::from((&a[1][0] * &b[0][0]).complete() + (&a[1][1] * &b[1][0]).complete()),
            Integer::from((&a[1][0] * &b[0][1]).complete() + (&a[1][1] * &b[1][1]).complete()),
        ],
    ]
}

fn matrix_exponentiation(mut base: [[Integer; 2]; 2], mut exp: u32) -> [[Integer; 2]; 2] {
    let mut result = [
        [Integer::from(1), Integer::from(0)],
        [Integer::from(0), Integer::from(1)],
    ];

    while exp > 0 {
        if exp % 2 == 1 {
            result = multiply_matrices(result, base.clone());
        }
        base = multiply_matrices(base.clone(), base);
        exp /= 2;
    }

    result
}

pub fn fib_matrix(n: u32) -> Integer {
    if n == 0 {
        return Integer::from(0);
    }
    let base_matrix = [
        [Integer::from(1), Integer::from(1)],
        [Integer::from(1), Integer::from(0)],
    ];
    let result = matrix_exponentiation(base_matrix, n - 1);
    result[0][0].clone() // This holds F(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_fibonacci() {
        assert_eq!(fib_matrix(0), Integer::from(0));
        assert_eq!(fib_matrix(1), Integer::from(1));
        assert_eq!(fib_matrix(2), Integer::from(1));
        assert_eq!(fib_matrix(3), Integer::from(2));
        assert_eq!(fib_matrix(4), Integer::from(3));
        assert_eq!(fib_matrix(5), Integer::from(5));
        assert_eq!(fib_matrix(10), Integer::from(55));
    }

    #[test]
    fn test_small_fibonacci_2() {
        assert_eq!(fib_matrix(10), Integer::from(55));
        assert_eq!(fib_matrix(20), Integer::from(6765));
        assert_eq!(fib_matrix(30), Integer::from(832040));
    }

    #[test]
    fn test_large_fibonacci() {
        let expected_50 = "12586269025".parse::<Integer>().unwrap();
        let expected_100 = "354224848179261915075".parse::<Integer>().unwrap();
        let expected_200 = "280571172992510140037611932413038677189525"
            .parse::<Integer>()
            .unwrap();

        assert_eq!(fib_matrix(50), expected_50);
        assert_eq!(fib_matrix(100), expected_100);
        assert_eq!(fib_matrix(200), expected_200);
    }

    #[test]
    fn test_large_fibonacci_2() {
        let expected_1000 = "43466557686937456435688527675040625802564660517371780402481729089536555417949051890403879840079255169295922593080322634775209689623239873322471161642996440906533187938298969649928516003704476137795166849228875".parse::<Integer>().unwrap();

        assert_eq!(fib_matrix(1000), expected_1000);
    }

    #[test]
    fn test_large_fibonacci_adding_previous() {
        let first = fib_matrix(1000);
        let second = fib_matrix(1001);
        let third = fib_matrix(1002);

        assert_eq!(third, second + first);
    }
}
