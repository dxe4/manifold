use super::padic::TwoAdicInteger;
use rug::{Integer, Rational};

#[derive(Debug, Clone)]
struct Collatz {
    n: Integer,
    seq: Vec<Integer>,
    two_adic_distance: Vec<Rational>,
}

impl Collatz {
    pub fn two_adic_disntace_str(&self) -> Vec<String> {
        /*
         we need the internal representation to be rationals and not reals
         for calculation purposes
        */
        self.two_adic_distance
            .iter()
            .map(|item| format!("{}/{}", item.numer(), item.denom()))
            .collect::<Vec<String>>()
    }
}

fn collatz_sequence(n: Integer) -> Collatz {
    let mut sequence = vec![n.clone()];
    let mut two_adic_distance: Vec<Rational> = Vec::new();

    let mut current = n.clone();
    while &current != Integer::ONE {
        let left = TwoAdicInteger {
            value: current.clone(),
        };
        current = if current.is_even() {
            current.clone() / 2
        } else {
            current.clone() * 3 + 1
        };
        let right = TwoAdicInteger {
            value: current.clone(),
        };
        sequence.push(current.clone());
        two_adic_distance.push(left.distance(&right));
    }

    return Collatz {
        n: n.clone(),
        seq: sequence,
        two_adic_distance: two_adic_distance,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rug_int_vec;
    use rug::Integer;

    #[test]
    fn test_collatz_sequence_1() {
        assert_eq!(collatz_sequence(Integer::from(1)).seq, rug_int_vec![1]);
    }

    #[test]
    fn test_collatz_sequence_2() {
        assert_eq!(collatz_sequence(Integer::from(2)).seq, rug_int_vec![2, 1]);
    }

    #[test]
    fn test_collatz_sequence_3() {
        assert_eq!(
            collatz_sequence(Integer::from(3)).seq,
            rug_int_vec![3, 10, 5, 16, 8, 4, 2, 1]
        );
    }

    #[test]
    fn test_collatz_sequence_6() {
        assert_eq!(
            collatz_sequence(Integer::from(6)).seq,
            rug_int_vec![6, 3, 10, 5, 16, 8, 4, 2, 1]
        );
    }

    #[test]
    fn test_collatz_sequence_27() {
        assert_eq!(
            collatz_sequence(Integer::from(27)).seq,
            rug_int_vec![
                27, 82, 41, 124, 62, 31, 94, 47, 142, 71, 214, 107, 322, 161, 484, 242, 121, 364,
                182, 91, 274, 137, 412, 206, 103, 310, 155, 466, 233, 700, 350, 175, 526, 263, 790,
                395, 1186, 593, 1780, 890, 445, 1336, 668, 334, 167, 502, 251, 754, 377, 1132, 566,
                283, 850, 425, 1276, 638, 319, 958, 479, 1438, 719, 2158, 1079, 3238, 1619, 4858,
                2429, 7288, 3644, 1822, 911, 2734, 1367, 4102, 2051, 6154, 3077, 9232, 4616, 2308,
                1154, 577, 1732, 866, 433, 1300, 650, 325, 976, 488, 244, 122, 61, 184, 92, 46, 23,
                70, 35, 106, 53, 160, 80, 40, 20, 10, 5, 16, 8, 4, 2, 1
            ]
        );
    }
}
