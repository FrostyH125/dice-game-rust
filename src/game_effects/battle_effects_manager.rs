use raylib::{drawing::RaylibDrawHandle, math::Rectangle, text::Font, texture::Texture2D};

use crate::game_effects::{
    battle_effect::{BattleEffect, BattleEffectType},
    number_battle_effect::{self, NumberEffect, NumberEffectType},
};

///Responsible for slashes and explosions before and after attacks.
///Also responsible for damage, blocking, and healing numbers after an action
pub struct BattleEffectsManager {
    battle_effects: Vec<BattleEffect>,
    number_effects: Vec<NumberEffect>,
}

impl BattleEffectsManager {
    pub fn new() -> Self {
        return BattleEffectsManager {
            battle_effects: Vec::new(),
            number_effects: Vec::new(),
        };
    }

    pub fn add_effect(&mut self, effect_type: BattleEffectType, target_pos_rect: Rectangle) {
        let effect = BattleEffect::new(effect_type, target_pos_rect);
        self.battle_effects.push(effect);
    }

    pub fn add_number_effect(&mut self, num_effect: NumberEffect) {
        self.number_effects.push(num_effect);
    }

    pub fn update(&mut self, dt: f32, total_time: f32) {
        for i in (0..self.battle_effects.len()).rev() {
            let effect = &mut self.battle_effects[i];
            
            effect.update(dt);
            
            if effect.is_done() {
                self.battle_effects.remove(i);
            }

        }

        for i in (0..self.number_effects.len()).rev() {
            let num_effect = &mut self.number_effects[i];

            num_effect.update(dt, total_time);

            if num_effect.is_done() {
                self.number_effects.remove(i);
            }
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        for effect in &mut self.battle_effects {
            effect.draw(d, texture);
        }

        for num_effect in &mut self.number_effects {
            num_effect.draw(d, font);
            println!("my current positonn x: {}, y: {}", num_effect.pos.x, num_effect.pos.y);
        }
    }
}
