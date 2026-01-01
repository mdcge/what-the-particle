use crate::utils::vec3::Vec3;
use crate::sim::world::World;
use crate::utils::constants::C;
use crate::utils::physics::beta;

// Particle state
#[derive(Debug, Clone)]
pub struct ParticleState {
    pub r: Vec3,  // position (mm)
    pub p: Vec3,  // momentum (MeV)
    pub m: f64,   // mass (MeV)
    pub alive: bool,
}

impl ParticleState {
    pub fn new(pos: Vec3, mom: Vec3, mass: f64) -> Self {
        ParticleState { r: pos, p: mom, m: mass, alive: true }
    }
}

// Particle type
#[derive(Clone)]
pub enum ParticleType {
    Electron,
    Muon,
    Gamma,
}

// Particle
#[derive(Clone)]
pub struct Particle {
    pub species: ParticleType,
    pub state: ParticleState,
}

impl Particle {
    pub fn new(pos: Vec3, mom: Vec3, part_type: ParticleType) -> Self {
        let mass = match part_type {
            ParticleType::Electron =>   0.511,
            ParticleType::Muon     => 105.66,
            ParticleType::Gamma    =>   0.0,
        };
        let particle_state = ParticleState::new(pos, mom, mass);

        Particle { species: part_type, state: particle_state }
    }

    pub fn propagate(&mut self, dt: f64) {
        let dir = self.state.p.norm();
        let beta = beta(&self);
        self.state.r += dir * beta * C * dt;  // dir[1] * beta[1] * C[mm/ns] * dt[ns]
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_vec3_eq;
    use approx::assert_relative_eq;
    use crate::geometry::volume::Volume;

    #[test]
    fn test_particlestate_creation() {
        let v1 = Vec3(1.2, 4.3, -2.2);
        let v2 = Vec3(0.8, -3.3, 7.1);
        let m1 = 50.0;
        let m2 = 73.1;
        let ps1 = ParticleState::new(v1, v2, m1);
        let ps2 = ParticleState::new(v2, v1, m2);
        assert_vec3_eq!(ps1.r, v1);
        assert_vec3_eq!(ps1.p, v2);
        assert_relative_eq!(ps1.m, m1);
        assert_eq!(ps1.alive, true);
        assert_vec3_eq!(ps2.r, v2);
        assert_vec3_eq!(ps2.p, v1);
        assert_relative_eq!(ps2.m, m2);
        assert_eq!(ps2.alive, true);
    }

    #[test]
    fn test_particletype_creation() {
        let _pt1 = ParticleType::Electron;
        let _pt2 = ParticleType::Muon;
        let _pt3 = ParticleType::Gamma;
    }

    #[test]
    fn test_particle_creation() {
        let electron = Particle::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0), ParticleType::Electron);
        let muon = Particle::new(Vec3(5.0, -2.0, 10.0), Vec3(2.0, 3.0, -4.0), ParticleType::Muon);
        let gamma = Particle::new(Vec3(-1.2, 7.6, 6.7), Vec3(-9.8, -2.5, -1.1), ParticleType::Gamma);
        assert_vec3_eq!(electron.state.r, Vec3(0.0, 0.0, 0.0));
        assert_vec3_eq!(electron.state.p, Vec3(1.0, 0.0, 0.0));
        assert_relative_eq!(electron.state.m, 0.511);
        assert_eq!(electron.state.alive, true);
        assert_vec3_eq!(muon.state.r, Vec3(5.0, -2.0, 10.0));
        assert_vec3_eq!(muon.state.p, Vec3(2.0, 3.0, -4.0));
        assert_relative_eq!(muon.state.m, 105.66);
        assert_eq!(muon.state.alive, true);
        assert_vec3_eq!(gamma.state.r, Vec3(-1.2, 7.6, 6.7));
        assert_vec3_eq!(gamma.state.p, Vec3(-9.8, -2.5, -1.1));
        assert_relative_eq!(gamma.state.m, 0.0);
        assert_eq!(gamma.state.alive, true);
    }

    #[test]
    fn test_particle_propagate() {
        let mut electron1 = Particle::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0), ParticleType::Electron);
        let mut muon1 = Particle::new(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 1.0, 0.0), ParticleType::Muon);
        let mut gamma1 = Particle::new(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 1.0), ParticleType::Gamma);

        let volume = Volume::new(10.0);

        electron1.propagate(1.0);
        assert_vec3_eq!(electron1.state.r, Vec3(266.9576214377587, 0.0, 0.0));
        electron1.propagate(0.1);
        assert_vec3_eq!(electron1.state.r, Vec3(266.9576214377587+26.69576214377587, 0.0, 0.0));
        muon1.propagate(1.0);
        assert_vec3_eq!(muon1.state.r, Vec3(0.0, 2.837204544727953, 0.0));
        muon1.propagate(0.1);
        assert_vec3_eq!(muon1.state.r, Vec3(0.0, 2.837204544727953+0.2837204544727953, 0.0));
        gamma1.propagate(1.0);
        assert_vec3_eq!(gamma1.state.r, Vec3(0.0, 0.0, 299.792458));
        gamma1.propagate(0.1);
        assert_vec3_eq!(gamma1.state.r, Vec3(0.0, 0.0, 299.792458+29.9792458));
    }
}
