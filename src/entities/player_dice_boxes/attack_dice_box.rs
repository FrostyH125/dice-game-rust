use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::Font,
    texture::Texture2D,
};

use crate::{entities::{
    dice::Dice,
    dice_box_data::{BASE_MULTI_OFFSET, CURRENT_MULTI_OFFSET, CURRENT_STREAK_OFFSET, DiceBoxData, DiceBoxState}, hand::Hand,
}, system::input_handler::InputState};

static ATTACK_DICE_BOX_SPRITE: Sprite = Sprite::new(14.0, 80.0, 52.0, 16.0);

pub struct AttackDiceBox {
    pub data: DiceBoxData,
}

impl AttackDiceBox {
    pub fn new() -> Self {
        let pos = Vector2 { x: 5.0, y: 100.0 };

        AttackDiceBox {
            data: DiceBoxData::new(
                pos,
                Rectangle {
                    x: pos.x + 2.0,
                    y: pos.y - 31.0,
                    width: 48.0,
                    height: 32.0,
                },
            ),
        }
    }

    pub fn update(&mut self, is_player_dragging_any_dice: &mut bool, hand: &mut Hand, input_state: &InputState, dt: f32) {
        match self.data.state {
            //player will set this when start turn
            DiceBoxState::WaitingForDice => {
                self.data.check_for_dice_being_dragged_into_box(&mut hand.dice);
                self.data.update_dice(is_player_dragging_any_dice, hand, input_state, dt);
                self.data.set_dice_positions();
            }

            // will be set to this when the confirm button is pressed and
            // the player decides when to tally the points of each box
            DiceBoxState::TallyingPoints => {
                if self.data.dice_in_box.is_empty() {
                    self.data.state = DiceBoxState::Inactive;
                } else if self.data.tally_points(dt) {
                    self.data.total_value_for_current_round = self.data.get_value();
                    self.data.state = DiceBoxState::WaitingForAction;
                }
            }

            // player will act using the data inside of this box
            // may in the future add a function in this class for "Attack()"
            // Need to figure out how to connect player and enemy first
            DiceBoxState::WaitingForAction => (),

            // box is not drawn or updating
            DiceBoxState::Inactive => (),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        match self.data.state {
            DiceBoxState::Inactive => return,
            _ => {
                ATTACK_DICE_BOX_SPRITE.draw(d, self.data.pos, texture);
                d.draw_rectangle_lines(
                    self.data.dice_collect_rect.x as i32,
                    self.data.dice_collect_rect.y as i32,
                    self.data.dice_collect_rect.width as i32,
                    self.data.dice_collect_rect.height as i32,
                    Color::WHITE,
                );
                self.data.draw_dice(d, texture);
                self.draw_multi(d, font);
                self.draw_base_multi(d, font);
                self.draw_current_streak(d, font);
                self.data.draw_border_around_current_dice(d, texture);
                //draw multi, base multi, current streak, border around dice, arrow pointing to dice
            }
        }
    }

    fn draw_multi(&self, d: &mut RaylibDrawHandle, font: &Font) {
        let multi = self.data.total_multi_for_this_tally;

        if multi <= 1 {
            return;
        }

        d.draw_text_ex(
            font,
            &format!("x {}", multi),
            self.data.pos + CURRENT_MULTI_OFFSET, //pos + offset
            10.0 + 2.0 * multi as f32,            // 15 base size + (2 X MULTI) size
            0.5,
            Color::RED,
        );
    }

    fn draw_base_multi(&self, d: &mut RaylibDrawHandle, font: &Font) {
        d.draw_text_ex(
            font,
            &format!("base: x{}", self.data.base_multi_for_this_dice_box),
            self.data.pos + BASE_MULTI_OFFSET,
            3.0,
            0.0,
            Color { r: 208, g: 184, b: 184, a: 255 },
        );
    }
    
    fn draw_current_tally(&self, d: &mut RaylibDrawHandle, font: &Font) {
        
    }

    fn draw_current_streak(&self, d: &mut RaylibDrawHandle, font: &Font) {
        
        let streak = self.data.current_streak;
        
        if streak <= 1 {
            return;
        }
        
        d.draw_text_ex(
            font,
            &format!("Streak {} !", streak),
            self.data.pos + CURRENT_STREAK_OFFSET,
            5.0,
            0.0,
            Color { r: 208, g: 184, b: 184, a: 255 },
        );
    }

    pub fn reset(&mut self, hand_dice: &mut Vec<Dice>) {
        self.data.reset_box(hand_dice);
    }
}
