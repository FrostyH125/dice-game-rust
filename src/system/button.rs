use raylib::prelude::*;
use super::input_handler::{InputState, MouseState::*};

pub struct Button {
    pub pos: Vector2,
    pub rect: Rectangle,
}

impl Button {

    //rect is the destination rectangle onto the screen
    pub fn new(rect: Rectangle) -> Self {
        Button {
            pos: Vector2 { x: rect.x, y: rect.y },
            rect,
            is_active: false,
        }
    }

    pub fn is_pressed(&mut self, input_state: &InputState) -> bool {
        let is_clicked = self.rect.check_collision_point_rec(input_state.mouse_pos) && input_state.mouse_state == Clicked;

        if is_clicked {
            return true;
        }
        else {
            return false;
        }
    }
}
