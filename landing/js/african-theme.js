/**
 * African Theme JavaScript
 * Enhanced interactions and African-inspired animations
 */

class AfricanTheme {
    constructor() {
        this.isInitialized = false;
        this.notifications = [];
        this.cursor = null;
        this.themeActive = false;
        this.particles = [];
        this.animationFrame = null;
        
        this.init();
    }

    init() {
        if (this.isInitialized) return;
        
        this.createAfricanCursor();
        this.setupNotifications();
        this.setupAfricanAnimations();
        this.setupThemeToggle();
        this.setupAfricanTooltips();
        this.createParticleBackground();
        this.setupScrollAnimations();
        
        this.isInitialized = true;
        console.log('🌍 African Theme initialized');
    }

    /**
     * Create African-inspired custom cursor
     */
    createAfricanCursor() {
        if (window.innerWidth <= 768) return; // Skip on mobile
        
        const cursor = document.createElement('div');
        cursor.className = 'african-cursor';
        cursor.innerHTML = `
            <div class="cursor-dot"></div>
            <div class="cursor-ring"></div>
            <div class="cursor-trail"></div>
        `;
        
        const style = document.createElement('style');
        style.textContent = `
            .african-cursor {
                position: fixed;
                top: 0;
                left: 0;
                width: 20px;
                height: 20px;
                pointer-events: none;
                z-index: 9999;
                transition: transform 0.1s ease;
            }
            
            .cursor-dot {
                position: absolute;
                top: 50%;
                left: 50%;
                width: 4px;
                height: 4px;
                background: var(--african-gold);
                border-radius: 50%;
                transform: translate(-50%, -50%);
                transition: all 0.2s ease;
            }
            
            .cursor-ring {
                position: absolute;
                top: 50%;
                left: 50%;
                width: 20px;
                height: 20px;
                border: 2px solid var(--african-orange);
                border-radius: 50%;
                transform: translate(-50%, -50%);
                transition: all 0.3s ease;
                opacity: 0.7;
            }
            
            .cursor-trail {
                position: absolute;
                top: 50%;
                left: 50%;
                width: 40px;
                height: 40px;
                border: 1px solid var(--african-red);
                border-radius: 50%;
                transform: translate(-50%, -50%);
                transition: all 0.5s ease;
                opacity: 0.3;
            }
            
            .african-cursor.hover .cursor-dot {
                width: 6px;
                height: 6px;
                background: var(--african-green);
            }
            
            .african-cursor.hover .cursor-ring {
                width: 30px;
                height: 30px;
                border-color: var(--african-green);
                opacity: 1;
            }
            
            .african-cursor.hover .cursor-trail {
                width: 50px;
                height: 50px;
                border-color: var(--african-green);
                opacity: 0.5;
            }
            
            .african-cursor.click .cursor-dot {
                transform: translate(-50%, -50%) scale(1.5);
            }
            
            .african-cursor.click .cursor-ring {
                transform: translate(-50%, -50%) scale(0.8);
            }
        `;
        
        document.head.appendChild(style);
        document.body.appendChild(cursor);
        
        this.cursor = cursor;
        this.setupCursorEvents();
    }

    setupCursorEvents() {
        if (!this.cursor) return;
        
        let mouseX = 0;
        let mouseY = 0;
        let cursorX = 0;
        let cursorY = 0;
        
        document.addEventListener('mousemove', (e) => {
            mouseX = e.clientX;
            mouseY = e.clientY;
        });
        
        const animate = () => {
            cursorX += (mouseX - cursorX) * 0.1;
            cursorY += (mouseY - cursorY) * 0.1;
            
            this.cursor.style.transform = `translate(${cursorX}px, ${cursorY}px)`;
            requestAnimationFrame(animate);
        };
        
        animate();
        
        // Hover effects
        document.querySelectorAll('a, button, .btn, .card').forEach(element => {
            element.addEventListener('mouseenter', () => {
                this.cursor.classList.add('hover');
            });
            
            element.addEventListener('mouseleave', () => {
                this.cursor.classList.remove('hover');
            });
        });
        
        // Click effects
        document.addEventListener('mousedown', () => {
            this.cursor.classList.add('click');
        });
        
        document.addEventListener('mouseup', () => {
            this.cursor.classList.remove('click');
        });
    }

    /**
     * Setup African-themed notifications
     */
    setupNotifications() {
        const container = document.createElement('div');
        container.className = 'african-notifications-container';
        container.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            z-index: 10000;
            max-width: 400px;
        `;
        
        document.body.appendChild(container);
        
        // Add notification styles
        const style = document.createElement('style');
        style.textContent = `
            .african-notification {
                position: relative;
                margin-bottom: 12px;
                animation: slideInRight 0.3s cubic-bezier(0.68, -0.55, 0.265, 1.55);
            }
            
            @keyframes slideInRight {
                from {
                    transform: translateX(100%);
                    opacity: 0;
                }
                to {
                    transform: translateX(0);
                    opacity: 1;
                }
            }
            
            @keyframes slideOutRight {
                from {
                    transform: translateX(0);
                    opacity: 1;
                }
                to {
                    transform: translateX(100%);
                    opacity: 0;
                }
            }
        `;
        
        document.head.appendChild(style);
    }

    /**
     * Show African-themed notification
     */
    showNotification(message, type = 'info', duration = 5000) {
        const notification = document.createElement('div');
        notification.className = `african-notification african-notification-${type}`;
        
        const icons = {
            success: 'fas fa-check-circle',
            error: 'fas fa-exclamation-circle',
            warning: 'fas fa-exclamation-triangle',
            info: 'fas fa-info-circle'
        };
        
        const colorMap = {
            success: 'green',
            error: 'red',
            warning: 'orange',
            info: 'blue'
        };
        
        notification.innerHTML = `
            <div class="notification-content">
                <i class="${icons[type]}" style="color: var(--african-${colorMap[type]})"></i>
                <span>${message}</span>
            </div>
            <button class="notification-close" onclick="this.parentElement.parentElement.remove()">×</button>
        `;
        
        document.querySelector('.african-notifications-container').appendChild(notification);
        
        // Auto-remove after duration
        setTimeout(() => {
            if (notification.parentElement) {
                notification.style.animation = 'slideOutRight 0.3s forwards';
                setTimeout(() => notification.remove(), 300);
            }
        }, duration);
    }

    /**
     * Setup African-inspired animations
     */
    setupAfricanAnimations() {
        // Intersection Observer for scroll animations
        const observerOptions = {
            threshold: 0.1,
            rootMargin: '0px 0px -50px 0px'
        };
        
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('african-animate');
                }
            });
        }, observerOptions);
        
        // Observe elements for animation
        document.querySelectorAll('.ecosystem-card, .developer-card, .community-card').forEach(el => {
            observer.observe(el);
        });
        
        // Add animation styles
        const style = document.createElement('style');
        style.textContent = `
            .ecosystem-card, .developer-card, .community-card {
                opacity: 0;
                transform: translateY(30px);
                transition: all 0.6s cubic-bezier(0.4, 0, 0.2, 1);
            }
            
            .african-animate {
                opacity: 1 !important;
                transform: translateY(0) !important;
            }
            
            .african-animate:nth-child(1) { transition-delay: 0.1s; }
            .african-animate:nth-child(2) { transition-delay: 0.2s; }
            .african-animate:nth-child(3) { transition-delay: 0.3s; }
            .african-animate:nth-child(4) { transition-delay: 0.4s; }
        `;
        
        document.head.appendChild(style);
    }

    /**
     * Setup African-themed tooltips
     */
    setupAfricanTooltips() {
        const tooltips = document.querySelectorAll('[data-african-tooltip]');
        
        tooltips.forEach(element => {
            const tooltip = document.createElement('div');
            tooltip.className = 'african-tooltip-text';
            tooltip.textContent = element.getAttribute('data-african-tooltip');
            
            element.appendChild(tooltip);
            element.classList.add('african-tooltip');
        });
        
        // Add tooltip styles
        const style = document.createElement('style');
        style.textContent = `
            .african-tooltip {
                position: relative;
                display: inline-block;
            }
            
            .african-tooltip-text {
                visibility: hidden;
                width: 200px;
                background: linear-gradient(135deg, var(--african-dark), #34495e);
                color: white;
                text-align: center;
                border-radius: 8px;
                padding: 8px 12px;
                position: absolute;
                z-index: 1000;
                bottom: 125%;
                left: 50%;
                margin-left: -100px;
                opacity: 0;
                transition: opacity 0.3s;
                font-size: 14px;
                border: 1px solid var(--african-gold);
                box-shadow: 0 4px 12px rgba(255, 215, 0, 0.3);
            }
            
            .african-tooltip-text::after {
                content: "";
                position: absolute;
                top: 100%;
                left: 50%;
                margin-left: -5px;
                border-width: 5px;
                border-style: solid;
                border-color: var(--african-gold) transparent transparent transparent;
            }
            
            .african-tooltip:hover .african-tooltip-text {
                visibility: visible;
                opacity: 1;
            }
        `;
        
        document.head.appendChild(style);
    }

    /**
     * Setup theme toggle functionality
     */
    setupThemeToggle() {
        const toggleButton = document.createElement('button');
        toggleButton.className = 'african-theme-toggle';
        toggleButton.innerHTML = '🌍';
        toggleButton.title = 'Toggle African Theme';
        
        toggleButton.style.cssText = `
            position: fixed;
            bottom: 20px;
            right: 20px;
            width: 50px;
            height: 50px;
            border-radius: 50%;
            background: linear-gradient(135deg, var(--african-gold), var(--african-orange));
            border: none;
            color: white;
            font-size: 24px;
            cursor: pointer;
            box-shadow: 0 4px 12px rgba(255, 215, 0, 0.4);
            transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            z-index: 1000;
        `;
        
        document.body.appendChild(toggleButton);
        
        toggleButton.addEventListener('click', () => {
            this.toggleAfricanTheme();
        });
        
        // Add hover effect
        toggleButton.addEventListener('mouseenter', () => {
            toggleButton.style.transform = 'scale(1.1)';
        });
        
        toggleButton.addEventListener('mouseleave', () => {
            toggleButton.style.transform = 'scale(1)';
        });
    }

    /**
     * Toggle African theme
     */
    toggleAfricanTheme() {
        this.themeActive = !this.themeActive;
        document.body.classList.toggle('african-theme-active', this.themeActive);
        
        const message = this.themeActive 
            ? '🇿🇦 African theme activated! Experience the colors of Africa.'
            : 'Theme restored to default';
            
        this.showNotification(message, 'success');
    }

    /**
     * Add African-inspired particle effects
     */
    addParticleEffects() {
        const canvas = document.createElement('canvas');
        canvas.className = 'african-particles';
        canvas.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            pointer-events: none;
            z-index: 1;
        `;
        
        document.body.appendChild(canvas);
        
        const ctx = canvas.getContext('2d');
        const particles = [];
        
        // Resize canvas
        const resizeCanvas = () => {
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;
        };
        
        resizeCanvas();
        window.addEventListener('resize', resizeCanvas);
        
        // Particle class
        class Particle {
            constructor() {
                this.x = Math.random() * canvas.width;
                this.y = Math.random() * canvas.height;
                this.vx = (Math.random() - 0.5) * 0.5;
                this.vy = (Math.random() - 0.5) * 0.5;
                this.size = Math.random() * 3 + 1;
                this.color = this.getAfricanColor();
                this.opacity = Math.random() * 0.5 + 0.1;
            }
            
            getAfricanColor() {
                const colors = [
                    'var(--african-gold)',
                    'var(--african-orange)',
                    'var(--african-red)',
                    'var(--african-green)',
                    'var(--african-blue)',
                    'var(--african-purple)'
                ];
                return colors[Math.floor(Math.random() * colors.length)];
            }
            
            update() {
                this.x += this.vx;
                this.y += this.vy;
                
                if (this.x < 0 || this.x > canvas.width) this.vx *= -1;
                if (this.y < 0 || this.y > canvas.height) this.vy *= -1;
            }
            
            draw() {
                ctx.save();
                ctx.globalAlpha = this.opacity;
                ctx.fillStyle = this.color;
                ctx.beginPath();
                ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
                ctx.fill();
                ctx.restore();
            }
        }
        
        // Initialize particles
        for (let i = 0; i < 50; i++) {
            particles.push(new Particle());
        }
        
        // Animation loop
        const animate = () => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            
            particles.forEach(particle => {
                particle.update();
                particle.draw();
            });
            
            requestAnimationFrame(animate);
        };
        
        animate();
    }
}

// Initialize African Theme when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    const africanTheme = new AfricanTheme();
    
    // Add some demo notifications
    setTimeout(() => {
        africanTheme.showNotification('🌍 Welcome to OWami Network - Africa\'s Web3 future!', 'success');
    }, 2000);
    
    setTimeout(() => {
        africanTheme.showNotification('💡 Tip: Hover over elements to see African-themed tooltips', 'info');
    }, 5000);
    
    // Add particle effects
    africanTheme.addParticleEffects();
});

// Global notification function
window.showAfricanNotification = (message, type = 'info', duration = 5000) => {
    if (window.africanTheme) {
        window.africanTheme.showNotification(message, type, duration);
    }
};