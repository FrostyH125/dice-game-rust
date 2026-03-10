use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::Font,
    texture::Texture2D,
};

use crate::{
    entities::{
        dice::{DICE_WIDTH_HEIGHT, DiceState},
        dice_box_data::{DICE_BORDER_OFFSET, DiceBoxData},
    },
    system::{info_hover::InfoHover, input_handler::InputState},
};

static PLACEHOLDER_DICE_SPRITE: Sprite = Sprite::new(80.0, 128.0, DICE_WIDTH_HEIGHT, DICE_WIDTH_HEIGHT);
static SNAKE_EYES_DICE_BOX_SPRITE: Sprite = Sprite::new(14.0, 128.0, 36.0, 16.0);
const SNAKE_EYES_DICE_DRAW_START_OFFSET: Vector2 = Vector2 { x: 2.0 + DICE_WIDTH_HEIGHT, y: -15.0 };
const SNAKE_EYES_TEXT_OFFSET: Vector2 = Vector2 { x: 40.0, y: -15.0 };
const SNAKE_EYES_DAMAGE_DRAW_OFFSET: Vector2 = Vector2 { x: 40.0, y: -5.0 };

pub struct SnakeEyes {
    pub data: DiceBoxData,
    pub info_hover: InfoHover,
}

impl SnakeEyes {
    pub fn new(pos: Vector2, font: &Font) -> Self {
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
            info_hover: InfoHover::new(
                "Snake Eyes:
                Deals 11 base damage when loaded with two dice of value 1",
                Rectangle {
                    x: pos.x,
                    y: pos.y,
                    width: SNAKE_EYES_DICE_BOX_SPRITE.src_rect.width,
                    height: SNAKE_EYES_DICE_BOX_SPRITE.src_rect.height,
                },
                font,
                5.0,
                0.5,
            ),
        }
    }

    pub fn update(&mut self, input: &InputState, dt: f32) {
        self.info_hover.update(input, dt);
        for dice in &mut self.data.dice_in_box {
            dice.update_for_enemy(dt);
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        SNAKE_EYES_DICE_BOX_SPRITE.draw(d, self.data.pos, texture);

        if self.data.total_value_for_current_round != 0 {
            self.draw_snake_eyes_text(d, font);
            self.draw_damage(d, font);
            self.draw_dice_outlines(d, texture);
        }        
        
        self.snake_eyes_draw_dice(d, texture);
        self.draw_placeholder_dice(d, texture);
        self.info_hover.draw(d, font, texture);
    }

    pub fn tally_snake_eyes(&mut self) -> i64 {
        let mut num_of_ones = 0;

        for dice in &self.data.dice_in_box {
            if dice.value == 1 {
                num_of_ones += 1;
            }
        }

        if num_of_ones >= 2 {
            return 11;
        } else {
            return 0;
        }
    }

    pub fn snake_eyes_set_dice_positions(&mut self) {
        let mut target_pos = self.data.pos + SNAKE_EYES_DICE_DRAW_START_OFFSET; //can only be 0, 1 or 2
        for i in 0..self.data.dice_in_box.len() {
            let old_pos = self.data.dice_in_box[i].pos;
            self.data.dice_in_box[i].state = DiceState::Rearranging { old_pos, target_pos, should_roll_after: false };
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

    fn draw_snake_eyes_text(&self, d: &mut RaylibDrawHandle, font: &Font) {
        d.draw_text_ex(font, "Snake Eyes!", self.data.pos + SNAKE_EYES_TEXT_OFFSET, 5.0, 0.0, Color::FORESTGREEN);
    }

    fn draw_damage(&self, d: &mut RaylibDrawHandle, font: &Font) {

        d.draw_text_ex(
            font,
            &format!("{} damage!", self.data.total_value_for_current_round),
            self.data.pos + SNAKE_EYES_DAMAGE_DRAW_OFFSET,
            5.0,
            0.0,
            Color::FORESTGREEN,
        );
    }

    fn draw_dice_outlines(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {

        for i in 0..self.data.dice_in_box.len() {
            let dice = &self.data.dice_in_box[i];
            let sprite = dice.kind.outline_sprite();

            sprite.draw(d, dice.pos + DICE_BORDER_OFFSET, texture);
        }
    }
}
