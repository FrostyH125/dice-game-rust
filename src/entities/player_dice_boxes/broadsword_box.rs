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

use crate::{GameContext, entities::dice_box_data::{DiceBoxData, STANDARD_BOX_COLLECT_RECT_HEIGHT, STANDARD_BOX_COLLECT_RECT_OFFSET_X, STANDARD_BOX_COLLECT_RECT_OFFSET_Y, STANDARD_BOX_COLLECT_RECT_WIDTH, STANDARD_BOX_HEIGHT, STANDARD_BOX_WIDTH}, system::info_hover::InfoHover};

const RESULTS_TEXT_COLOR: Color = Color::new(208, 184, 184, 255);
static BROADSWORD_BOX_SPRITE: Sprite = Sprite::new(14, 112, 52, 16);
static PLAYER_ATTACK_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(0, 272, 32, 48),
        Sprite::new(32, 272, 32, 48),
        Sprite::new(64, 272, 32, 48),
        Sprite::new(96, 272, 32, 48),
        Sprite::new(128, 272, 32, 48),
        Sprite::new(160, 272, 32, 48),
        Sprite::new(160, 272, 32, 48),
        Sprite::new(192, 272, 32, 48),
        Sprite::new(224, 272, 32, 48),
    ],
    frame_duration: 0.075,
    should_loop: false
};

pub struct BroadSwordBox {
    pub data: DiceBoxData,
}

impl BroadSwordBox {
    pub fn new(font: &Font) -> Self {
        BroadSwordBox {
            data: DiceBoxData::new(
                STANDARD_BOX_COLLECT_RECT_OFFSET_X,
                STANDARD_BOX_COLLECT_RECT_OFFSET_Y,
                STANDARD_BOX_COLLECT_RECT_WIDTH,
                STANDARD_BOX_COLLECT_RECT_HEIGHT,
                STANDARD_BOX_WIDTH,
                STANDARD_BOX_HEIGHT,
                InfoHover::new(
                    "Broadsword:\n just an average weapon, should be enough to defend yourself for a while...",
                    Rectangle::new(
                        0.0,
                        0.0,
                        STANDARD_BOX_WIDTH,
                        STANDARD_BOX_HEIGHT,
                    ),
                    font,
                    5.0,
                    0.5,
                ),
                Color::DARKRED
            ),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, game_context: &GameContext) {
        BROADSWORD_BOX_SPRITE.draw(d, self.data.pos, &game_context.texture);
        d.draw_rectangle_lines(
            self.data.dice_collect_rect.x as i32,
            self.data.dice_collect_rect.y as i32,
            self.data.dice_collect_rect.width as i32,
            self.data.dice_collect_rect.height as i32,
            Color::WHITE,
        );
        self.data.draw_dice(d, &game_context.texture);
        self.data.draw_base_multi(d, &game_context.font, RESULTS_TEXT_COLOR);
        self.data.draw_current_streak(d, &game_context.font, RESULTS_TEXT_COLOR);
        self.data.draw_border_around_current_dice(d, &game_context.texture);
        self.data.draw_info_sprite_and_information(d, &game_context.font, RESULTS_TEXT_COLOR);
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
    }
}
