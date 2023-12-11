use std::usize;

pub fn double_int32(value: u32) -> u32 {
    value * 2
}

pub fn double_int64(value: u32) -> u64 {
    (value * 2) as u64
}

pub fn double_float32(value: f32) -> f32 {
    value * 2.0
}

pub fn double_float64(value: f32) -> f64 {
    (value * 2.0) as f64
}

pub fn int_plus_float_to_float(int_value: u32, float_value: f32) -> f64 {
    (int_value as f32 + float_value) as f64
}

pub fn int_plus_float_to_int(int_value: u32, float_value: f32) -> u64 {
    (int_value as f32 + float_value) as u64
}

pub fn tuple_sum(values: (usize, usize)) -> usize {
    values.0 + values.1
}

pub fn array_sum(arr: [usize; 3]) -> usize {
    arr.iter().sum()
}
