use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::Font,
    texture::Texture2D,
};

use crate::{
    entities::{
        dice::Dice,
        dice_box_data::{BASE_MULTI_OFFSET, CURRENT_STREAK_OFFSET, DiceBoxData, TOTAL_VALUE_OFFSET},
    },
    system::info_hover::InfoHover,
};

const RESULTS_TEXT_COLOR: Color = Color { r: 208, g: 184, b: 184, a: 255 };
static BROADSWORD_BOX_SPRITE: Sprite = Sprite::new(14.0, 112.0, 52.0, 16.0);

pub struct BroadSwordBox {
    pub data: DiceBoxData,
}

impl BroadSwordBox {
    pub fn new(font: &Font) -> Self {
        //5.0, 50.0

        let pos = Vector2 { x: 25.0, y: 50.0 };

        BroadSwordBox {
            data: DiceBoxData::new(
                pos,
                Rectangle {
                    x: pos.x + 2.0,
                    y: pos.y - 31.0,
                    width: 48.0,
                    height: 32.0,
                },
                InfoHover::new(
                    "Broadsword:\n just an average weapon, should be enough to defend yourself for a while...",
                    Rectangle::new(
                        pos.x,
                        pos.y,
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
        self.draw_base_multi(d, font);
        self.draw_current_streak(d, font);
        self.data.draw_border_around_current_dice(d, texture);
        self.draw_total_amounts(d, font);
    }

    fn draw_base_multi(&self, d: &mut RaylibDrawHandle, font: &Font) {
        d.draw_text_ex(
            font,
            &format!("base: x{}", self.data.base_multi_for_this_dice_box),
            self.data.pos + BASE_MULTI_OFFSET,
            3.0,
            0.0,
            Color { r: 208, g: 184, b: 184, a: 255 },
        );
    }

    fn draw_total_amounts(&self, d: &mut RaylibDrawHandle, font: &Font) {
        let no_dice_counted_yet = self.data.current_index_of_dice_just_tallied == None;
        if no_dice_counted_yet {
            return;
        }

        let base = self.data.base_multi_for_this_dice_box;
        let tally = self.data.total_tally;
        let multi = self.data.total_multi_for_this_tally;

        d.draw_text_ex(
            font,
            &format!(
                "total:\n{} tally\n* {} multi \n* {} base\n= {} damage!",
                tally,
                multi,
                base,
                tally * multi * base
            ),
            self.data.pos + TOTAL_VALUE_OFFSET,
            8.0,
            0.0,
            RESULTS_TEXT_COLOR,
        );
    }

    fn draw_current_streak(&self, d: &mut RaylibDrawHandle, font: &Font) {
        let streak = self.data.current_streak;

        if streak <= 1 {
            return;
        }

        d.draw_text_ex(
            font,
            &format!("Streak {} !", streak),
            self.data.pos + CURRENT_STREAK_OFFSET,
            8.0,
            0.0,
            RESULTS_TEXT_COLOR,
        );
    }

    pub fn reset(&mut self, hand_dice: &mut Vec<Dice>) {
        self.data.reset_box(hand_dice);
    }
}
