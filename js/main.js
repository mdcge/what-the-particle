import * as THREE from 'https://cdn.skypack.dev/three@0.132.2';
import { OrbitControls } from 'https://cdn.skypack.dev/three@0.132.2/examples/jsm/controls/OrbitControls.js';
import Stats from 'https://cdnjs.cloudflare.com/ajax/libs/stats.js/17/Stats.js';

import init, { WASMWorld } from "../pkg/mount_charles.js";

import Visualizer from './visualizer.js';
import CameraManager from './cameramanager.js';

// === Program stats (FPS, memory) ===
const stats = new Stats();
stats.showPanel(0); // 0: fps, 1: ms, 2: memory
document.body.appendChild(stats.dom);

// === Camera ===
const camera_manager = new CameraManager();

// === Visualizer ===
const visualizer = new Visualizer(camera_manager);
const volume_size = 500;
visualizer.add_volume(volume_size);

// === Switch between views ===
window.addEventListener('keydown', e => {
    if ((e.key === '1') || (e.key === 'x')) { visualizer.set_ortho_view("x"); }
    if ((e.key === '2') || (e.key === 'y')) { visualizer.set_ortho_view("y"); }
    if ((e.key === '3') || (e.key === 'z')) { visualizer.set_ortho_view("z"); }
    if ((e.key === '0') || (e.key === 'p')) { visualizer.set_persp_view(); }
});

await init(); // load WASM module
const world = new WASMWorld(volume_size, 0.001, 1);  // create world
world.add_particle("e-", 0, 0, 0, 10, 0, 0);  // add particle

function animate() {
    stats.begin();

    requestAnimationFrame(animate);

    visualizer.update_controls();
    visualizer.render();

    world.step();
    const points = world.get_particle_position_history().map(([x, y, z]) => new THREE.Vector3(x, y, z));
    const geometry = new THREE.BufferGeometry().setFromPoints(points);
    const material = new THREE.LineBasicMaterial({ color: 0xff0000 });
    const line = new THREE.Line(geometry, material);
    visualizer.scene.add(line);

    stats.end();
}
animate();
