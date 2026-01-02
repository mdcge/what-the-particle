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
visualizer.add_volume(500);

let i = 0;
function animate() {
    stats.begin();

    requestAnimationFrame(animate);

    if (i == 120) {
        visualizer.set_ortho_view();
        visualizer.set_ortho_view_axis("x");
        console.log("Switched to x");
    } else if (i == 240) {
        visualizer.set_ortho_view_axis("y");
        console.log("Switched to y");
    } else if (i == 360) {
        visualizer.set_ortho_view_axis("z");
        console.log("Switched to z");
    } else if (i == 480) {
        visualizer.set_persp_view();
        console.log("Switched to persp");
        i = 0;
    }
    i += 1;

    visualizer.update_controls();
    visualizer.render();

    stats.end();
}
animate();
