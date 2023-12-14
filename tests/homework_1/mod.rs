use rust_learn::homework_1;

#[test]
fn double_int32() {
    assert_eq!(homework_1::double_int32(0), 0);
    assert_eq!(homework_1::double_int32(5), 10);
    assert_eq!(homework_1::double_int32(3_000_000_000), u32::MAX);
}

#[test]
fn double_int64() {
    assert_eq!(homework_1::double_int64(0), 0);
    assert_eq!(homework_1::double_int64(25), 50);
    assert_eq!(homework_1::double_int64(3_000_000_000), 6_000_000_000);
}

#[test]
fn double_float32() {
    assert_eq!(homework_1::double_float32(0.0), 0.0);
    assert_eq!(homework_1::double_float32(10.0), 20.0);
    assert_eq!(homework_1::double_float32(f32::MAX), f32::INFINITY);
}

#[test]
fn double_float64() {
    assert_eq!(homework_1::double_float64(0.0), 0.0);
    assert_eq!(homework_1::double_float64(10.0), 20.0);
    assert_eq!(homework_1::double_float64(10.0), 20.0);
    assert!(homework_1::double_float64(f32::MAX) <= f64::MAX)
}

#[test]
fn int_plus_float_to_float() {
    assert_eq!(homework_1::int_plus_float_to_float(0, 0.0), 0.0);
    assert_eq!(homework_1::int_plus_float_to_float(10, 10.0), 20.0);
    assert!(homework_1::int_plus_float_to_float(u32::MAX, f32::MAX) <= f64::MAX);
}

#[test]
fn int_plus_float_to_int() {
    assert_eq!(homework_1::int_plus_float_to_int(0, 0.0), 0);
    assert_eq!(homework_1::int_plus_float_to_int(10, 10.0), 20);
    assert_eq!(
        homework_1::int_plus_float_to_int(u32::MAX, f32::MAX),
        u64::MAX
    );
    assert_eq!(
        homework_1::int_plus_float_to_int(u32::MIN, f32::MIN),
        u64::MIN
    );
}

#[test]
fn tuple_sum() {
    assert_eq!(homework_1::tuple_sum((0, 0)), 0);
    assert_eq!(homework_1::tuple_sum((5, 1)), 6);
    assert_eq!(homework_1::tuple_sum((1, usize::MAX)), usize::MAX);
    assert!(homework_1::tuple_sum((1, usize::MIN)) > usize::MIN);
    assert_eq!(homework_1::tuple_sum((usize::MIN, usize::MIN)), usize::MIN);
}

#[test]
fn array_sum() {
    assert_eq!(homework_1::array_sum([0, 0, 0]), 0);
    assert_eq!(homework_1::array_sum([1, 2, 3]), 6);
    assert_eq!(
        homework_1::array_sum([1, usize::MIN, usize::MAX]),
        usize::MAX
    );
    assert_eq!(homework_1::array_sum([usize::MAX; 3]), usize::MAX);
    assert_eq!(homework_1::array_sum([usize::MIN; 3]), usize::MIN);
}
