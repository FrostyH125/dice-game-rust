use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{color::Color, math::{Rectangle, Vector2}, prelude::{RaylibDraw, RaylibDrawHandle}, text::Font, texture::Texture2D};

use crate::entities::{
    dice::{DICE_WIDTH_HEIGHT, Dice},
    dice_box_data::{DiceBoxData, DiceBoxState},
};

static PLACEHOLDER_DICE_SPRITE: Sprite = Sprite::new( 64.0, 128.0, DICE_WIDTH_HEIGHT, DICE_WIDTH_HEIGHT);
static SNAKE_EYES_DICE_BOX_SPRITE: Sprite = Sprite::new(14.0, 128.0, 36.0, 16.0);
const SNAKE_EYES_DICE_DRAW_START_OFFSET: Vector2 = Vector2 { x: 2.0 + DICE_WIDTH_HEIGHT,  y: -15.0};
const SNAKE_EYES_PLACEHOLDER_DICE_DRAW_START_OFFSET: Vector2 = Vector2 { x: 2.0, y: -15.0};

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

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        match self.data.state {
            DiceBoxState::Inactive => return,
            _ => {
                SNAKE_EYES_DICE_BOX_SPRITE.draw(d, self.data.pos, texture);
                d.draw_rectangle_lines(
                    self.data.dice_collect_rect.x as i32,
                    self.data.dice_collect_rect.y as i32,
                    self.data.dice_collect_rect.width as i32,
                    self.data.dice_collect_rect.height as i32,
                    Color::WHITE,
                );
                self.draw_dice(d, texture);
                self.draw_placeholder_dice(d, texture);
            }
        }
        
        // draw tally()
        // draw_snake_eyes()
        
    }
    
    fn draw_dice(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let mut draw_pos = self.data.pos + SNAKE_EYES_DICE_DRAW_START_OFFSET;
        
        //can only be 0, 1 or 2
        for i in 0..self.data.dice_in_box.len() {
            self.data.dice_in_box[i].draw(d, texture);
            draw_pos.x -= DICE_WIDTH_HEIGHT;
        }
    }
    
    fn draw_placeholder_dice(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let mut start_pos = self.data.pos + SNAKE_EYES_DICE_DRAW_START_OFFSET;
        
        // the reverse of drawing the dice
        // if len is 1, this will only draw once, starting from the opposite side
        for _ in 0..2 - self.data.dice_in_box.len() {
            PLACEHOLDER_DICE_SPRITE.draw(d, start_pos, texture);
            start_pos.x -= DICE_WIDTH_HEIGHT;
        }
    }
    
    fn draw_snake_eyes_text() {
        
    }
    
    fn draw_25_if_score() {
        
    }
}
