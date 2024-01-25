#[derive(Default, Debug, PartialEq)]
pub struct SignedCounter(isize);

#[derive(Default, Debug, PartialEq)]
pub struct UnsignedCounter(usize);

impl SignedCounter {
    pub fn prev_value(&mut self) -> &Self {
        self.0 -= 1;

        self
    }

    pub fn next_value(&mut self) -> &Self {
        self.0 += 1;

        self
    }

    pub fn value(&self) -> isize {
        self.0
    }
}

impl UnsignedCounter {
    pub fn next_value(&mut self) -> &Self {
        self.0 += 1;

        self
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod test {

    use super::{SignedCounter, UnsignedCounter};

    #[test]
    fn test_default_signed_counter() {
        let signed_counter = SignedCounter::default();
        let expected = SignedCounter(0);
        assert_eq!(signed_counter, expected);
    }

    #[test]
    fn test_default_unsigned_counter() {
        let unsigned_counter = UnsignedCounter::default();
        let expected = UnsignedCounter(0);
        assert_eq!(unsigned_counter, expected);
    }

    #[test]
    fn test_next_signed() {
        let mut signed_counter = SignedCounter::default();

        signed_counter.next_value();
        let expected = SignedCounter(1);
        assert_eq!(signed_counter.value(), expected.value());

        signed_counter.next_value();
        let expected = SignedCounter(2);
        assert_eq!(signed_counter.value(), expected.value());
    }

    #[test]
    fn test_next_unsigned() {
        let mut unsigned_counter = UnsignedCounter::default();

        unsigned_counter.next_value();
        let expected = UnsignedCounter(1);
        assert_eq!(unsigned_counter.value(), expected.value());

        unsigned_counter.next_value();
        let expected = UnsignedCounter(2);
        assert_eq!(unsigned_counter.value(), expected.value());
    }

    #[test]
    fn test_prev_signed() {
        let mut signed_counter = SignedCounter::default();

        signed_counter.prev_value();
        let expected = SignedCounter(-1);
        assert_eq!(signed_counter.value(), expected.value());

        signed_counter.prev_value();
        let expected = SignedCounter(-2);
        assert_eq!(signed_counter.value(), expected.value());
    }
}
