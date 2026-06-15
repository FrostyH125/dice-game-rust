use raylib::{drawing::RaylibDrawHandle, math::Rectangle, texture::Texture2D};

use crate::game_effects::battle_effect::{BattleEffect, BattleEffectType};

pub struct BattleEffectsManager {
    battle_effects: Vec<BattleEffect>,
}

impl BattleEffectsManager {
    pub fn new() -> Self {
        return BattleEffectsManager { battle_effects: Vec::new() };
    }
    
    pub fn add_effect(&mut self, effect_type: BattleEffectType, target_pos_rect: Rectangle) {
        let effect = BattleEffect::new(effect_type, target_pos_rect);
        self.battle_effects.push(effect);
    }

    pub fn update(&mut self, dt: f32) {
        for i in (0..self.battle_effects.len()).rev() {
            let effect = &mut self.battle_effects[i];
            
            effect.update(dt);
            
            if effect.is_done() {
                self.battle_effects.remove(i);
            }
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        for effect in &mut self.battle_effects {
            effect.draw(d, texture);
        }
    }
}

