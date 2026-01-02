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

// === OrbitControls ===
let controls = new OrbitControls(persp_camera, renderer.domElement);
controls.enableDamping = true;
controls.dampingFactor = 0.05;
controls.enablePan = true;
controls.enableZoom = true;
controls.rotateSpeed = 0.8;

// === Set camera ===
let active_camera = persp_camera;
controls.object = active_camera;

// === Detector volume ===
const size = 500;
const geometry = new THREE.BoxGeometry(size, size, size);
const edges = new THREE.EdgesGeometry(geometry);
const edge_material = new THREE.LineBasicMaterial({ color: 0x555555 });
const box_edges = new THREE.LineSegments(edges, edge_material);
scene.add(box_edges);

const volume_material = new THREE.MeshBasicMaterial({
    color: 0x222222,
    transparent: true,
    opacity: 0.2,
    side: THREE.BackSide,
});
const box_volume = new THREE.Mesh(geometry, volume_material);
scene.add(box_volume);

function animate() {
    stats.begin();

    requestAnimationFrame(animate);

    controls.update();
    renderer.render(scene, active_camera);

    stats.end();
}
animate();
