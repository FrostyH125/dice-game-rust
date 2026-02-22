use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{math::Vector2, prelude::RaylibDrawHandle, texture::Texture2D};

use crate::{entities::{dice::{Dice}, dice_box::{self, DiceBoxData}}};

static ATTACK_DICE_BOX_SPRITE: Sprite = Sprite::new(14.0, 80.0, 52.0, 16.0);
const TOTAL_TIME_FOR_ATTACK: f32 = 1.0;

pub struct AttackDiceBox {
    pub data: DiceBoxData,
    player_damage_this_turn: i64,
    
    // maybe add these fields to diceboxdata later to avoid repetition
    // have them be reset by dice_box::reset()
    attack_timer: f32,
    added_tally: bool,
    added_base_multi: bool,
    added_streak_multi: bool
}

impl AttackDiceBox {
    pub fn new() -> Self {
        AttackDiceBox { 
            data: DiceBoxData::new(Vector2 { x: 5.0, y: 100.0}),
            player_damage_this_turn: 0,
            attack_timer: 0.0,
            added_tally: false,
            added_base_multi: false,
            added_streak_multi: false,
        }
    }

    pub fn update(&mut self, dice_in_hand: &mut Vec<Dice>,  dt: f32) {
        dice_box::update(&mut self.data, dice_in_hand, dt);
    }
    
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        dice_box::draw(d, texture, &mut self.data, &ATTACK_DICE_BOX_SPRITE);
    }
    
    pub fn reset(&mut self, hand_dice: &mut Vec<Dice>) {
        self.attack_timer = 0.0;
        self.added_tally = false;
        self.added_base_multi = false;
        self.added_streak_multi = false;
        dice_box::reset_box(&mut self.data, hand_dice);
    }
}
