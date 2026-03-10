use super::input_handler::{InputState, MouseState::*};
use raylib::prelude::*;

pub struct Button {
    pub rect: Rectangle,
}

impl Button {
    //rect is the destination rectangle onto the screen
    pub fn new(rect: Rectangle) -> Self {
        Button { rect }
    }

    pub fn is_pressed(&self, input_state: &InputState) -> bool {
        let is_clicked =
            self.rect.check_collision_point_rec(input_state.mouse_pos) && input_state.mouse_state == Clicked;

        if is_clicked {
            return true;
        } else {
            return false;
        }
    }
}
