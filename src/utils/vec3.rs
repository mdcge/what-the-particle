use std::ops::{Add, Sub, Mul};
use std::cmp::{PartialEq};
use approx::{relative_eq, assert_relative_eq};

#[derive(Copy, Clone, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }
}

// Addition (+)
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0+rhs.0, self.1+rhs.1, self.2+rhs.2)
    }
}

// Subtraction (-)
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0-rhs.0, self.1-rhs.1, self.2-rhs.2)
    }
}

// Multiplication (*)
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0*rhs, self.1*rhs, self.2*rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self*rhs.0, self*rhs.1, self*rhs.2)
    }
}

// Comparison (==)
impl PartialEq<Vec3> for Vec3 {
    fn eq(&self, rhs: &Vec3) -> bool {
        relative_eq!(self.0, rhs.0) && relative_eq!(self.1, rhs.1) && relative_eq!(self.2, rhs.2)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_vec3_eq;

    #[test]
    fn test_indexing() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(5.5, 3.2, 4.5);
        assert_relative_eq!(v1.0, 1.0);
        assert_relative_eq!(v1.1, 2.0);
        assert_relative_eq!(v1.2, 3.0);
        assert_relative_eq!(v2.0, 5.5);
        assert_relative_eq!(v2.1, 3.2);
        assert_relative_eq!(v2.2, 4.5);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3(0.0, 0.0, 0.0);
        let v2 = Vec3(1.0, 2.0, 3.0);
        let v3 = Vec3(3.7, -0.8, -1.5);
        assert_vec3_eq!(v1 + v2, Vec3(1.0, 2.0, 3.0));
        assert_vec3_eq!(v1 + v3, Vec3(3.7, -0.8, -1.5));
        assert_vec3_eq!(v2 + v3, Vec3(4.7, 1.2, 1.5));
        assert_vec3_eq!(v1 + v2, v2 + v1);
        assert_vec3_eq!(v1 + v3, v3 + v1);
        assert_vec3_eq!(v2 + v3, v3 + v2);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3(0.0, 0.0, 0.0);
        let v2 = Vec3(1.0, 2.0, 3.0);
        let v3 = Vec3(3.7, -0.8, -1.5);
        assert_vec3_eq!(v2 - v1, Vec3(1.0, 2.0, 3.0));
        assert_vec3_eq!(v1 - v3, Vec3(-3.7, 0.8, 1.5));
        assert_vec3_eq!(v2 - v3, Vec3(-2.7, 2.8, 4.5));
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3(0.0, 1.0, 2.0);
        let v2 = Vec3(5.2, 3.1, 4.7);
        let v3 = Vec3(-0.9, 7.2, -2.3);
        let f1 = 4.3;
        let f2 = -1.2;
        let f3 = 0.0;
        assert_vec3_eq!(v1 * f1, Vec3(0.0, 4.3, 8.6));
        assert_vec3_eq!(v3 * f2, Vec3(1.08, -8.64, 2.76));
        assert_vec3_eq!(v1 * f1, f1 * v1);
        assert_vec3_eq!(v1 * f2, f2 * v1);
        assert_vec3_eq!(v1 * f3, f3 * v1);
        assert_vec3_eq!(v2 * f1, f1 * v2);
        assert_vec3_eq!(v2 * f2, f2 * v2);
        assert_vec3_eq!(v2 * f3, f3 * v2);
        assert_vec3_eq!(v3 * f1, f1 * v3);
        assert_vec3_eq!(v3 * f2, f2 * v3);
        assert_vec3_eq!(v3 * f3, f3 * v3);
    }
}
