use basic_raylib_core::graphics::sprite::Sprite;
use raylib::math::Vector2;

use crate::VIRTUAL_WIDTH;

pub const SCOREBOARD_SPRITE: Sprite = Sprite::new(245.0, 0.0, 145.0, 25.0);
pub const SCOREBOARD_POS: Vector2 = Vector2::new(VIRTUAL_WIDTH / 2.0 - SCOREBOARD_SPRITE.src_rect.width / 2.0, 0.0);
pub const BASE_CENTER_X_POS: f32 = SCOREBOARD_POS.x + 20.0;
pub const TALLY_CENTER_X_POS: f32 = SCOREBOARD_POS.x + 55.0;
pub const MULTI_CENTER_X_POS: f32 = SCOREBOARD_POS.x + 88.0;
pub const TOTAL_CENTER_X_POS: f32 = SCOREBOARD_POS.x + 124.0;
pub const VALUES_CENTER_Y_POS: f32 = SCOREBOARD_POS.y + 16.0;

// needs to simply keep track of player and enemy dice boxes
// draw the data for them
// and play the animation before starting the tally for an enemy or a player, and after
// reset after. easy probably at end turn delay, play a reverse animation at the end