use rust_learn::homework_2::counter;

#[test]
fn default_signed_counter() {
    assert_eq!(counter::default_signed_counter(), 0);
}

#[test]
fn default_unsigned_counter() {
    assert_eq!(counter::default_unsigned_counter(), 0);
}

#[test]
fn next_signed() {
    assert_eq!(counter::next_signed(-1), 0);
}

#[test]
fn next_unsigned() {
    assert_eq!(counter::next_unsigned(4), 5);
}

#[test]
fn prev_signed() {
    assert_eq!(counter::prev_signed(4), 3);
}
