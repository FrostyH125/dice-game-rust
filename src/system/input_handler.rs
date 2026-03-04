use raylib::prelude::{*, MouseButton::*};
use basic_raylib_core::graphics::sprite::Sprite;
use self::MouseState::*;

static MOUSE_SPRITE: Sprite = Sprite::new(0.0, 16.0, 16.0, 16.0);

#[derive(PartialEq, Debug)]
pub enum MouseState {
    Inactive,
    Clicked,
    Dragging,
}

pub struct InputState {
    pub mouse_pos: Vector2,
    pub click_pos: Vector2,
    pub mouse_state: MouseState,
}

impl InputState {
    pub fn update(&mut self, rl: &mut RaylibHandle, camera_zoom: f32) {
        
        self.mouse_pos = rl.get_mouse_position() / camera_zoom;
        
        let clicked = rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT);
        let held = rl.is_mouse_button_down(MOUSE_BUTTON_LEFT);
        
        if !held && !clicked {
            self.mouse_state = Inactive;
        }
        
        if clicked {
            self.mouse_state = Clicked;
            self.click_pos = self.mouse_pos;
        }
        
        let dx = self.mouse_pos.x - self.click_pos.x;
        let dy = self.mouse_pos.y - self.click_pos.y;
        
        //avoid sqrt just for fun
        let distance_between_click_and_current_pos_squared = dx * dx + dy * dy;
        
        //still correct, just using radius squared of the radius wanted, which is 0.1 here
        if held && distance_between_click_and_current_pos_squared >= 0.1 * 0.1 {
            self.mouse_state = Dragging;
        }    
    }
    
    pub fn draw_mouse(&self, d: &mut RaylibDrawHandle, sprite_sheet: &Texture2D) {
        MOUSE_SPRITE.draw(d, self.mouse_pos, sprite_sheet);
    }
}