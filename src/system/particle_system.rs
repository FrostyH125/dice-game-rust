use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{math::Vector2, prelude::RaylibDrawHandle, texture::Texture2D};

use crate::system::particle::Particle;

pub struct ParticleSystem {
    particles: [Particle; 1000]
}

impl ParticleSystem {
    
    pub fn new() -> Self {
        ParticleSystem { particles: core::array::from_fn(|_| Particle::new_default()) }
    }
    
    pub fn emit(&mut self, sprite: &'static Sprite, position: Vector2, velocity: Vector2, acceleration: Vector2, lifetime: f32) {
        for particle in &mut self.particles {
            if particle.is_active {
                continue;
            }
            
            *particle = Particle { sprite, position, velocity, acceleration, lifetime, is_active: true }
        }
    }
    
    pub fn update(&mut self) {
        
    }
    
    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: Texture2D) {
        
    }
}