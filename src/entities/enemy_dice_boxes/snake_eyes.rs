use basic_raylib_core::graphics::sprite::Sprite;
use raylib::math::{Rectangle, Vector2};

use crate::entities::{
    dice::{DICE_WIDTH_HEIGHT, Dice},
    dice_box_data::DiceBoxData,
    enemy::snake::Snake,
};

static SNAKE_EYES_DICE_BOX: Sprite = Sprite::new(14.0, 128.0, 36.0, 16.0);

pub struct SnakeEyes {
    data: DiceBoxData,
}

impl SnakeEyes {
    pub fn new(pos: Vector2) -> Self {
        SnakeEyes {
            data: DiceBoxData::new(
                pos,
                Rectangle {
                    x: pos.x + 2.0,
                    y: pos.y - DICE_WIDTH_HEIGHT,
                    width: DICE_WIDTH_HEIGHT * 2.0,
                    height: DICE_WIDTH_HEIGHT,
                },
            ),
        }
    }

    pub fn update(&mut self, dice_in_hand: &mut Vec<Dice>, dt: f32) {
        // waiting for dice: if dice is not 1, return to hand
        // if dice.len > 2, return dice to hand
        // tallying, if contains 2 1s, tally = 25, get value, waiting for action
        // if empty, inactive
        // controller will take over on waiting for action and do what they want with the value
    }

    pub fn draw() {
        // draw_box()
        // draw tally()
        // draw_snake_eyes()
        // draw dice in box
        // draw placeholder dice in box in both spots, unless either spot is taken up by a dice
    }
}
