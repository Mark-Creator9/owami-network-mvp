// 3D Background Animation for Owami Network
class Background3D {
  constructor() {
    this.canvas = document.getElementById('background-canvas');
    if (!this.canvas) return;

    this.ctx = this.canvas.getContext('2d');
    this.particles = [];
    this.particleCount = 50;
    this.colors = ['#6366f1', '#06b6d4', '#10b981', '#f59e0b'];
    this.shapes = ['cube', 'sphere', 'pyramid', 'cylinder'];

    this.resize();
    this.initParticles();
    this.animate();

    window.addEventListener('resize', () => this.resize());
  }

  resize() {
    this.canvas.width = window.innerWidth;
    this.canvas.height = window.innerHeight;
  }

  initParticles() {
    this.particles = [];
    for (let i = 0; i < this.particleCount; i++) {
      this.particles.push(this.createParticle());
    }
  }

  createParticle() {
    const shape = this.shapes[Math.floor(Math.random() * this.shapes.length)];
    const color = this.colors[Math.floor(Math.random() * this.colors.length)];
    
    return {
      x: Math.random() * this.canvas.width,
      y: Math.random() * this.canvas.height,
      z: Math.random() * 100 + 50,
      size: Math.random() * 15 + 10,
      shape: shape,
      color: color,
      speedX: (Math.random() - 0.5) * 0.5,
      speedY: (Math.random() - 0.5) * 0.5,
      speedZ: Math.random() * 0.2 + 0.1,
      rotation: Math.random() * Math.PI * 2,
      rotationSpeed: (Math.random() - 0.5) * 0.02
    };
  }

  drawParticle(particle) {
    const scale = 100 / (100 + particle.z);
    const alpha = Math.min(1, particle.z / 200);
    
    this.ctx.save();
    this.ctx.translate(particle.x, particle.y);
    this.ctx.scale(scale, scale);
    this.ctx.rotate(particle.rotation);
    
    // Set color with alpha
    const rgbaColor = this.hexToRgba(particle.color, alpha);
    this.ctx.fillStyle = rgbaColor;
    this.ctx.strokeStyle = this.hexToRgba(particle.color, alpha * 0.5);
    
    // Draw different shapes
    switch (particle.shape) {
      case 'cube':
        this.drawCube(particle.size);
        break;
      case 'sphere':
        this.drawSphere(particle.size);
        break;
      case 'pyramid':
        this.drawPyramid(particle.size);
        break;
      case 'cylinder':
        this.drawCylinder(particle.size);
        break;
    }
    
    this.ctx.restore();
  }

  drawCube(size) {
    const halfSize = size / 2;
    
    // Front face
    this.ctx.fillRect(-halfSize, -halfSize, size, size);
    
    // Top face
    this.ctx.beginPath();
    this.ctx.moveTo(-halfSize, -halfSize);
    this.ctx.lineTo(halfSize, -halfSize);
    this.ctx.lineTo(halfSize * 1.2, -halfSize * 1.2);
    this.ctx.lineTo(-halfSize * 1.2, -halfSize * 1.2);
    this.ctx.closePath();
    this.ctx.fill();
    
    // Side face
    this.ctx.beginPath();
    this.ctx.moveTo(halfSize, -halfSize);
    this.ctx.lineTo(halfSize, halfSize);
    this.ctx.lineTo(halfSize * 1.2, halfSize * 0.8);
    this.ctx.lineTo(halfSize * 1.2, -halfSize * 1.2);
    this.ctx.closePath();
    this.ctx.fill();
  }

  drawSphere(size) {
    const radius = size / 2;
    
    this.ctx.beginPath();
    this.ctx.arc(0, 0, radius, 0, Math.PI * 2);
    this.ctx.fill();
    
    // Add some 3D effect with gradient
    const gradient = this.ctx.createRadialGradient(0, 0, 0, 0, 0, radius);
    gradient.addColorStop(0, this.hexToRgba('#ffffff', 0.3));
    gradient.addColorStop(1, this.hexToRgba('#000000', 0.1));
    this.ctx.fillStyle = gradient;
    this.ctx.fill();
  }

  drawPyramid(size) {
    const base = size;
    const height = size * 1.2;
    
    // Base
    this.ctx.beginPath();
    this.ctx.moveTo(-base/2, height/2);
    this.ctx.lineTo(base/2, height/2);
    this.ctx.lineTo(0, -height/2);
    this.ctx.closePath();
    this.ctx.fill();
    
    // Sides
    this.ctx.beginPath();
    this.ctx.moveTo(-base/2, height/2);
    this.ctx.lineTo(0, -height/2);
    this.ctx.lineTo(-base/4, height/4);
    this.ctx.closePath();
    this.ctx.fill();
    
    this.ctx.beginPath();
    this.ctx.moveTo(base/2, height/2);
    this.ctx.lineTo(0, -height/2);
    this.ctx.lineTo(base/4, height/4);
    this.ctx.closePath();
    this.ctx.fill();
  }

  drawCylinder(size) {
    const radius = size / 2;
    const height = size;
    
    // Top circle
    this.ctx.beginPath();
    this.ctx.arc(0, -height/2, radius, 0, Math.PI * 2);
    this.ctx.fill();
    
    // Bottom circle
    this.ctx.beginPath();
    this.ctx.arc(0, height/2, radius, 0, Math.PI * 2);
    this.ctx.fill();
    
    // Side
    this.ctx.beginPath();
    this.ctx.moveTo(radius, -height/2);
    this.ctx.lineTo(radius, height/2);
    this.ctx.lineTo(radius * 0.8, height/2);
    this.ctx.lineTo(radius * 0.8, -height/2);
    this.ctx.closePath();
    this.ctx.fill();
  }

  hexToRgba(hex, alpha) {
    const r = parseInt(hex.slice(1, 3), 16);
    const g = parseInt(hex.slice(3, 5), 16);
    const b = parseInt(hex.slice(5, 7), 16);
    
    return `rgba(${r}, ${g}, ${b}, ${alpha})`;
  }

  updateParticles() {
    this.particles.forEach(particle => {
      // Update position
      particle.x += particle.speedX;
      particle.y += particle.speedY;
      particle.z -= particle.speedZ;
      particle.rotation += particle.rotationSpeed;
      
      // Bounce off edges
      if (particle.x < 0 || particle.x > this.canvas.width) {
        particle.speedX *= -1;
      }
      
      if (particle.y < 0 || particle.y > this.canvas.height) {
        particle.speedY *= -1;
      }
      
      // Reset if too far away
      if (particle.z < -50) {
        Object.assign(particle, this.createParticle());
      }
    });
  }

  animate() {
    // Clear canvas with gradient background
    const gradient = this.ctx.createLinearGradient(0, 0, 0, this.canvas.height);
    gradient.addColorStop(0, '#f8fafc');
    gradient.addColorStop(1, '#e2e8f0');
    this.ctx.fillStyle = gradient;
    this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

    // Sort particles by z-index (closer particles first)
    this.particles.sort((a, b) => b.z - a.z);

    // Update and draw particles
    this.updateParticles();
    this.particles.forEach(particle => this.drawParticle(particle));

    // Animation loop
    requestAnimationFrame(() => this.animate());
  }
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  new Background3D();
});

// Export for testing
export default Background3D;