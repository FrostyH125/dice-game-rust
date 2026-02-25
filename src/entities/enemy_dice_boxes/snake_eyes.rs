use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::Font,
    texture::Texture2D,
};

use crate::entities::{
    dice::{DICE_WIDTH_HEIGHT, Dice, DiceState},
    dice_box_data::{DICE_POINT_OFFSET_FOR_DETECTING_IF_INSIDE_BOX, DiceBoxData, DiceBoxState},
};

static PLACEHOLDER_DICE_SPRITE: Sprite = Sprite::new(64.0, 128.0, DICE_WIDTH_HEIGHT, DICE_WIDTH_HEIGHT);
static SNAKE_EYES_DICE_BOX_SPRITE: Sprite = Sprite::new(14.0, 128.0, 36.0, 16.0);
const SNAKE_EYES_DICE_DRAW_START_OFFSET: Vector2 = Vector2 { x: 2.0 + DICE_WIDTH_HEIGHT, y: -15.0 };
const SNAKE_EYES_PLACEHOLDER_DICE_DRAW_START_OFFSET: Vector2 = Vector2 { x: 2.0, y: -15.0 };

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
        match self.data.state {
            DiceBoxState::WaitingForDice => {
                self.snake_eyes_set_dice_positions();
            }
            DiceBoxState::TallyingPoints => {
                self.data.total_value_for_current_round = self.tally_snake_eyes();
                self.data.state = DiceBoxState::WaitingForAction;
            },
            DiceBoxState::WaitingForAction => (),
            DiceBoxState::Inactive => (),
        }
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
                self.snake_eyes_draw_dice(d, texture);
                self.draw_placeholder_dice(d, texture);
            }
        }

        // draw tally()
        // draw_snake_eyes()
    }
    
    fn tally_snake_eyes(&self) -> i64 {
        
        let mut num_of_ones = 0;
        
        for dice in &self.data.dice_in_box {
            if dice.value == 1 {
                num_of_ones += 1;
            }
        }
        
        if num_of_ones >= 2 {
            return 25;
        } else {
            return 0;
        }
    }
    
    fn check_for_two_dice_with_value_one_in_hand(dice_in_hand: &[Dice]) -> bool {
        
        let mut num_of_ones = 0;
        
        for dice in dice_in_hand {
            if dice.value == 1 {
                num_of_ones += 1;
            }
        }
        
        if num_of_ones >= 2 {
            return true;
        } else {
            return false;
        }
    }
    
    //this will only be for player usage
    fn snake_eyes_check_dice_dragging_into_box(&mut self, dice_in_hand: &mut Vec<Dice>) {
        for i in (0..dice_in_hand.len()).rev() {
            if dice_in_hand[i].state == DiceState::Stopped
                && self
                    .data
                    .dice_collect_rect
                    .check_collision_point_rec(dice_in_hand[i].pos + DICE_POINT_OFFSET_FOR_DETECTING_IF_INSIDE_BOX)
            {
                if dice_in_hand[i].value != 1 {
                    return;
                }
                let dice_to_add = dice_in_hand.remove(i);
                self.data.dice_in_box.push(dice_to_add);
                self.data.dice_in_box.sort_by(|a, b| a.value.cmp(&b.value));
            }
        }
    }

    fn snake_eyes_set_dice_positions(&mut self) {
        let mut pos = self.data.pos + SNAKE_EYES_DICE_DRAW_START_OFFSET;

        //can only be 0, 1 or 2
        for i in 0..self.data.dice_in_box.len() {
            self.data.dice_in_box[i].pos = Vector2 { x: pos.x, y: pos.y };
            pos.x -= DICE_WIDTH_HEIGHT;
        }
    }

    fn snake_eyes_draw_dice(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        for i in 0..self.data.dice_in_box.len() {
            self.data.dice_in_box[i].draw(d, texture);
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

    fn draw_snake_eyes_text() {}

    fn draw_25_if_score() {}
}
