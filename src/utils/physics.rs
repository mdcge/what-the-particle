use crate::utils::vec3::Vec3;
use crate::particle::particle::{Particle, ParticleType};
use approx::assert_relative_eq;

// Get particle energy
pub fn energy(particle: &Particle) -> f64 {
    (particle.state.p.mag().powf(2.0) + particle.state.m.powf(2.0)).sqrt()
}

// Get gamma factor of the particle
pub fn gamma(particle: &Particle) -> Option<f64> {
    let energy = energy(&particle);
    match particle.species {
        ParticleType::Gamma => None,
        _                   => Some(energy / particle.state.m),
    }
}


// Tests
fn assert_gamma_eq(lhs: Option<f64>, rhs: Option<f64>) {
    match (lhs, rhs) {
        (None, None)       => (),
        (None, Some(_))    => panic!("Mismatched variants: LHS is None, RHS is Some"),
        (Some(_), None)    => panic!("Mismatched variants: LHS is Some, RHS is None"),
        (Some(a), Some(b)) => assert_relative_eq!(a, b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_physics_gamma() {
        let p1 = Particle::new(Vec3(1.0, -1.0, 0.0), Vec3(4.0, -3.0, 0.0), ParticleType::Electron);
        let p2 = Particle::new(Vec3(0.0, 3.0, 5.0), Vec3(5.6, -2.1, -1.3), ParticleType::Muon);
        let p3 = Particle::new(Vec3(0.2, 2.5, -0.1), Vec3(5.2, -3.1, 4.4), ParticleType::Gamma);
        let p4 = Particle::new(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0), ParticleType::Electron);
        let p5 = Particle::new(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0), ParticleType::Muon);
        let p6 = Particle::new(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0), ParticleType::Gamma);
        assert_gamma_eq(gamma(&p1), Some(9.835703071628355));
        assert_gamma_eq(gamma(&p2), Some(1.001676303733957));
        assert_gamma_eq(gamma(&p3), None);
        assert_gamma_eq(gamma(&p4), Some(1.0));
        assert_gamma_eq(gamma(&p5), Some(1.0));
        assert_gamma_eq(gamma(&p6), None);
    }
}
