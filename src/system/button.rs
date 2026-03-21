use super::input_handler::{InputState, MouseState::*};
use basic_raylib_core::graphics::sprite::Sprite;
use raylib::prelude::*;

pub struct Button {
    rect: Rectangle,
    sprite: Sprite,
    down_sprite: Sprite,
    down_clicked_sprite: Sprite,
    text: Option<&'static str>,
    text_draw_offset: Option<Vector2>,
    inactive: bool,
}

impl Button {
    //rect is the destination rectangle onto the screen
    pub fn new(
        rect: Rectangle,
        sprite: Sprite,
        down_sprite: Sprite,
        down_clicked_sprite: Sprite,
        text: Option<&'static str>,
        text_draw_offset: Option<Vector2>,
    ) -> Self {
        Button {
            rect,
            sprite,
            down_sprite,
            down_clicked_sprite,
            text,
            text_draw_offset,
            inactive: false,
        }
    }

    pub fn is_pressed(&mut self, input_state: &InputState) -> bool {
        let is_clicked =
            self.rect.check_collision_point_rec(input_state.mouse_pos) && matches!(input_state.mouse_state, Clicked);

        return is_clicked;
    }


    pub fn draw_with_text(&mut self, d: &mut RaylibDrawHandle, sprite_sheet: &Texture2D, font: &Font, input_state: &InputState) {
        let pos = Vector2::new(self.rect.x, self.rect.y);

        match self.inactive {
            true => match self.is_pressed(input_state) {
                true => self.down_clicked_sprite.draw(d, pos, sprite_sheet),
                false => self.down_sprite.draw(d, pos, sprite_sheet),
            },
            false => self.sprite.draw(d, pos, sprite_sheet)
        }

        if let Some(text) = self.text {
            let offset = self.text_draw_offset.unwrap();
            d.draw_text_ex(font, text, pos + offset, 9.0, 0.5, Color::WHITE);
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, sprite_sheet: &Texture2D, input_state: &InputState) {
        let pos = Vector2::new(self.rect.x, self.rect.y);
        
        match self.inactive {
            true => match self.is_pressed(input_state) {
                true => self.down_clicked_sprite.draw(d, pos, sprite_sheet),
                false => self.down_sprite.draw(d, pos, sprite_sheet),
            },
            false => self.sprite.draw(d, pos, sprite_sheet)
        }
    }
    
    // purely for visual effect
    pub fn reset(&mut self) {
        self.inactive = false;
    }
    
    // purely for visual effect
    // really, this method was born out of having an annoying time dealing with the timing of the buttons
    // much easier to just manually deactivate it from the code calling it, and then manually reset it, instead of having the 
    // pressing method do two things at once and sometimes yield results you dont want (unfortunate, because it really
    // should be simple, so maybe a skill issue)
    pub fn deactivate(&mut self) {
        self.inactive = true;
    }
}
