use rug::{Complete, Integer};

pub struct IntegerIterator {
    current: Integer,
    end: Integer,
    inclusive: bool,
}

impl IntegerIterator {
    fn new(start: Integer, end: Integer, inclusive: bool) -> Self {
        Self {
            current: start,
            end,
            inclusive,
        }
    }
}

impl Iterator for IntegerIterator {
    type Item = Integer;

    fn next(&mut self) -> Option<Self::Item> {
        let in_bounds = if self.inclusive {
            self.current <= self.end
        } else {
            self.current < self.end
        };
        if in_bounds {
            let result = self.current.clone();
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

pub trait IntegerGenerator {
    fn range_to(self) -> IntegerIterator;
    fn range_to_inclusive(self) -> IntegerIterator;
}

impl IntegerGenerator for Integer {
    fn range_to(self) -> IntegerIterator {
        IntegerIterator::new(Integer::from(0), self, false)
    }
    fn range_to_inclusive(self) -> IntegerIterator {
        IntegerIterator::new(Integer::from(0), self, true)
    }
}

#[cfg(test)]
mod tests {
    use crate::rug_int_vec;

    use super::*;

    #[test]
    fn test_range_to_small() {
        let end = Integer::from(5);
        let result: Vec<Integer> = end.range_to().collect();
        let expected: Vec<Integer> = rug_int_vec![0, 1, 2, 3, 4];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_range_to_inclusive() {
        let end = Integer::from(5);
        let result: Vec<Integer> = end.range_to_inclusive().collect();
        let expected: Vec<Integer> = rug_int_vec![0, 1, 2, 3, 4, 5];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_range_to_zero() {
        let end = Integer::from(0);
        let result: Vec<Integer> = end.range_to().collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_range_to_large() {
        let end = Integer::from(100);
        let count = end.range_to().count();
        assert_eq!(count, 100);
    }

    #[test]
    fn test_range_to_negative() {
        let end = Integer::from(-5);
        let result: Vec<Integer> = end.range_to().collect();
        assert!(result.is_empty());
    }
}
