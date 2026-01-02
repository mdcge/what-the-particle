import * as THREE from 'https://cdn.skypack.dev/three@0.132.2';
import { OrbitControls } from 'https://cdn.skypack.dev/three@0.132.2/examples/jsm/controls/OrbitControls.js';
import Stats from 'https://cdnjs.cloudflare.com/ajax/libs/stats.js/17/Stats.js';

import init, { WASMWorld } from "../pkg/mount_charles.js";

// === Program stats (FPS, memory) ===
const stats = new Stats();
stats.showPanel(0); // 0: fps, 1: ms, 2: memory
document.body.appendChild(stats.dom);

// === Basic scene setup ===
const scene = new THREE.Scene();
scene.background = new THREE.Color(0x111111);

// === Renderer ===
const renderer = new THREE.WebGLRenderer({ canvas: document.getElementById('scene') });
renderer.setSize(window.innerWidth, window.innerHeight);

// === Perspective camera ===
const persp_camera = new THREE.PerspectiveCamera(
    40, // field of view
    window.innerWidth / window.innerHeight, // aspect ratio
    0.1, // near clipping plane
    10000 // far clipping plane
);
persp_camera.position.set(400, 400, 1100);

renderer.render(scene, persp_camera);

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
