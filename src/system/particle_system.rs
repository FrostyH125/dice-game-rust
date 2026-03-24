use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{math::Vector2, prelude::RaylibDrawHandle, texture::Texture2D};

use crate::system::particle::Particle;

pub struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        ParticleSystem { particles: Vec::new() }
    }

    pub fn emit(
        &mut self,
        sprite: &'static Sprite,
        position: Vector2,
        velocity: Vector2,
        acceleration: Vector2,
        lifetime: f32,
    ) {
        let sprite_half_width = sprite.src_rect.width / 2.0;
        let sprite_half_height = sprite.src_rect.height / 2.0;

        // this is to make it spawn the particle with the center of the particle being the passed position
        // i did this so it would be easier to make things look more uniform, for example, now if particles
        // are emitted randomly along a straight edge, it'll not require any width or height math to account
        // for the width or height of the particle so that one side's particles doesn't extend further than the
        // other side
        let real_emit_pos = Vector2::new(position.x - sprite_half_width, position.y - sprite_half_height);

        let particle = Particle {
            sprite,
            position: real_emit_pos,
            velocity,
            acceleration,
            lifetime,
        };

        self.particles.push(particle);
    }

    pub fn update(&mut self, dt: f32) {
        for i in (0..self.particles.len()).rev() {
            let particle = &mut self.particles[i];

            particle.position += particle.velocity * dt;
            particle.velocity += particle.acceleration * dt;
            particle.lifetime -= dt;

            if particle.lifetime <= 0.0 {
                self.particles.remove(i);
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        for particle in &self.particles {
            particle.sprite.draw(d, particle.position, texture);
        }
    }
}
