use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{math::Vector2, prelude::RaylibDrawHandle, texture::Texture2D};

use crate::{entities::{confirm_button::ConfirmButton, dice::{Dice, ROLL_ANIM}, dice_box::{self, DiceBoxData, DiceBoxState, reset_box}}, system::{button::{self, Button}, input_handler::InputState}};

static ATTACK_DICE_BOX_SPRITE: Sprite = Sprite::new(14.0, 80.0, 52.0, 16.0);
const TOTAL_TIME_FOR_ATTACK: f32 = 1.0;

pub struct AttackDiceBox {
    pub data: DiceBoxData,
    player_damage_this_turn: i64,
    attack_timer: f32
}

impl AttackDiceBox {

    pub fn new(pos: Vector2) -> Self {
        AttackDiceBox { 
            data: DiceBoxData::new(pos),
            player_damage_this_turn: 0,
            attack_timer: 0.0
        }
    }

    pub fn update(&mut self, dice_in_hand: &mut Vec<Dice>, input_state: &InputState, confirm_button: &mut ConfirmButton, dt: f32) {
        dice_box::update(&mut self.data, dice_in_hand, input_state, confirm_button, dt);

        if self.data.state == DiceBoxState::Acting {
            if self.attack(dt) {
                dice_box::reset_box(&mut self.data, dice_in_hand);
            }
        }
    }
    
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        dice_box::draw(d, texture, &mut self.data, &ATTACK_DICE_BOX_SPRITE);
    }
    
    fn attack(&mut self, dt: f32) -> bool {
        self.attack_timer += dt;
        
        if self.data.total_tally == 0 {
            return true;
        }
        
        if self.attack_timer >= TOTAL_TIME_FOR_ATTACK / 3.0 {
            self.player_damage_this_turn = self.data.total_tally;  
            println!("player damage accumulated (tally): {}", self.player_damage_this_turn);
        }
        
        if self.attack_timer >= (TOTAL_TIME_FOR_ATTACK / 3.0) * 2.0 {
            self.player_damage_this_turn = self.data.total_tally * self.data.base_multi_for_this_dice_box;
            println!("player damage accumulated (tally * base multi): {}", self.player_damage_this_turn);
        }
        
        if self.attack_timer >= TOTAL_TIME_FOR_ATTACK {
            self.player_damage_this_turn = self.data.total_tally * self.data.base_multi_for_this_dice_box * self.data.total_multi_for_this_tally;
            println!("player damage accumulated (tally * base multi * streak multi): {}", self.player_damage_this_turn);
            return true;
        }
        
        
        return false;
        
    }
}
