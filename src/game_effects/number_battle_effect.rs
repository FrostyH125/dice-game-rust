use rand::RngExt;
use raylib::{
    color::Color,
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
    pos: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    font_size: f32,
    font_spacing: f32,
    color: Color,

    // used in some update instances to stop the number from going below this
    start_pos_y: f32,
    lifespan: f32,
    vertical_sine_wave: bool,
    num_effect_type: NumberEffectType,
}

impl NumberEffect {
    ///Font spacing and font size are automatic based on value, this system is very much not general use
    pub fn new(num_effect_type: NumberEffectType, value: i64, pos_rect: Rectangle, font: &Font) -> Self {
        let (font_size, font_spacing) = match value {
            0..=10 => (5.0, 0.0),
            11..=100 => (10.0, 1.0),
            101.. => (15.0, 2.0),
        };

        let mut rng = rand::rng();
        let value_as_str = value.to_string();
        let text_size = font.measure_text(&value_as_str, font_size, font_spacing);
        let start_pos_y = pos_rect.y + pos_rect.width - text_size.y;
        let start_pos_x = pos_rect.x + (pos_rect.width / 2.0) - (text_size.x / 2.0);
        let pos = Vector2::new(start_pos_x, start_pos_y);

        let effect = match num_effect_type {
            NumberEffectType::Damage => {
                let vel_x: f32 = rng.random_range(-5.0..=5.0);
                let vel_y: f32 = rng.random_range(-60.0..=-30.0);

                let acc_x: f32 = rng.random_range(-2.0..=2.0);
                let acc_y: f32 = GRAVITY;

                NumberEffect {
                    num_effect_type,
                    value_as_str,
                    pos,
                    velocity: Vector2::new(vel_x, vel_y),
                    acceleration: Vector2::new(acc_x, acc_y),
                    color: Color::WHITE,
                    vertical_sine_wave: false,
                    font_size,
                    font_spacing,
                    start_pos_y,
                    lifespan: 2.0,
                }
            }
            NumberEffectType::Heal => {
                let vel_x: f32 = 0.0;
                let vel_y: f32 = -15.0;

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
                    start_pos_y,
                    lifespan: 2.0,
                    vertical_sine_wave: true,
                    num_effect_type,
                }
            }
            NumberEffectType::Block => {
                let vel_x: f32 = rng.random_range(-5.0..=1.0);
                let vel_y: f32 = rng.random_range(-30.0..=-15.0);

                let acc_x: f32 = rng.random_range(-2.0..=1.0);
                let acc_y: f32 = GRAVITY;

                NumberEffect {
                    value_as_str,
                    pos,
                    velocity: Vector2::new(vel_x, vel_y),
                    acceleration: Vector2::new(acc_x, acc_y),
                    font_size,
                    font_spacing,
                    color: Color::SANDYBROWN,
                    start_pos_y,
                    lifespan: 2.0,
                    vertical_sine_wave: false,
                    num_effect_type,
                }
            }
        };

        return effect;
    }
}

//new makes new and sets these fields, should center x pos ideally
//update adds vel to pos and acc to vel, and if sin_wave,
//adds '(dt * sin(total_game_time)) * magnitude' to vel.x
//draw simply draws number at pos in color of color field
