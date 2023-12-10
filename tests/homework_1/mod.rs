use rust_learn::homework_1::Home;

#[test]
fn double_int32() {
    let test_value = 5;

    assert_eq!(Home::double_int32(test_value), 10);
}

#[test]
fn double_int64() {
    let test_value = 25;

    assert_eq!(Home::double_int64(test_value), 50);
}

#[test]
fn double_float32() {
    let test_value = 10.0;

    assert_eq!(Home::double_float32(test_value), 20.0);
}

#[test]
fn double_float64() {
    let test_value = 10.0;

    assert_eq!(Home::double_float64(test_value), 20.0);
}

#[test]
fn int_plus_float_to_float() {
    let test_int_value = 10;
    let test_float_value = 10.0;

    assert_eq!(
        Home::int_plus_float_to_float(test_int_value, test_float_value),
        20.0
    );
}

#[test]
fn int_plus_float_to_int() {
    let test_int_value = 10;
    let test_float_value = 10.0;

    assert_eq!(
        Home::int_plus_float_to_int(test_int_value, test_float_value),
        20
    );
}

#[test]
fn tuple_sum() {
    let values = (5, 1);

    assert_eq!(Home::tuple_sum(values), 6);
}

#[test]
fn array_sum() {
    let values = [1, 2, 3];

    assert_eq!(Home::array_sum(values), 6);
}
