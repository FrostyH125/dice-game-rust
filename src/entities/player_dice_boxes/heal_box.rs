use basic_raylib_core::graphics::{animation_data::AnimationData, sprite::Sprite};
use raylib::{color::Color, math::Rectangle, text::Font};

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
        Sprite::new(224.0, 320.0, 32.0, 48.0)
    ],
    frame_duration: 0.075,
    should_loop: false
};

pub struct HealBox {
    pub data: DiceBoxData,
}

impl HealBox {
    pub fn new(font: &Font) -> Self {
        HealBox {
            data: DiceBoxData::new(
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
            ),
        }
    }
}
