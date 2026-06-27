use rand::RngExt;
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::{Rectangle, Vector2},
    text::{Font, RaylibFont},
};

use crate::GRAVITY;

pub enum NumberEffectType {
    Damage,
    Heal,
    Block,
}

pub struct NumberEffect {
    value_as_str: String,

    // remove public access eventually
    pub pos: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    font_size: f32,
    font_spacing: f32,
    color: Color,

    // used in some update instances to stop the number from going below this
    start_pos_y: f32,

    // used for sin wave calculations
    start_pos_x: f32,

    lifespan: f32,
    vertical_sine_wave: bool,
}

impl NumberEffect {
    /// Font spacing and font size are automatic based on value, this system is very much not general use
    pub fn new(num_effect_type: NumberEffectType, value: i32, pos_rect: Rectangle, font: &Font) -> Self {
        let (font_size, font_spacing) = match value {
            ..=10 => (10.0, 0.0),
            11..=100 => (14.0, 1.0),
            101.. => (18.0, 2.0),
        };

        let mut rng = rand::rng();
        let value_as_str = value.to_string();
        let text_size = font.measure_text(&value_as_str, font_size, font_spacing);
        let start_pos_y = pos_rect.y + pos_rect.height - text_size.y;
        let start_pos_x = pos_rect.x + (pos_rect.width / 2.0) - (text_size.x / 2.0);
        let pos = Vector2::new(start_pos_x, start_pos_y);

        let effect = match num_effect_type {
            NumberEffectType::Damage => {         
                let vel_x: f32 = rng.random_range(-10.0..=10.0);
                let vel_y: f32 = rng.random_range(-180.0..=-160.0);

                let acc_x: f32 = rng.random_range(-5.0..=5.0);
                let acc_y: f32 = GRAVITY;

                NumberEffect {
                    value_as_str,
                    pos,
                    velocity: Vector2::new(vel_x, vel_y),
                    acceleration: Vector2::new(acc_x, acc_y),
                    color: Color::WHITE,
                    vertical_sine_wave: false,
                    font_size,
                    font_spacing,
                    start_pos_x,
                    start_pos_y,
                    lifespan: 3.0,
                }
            }
            NumberEffectType::Heal => {
                
                let vel_x: f32 = 0.0;
                let vel_y: f32 = -25.0;

                let acc_x = 0.0;
                let acc_y = 0.0;

                NumberEffect {
                    value_as_str,
                    pos,
                    velocity: Vector2::new(vel_x, vel_y),
                    acceleration: Vector2::new(acc_x, acc_y),
                    font_size,
                    font_spacing,
                    color: Color::LIGHTGREEN,
                    start_pos_x,
                    start_pos_y,
                    lifespan: 2.0,
                    vertical_sine_wave: true,
                }
            }
            NumberEffectType::Block => {

                // WHENEVER YOU COME HERE
                // make the block number move very slowly upward
                // this signifies a different movement from being attacked
                // so you can tell at a glance this is a block (unmoving) number vs
                // the number flying out of you at an angle like damage would
                
                let vel_x: f32 = rng.random_range(-5.0..=1.0);
                let vel_y: f32 = rng.random_range(-150.0..=-120.0);

                let acc_x: f32 = 0.0;
                let acc_y: f32 = GRAVITY;

                NumberEffect {
                    value_as_str,
                    pos,
                    velocity: Vector2::new(vel_x, vel_y),
                    acceleration: Vector2::new(acc_x, acc_y),
                    font_size,
                    font_spacing,
                    color: Color::SANDYBROWN,
                    start_pos_x,
                    start_pos_y,
                    lifespan: 2.5,
                    vertical_sine_wave: false,
                }
            }
        };

        return effect;
    }

    pub fn update(&mut self, dt: f32, total_time: f32) {

        self.lifespan -= dt;

        if self.pos.y > self.start_pos_y {
            return;
        }
        
        self.pos.x += self.velocity.x * dt;
        self.pos.y += self.velocity.y * dt;

        self.velocity.x += self.acceleration.x * dt;
        self.velocity.y += self.acceleration.y * dt;

        match self.vertical_sine_wave {
            true => {
                let frequency = 4.0;
                let magnitude = 15.0;
                self.pos.x = self.start_pos_x + (total_time * frequency).sin() * magnitude;
            }
            false => (),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, font: &Font) {
        d.draw_text_ex(font, &self.value_as_str, self.pos, self.font_size, self.font_spacing, self.color);
    }

    pub fn is_done(&self) -> bool {
        if self.lifespan <= 0.0 {
            return true;
        } else {
            return false;
        }
    }
}
