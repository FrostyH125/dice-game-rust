use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{
    color::Color, math::{Rectangle, Vector2}, prelude::RaylibDrawHandle, text::Font, texture::Texture2D
};

use crate::{
    GameContext, entities::{
        dice::{DICE_WIDTH_HEIGHT, DiceState},
        dice_box_data::{DICE_BORDER_OFFSET, DiceBoxData},
    }, system::info_hover::InfoHover
};

static PLACEHOLDER_DICE_SPRITE: Sprite = Sprite::new(80, 160, DICE_WIDTH_HEIGHT as u32, DICE_WIDTH_HEIGHT as u32);
static SNAKE_EYES_DICE_BOX_SPRITE: Sprite = Sprite::new(14, 160, 36, 16);
const SNAKE_EYES_DICE_DRAW_START_OFFSET: Vector2 = Vector2 { x: 2.0 + DICE_WIDTH_HEIGHT, y: -15.0 };

pub struct SnakeEyes {
    pub data: DiceBoxData,
}

impl SnakeEyes {
    pub fn new(font: &Font) -> Self {
        
        let collect_rect_offset_x = 2.0;
        let collect_rect_offset_y = -16.0;
        let collect_rect_width = 32.0;
        let collect_rect_height = 16.0;
        let dice_box_width = 38.0;
        let dice_box_height = 16.0;
        
        SnakeEyes {
            data: DiceBoxData::new(
                collect_rect_offset_x,
                collect_rect_offset_y,
                collect_rect_width,
                collect_rect_height,
                dice_box_width,
                dice_box_height,
                InfoHover::new(
                    "Snake Eyes:
                Deals 11 base damage when loaded with two dice of value 1",
                    Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: SNAKE_EYES_DICE_BOX_SPRITE.src_rect.width,
                        height: SNAKE_EYES_DICE_BOX_SPRITE.src_rect.height,
                    },
                    font,
                    5.0,
                    0.5,
                ),
                Color::DARKOLIVEGREEN
            ),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, game_context: &GameContext) {
        SNAKE_EYES_DICE_BOX_SPRITE.draw(d, self.data.pos, &game_context.texture);

        if self.data.total_points != 0.0f64.floor() {
            self.draw_dice_outlines(d, &game_context.texture);
        }

        self.snake_eyes_draw_dice(d, &game_context.texture);
        self.draw_placeholder_dice(d, &game_context.texture);
    }

    pub fn check_if_two_ones(&mut self) -> bool {
        let mut num_of_ones = 0;

        for dice in &self.data.dice_in_box {
            if dice.value == 1 {
                num_of_ones += 1;
            }
        }

        if num_of_ones >= 2 {
            self.data.tally = 11.0;
            return true;
        } else {
            self.data.tally = 0.0f64.floor();
            return false;
        }
    }

    pub fn snake_eyes_set_dice_positions(&mut self) {
        let mut target_pos = self.data.pos + SNAKE_EYES_DICE_DRAW_START_OFFSET; //can only be 0, 1 or 2
        for i in 0..self.data.dice_in_box.len() {
            let old_pos = self.data.dice_in_box[i].pos;
            self.data.dice_in_box[i].state = DiceState::Rearranging {
                old_pos,
                target_pos,
                should_roll_after: false,
            };
            target_pos.x -= DICE_WIDTH_HEIGHT;
        }
    }

    fn snake_eyes_draw_dice(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        for i in 0..self.data.dice_in_box.len() {
            self.data.dice_in_box[i].draw(d, texture);
        }
    }

    fn draw_placeholder_dice(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let mut start_pos =
            self.data.pos + SNAKE_EYES_DICE_DRAW_START_OFFSET + Vector2 { x: -DICE_WIDTH_HEIGHT, y: 0.0 };

        // the reverse of drawing the dice
        // if len is 1, this will only draw once, starting from the opposite side
        for _ in 0..2 - self.data.dice_in_box.len() {
            PLACEHOLDER_DICE_SPRITE.draw(d, start_pos, texture);
            start_pos.x += DICE_WIDTH_HEIGHT;
        }
    }

    fn draw_dice_outlines(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        for i in 0..self.data.dice_in_box.len() {
            let dice = &self.data.dice_in_box[i];
            let sprite = dice.kind.outline_sprite();

            sprite.draw(d, dice.pos + DICE_BORDER_OFFSET, texture);
        }
    }
}
