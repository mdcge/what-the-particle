use std::ops::{Add};
use approx::assert_relative_eq;

#[derive(Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0+rhs.0, self.1+rhs.1, self.2+rhs.2)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

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
        assert_relative_eq!((v1+v2).0, 1.0);
        assert_relative_eq!((v1+v2).1, 2.0);
        assert_relative_eq!((v1+v2).2, 3.0);
        assert_relative_eq!((v1+v3).0, 3.7);
        assert_relative_eq!((v1+v3).1, -0.8);
        assert_relative_eq!((v1+v3).2, -1.5);
        assert_relative_eq!((v2+v3).0, 4.7);
        assert_relative_eq!((v2+v3).1, 1.2);
        assert_relative_eq!((v2+v3).2, 1.5);
    }
}
