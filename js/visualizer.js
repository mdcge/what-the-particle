import * as THREE from 'https://cdn.skypack.dev/three@0.132.2';
import { OrbitControls } from 'https://cdn.skypack.dev/three@0.132.2/examples/jsm/controls/OrbitControls.js';

import CameraManager from './cameramanager.js';

export default class Visualizer {

    constructor(camera_manager) {
        // === Scene ===
        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0x111111);
        // === Renderer ===
        const renderer = new THREE.WebGLRenderer({ canvas: document.getElementById('scene') });
        renderer.setSize(window.innerWidth, window.innerHeight);
        // === OrbitControls ===
        let controls = new OrbitControls(camera_manager.active_camera, renderer.domElement);
        controls.enableDamping = true;
        controls.dampingFactor = 0.05;
        controls.enablePan = true;
        controls.enableZoom = true;
        controls.rotateSpeed = 0.8;
        controls.object = camera_manager.active_camera;

        this.scene = scene;
        this.renderer = renderer;
        this.controls = controls;
        this.camera_manager = camera_manager;
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

    update_controls() {
        this.controls.update();
    }

    render() {
        this.renderer.render(this.scene, this.camera_manager.active_camera)
    }

    set_ortho_view(axis) {
        this.controls.dispose();
        this.controls = new OrbitControls(this.camera_manager.ortho_camera, this.renderer.domElement);

        // Don't enable rotation in orthographic view
        this.controls.enableDamping = false;
        this.controls.enableRotate = false;
        this.controls.enableZoom = true;
        this.controls.screenSpacePanning = true;

        this.camera_manager.set_ortho_camera(axis);
        this.controls.object = this.camera_manager.active_camera;
    }

    set_persp_view() {
        this.controls.dispose();
        this.controls = new OrbitControls(this.camera_manager.persp_camera, this.renderer.domElement);

        // Set perspective camera behaviour
        this.controls.enableDamping = true;
        this.controls.dampingFactor = 0.05;
        this.controls.enablePan = true;
        this.controls.enableZoom = true;
        this.controls.rotateSpeed = 0.8;

        this.camera_manager.set_persp_camera();
        this.controls.object = this.camera_manager.active_camera;
    }

}
