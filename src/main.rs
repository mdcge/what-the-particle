use rust::sim::world::World;
use rust::particle::particle::{Particle, ParticleType};
use rust::geometry::volume::Volume;
use rust::utils::vec3::Vec3;

fn main() {
    let volume = Volume::new(10.0);
    let electron = Particle::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0), ParticleType::Electron);
    let mut world = World::new(vec![electron], volume, 0.001);
    while world.has_alive_particles() {
        world.step();
    }
}
