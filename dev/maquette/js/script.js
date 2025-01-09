ScrollReveal().reveal('.about-content', {
  origin: 'left',
  distance: '60px',
  duration: 1000,
  delay: 200,
  easing: 'cubic-bezier(0.5, 0, 0, 1)'
});

ScrollReveal().reveal('.about-stats', {
  origin: 'right',
  distance: '60px',
  duration: 1000,
  delay: 200,
  easing: 'cubic-bezier(0.5, 0, 0, 1)'
});

// Animations des sections avec ScrollReveal
ScrollReveal().reveal('.about-intro', { delay: 200, distance: '60px', origin: 'bottom', duration: 800 });
ScrollReveal().reveal('.skills', { delay: 200, distance: '60px', origin: 'left', duration: 800 });
ScrollReveal().reveal('.timeline', { delay: 200, distance: '60px', origin: 'right', duration: 800 });
ScrollReveal().reveal('.hobbies', { delay: 200, distance: '60px', origin: 'bottom', duration: 800 });

const canvas = document.getElementById('hero-canvas');

if (canvas) {
  const ctx = canvas.getContext('2d');

  // Ajuster la taille du canvas à la taille de la fenêtre
  function resizeCanvas() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
  }

  resizeCanvas();
  window.addEventListener('resize', resizeCanvas);

  // Classe représentant une goutte d'eau
  class Splatter {
    constructor(x, y, vx, vy, radius, opacity) {
      this.x = x;
      this.y = y;
      this.vx = vx;
      this.vy = vy;
      this.radius = radius;
      this.opacity = opacity;
    }

    draw() {
      ctx.beginPath();
      ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2);
      ctx.fillStyle = `rgba(180, 225, 255, ${this.opacity})`;
      ctx.fill();
    }

    update() {
      this.x += this.vx;
      this.y += this.vy;
      this.vy += 0.1;
      this.opacity -= 0.02;
    }
  }

  class Droplet {
    constructor(x, y) {
      this.x = x;
      this.y = y;
      this.radius = Math.random() * 20 + 10;
      this.opacity = Math.random() * 0.4 + 0.3;
      this.speed = Math.random() * 5 + 1;
      this.ellipseWidth = this.radius * 2;
      this.ellipseHeight = this.radius * 0.7;
      this.ripples = [];
    }

    draw() {
      ctx.beginPath();
      const gradient = ctx.createRadialGradient(
        this.x, this.y, 0, this.x, this.y, this.radius
      );
      gradient.addColorStop(0, `rgba(180, 225, 255, ${this.opacity})`);
      gradient.addColorStop(1, `rgba(180, 225, 255, 0)`);
      ctx.fillStyle = gradient;
      ctx.ellipse(this.x, this.y, this.ellipseWidth / 2, this.ellipseHeight / 2, 0, 0, Math.PI * 2);
      ctx.fill();

      this.ripples.forEach(ripple => {
        ctx.beginPath();
        ctx.strokeStyle = `rgba(180, 225, 255, ${ripple.opacity})`;
        ctx.arc(this.x, this.y, ripple.radius, 0, Math.PI * 2);
        ctx.stroke();
      });
    }

    update() {
      this.y += this.speed;
      this.ellipseWidth += this.speed * 1.5;
      this.ellipseHeight += this.speed;
      this.opacity -= 0.005;

      if (this.ellipseWidth > this.radius * 2) {
        this.ellipseWidth = this.radius * 2;
        this.ellipseHeight = this.radius * 2;
      }

      this.ripples.forEach(ripple => {
        ripple.radius += ripple.speed;
        ripple.opacity -= 0.01;
      });

      this.ripples = this.ripples.filter(ripple => ripple.opacity > 0);

      if (this.ripples.length === 0 && Math.random() < 0.03) {
        this.ripples.push({
          radius: this.radius,
          opacity: this.opacity * 1.5,
          speed: Math.random() * 1.5 + 0.5
        });
      }

      if (this.y > canvas.height - this.radius) {
        this.splash();
        return true;
      }

      return false;
    }

    splash() {
      const numSplatters = Math.floor(this.radius / 3);
      const baseSplatterSize = this.radius / 4;

      for (let i = 0; i < numSplatters; i++) {
        const vx = (Math.random() - 0.5) * this.speed * 3;
        const vy = -Math.random() * this.speed * 4;
        const radius = Math.random() * baseSplatterSize + baseSplatterSize / 2;
        const opacity = Math.random() * 0.3 + 0.3;
        splatters.push(new Splatter(this.x, canvas.height - this.radius, vx, vy, radius, opacity));
      }
    }
  }

  const droplets = [];
  const splatters = [];

  function createRandomDroplet() {
    const x = Math.random() * canvas.width;
    const y = -50;
    droplets.push(new Droplet(x, y));
  }

  canvas.addEventListener('click', (event) => {
    const x = event.clientX;
    const y = event.clientY;
    droplets.push(new Droplet(x, y));
  });

  function animate() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    droplets.forEach((droplet, index) => {
      droplet.draw();
      if (droplet.update()) {
        droplets.splice(index, 1);
      }
    });

    splatters.forEach((splatter, index) => {
      splatter.draw();
      splatter.update();

      if (splatter.opacity <= 0) {
        splatters.splice(index, 1);
      }
    });

    requestAnimationFrame(animate);
  }

  setInterval(createRandomDroplet, 400);
  animate();
}

// Défilement fluide pour le bouton "Retour en haut"
const backToTopButton = document.querySelector('.back-to-top');
if (backToTopButton) {
  backToTopButton.addEventListener('click', () => {
    window.scrollTo({
      top: 0,
      behavior: 'smooth'
    });
  });
}

// Afficher/masquer le bouton en fonction du scroll
window.addEventListener('scroll', () => {
  if (window.pageYOffset > 100) {
    backToTopButton.classList.add('show');
  } else {
    backToTopButton.classList.remove('show');
  }
});
