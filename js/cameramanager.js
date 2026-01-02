import * as THREE from 'https://cdn.skypack.dev/three@0.132.2';

export default class CameraManager {

    constructor(persp_fov=40, near_clip=0.1, far_clip=10000, ortho_size=400) {
        const aspect = window.innerWidth / window.innerHeight;

        this.persp_camera = new THREE.PerspectiveCamera(
            persp_fov, // field of view
            aspect, // aspect ratio
            near_clip, // near clipping plane
            far_clip // far clipping plane
        );
        this.persp_camera.position.set(400, 400, 1100);

        this.ortho_camera = new THREE.OrthographicCamera(
            -ortho_size * aspect,
            ortho_size * aspect,
            ortho_size,
            -ortho_size,
            near_clip,
            far_clip
        );
        this.ortho_camera.position.set(1000, 0, 0); // looking along +x
        this.ortho_camera.up.set(0, 1, 0); // keep vertical orientation consistent
        this.ortho_camera.lookAt(0, 0, 0);

        this.active_camera = this.persp_camera; // by default, start in perspective view
    }

    switch_camera() {
        this.active_camera = this.active_camera == this.persp_camera ? this.ortho_camera : this.persp_camera;
    }

}
