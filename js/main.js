import init, { WASMWorld } from "../pkg/mount_charles.js";

async function run() {
    await init(); // load WASM module

    const world = new WASMWorld(10.0, 0.001);  // create world
    world.add_particle("e-", 0, 0, 0, 1, 0, 0);  // add particle

    for (let i = 0; i < 5; i++) {
        world.step();
        console.log(`Step ${i+1}:`, world.get_particle_position());
    }
}

run();
