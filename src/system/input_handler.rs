use basic_raylib_core::graphics::sprite::Sprite;
use raylib::prelude::{MouseButton::*, *};

static MOUSE_SPRITE: Sprite = Sprite::new(0.0, 16.0, 16.0, 16.0);

pub struct InputState {
    pub mouse_pos: Vector2,
    click_pos: Vector2,
    pub stopped_dragging_this_frame: bool,
    pub currently_pressed: bool,
    pub clicked_once: bool,
    pub dragging: bool,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            mouse_pos: Default::default(),
            click_pos: Default::default(),
            stopped_dragging_this_frame: false,
            currently_pressed: false,
            clicked_once: false,
            dragging: false,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, camera_zoom: f32) {
        self.mouse_pos = rl.get_mouse_position() / camera_zoom;
        self.currently_pressed = rl.is_mouse_button_down(MOUSE_BUTTON_LEFT);
        self.clicked_once = rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT);
            
        if self.clicked_once {
            self.click_pos = self.mouse_pos;
        }

        let dx = self.mouse_pos.x - self.click_pos.x;
        let dy = self.mouse_pos.y - self.click_pos.y;

        let distance_between_click_and_current_pos_squared = dx * dx + dy * dy;

        self.dragging = self.currently_pressed && distance_between_click_and_current_pos_squared >= 0.1 * 0.1;
    }

    pub fn draw_mouse(&self, d: &mut RaylibDrawHandle, sprite_sheet: &Texture2D) {
        MOUSE_SPRITE.draw(d, self.mouse_pos, sprite_sheet);
    }
}
