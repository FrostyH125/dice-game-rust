use basic_raylib_core::graphics::{
    animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance,
};
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::{Rectangle, Vector2},
    text::Font,
    texture::Texture2D,
};

use crate::{
    GameContext,
    entities::dice_box_data::{
        DiceBoxData, STANDARD_BOX_COLLECT_RECT_HEIGHT, STANDARD_BOX_COLLECT_RECT_OFFSET_X,
        STANDARD_BOX_COLLECT_RECT_OFFSET_Y, STANDARD_BOX_COLLECT_RECT_WIDTH, STANDARD_BOX_HEIGHT, STANDARD_BOX_WIDTH,
    },
    system::info_hover::InfoHover,
};


static SHIELD_BOX_SPRITE: Sprite = Sprite::new(14, 128, 52, 16);
static PLAYER_SHIELD_UP_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(0, 368, 32, 48),
        Sprite::new(32, 368, 32, 48),
        Sprite::new(64, 368, 32, 48),
        Sprite::new(64, 368, 32, 48),
        Sprite::new(64, 368, 32, 48),
        Sprite::new(64, 368, 32, 48),
        Sprite::new(64, 368, 32, 48),
        Sprite::new(64, 368, 32, 48),
        Sprite::new(64, 368, 32, 48),
    ],
    frame_duration: 0.30,
    should_loop: false,
};

// before action: shield up anim

// block, break, perfect block, and shield down will all be in player.rs,
// these will all be handled within PlayerState::Hit as well

pub struct ShieldBox {
    pub data: DiceBoxData,
}

impl ShieldBox {
    pub fn new(font: &Font) -> Self {
        let data = DiceBoxData::new(
            STANDARD_BOX_COLLECT_RECT_OFFSET_X,
            STANDARD_BOX_COLLECT_RECT_OFFSET_Y,
            STANDARD_BOX_COLLECT_RECT_WIDTH,
            STANDARD_BOX_COLLECT_RECT_HEIGHT,
            STANDARD_BOX_WIDTH,
            STANDARD_BOX_HEIGHT,
            InfoHover::new(
                "Shield:\n a basic charge for your shield",
                Rectangle::new(0.0, 0.0, STANDARD_BOX_WIDTH, STANDARD_BOX_HEIGHT),
                font,
                5.0,
                0.5,
            ),
            Color::DARKGRAY,
            0.5,
        );

        return ShieldBox { data };
    }

    pub fn draw_box_and_dice(&self, d: &mut RaylibDrawHandle, game_context: &GameContext) {

        const BASE_MULTI_TEXT_COLOR: Color = Color::new(180, 197, 209, 255);
        
        SHIELD_BOX_SPRITE.draw(d, self.data.pos, &game_context.texture);
        d.draw_rectangle_lines(
            self.data.dice_collect_rect.x as i32,
            self.data.dice_collect_rect.y as i32,
            self.data.dice_collect_rect.width as i32,
            self.data.dice_collect_rect.height as i32,
            Color::WHITE,
        );
        self.data.draw_dice(d, &game_context.texture);
        self.data.draw_border_around_current_dice(d, &game_context.texture);
        self.data.draw_base_multi(d, &game_context.font, BASE_MULTI_TEXT_COLOR);
    }

    pub fn player_update_put_shield_up(anim: &mut SpriteAnimationInstance, dt: f32) -> bool {
        PLAYER_SHIELD_UP_ANIM.update(anim, dt);

        return anim.finished_playing;
    }

    pub fn player_draw_put_shield_up(
        d: &mut RaylibDrawHandle,
        anim: &SpriteAnimationInstance,
        pos: Vector2,
        texture: &Texture2D,
    ) {
        PLAYER_SHIELD_UP_ANIM.draw(anim, d, pos, texture);
    }
}
