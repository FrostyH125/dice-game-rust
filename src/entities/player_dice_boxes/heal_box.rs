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

use crate::{
    entities::dice_box_data::{
        DiceBoxData, STANDARD_BOX_COLLECT_RECT_HEIGHT, STANDARD_BOX_COLLECT_RECT_OFFSET_X,
        STANDARD_BOX_COLLECT_RECT_OFFSET_Y, STANDARD_BOX_COLLECT_RECT_WIDTH, STANDARD_BOX_HEIGHT, STANDARD_BOX_WIDTH,
    },
    system::info_hover::InfoHover,
};

// need to add the draw method and the heal method,
// probably will be a basic heal method for the actual logic just like the basic attack method

const RESULTS_TEXT_COLOR: Color = Color::new(146, 215, 200, 255);
static HEAL_BOX_SPRITE: Sprite = Sprite::new(14.0, 144.0, 52.0, 16.0);
static PLAYER_HEAL_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(0.0, 320.0, 32.0, 48.0),
        Sprite::new(32.0, 320.0, 32.0, 48.0),
        Sprite::new(64.0, 320.0, 32.0, 48.0),
        Sprite::new(96.0, 320.0, 32.0, 48.0),
        Sprite::new(128.0, 320.0, 32.0, 48.0),
        Sprite::new(160.0, 320.0, 32.0, 48.0),
        Sprite::new(192.0, 320.0, 32.0, 48.0),
        Sprite::new(64.0, 320.0, 32.0, 48.0),
        Sprite::new(96.0, 320.0, 32.0, 48.0),
        Sprite::new(128.0, 320.0, 32.0, 48.0),
        Sprite::new(160.0, 320.0, 32.0, 48.0),
        Sprite::new(192.0, 320.0, 32.0, 48.0),
        Sprite::new(224.0, 320.0, 32.0, 48.0),
    ],
    frame_duration: 0.20,
    should_loop: false,
};

pub struct HealBox {
    pub data: DiceBoxData,
}

impl HealBox {
    pub fn new(font: &Font) -> Self {
        let mut data = DiceBoxData::new(
            STANDARD_BOX_COLLECT_RECT_OFFSET_X,
            STANDARD_BOX_COLLECT_RECT_OFFSET_Y,
            STANDARD_BOX_COLLECT_RECT_WIDTH,
            STANDARD_BOX_COLLECT_RECT_HEIGHT,
            STANDARD_BOX_WIDTH,
            STANDARD_BOX_HEIGHT,
            InfoHover::new(
                "Heal:\n a basic healing spell, it will heal you with a quarter of the value placed inside",
                Rectangle::new(0.0, 0.0, STANDARD_BOX_WIDTH, STANDARD_BOX_HEIGHT),
                font,
                5.0,
                0.5,
            ),
        );
        
        data.base_multi_for_this_dice_box = 0.25;

        HealBox { data }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        HEAL_BOX_SPRITE.draw(d, self.data.pos, texture);
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
        self.data.draw_info_sprite_and_information(d, font, RESULTS_TEXT_COLOR);
    }

    pub fn player_draw_heal(
        d: &mut RaylibDrawHandle,
        anim: &mut SpriteAnimationInstance,
        pos: Vector2,
        texture: &Texture2D,
    ) {
        PLAYER_HEAL_ANIM.draw(anim, d, pos, texture);
    }

    pub fn player_update_heal(anim: &mut SpriteAnimationInstance, dt: f32) -> bool {
        PLAYER_HEAL_ANIM.update(anim, dt);

        if !anim.can_play {
            return true;
        } else {
            return false;
        }
    }
}
