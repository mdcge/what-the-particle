import * as THREE from 'https://cdn.skypack.dev/three@0.132.2';
import { OrbitControls } from 'https://cdn.skypack.dev/three@0.132.2/examples/jsm/controls/OrbitControls.js';
import Stats from 'https://cdnjs.cloudflare.com/ajax/libs/stats.js/17/Stats.js';

import init, { WASMWorld } from "../pkg/mount_charles.js";

import Visualizer from './visualizer.js';

// === Program stats (FPS, memory) ===
const stats = new Stats();
stats.showPanel(0); // 0: fps, 1: ms, 2: memory
document.body.appendChild(stats.dom);

// === Perspective camera ===
const persp_camera = new THREE.PerspectiveCamera(
    40, // field of view
    window.innerWidth / window.innerHeight, // aspect ratio
    0.1, // near clipping plane
    10000 // far clipping plane
);
persp_camera.position.set(400, 400, 1100);

// === Visualizer ===
const visualizer = new Visualizer(persp_camera);
visualizer.add_volume(500);

// === Set camera ===
let active_camera = persp_camera;
visualizer.controls.object = active_camera;

function animate() {
    stats.begin();

    requestAnimationFrame(animate);

    visualizer.controls.update();
    visualizer.renderer.render(visualizer.scene, active_camera);

    stats.end();
}
animate();
