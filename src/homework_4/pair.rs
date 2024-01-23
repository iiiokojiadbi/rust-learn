#[derive(Default, PartialEq, Debug)]
pub struct Pair(i32, i32);

impl Pair {
    pub fn vector_sum(&self, rhs: &Pair) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }

    pub fn scalar_sum(&self, rhs: &Pair) -> i32 {
        self.0 + self.1 + rhs.0 + rhs.1
    }
}

#[cfg(test)]
mod test {
    use super::Pair;

    #[test]
    fn test_default_pair() {
        let pair = Pair::default();
        let expected = Pair(0, 0);
        assert_eq!(pair, expected);
    }

    #[test]
    fn test_vector_sum() {
        let pair = Pair(1, 1);
        let expected = Pair(2, 2);
        assert_eq!(pair.vector_sum(&pair), expected);

        let pair = Pair(2, 2);
        let expected = Pair(4, 4);
        assert_eq!(pair.vector_sum(&pair), expected);
    }

    #[test]
    fn test_scalar_sum() {
        let pair = Pair(1, 1);
        let expected = Pair(2, 2);
        assert_eq!(pair.scalar_sum(&expected), 6);

        let pair = Pair(2, 1);
        let expected = Pair(4, 2);
        assert_eq!(pair.scalar_sum(&expected), 9);
    }
}
