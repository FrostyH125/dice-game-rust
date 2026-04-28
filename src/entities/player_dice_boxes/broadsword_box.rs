use basic_raylib_core::graphics::{
    animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance,
};
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::Font,
    texture::Texture2D,
};

use crate::{entities::dice_box_data::DiceBoxData, system::info_hover::InfoHover};

const RESULTS_TEXT_COLOR: Color = Color { r: 208, g: 184, b: 184, a: 255 };
static BROADSWORD_BOX_SPRITE: Sprite = Sprite::new(14.0, 112.0, 52.0, 16.0);
static PLAYER_ATTACK_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(0.0, 272.0, 32.0, 48.0),
        Sprite::new(32.0, 272.0, 32.0, 48.0),
        Sprite::new(64.0, 272.0, 32.0, 48.0),
        Sprite::new(96.0, 272.0, 32.0, 48.0),
        Sprite::new(128.0, 272.0, 32.0, 48.0),
        Sprite::new(160.0, 272.0, 32.0, 48.0),
        Sprite::new(160.0, 272.0, 32.0, 48.0),
        Sprite::new(192.0, 272.0, 32.0, 48.0),
        Sprite::new(224.0, 272.0, 32.0, 48.0),
    ],
    frame_duration: 0.075,
    should_loop: false,
};

pub struct BroadSwordBox {
    pub data: DiceBoxData,
}

impl BroadSwordBox {
    pub fn new(font: &Font) -> Self {
        let collect_rect_offset_x = 2.0;
        let collect_rect_offset_y = -31.0;
        let collect_rect_width = 48.0;
        let collect_rect_height = 32.0;
        let dice_box_width = 52.0;
        let dice_box_height = 16.0;

        BroadSwordBox {
            data: DiceBoxData::new(
                collect_rect_offset_x,
                collect_rect_offset_y,
                collect_rect_width,
                collect_rect_height,
                dice_box_width,
                dice_box_height,
                InfoHover::new(
                    "Broadsword:\n just an average weapon, should be enough to defend yourself for a while...",
                    Rectangle::new(
                        0.0,
                        0.0,
                        BROADSWORD_BOX_SPRITE.src_rect.width,
                        BROADSWORD_BOX_SPRITE.src_rect.height,
                    ),
                    font,
                    5.0,
                    0.5,
                ),
            ),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        BROADSWORD_BOX_SPRITE.draw(d, self.data.pos, texture);
        d.draw_rectangle_lines(
            self.data.dice_collect_rect.x as i32,
            self.data.dice_collect_rect.y as i32,
            self.data.dice_collect_rect.width as i32,
            self.data.dice_collect_rect.height as i32,
            Color::WHITE,
        );
        self.data.draw_dice(d, texture);
        self.data.draw_base_multi(d, font, RESULTS_TEXT_COLOR);
        self.data.draw_current_streak(d, font, RESULTS_TEXT_COLOR);
        self.data.draw_border_around_current_dice(d, texture);
        self.data.draw_total_amounts(d, font, RESULTS_TEXT_COLOR);
    }

    pub fn player_draw_attack(
        d: &mut RaylibDrawHandle,
        anim: &mut SpriteAnimationInstance,
        pos: Vector2,
        texture: &Texture2D,
    ) {
        PLAYER_ATTACK_ANIM.draw(anim, d, pos, texture);
    }

    pub fn player_update_attack(anim: &mut SpriteAnimationInstance, dt: f32) -> bool {
        PLAYER_ATTACK_ANIM.update(anim, dt);

        if !anim.can_play {
            return true;
        } else {
            return false;
        }
        // return true if complete and reset, else return false
    }
}
