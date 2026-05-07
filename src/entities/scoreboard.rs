use basic_raylib_core::graphics::{
    animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance,
};
use raylib::{math::Vector2, prelude::RaylibDrawHandle, texture::Texture2D};

use crate::{
    VIRTUAL_WIDTH,
    entities::{
        enemy::{Enemy, EnemyState},
        player::{Player, PlayerState},
    },
};

const SCOREBOARD_SPRITE: Sprite = Sprite::new(245.0, 0.0, 145.0, 25.0);
const SCOREBOARD_POS: Vector2 = Vector2::new(VIRTUAL_WIDTH / 2.0 - SCOREBOARD_SPRITE.src_rect.width / 2.0, 0.0);
const BASE_CENTER_X_POS: f32 = SCOREBOARD_POS.x + 20.0;
const TALLY_CENTER_X_POS: f32 = SCOREBOARD_POS.x + 55.0;
const MULTI_CENTER_X_POS: f32 = SCOREBOARD_POS.x + 88.0;
const TOTAL_CENTER_X_POS: f32 = SCOREBOARD_POS.x + 124.0;
const VALUES_CENTER_Y_POS: f32 = SCOREBOARD_POS.y + 16.0;

static OPEN_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(254.0, 25.0, 23.0, 11.0),
        Sprite::new(279.0, 25.0, 23.0, 11.0),
        Sprite::new(303.0, 25.0, 23.0, 11.0),
        Sprite::new(327.0, 25.0, 23.0, 11.0),
        Sprite::new(351.0, 25.0, 23.0, 11.0),
        Sprite::new(375.0, 25.0, 23.0, 11.0),
    ],
    frame_duration: 0.25,
    should_loop: false,
};

static CLOSE_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(375.0, 25.0, 23.0, 11.0),
        Sprite::new(351.0, 25.0, 23.0, 11.0),
        Sprite::new(327.0, 25.0, 23.0, 11.0),
        Sprite::new(303.0, 25.0, 23.0, 11.0),
        Sprite::new(279.0, 25.0, 23.0, 11.0),
        Sprite::new(254.0, 25.0, 23.0, 11.0),
    ],
    frame_duration: 0.25,
    should_loop: false,
};

#[derive(PartialEq, Eq)]
enum ScoreBoardState {
    Closed,
    Opening,
    Open,
    Closing,
}

enum TurnIdentity {
    Player,
    Enemy,
    None,
}

// unfortunately i dont wanna determine the enemy every single frame, so im forced
// to save a reference to it in the struct, meaning this is the first lifetime of
// the project so far :(, if you have any ideas, hit me up
pub struct ScoreBoard<'a> {
    current_enemy: Option<&'a Enemy>,
    open_anim: SpriteAnimationInstance,
    close_anim: SpriteAnimationInstance,
    state: ScoreBoardState,
    turn_identity: TurnIdentity,
    is_player_before_tally: bool,
}

impl<'a> ScoreBoard<'a> {
    pub fn new() -> Self {
        ScoreBoard {
            current_enemy: None,
            open_anim: SpriteAnimationInstance::new(),
            close_anim: SpriteAnimationInstance::new(),
            state: ScoreBoardState::Closed,
            turn_identity: TurnIdentity::None,
            is_player_before_tally: false,
        }
    }

    pub fn update(&mut self, player: &mut Player, enemies: &'a [Enemy], dt: f32) {
        // find an enemy or player that is currently requiring the scoreboard
        if !self.current_enemy.is_none() && !self.is_player_before_tally {
            self.is_player_before_tally = player.state == PlayerState::TallyingCurrentBox;
            self.current_enemy = enemies.iter().find(|enemy| enemy.get_data().state == EnemyState::BeforeTallyDelay);

            if self.is_player_before_tally {
                self.turn_identity = TurnIdentity::Player;
            } else if self.current_enemy.is_some() {
                self.turn_identity = TurnIdentity::Enemy;
            }
        }

        match self.state {
            ScoreBoardState::Closed => {
                let should_start_opening = self.is_player_before_tally || self.current_enemy.is_some();

                if should_start_opening {
                    self.state = ScoreBoardState::Opening
                }
            }
            ScoreBoardState::Opening => {
                OPEN_ANIM.update(&mut self.open_anim, dt);
                if !self.open_anim.can_play {
                    self.state = ScoreBoardState::Open;
                    self.open_anim.reset();
                }
            }
            ScoreBoardState::Open => {
                match self.turn_identity {
                    TurnIdentity::Player => {
                        if player.state == PlayerState::EndTurnDelay {
                            self.state = ScoreBoardState::Closing;
                        }

                        // if player dies, might just disable scoreboard entirely, probably will be handled in main.rs
                    }
                    TurnIdentity::Enemy => {
                        if self.current_enemy.unwrap().get_data().state == EnemyState::EndTurnDelay {
                            self.state = ScoreBoardState::Closing;
                        } else if self.current_enemy.unwrap().get_data().state == EnemyState::Dead {
                            self.state = ScoreBoardState::Closing;
                            self.turn_identity = TurnIdentity::None;
                        }
                    }
                    TurnIdentity::None => panic!("the scoreboard shouldnt be open with no ones turn"),
                }
            }
            ScoreBoardState::Closing => {
                CLOSE_ANIM.update(&mut self.close_anim, dt);
                if !self.close_anim.can_play {
                    self.state = ScoreBoardState::Closed;
                    self.close_anim.reset();
                }
            }
        }
    }
    
    pub fn draw(&self, d: &mut RaylibDrawHandle, player: &mut Player, enemies: &[Enemy], texture: &Texture2D) {
        SCOREBOARD_SPRITE.draw(d, SCOREBOARD_POS, texture);

        match self.state {
            ScoreBoardState::Closed => todo!("Draw open anim here"),
            ScoreBoardState::Opening => todo!("Draw open anim here (its updating now), draw random numbers under open anim"),
            ScoreBoardState::Open => todo!("Dont need to draw any anims, match turn identity, 
                if player turn and player state is tallying acting etc then draw player stats,
                if enemy turn and enemy state is tallying acting etc then draw enemy stats,
                if the one whos turn it is isnt at tallying state yet, then draw random numbers"),
            ScoreBoardState::Closing => todo!("draw closing anim"),
        }
    }
}
