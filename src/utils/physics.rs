use crate::utils::vec3::Vec3;
use crate::particle::particle::Particle;

// Get particle energy
pub fn energy(particle: &Particle) -> f64 {
    (particle.state.p.mag().powf(2.0) + particle.state.m.powf(2.0)).sqrt()
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use crate::particle::particle::ParticleType;

    #[test]
    fn test_physics_energy() {
        let p1 = Particle::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0), ParticleType::Electron);
        let p2 = Particle::new(Vec3(6.6, -3.0, 5.2), Vec3(-3.0, 0.0, 4.0), ParticleType::Muon);
        let p3 = Particle::new(Vec3(-10.2, -3.5, -1.9), Vec3(10.0, -20.0, 30.0), ParticleType::Gamma);
        let p4 = Particle::new(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0), ParticleType::Electron);
        let p5 = Particle::new(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0), ParticleType::Muon);
        let p6 = Particle::new(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0), ParticleType::Gamma);
        assert_relative_eq!(energy(&p1), 1.1229964381065507);
        assert_relative_eq!(energy(&p2), 105.77823783746825);
        assert_relative_eq!(energy(&p3), 37.416573867739416);
        assert_relative_eq!(energy(&p4), p4.state.m);
        assert_relative_eq!(energy(&p5), p5.state.m);
        assert_relative_eq!(energy(&p6), p6.state.m);
    }
}
