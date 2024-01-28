use std::{
    ops::{Index, IndexMut},
    usize,
};

const VEC3_LEN: usize = 3;

#[derive(Default, PartialEq, Debug)]
pub struct Vec3([i32; VEC3_LEN]);

impl Vec3 {
    pub fn vector_sum(&self, rhs: Vec3) -> Vec3 {
        let mut result = Vec3::default();

        for i in 0..VEC3_LEN {
            result[i] = self[i] + rhs[i];
        }

        result
    }

    pub fn scalar_sum(&self, rhs: Vec3) -> i32 {
        let mut result = 0;

        for i in 0..VEC3_LEN {
            result += self[i] + rhs[i];
        }

        result
    }
}

impl Index<usize> for Vec3 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod test {
    use super::Vec3;

    #[test]
    fn test_default_vec3() {
        let vec3 = Vec3::default();
        let expected = Vec3([0; 3]);
        assert_eq!(vec3, expected);
    }

    #[test]
    fn test_vec3_vector_sum() {
        let vec3 = Vec3::default();
        let rhs: Vec3 = Vec3::default();
        let expected = Vec3([0; 3]);
        assert_eq!(vec3.vector_sum(rhs), expected);

        let vec3 = Vec3([1; 3]);
        let rhs: Vec3 = Vec3([1; 3]);
        let expected = Vec3([2; 3]);
        assert_eq!(vec3.vector_sum(rhs), expected);
    }

    #[test]
    fn test_scalar_sum() {
        let vec3 = Vec3([0; 3]);
        let rhs: Vec3 = Vec3([0; 3]);
        assert_eq!(vec3.scalar_sum(rhs), 0);

        let vec3 = Vec3([0; 3]);
        let rhs: Vec3 = Vec3([1; 3]);
        assert_eq!(vec3.scalar_sum(rhs), 3);
    }
}
