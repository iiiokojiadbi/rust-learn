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
        .unwrap()
}
