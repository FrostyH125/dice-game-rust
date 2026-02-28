use crate::entities::dice::DICE_WIDTH_HEIGHT;
use crate::entities::hand::DICE_Y_OFFSET;
use basic_raylib_core::graphics::sprite::Sprite;
use raylib::color::Color;
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::{RaylibDraw, RaylibDrawHandle};
use raylib::text::Font;
use raylib::texture::Texture2D;

use crate::{
    VIRTUAL_HEIGHT, VIRTUAL_WIDTH,
    system::{button::Button, input_handler::InputState},
};

const CONFIRM_BUTTON_WIDTH: f32 = 64.0;
const CONFIRM_BUTTON_HEIGHT: f32 = 32.0;
static CONFIRM_BUTTON_SPRITE: Sprite = Sprite::new(144.0, 16.0, 64.0, 32.0);
static CONFIRM_BUTTON_DOWN_SPRITE: Sprite = Sprite::new(144.0, 48.0, 64.0, 32.0);

pub struct ConfirmButton {
    pub button: Button,
    pos: Vector2,
    down: bool,
}

impl ConfirmButton {
    pub fn new() -> Self {
        ConfirmButton {
            button: Button::new(Rectangle {
                x: VIRTUAL_WIDTH / 2.0,
                y: VIRTUAL_HEIGHT - DICE_Y_OFFSET + DICE_WIDTH_HEIGHT + 8.0,
                width: CONFIRM_BUTTON_WIDTH,
                height: CONFIRM_BUTTON_HEIGHT,
            }),
            pos: Vector2 {
                x: VIRTUAL_WIDTH / 2.0,
                y: VIRTUAL_HEIGHT - DICE_Y_OFFSET + DICE_WIDTH_HEIGHT + 8.0,
            },
            down: false,
        }
    }

    pub fn is_pressed(&mut self, input_state: &InputState) -> bool {
        self.down = self.button.is_pressed(input_state);
        
        return self.down;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        match self.down {
            true => CONFIRM_BUTTON_DOWN_SPRITE.draw(d, self.pos, texture),
            false => CONFIRM_BUTTON_SPRITE.draw(d, self.pos, texture)
        }
        
        d.draw_text_ex(font, "TALLY", self.pos + Vector2 { x: 5.0, y: 10.0 }, 10.0, 0.5, Color::WHITE);
    }
    
    pub fn reset(&mut self) {
        self.down = false;
    }
}
