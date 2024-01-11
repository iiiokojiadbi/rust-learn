#![allow(dead_code)]

pub fn double_int32(value: u32) -> u32 {
    value.saturating_mul(2)
}

pub fn double_int64(value: u32) -> u64 {
    (value as u64).saturating_mul(2)
}

pub fn double_float32(value: f32) -> f32 {
    value + value
}

pub fn double_float64(value: f32) -> f64 {
    (value as f64) + (value as f64)
}

pub fn int_plus_float_to_float(int_value: u32, float_value: f32) -> f64 {
    (int_value as f64) + (float_value as f64)
}

pub fn int_plus_float_to_int(int_value: u32, float_value: f32) -> u64 {
    (int_value as f32 + float_value).round() as u64
}

pub fn tuple_sum(values: (usize, usize)) -> usize {
    (values.0).saturating_add(values.1)
}

pub fn array_sum(arr: [usize; 3]) -> usize {
    arr.into_iter()
        .reduce(|acc, value| acc.saturating_add(value))
        .unwrap_or(usize::MAX)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_double_int32() {
        assert_eq!(double_int32(0), 0);
        assert_eq!(double_int32(5), 10);
        assert_eq!(double_int32(3_000_000_000), u32::MAX);
    }

    #[test]
    fn test_double_int64() {
        assert_eq!(double_int64(0), 0);
        assert_eq!(double_int64(25), 50);
        assert_eq!(double_int64(3_000_000_000), 6_000_000_000);
    }

    #[test]
    fn test_double_float32() {
        assert_eq!(double_float32(0.0), 0.0);
        assert_eq!(double_float32(10.0), 20.0);
        assert_eq!(double_float32(f32::MAX), f32::INFINITY);
    }

    #[test]
    fn test_double_float64() {
        assert_eq!(double_float64(0.0), 0.0);
        assert_eq!(double_float64(10.0), 20.0);
        assert_eq!(double_float64(10.0), 20.0);
        assert!(double_float64(f32::MAX) <= f64::MAX)
    }

    #[test]
    fn test_int_plus_float_to_float() {
        assert_eq!(int_plus_float_to_float(0, 0.0), 0.0);
        assert_eq!(int_plus_float_to_float(10, 10.0), 20.0);
        assert!(int_plus_float_to_float(u32::MAX, f32::MAX) <= f64::MAX);
    }

    #[test]
    fn test_int_plus_float_to_int() {
        assert_eq!(int_plus_float_to_int(0, 0.0), 0);
        assert_eq!(int_plus_float_to_int(10, 10.0), 20);
        assert_eq!(int_plus_float_to_int(u32::MAX, f32::MAX), u64::MAX);
        assert_eq!(int_plus_float_to_int(u32::MIN, f32::MIN), u64::MIN);
    }

    #[test]
    fn test_tuple_sum() {
        assert_eq!(tuple_sum((0, 0)), 0);
        assert_eq!(tuple_sum((5, 1)), 6);
        assert_eq!(tuple_sum((1, usize::MAX)), usize::MAX);
        assert!(tuple_sum((1, usize::MIN)) > usize::MIN);
        assert_eq!(tuple_sum((usize::MIN, usize::MIN)), usize::MIN);
    }

    #[test]
    fn test_array_sum() {
        assert_eq!(array_sum([0, 0, 0]), 0);
        assert_eq!(array_sum([1, 2, 3]), 6);
        assert_eq!(array_sum([1, usize::MIN, usize::MAX]), usize::MAX);
        assert_eq!(array_sum([usize::MAX; 3]), usize::MAX);
        assert_eq!(array_sum([usize::MIN; 3]), usize::MIN);
    }
}
