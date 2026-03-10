use super::input_handler::{InputState, MouseState::*};
use basic_raylib_core::graphics::sprite::Sprite;
use raylib::prelude::*;

pub struct Button {
    rect: Rectangle,
    sprite: Sprite,
    down_sprite: Sprite,
    text: Option<&'static str>,
    text_draw_offset: Option<Vector2>,
    down: bool,
}

impl Button {
    //rect is the destination rectangle onto the screen
    pub fn new(rect: Rectangle, sprite: Sprite, down_sprite: Sprite, text: Option<&'static str>, text_draw_offset: Option<Vector2>) -> Self {
        Button { rect, sprite, down_sprite, text, text_draw_offset, down: false }
    }

    pub fn is_pressed(&mut self, input_state: &InputState) -> bool {
        let is_clicked =
            self.rect.check_collision_point_rec(input_state.mouse_pos) && input_state.mouse_state == Clicked;
        
        self.down = is_clicked;

        if is_clicked {
            return true;
        } else {
            return false;
        }
    }
    
    pub fn draw_with_text(&mut self, d: &mut RaylibDrawHandle, sprite_sheet: &Texture2D, font: &Font) {
        
        let pos = Vector2::new(self.rect.x, self.rect.y);
        
        if self.down {
            self.down_sprite.draw(d, pos, sprite_sheet);
        } else {
            self.sprite.draw(d, pos, sprite_sheet);
        }
        
        if let Some(text) = self.text {
            let offset = self.text_draw_offset.unwrap();
            d.draw_text_ex(font, text, pos + offset, 9.0, 0.5, Color::WHITE);
        }
    }
    
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, sprite_sheet: &Texture2D) {
        
        let pos = Vector2::new(self.rect.x, self.rect.y);
        
        if self.down {
            self.down_sprite.draw(d, pos, sprite_sheet);
        } else {
            self.sprite.draw(d, pos, sprite_sheet);
        }
    }
    
    pub fn reset(&mut self) {
        self.down = false;
    }
}
