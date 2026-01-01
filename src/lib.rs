pub mod utils;
pub mod particle;
pub mod geometry;
pub mod sim;

use wasm_bindgen::prelude::*;
use crate::sim::world::World;
use crate::geometry::volume::Volume;
use crate::particle::particle::{Particle, ParticleType};
use crate::utils::vec3::Vec3;

#[wasm_bindgen]
pub struct WASMWorld {
    world: World,
}

#[wasm_bindgen]
impl WASMWorld {
    #[wasm_bindgen(constructor)]
    pub fn new(volume_size: f64, dt: f64) -> Self {
        let volume = Volume::new(volume_size);
        WASMWorld { world: World::new(vec![], volume, dt) }
    }

    // Add particle to simulation: API will need to be reviewed
    pub fn add_particle(&mut self, name: &str, x: f64, y: f64, z: f64, px: f64, py: f64, pz: f64) {
        let particle = match name {
            "e-"    => Particle::new(Vec3(x, y, z), Vec3(px, py, pz), ParticleType::Electron),
            "mu-"   => Particle::new(Vec3(x, y, z), Vec3(px, py, pz), ParticleType::Muon),
            "gamma" => Particle::new(Vec3(x, y, z), Vec3(px, py, pz), ParticleType::Gamma),
            _       => panic!("Unknown particle type."),
        };
        self.world.particles.push(particle);
    }

    // Get particle position
    // This is a temporary function for quick testing in JS
    pub fn get_particle_position(&self) -> Vec<f64> {
        let p = &self.world.particles[0];
        vec![p.state.r.0, p.state.r.1, p.state.r.2]
    }
}
