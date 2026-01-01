pub mod utils;
pub mod particle;
pub mod geometry;
pub mod sim;

use wasm_bindgen::prelude::*;
use crate::sim::world::World;
use crate::geometry::volume::Volume;
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
}
