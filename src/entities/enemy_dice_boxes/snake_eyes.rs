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
        dice_box_data::{DiceBoxData, DiceBoxState},
    },
    system::{info_hover::InfoHover, input_handler::InputState},
};

static PLACEHOLDER_DICE_SPRITE: Sprite = Sprite::new(64.0, 128.0, DICE_WIDTH_HEIGHT, DICE_WIDTH_HEIGHT);
static SNAKE_EYES_DICE_BOX_SPRITE: Sprite = Sprite::new(14.0, 128.0, 36.0, 16.0);
const SNAKE_EYES_DICE_DRAW_START_OFFSET: Vector2 = Vector2 { x: 2.0 + DICE_WIDTH_HEIGHT, y: -15.0 };

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
                Deals 11 damage when loaded with two dice of value 1",
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
        self.snake_eyes_set_dice_positions();
        match self.data.state {
            DiceBoxState::WaitingForDice => {}
            DiceBoxState::TallyingPoints => {
                self.data.total_value_for_current_round = self.tally_snake_eyes();
                self.data.state = DiceBoxState::WaitingForAction;
            }
            DiceBoxState::WaitingForAction => (),
            DiceBoxState::Inactive => (),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        SNAKE_EYES_DICE_BOX_SPRITE.draw(d, self.data.pos, texture);
        self.draw_placeholder_dice(d, texture);
        
        match self.data.state {
            DiceBoxState::Inactive => {
                self.draw_placeholder_dice(d, texture);
            }
            _ => {
                SNAKE_EYES_DICE_BOX_SPRITE.draw(d, self.data.pos, texture);
                d.draw_rectangle_lines(
                    self.data.dice_collect_rect.x as i32,
                    self.data.dice_collect_rect.y as i32,
                    self.data.dice_collect_rect.width as i32,
                    self.data.dice_collect_rect.height as i32,
                    Color::WHITE,
                );
                self.snake_eyes_draw_dice(d, texture);
            }
        }

        self.info_hover.draw(d, font, texture);
        // draw tally()
        // draw_snake_eyes()
    }

    pub fn tally_snake_eyes(&self) -> i64 {
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
        let mut pos = self.data.pos + SNAKE_EYES_DICE_DRAW_START_OFFSET;

        //can only be 0, 1 or 2
        for i in 0..self.data.dice_in_box.len() {
            if self.data.dice_in_box[i].state != DiceState::Dragging {
                self.data.dice_in_box[i].pos = pos;
            }
            pos.x -= DICE_WIDTH_HEIGHT;
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

    fn draw_snake_eyes_text() {}

    fn draw_11_if_score() {}
}
