use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{math::Vector2, prelude::RaylibDrawHandle, texture::Texture2D};

use crate::{entities::{confirm_button::ConfirmButton, dice::{Dice, ROLL_ANIM}, dice_box::{self, DiceBoxData, DiceBoxState}}, system::{button::{self, Button}, input_handler::InputState}};

static ATTACK_DICE_BOX_SPRITE: Sprite = Sprite::new(14.0, 80.0, 52.0, 16.0);

pub struct AttackDiceBox {
    pub data: DiceBoxData,
}

impl AttackDiceBox {

    pub fn new(pos: Vector2) -> Self {
        AttackDiceBox { 
            data: DiceBoxData::new(pos)
        }
    }

    pub fn update(&mut self, dice_in_hand: &mut Vec<Dice>, input_state: &InputState, confirm_button: &mut ConfirmButton, dt: f32) {
        dice_box::update(&mut self.data, dice_in_hand, input_state, confirm_button, dt);

        if self.data.state == DiceBoxState::Acting {
            self.Attack();
        }
        
        //dice_box::reset_box(&mut self.data, dice_in_hand);
        
        println!("number of dice in box: {}", self.data.dice_in_box.len());
    }
    
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        dice_box::draw(d, texture, &mut self.data, &ATTACK_DICE_BOX_SPRITE);
    }
    
    fn Attack(&self) {
        
    }
}
