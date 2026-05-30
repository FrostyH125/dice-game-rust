use basic_raylib_core::{
    graphics::sprite::Sprite,
    system::{input_handler::InputState, timer::Timer},
    utils::string_utils::wrap_string,
};
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::{Font, RaylibFont},
};

use crate::GameContext;

const CONSTANT_EDGE_SIZE: f32 = 4.0;
const VARYING_EDGE_DEFAULT_SIZE: f32 = 1.0;

static TOP_LEFT_CORNER_SPRITE: Sprite = Sprite::new(145, 33, CONSTANT_EDGE_SIZE as u32, CONSTANT_EDGE_SIZE as u32);
static TOP_RIGHT_CORNER_SPRITE: Sprite = Sprite::new(152, 33, CONSTANT_EDGE_SIZE as u32, CONSTANT_EDGE_SIZE as u32);
static BOTTOM_LEFT_CORNER_SPRITE: Sprite = Sprite::new(145, 40, CONSTANT_EDGE_SIZE as u32, CONSTANT_EDGE_SIZE as u32);
static BOTTOM_RIGHT_CORNER_SPRITE: Sprite = Sprite::new(152, 40, CONSTANT_EDGE_SIZE as u32, CONSTANT_EDGE_SIZE as u32);

static INNER_RECT_SPRITE: Sprite = Sprite::new(150, 38, VARYING_EDGE_DEFAULT_SIZE as u32, VARYING_EDGE_DEFAULT_SIZE as u32);
static LEFT_EDGE_SPRITE: Sprite = Sprite::new(145, 38, CONSTANT_EDGE_SIZE as u32, VARYING_EDGE_DEFAULT_SIZE as u32);
static TOP_EDGE_SPRITE: Sprite = Sprite::new(150, 33, VARYING_EDGE_DEFAULT_SIZE as u32, CONSTANT_EDGE_SIZE as u32);
static RIGHT_EDGE_SPRITE: Sprite = Sprite::new(152, 38, CONSTANT_EDGE_SIZE as u32, VARYING_EDGE_DEFAULT_SIZE as u32);
static BOTTOM_EDGE_SPRITE: Sprite = Sprite::new(150, 40, VARYING_EDGE_DEFAULT_SIZE as u32, CONSTANT_EDGE_SIZE as u32);

pub struct InfoHover {
    text: String,
    font_size: f32,
    spacing: f32,
    text_width: f32,
    text_height: f32,
    pub activation_rect: Rectangle,
    inner_rect: Rectangle,
    activation_timer: Timer,
}

impl InfoHover {
    pub fn new(text: &str, activation_rect: Rectangle, font: &Font, font_size: f32, spacing: f32) -> Self {
        let wrapped_text = wrap_string(text, 120.0, font, font_size, spacing);
        let text_size = font.measure_text(&wrapped_text, font_size, spacing);

        InfoHover {
            text: wrapped_text,
            font_size,
            spacing,
            text_width: text_size.x,
            text_height: text_size.y,
            activation_rect,
            inner_rect: Rectangle {
                x: Default::default(),
                y: Default::default(),
                width: text_size.x,
                height: text_size.y,
            },
            activation_timer: Timer::new(0.5),
        }
    }

    pub fn update(&mut self, input: &InputState, dt: f32) {
        if !self.activation_rect.check_collision_point_rec(input.mouse_pos) {
            self.activation_timer.reset();
            return;
        }

        self.activation_timer.track(dt);

        if !self.activation_timer.is_done() {
            return;
        }
        
        let margin = 10.0;

        let start_pos_x = input.mouse_pos.x - self.text_width / 2.0;
        let start_pos_y = input.mouse_pos.y - self.text_height - margin;

        self.inner_rect.x = start_pos_x.round();
        self.inner_rect.y = start_pos_y.round();
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, game_context: &GameContext) {
        if !self.activation_timer.is_done() {
            return;
        }

        let zero_v = Vector2 { x: 0.0, y: 0.0 };

        INNER_RECT_SPRITE.draw_pro(d, self.inner_rect, zero_v, 0.0, &game_context.texture);

        // draw the sides
        LEFT_EDGE_SPRITE.draw_pro(
            d,
            Rectangle {
                x: self.inner_rect.x - CONSTANT_EDGE_SIZE,
                y: self.inner_rect.y,
                width: CONSTANT_EDGE_SIZE,
                height: self.text_height,
            },
            zero_v,
            0.0,
            &game_context.texture,
        );

        RIGHT_EDGE_SPRITE.draw_pro(
            d,
            Rectangle {
                x: self.inner_rect.x + self.inner_rect.width,
                y: self.inner_rect.y,
                width: CONSTANT_EDGE_SIZE,
                height: self.text_height,
            },
            zero_v,
            0.0,
            &game_context.texture,
        );

        // draw the top and bottom
        TOP_EDGE_SPRITE.draw_pro(
            d,
            Rectangle {
                x: self.inner_rect.x,
                y: self.inner_rect.y - CONSTANT_EDGE_SIZE,
                width: self.text_width,
                height: CONSTANT_EDGE_SIZE,
            },
            zero_v,
            0.0,
            &game_context.texture,
        );

        BOTTOM_EDGE_SPRITE.draw_pro(
            d,
            Rectangle {
                x: self.inner_rect.x,
                y: self.inner_rect.y + self.inner_rect.height,
                width: self.text_width,
                height: CONSTANT_EDGE_SIZE,
            },
            zero_v,
            0.0,
            &game_context.texture,
        );

        //top left corner
        TOP_LEFT_CORNER_SPRITE.draw(
            d,
            Vector2 {
                x: self.inner_rect.x - CONSTANT_EDGE_SIZE,
                y: self.inner_rect.y - CONSTANT_EDGE_SIZE,
            },
            &game_context.texture,
        );

        //top right corner
        TOP_RIGHT_CORNER_SPRITE.draw(
            d,
            Vector2 {
                x: self.inner_rect.x + self.inner_rect.width,
                y: self.inner_rect.y - CONSTANT_EDGE_SIZE,
            },
            &game_context.texture,
        );
        //bottom left corner
        BOTTOM_LEFT_CORNER_SPRITE.draw(
            d,
            Vector2 {
                x: self.inner_rect.x - CONSTANT_EDGE_SIZE,
                y: self.inner_rect.y + self.inner_rect.height,
            },
            &game_context.texture,
        );
        //bottom right corner
        BOTTOM_RIGHT_CORNER_SPRITE.draw(
            d,
            Vector2 {
                x: self.inner_rect.x + self.inner_rect.width,
                y: self.inner_rect.y + self.inner_rect.height,
            },
            &game_context.texture,
        );

        d.draw_text_ex(
            &game_context.font,
            &self.text,
            Vector2 {
                x: self.inner_rect.x,
                y: self.inner_rect.y,
            },
            self.font_size,
            self.spacing,
            Color::WHITE,
        );
    }
}
