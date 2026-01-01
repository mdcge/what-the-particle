pub struct Volume {
    pub size: f64,  // cube edge length
}

impl Volume {
    pub fn new(s: f64) -> Self {
        Volume { size: s }
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_volume_creation() {
        let v1 = Volume::new(5.0);
        let v2 = Volume::new(15.0);
        let v3 = Volume::new(62.3);
        assert_relative_eq!(v1.size, 5.0);
        assert_relative_eq!(v2.size, 15.0);
        assert_relative_eq!(v3.size, 62.3);
    }
}
