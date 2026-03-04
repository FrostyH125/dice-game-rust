use basic_raylib_core::{
    graphics::sprite::Sprite,
    system::timer::Timer,
    utils::string_utils::wrap_string,
};
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::{Font, RaylibFont},
    texture::Texture2D,
};

use crate::system::input_handler::InputState;

const CONSTANT_EDGE_SIZE: f32 = 4.0;
const VARYING_EDGE_DEFAULT_SIZE: f32 = 1.0;

static TOP_LEFT_CORNER_SPRITE: Sprite = Sprite::new(145.0, 33.0, CONSTANT_EDGE_SIZE, CONSTANT_EDGE_SIZE);
static TOP_RIGHT_CORNER_SPRITE: Sprite = Sprite::new(152.0, 33.0, CONSTANT_EDGE_SIZE, CONSTANT_EDGE_SIZE);
static BOTTOM_LEFT_CORNER_SPRITE: Sprite = Sprite::new(145.0, 40.0, CONSTANT_EDGE_SIZE, CONSTANT_EDGE_SIZE);
static BOTTOM_RIGHT_CORNER_SPRITE: Sprite = Sprite::new(152.0, 40.0, CONSTANT_EDGE_SIZE, CONSTANT_EDGE_SIZE);

static INNER_RECT_SPRITE: Sprite = Sprite::new(150.0, 38.0, VARYING_EDGE_DEFAULT_SIZE, VARYING_EDGE_DEFAULT_SIZE);
static LEFT_EDGE_SPRITE: Sprite = Sprite::new(145.0, 38.0, CONSTANT_EDGE_SIZE, VARYING_EDGE_DEFAULT_SIZE);
static TOP_EDGE_SPRITE: Sprite = Sprite::new(150.0, 33.0, VARYING_EDGE_DEFAULT_SIZE, CONSTANT_EDGE_SIZE);
static RIGHT_EDGE_SPRITE: Sprite = Sprite::new(152.0, 38.0, CONSTANT_EDGE_SIZE, VARYING_EDGE_DEFAULT_SIZE);
static BOTTOM_EDGE_SPRITE: Sprite = Sprite::new(150.0, 40.0, VARYING_EDGE_DEFAULT_SIZE, CONSTANT_EDGE_SIZE);

pub struct InfoHover {
    text: String,
    font_size: f32,
    spacing: f32,
    text_width: f32,
    text_height: f32,
    activation_rect: Rectangle,
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

        let start_pos_x = input.mouse_pos.x - self.text_width / 2.0;
        let start_pos_y = input.mouse_pos.y - self.text_height;

        self.inner_rect.x = start_pos_x.round();
        self.inner_rect.y = start_pos_y.round();
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, font: &Font, texture: &Texture2D) {
        if !self.activation_timer.is_done() {
            return;
        }

        let zero_v = Vector2 { x: 0.0, y: 0.0 };

        INNER_RECT_SPRITE.draw_pro(d, self.inner_rect, zero_v, 0.0, texture);

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
            texture,
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
            texture,
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
            texture,
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
            texture,
        );

        //top left corner
        TOP_LEFT_CORNER_SPRITE.draw(
            d,
            Vector2 {
                x: self.inner_rect.x - CONSTANT_EDGE_SIZE,
                y: self.inner_rect.y - CONSTANT_EDGE_SIZE,
            },
            texture,
        );

        //top right corner
        TOP_RIGHT_CORNER_SPRITE.draw(
            d,
            Vector2 {
                x: self.inner_rect.x + self.inner_rect.width,
                y: self.inner_rect.y - CONSTANT_EDGE_SIZE,
            },
            texture,
        );
        //bottom left corner
        BOTTOM_LEFT_CORNER_SPRITE.draw(
            d,
            Vector2 {
                x: self.inner_rect.x - CONSTANT_EDGE_SIZE,
                y: self.inner_rect.y + self.inner_rect.height,
            },
            texture,
        );
        //bottom right corner
        BOTTOM_RIGHT_CORNER_SPRITE.draw(
            d,
            Vector2 {
                x: self.inner_rect.x + self.inner_rect.width,
                y: self.inner_rect.y + self.inner_rect.height,
            },
            texture,
        );

        d.draw_text_ex(
            font,
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
