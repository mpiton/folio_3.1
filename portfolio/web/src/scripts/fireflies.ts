import * as THREE from 'three';

export class FirefliesAnimation {
  private scene: THREE.Scene;
  private camera!: THREE.PerspectiveCamera;
  private renderer!: THREE.WebGLRenderer;
  private particles!: THREE.Points;
  private particlesCount: number = 200;
  private positions!: Float32Array;
  private canvas: HTMLCanvasElement;
  private clock: THREE.Clock;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.scene = new THREE.Scene();
    this.clock = new THREE.Clock();
    this.setupCamera();
    this.setupRenderer();
    this.createParticles();
    this.animate();
    this.handleResize();
  }

  private setupCamera() {
    const fov = 75;
    const aspect = window.innerWidth / window.innerHeight;
    const near = 0.1;
    const far = 1000;
    this.camera = new THREE.PerspectiveCamera(fov, aspect, near, far);
    this.camera.position.z = 20;
  }

  private setupRenderer() {
    this.renderer = new THREE.WebGLRenderer({
      canvas: this.canvas,
      alpha: true,
      antialias: true
    });
    this.renderer.setClearColor(0x00_00_00, 0);
    this.renderer.setSize(window.innerWidth, window.innerHeight);
    this.renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  }

  private createParticles() {
    const geometry = new THREE.BufferGeometry();
    this.positions = new Float32Array(this.particlesCount * 3);

    // Distribution des particules dans un volume plus naturel
    for(let i = 0; i < this.particlesCount; i++) {
      const i3 = i * 3;
      this.positions[i3] = (Math.random() - 0.5) * 40; // Plus large sur X
      this.positions[i3 + 1] = Math.random() * 25; // Plus haut sur Y
      this.positions[i3 + 2] = (Math.random() - 0.5) * 15; // Plus de profondeur sur Z
    }

    geometry.setAttribute('position', new THREE.BufferAttribute(this.positions, 3));

    const textureLoader = new THREE.TextureLoader();
    const particleTexture = textureLoader.load('/particles/circle.svg');

    const material = new THREE.PointsMaterial({
      size: 0.5,
      map: particleTexture,
      transparent: true,
      opacity: 0.9,
      color: '#7AC4AA',
      sizeAttenuation: true,
      blending: THREE.AdditiveBlending,
      depthWrite: false
    });

    this.particles = new THREE.Points(geometry, material);
    this.scene.add(this.particles);
  }

  private animate() {
    requestAnimationFrame(() => this.animate());
    const elapsedTime = this.clock.getElapsedTime();

    // Animation plus naturelle des lucioles
    const positions = this.particles.geometry.attributes.position.array as Float32Array;
    for (let i = 0; i < positions.length; i += 3) {
      positions[i] += Math.sin(elapsedTime * 0.5 + i) * 0.02;
      positions[i + 1] += Math.cos(elapsedTime * 0.3 + i) * 0.02;
      positions[i + 2] += Math.sin(elapsedTime * 0.4 + i) * 0.01;
    }
    this.particles.geometry.attributes.position.needsUpdate = true;

    // Effet de clignotement plus subtil
    const material = this.particles.material as THREE.PointsMaterial;
    material.opacity = 0.7 + Math.sin(elapsedTime * 1.5) * 0.3;
    material.size = 0.5 + Math.sin(elapsedTime * 2 + Math.PI) * 0.2;

    this.renderer.render(this.scene, this.camera);
  }

  private handleResize() {
    window.addEventListener('resize', () => {
      this.camera.aspect = window.innerWidth / window.innerHeight;
      this.camera.updateProjectionMatrix();
      this.renderer.setSize(window.innerWidth, window.innerHeight);
    });
  }

  public destroy() {
    this.scene.remove(this.particles);
    this.particles.geometry.dispose();
    (this.particles.material as THREE.PointsMaterial).dispose();
    this.renderer.dispose();
  }
}
