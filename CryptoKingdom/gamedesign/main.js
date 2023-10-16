// main.js

import * as THREE from 'three';

// Initialize Three.js scene, camera, and renderer
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

// Create a basic 3D terrain (placeholder)
const terrainGeometry = new THREE.PlaneGeometry(100, 100, 50, 50);
const terrainMaterial = new THREE.MeshBasicMaterial({ color: 0x00ff00, wireframe: true });
const terrain = new THREE.Mesh(terrainGeometry, terrainMaterial);
terrain.rotation.x = -Math.PI / 2; // Rotate to lie flat on the ground
scene.add(terrain);

// Create a basic player character (placeholder)
const playerGeometry = new THREE.BoxGeometry(1, 2, 1);
const playerMaterial = new THREE.MeshBasicMaterial({ color: 0xff0000 });
const player = new THREE.Mesh(playerGeometry, playerMaterial);
player.position.y = 1; // Place the player above the ground
scene.add(player);

// Set initial player position
const playerPosition = new THREE.Vector3(0, 0, 0);
player.position.copy(playerPosition);

// Initialize player controls
const playerControls = {
    moveForward: false,
    moveBackward: false,
    moveLeft: false,
    moveRight: false,
};

// Handle player movement
const playerSpeed = 0.1;
const playerRotationSpeed = 0.03;

function handlePlayerControls() {
    if (playerControls.moveForward) {
        playerPosition.z -= playerSpeed;
    }
    if (playerControls.moveBackward) {
        playerPosition.z += playerSpeed;
    }
    if (playerControls.moveLeft) {
        player.rotation.y += playerRotationSpeed;
    }
    if (playerControls.moveRight) {
        player.rotation.y -= playerRotationSpeed;
    }

    player.position.copy(playerPosition);
}

// Handle keyboard input for player controls
document.addEventListener('keydown', (event) => {
    const key = event.key.toLowerCase();
    if (key === 'w') {
        playerControls.moveForward = true;
    }
    if (key === 's') {
        playerControls.moveBackward = true;
    }
    if (key === 'a') {
        playerControls.moveLeft = true;
    }
    if (key === 'd') {
        playerControls.moveRight = true;
    }
});

document.addEventListener('keyup', (event) => {
    const key = event.key.toLowerCase();
    if (key === 'w') {
        playerControls.moveForward = false;
    }
    if (key === 's') {
        playerControls.moveBackward = false;
    }
    if (key === 'a') {
        playerControls.moveLeft = false;
    }
    if (key === 'd') {
        playerControls.moveRight = false;
    }
});

// Animation loop
function animate() {
    requestAnimationFrame(animate);

    // Update player position based on controls
    handlePlayerControls();

    renderer.render(scene, camera);
}

// Handle window resizing
window.addEventListener('resize', () => {
    const newWidth = window.innerWidth;
    const newHeight = window.innerHeight;
    camera.aspect = newWidth / newHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(newWidth, newHeight);
});

// Start the game
animate();
