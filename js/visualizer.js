import * as THREE from 'https://cdn.skypack.dev/three@0.132.2';
import { OrbitControls } from 'https://cdn.skypack.dev/three@0.132.2/examples/jsm/controls/OrbitControls.js';

export default class Visualizer {

    constructor(camera) {
        // === Scene ===
        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0x111111);
        // === Renderer ===
        const renderer = new THREE.WebGLRenderer({ canvas: document.getElementById('scene') });
        renderer.setSize(window.innerWidth, window.innerHeight);
        // === OrbitControls ===
        let controls = new OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;
        controls.dampingFactor = 0.05;
        controls.enablePan = true;
        controls.enableZoom = true;
        controls.rotateSpeed = 0.8;
        controls.object = camera;

        this.scene = scene;
        this.renderer = renderer;
        this.controls = controls;
    }

    add_volume(size, opacity=0.2) {
        const geometry = new THREE.BoxGeometry(size, size, size);
        const edges = new THREE.EdgesGeometry(geometry);
        const edge_material = new THREE.LineBasicMaterial({ color: 0x555555 });
        const box_edges = new THREE.LineSegments(edges, edge_material);
        this.scene.add(box_edges);

        const volume_material = new THREE.MeshBasicMaterial({
            color: 0x222222,
            transparent: true,
            opacity: opacity,
            side: THREE.BackSide,
        });
        const box_volume = new THREE.Mesh(geometry, volume_material);
        this.scene.add(box_volume);
    }
}
