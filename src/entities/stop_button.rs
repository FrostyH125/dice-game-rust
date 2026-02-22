use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{math::{Rectangle, Vector2}, prelude::RaylibDrawHandle, texture::Texture2D};

use crate::{
    VIRTUAL_HEIGHT, VIRTUAL_WIDTH,
    entities::{dice::DICE_WIDTH_HEIGHT, hand::DICE_Y_OFFSET},
    system::{button::Button, input_handler::InputState},
};

const STOP_BUTTON_WIDTH: f32 = 64.0;
const STOP_BUTTON_HEIGHT: f32 = 32.0;
static STOP_BUTTON_DEFAULT_SPRITE: Sprite = Sprite::new(16.0, 16.0, 64.0, 32.0);
static STOP_BUTTON_CLICK_SPRITE: Sprite = Sprite::new(16.0, 48.0, 64.0, 32.0);

pub struct StopButton {
    pub button: Button,
    pos: Vector2,
    down: bool,
}

impl StopButton {
    pub fn new() -> Self {
        StopButton {
            button: Button::new(Rectangle {
                x: VIRTUAL_WIDTH / 2.0 - STOP_BUTTON_WIDTH - 5.0,
                y: VIRTUAL_HEIGHT - DICE_Y_OFFSET + DICE_WIDTH_HEIGHT + 8.0,
                width: STOP_BUTTON_WIDTH,
                height: STOP_BUTTON_HEIGHT,
            }),
            pos: Vector2 {
                x: VIRTUAL_WIDTH / 2.0 - STOP_BUTTON_WIDTH - 5.0,
                y: VIRTUAL_HEIGHT - DICE_Y_OFFSET + DICE_WIDTH_HEIGHT + 8.0,
            },
            down: false,
        }
    }
    
    pub fn is_pressed(&mut self, input_state: &InputState) -> bool {
        self.down = self.button.is_pressed(input_state);
        
        return self.down;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        match self.down {
            true => STOP_BUTTON_CLICK_SPRITE.draw(d, self.pos, texture),
            false => STOP_BUTTON_DEFAULT_SPRITE.draw(d, self.pos, texture)
        }
    }
    
    pub fn reset(&mut self) {
        self.down = false;
    }
}
