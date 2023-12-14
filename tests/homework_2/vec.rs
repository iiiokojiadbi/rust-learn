use rust_learn::homework_2::vec;

#[test]
fn default_vec3() {
    assert_eq!(vec::default_vec3(), [0; 3]);
}

#[test]
fn vec3_vector_sum() {
    assert_eq!(vec::vec3_vector_sum([1; 3], [1; 3]), [2; 3]);
}

#[test]
fn vec3_scalar_sum() {
    assert_eq!(vec::vec3_scalar_sum([1; 3], [1; 3]), 6);
}
