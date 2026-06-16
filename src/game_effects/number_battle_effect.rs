use raylib::{color::Color, math::Vector2};

pub enum NumberEffectType {
    Damage,
    Heal,
    Block,
}

pub struct NumberEffect {
    num_effect_type: NumberEffectType,
    value_as_str: String,
    pos: Vector2,
    velocity: Vector2,
    Acceleration: Vector2,
    color: Color,
    sine_wave: bool,
}

//new makes new and sets these fields, should center x pos ideally
//update adds vel to pos and acc to vel, and if sin_wave, 
//adds '(dt * sin(total_game_time)) * magnitude' to vel.x
//draw simply draws number at pos in color of color field
