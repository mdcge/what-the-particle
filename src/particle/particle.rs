use crate::utils::vec3::Vec3;

#[derive(Debug)]
pub struct ParticleState {
    pub r: Vec3,
    pub p: Vec3,
    pub e: f64,
    pub alive: bool,
}

impl ParticleState {
    pub fn new(pos: Vec3, mom: Vec3, energy: f64) -> Self {
        ParticleState { r: pos, p: mom, e: energy, alive: true }
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_vec3_eq;
    use approx::assert_relative_eq;

    #[test]
    fn test_particlestate_creation() {
        let v1 = Vec3(1.2, 4.3, -2.2);
        let v2 = Vec3(0.8, -3.3, 7.1);
        let e1 = 50.0;
        let e2 = 73.1;
        let ps1 = ParticleState::new(v1, v2, e1);
        let ps2 = ParticleState::new(v2, v1, e2);
        assert_vec3_eq!(ps1.r, v1);
        assert_vec3_eq!(ps1.p, v2);
        assert_relative_eq!(ps1.e, e1);
        assert_eq!(ps1.alive, true);
        assert_vec3_eq!(ps2.r, v2);
        assert_vec3_eq!(ps2.p, v1);
        assert_relative_eq!(ps2.e, e2);
        assert_eq!(ps2.alive, true);
    }
}
