use rust_learn::homework_2::pair;

#[test]
fn default_pair() {
    assert_eq!(pair::default_pair(), (0, 0));
}

#[test]
fn pair_vector_sum() {
    assert_eq!(pair::pair_vector_sum((1, 1), (1, 1)), (2, 2));
}

#[test]
fn pair_scalar_sum() {
    assert_eq!(pair::pair_scalar_sum((1, 1), (1, 1)), 4);
}
