use approx::assert_relative_eq;

pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
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
}
