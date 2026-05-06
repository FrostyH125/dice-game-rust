use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{math::Vector2, prelude::RaylibDrawHandle, texture::Texture2D};

use crate::{
    VIRTUAL_WIDTH,
    entities::{
        enemy::{Enemy, EnemyState},
        player::{Player, PlayerState},
    },
};

pub const SCOREBOARD_SPRITE: Sprite = Sprite::new(245.0, 0.0, 145.0, 25.0);
pub const SCOREBOARD_POS: Vector2 = Vector2::new(VIRTUAL_WIDTH / 2.0 - SCOREBOARD_SPRITE.src_rect.width / 2.0, 0.0);
pub const BASE_CENTER_X_POS: f32 = SCOREBOARD_POS.x + 20.0;
pub const TALLY_CENTER_X_POS: f32 = SCOREBOARD_POS.x + 55.0;
pub const MULTI_CENTER_X_POS: f32 = SCOREBOARD_POS.x + 88.0;
pub const TOTAL_CENTER_X_POS: f32 = SCOREBOARD_POS.x + 124.0;
pub const VALUES_CENTER_Y_POS: f32 = SCOREBOARD_POS.y + 16.0;

// unfortunately i dont wanna determine the enemy every single frame, so im forced
// to save a reference to it in the struct, meaning this is the first lifetime of
// the project so far :(, if you have any ideas, hit me up
pub struct ScoreBoard<'a> {
    current_enemy: Option<&'a Enemy>,
    is_open: bool,
    is_closed: bool,
    is_player_tallying: bool,
}
impl<'a> ScoreBoard<'a> {
    pub fn new() -> Self {
        ScoreBoard {
            current_enemy: None,
            is_open: false,
            is_closed: false,
            is_player_tallying: false,
        }
    }

    pub fn update(&mut self, player: &mut Player, enemies: &'a [Enemy]) {
        // find an enemy or player that is currently requiring the scoreboard
        if !self.current_enemy.is_none() && !self.is_player_tallying {
            self.is_player_tallying = player.state == PlayerState::TallyingCurrentBox;
            self.current_enemy = enemies.iter().find(|enemy| enemy.get_data().state == EnemyState::BeforeTallyDelay);
        }

        // if it has an enemy or player tallying currently !self.is_open
        // {
        //      open animation update
        //      if open animation is done, self.is_open = true;
        //      reset open animation
        // }
        //
        // if the player or enemy is ending turn, and !self.is_closed
        // {
        //      close animation update
        //      if close animation is done, self.is_closed
        //      reset close animation
        // }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, player: &mut Player, enemies: &[Enemy], texture: &Texture2D) {
        SCOREBOARD_SPRITE.draw(d, SCOREBOARD_POS, texture);
        if self.current_enemy.is_some() || self.is_player_tallying {
            if self.current_enemy.is_none() {
                todo!("Draw enemy stats")
            } else {
                todo!("Draw player stats")
            }
        }

        // if !self.is_open {
        //  if !close_animation.is_playing {
        //      draw_open_animation (will be stagnant until its playing, then disappear once open)
        // } else if close_animation.is_playing{
        //      draw_close_animation (otherwise, will draw nothing)
        // }
    }
}
