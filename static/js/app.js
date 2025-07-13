// Rust + Actix Web Website JavaScript
document.addEventListener('DOMContentLoaded', function() {
    console.log('🦀 Rust + Actix Web website loaded successfully!');
    
    // Smooth scrolling for anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({ 
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
    
    // Form validation and enhancement
    const contactForm = document.querySelector('form[method="POST"]');
    if (contactForm) {
        contactForm.addEventListener('submit', function(e) {
            const name = document.getElementById('name');
            const email = document.getElementById('email');
            const message = document.getElementById('message');
            
            // Basic validation
            if (name && name.value.trim().length < 2) {
                alert('Name must be at least 2 characters long');
                e.preventDefault();
                return;
            }
            
            if (email && !isValidEmail(email.value)) {
                alert('Please enter a valid email address');
                e.preventDefault();
                return;
            }
            
            if (message && message.value.trim().length < 10) {
                alert('Message must be at least 10 characters long');
                e.preventDefault();
                return;
            }
            
            // Show loading state
            const submitBtn = contactForm.querySelector('button[type="submit"]');
            if (submitBtn) {
                submitBtn.textContent = 'Sending...';
                submitBtn.disabled = true;
            }
        });
    }
    
    // Email validation function
    function isValidEmail(email) {
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return emailRegex.test(email);
    }
    
    // Auto-hide success messages
    const messages = document.querySelectorAll('.message');
    messages.forEach(message => {
        if (message.classList.contains('success')) {
            setTimeout(() => {
                message.style.opacity = '0';
                setTimeout(() => {
                    message.remove();
                }, 300);
            }, 5000);
        }
    });
    
    // Add some interactive effects
    const featureCards = document.querySelectorAll('.feature-card');
    featureCards.forEach(card => {
        card.addEventListener('mouseenter', function() {
            this.style.transform = 'translateY(-10px) scale(1.02)';
        });
        
        card.addEventListener('mouseleave', function() {
            this.style.transform = 'translateY(0) scale(1)';
        });
    });
    
    // Navbar scroll effect
    window.addEventListener('scroll', function() {
        const navbar = document.querySelector('.navbar');
        if (window.scrollY > 50) {
            navbar.style.background = 'rgba(255, 255, 255, 0.95)';
            navbar.style.backdropFilter = 'blur(20px)';
        } else {
            navbar.style.background = '#fff';
            navbar.style.backdropFilter = 'blur(10px)';
        }
    });
    
    console.log('🚀 All JavaScript enhancements loaded!');
});
