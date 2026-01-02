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
const visualizer = new Visualizer(camera_manager.active_camera);
visualizer.add_volume(500);

function animate() {
    stats.begin();

    requestAnimationFrame(animate);

    visualizer.controls.update();
    visualizer.renderer.render(visualizer.scene, camera_manager.active_camera);

    stats.end();
}
animate();
