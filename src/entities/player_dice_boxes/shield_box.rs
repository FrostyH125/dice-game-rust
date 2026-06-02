use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, math::Rectangle, text::Font};

use crate::{
    GameContext, entities::dice_box_data::{
        DiceBoxData, STANDARD_BOX_COLLECT_RECT_HEIGHT, STANDARD_BOX_COLLECT_RECT_OFFSET_X,
        STANDARD_BOX_COLLECT_RECT_OFFSET_Y, STANDARD_BOX_COLLECT_RECT_WIDTH, STANDARD_BOX_HEIGHT, STANDARD_BOX_WIDTH,
    }, system::info_hover::InfoHover
};

const BASE_MULTI_TEXT_COLOR: Color = Color::new(161, 179, 174, 255);
static SHIELD_BOX_SPRITE: Sprite = Sprite::new(14, 128, 52, 16);

// need: putting shield up anim, block sprite, break sprite, perfect block sprite, shield down anim
 
// before action: shield up anim

// block, break, perfect block, and sheild down will all be in player.rs, 
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
            0.5
        );

        return ShieldBox { data };
    }

    pub fn draw_box_and_dice(&self, d: &mut RaylibDrawHandle, game_context: &GameContext) {
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

    // fn player_draw_heal() and fn player_update_heal() need to be here once we have an animation for putting shield up
}
