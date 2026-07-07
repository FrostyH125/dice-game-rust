// next make the colors of the boxes (which will be a new field in dice box data) the info disp color

use basic_raylib_core::{
    graphics::{animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance},
    system::timer::Timer,
    utils::string_utils,
};
use rand::{RngExt, rngs::ThreadRng};
use raylib::{color::Color, math::Vector2, prelude::RaylibDrawHandle, text::Font};

use crate::{
    GameContext, VIRTUAL_WIDTH,
    entities::{
        dice_box::DiceBox,
        enemy::{Enemy, EnemyState},
        player::{Player, PlayerState},
    },
};

const SCOREBOARD_SPRITE: Sprite = Sprite::new(245, 0, 145, 25);
const SCOREBOARD_POS: Vector2 = Vector2::new(VIRTUAL_WIDTH / 2.0 - SCOREBOARD_SPRITE.src_rect.width / 2.0, 0.0);

const VALUES_CENTER_Y_POS: f32 = SCOREBOARD_POS.y + 16.0;

const BASE_CENTER_POS: Vector2 = Vector2::new(SCOREBOARD_POS.x + 20.0, VALUES_CENTER_Y_POS);
const TALLY_CENTER_POS: Vector2 = Vector2::new(SCOREBOARD_POS.x + 55.0, VALUES_CENTER_Y_POS);
const MULTI_CENTER_POS: Vector2 = Vector2::new(SCOREBOARD_POS.x + 88.0, VALUES_CENTER_Y_POS);
const TOTAL_CENTER_POS: Vector2 = Vector2::new(SCOREBOARD_POS.x + 124.0, VALUES_CENTER_Y_POS);

const HALF_ANIM_WIDTH: f32 = 23.0 / 2.0 - 0.25;
const HALF_ANIM_HEIGHT: f32 = 5.0;

const BASE_ANIM_POS: Vector2 =
    Vector2::new(BASE_CENTER_POS.x - HALF_ANIM_WIDTH, VALUES_CENTER_Y_POS - HALF_ANIM_HEIGHT);
const TALLY_ANIM_POS: Vector2 =
    Vector2::new(TALLY_CENTER_POS.x - HALF_ANIM_WIDTH, VALUES_CENTER_Y_POS - HALF_ANIM_HEIGHT);
const MULTI_ANIM_POS: Vector2 =
    Vector2::new(MULTI_CENTER_POS.x - HALF_ANIM_WIDTH, VALUES_CENTER_Y_POS - HALF_ANIM_HEIGHT);
const TOTAL_ANIM_POS: Vector2 =
    Vector2::new(TOTAL_CENTER_POS.x - HALF_ANIM_WIDTH, VALUES_CENTER_Y_POS - HALF_ANIM_HEIGHT);

static COVER_SPRITE: Sprite = Sprite::new(254, 25, 23, 11);

static OPEN_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(254, 25, 23, 11),
        Sprite::new(279, 25, 23, 11),
        Sprite::new(303, 25, 23, 11),
        Sprite::new(327, 25, 23, 11),
        Sprite::new(351, 25, 23, 11),
        Sprite::new(375, 25, 23, 11),
    ],
    frame_duration: 0.1,
    should_loop: false,
};

static CLOSE_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(375, 25, 23, 11),
        Sprite::new(351, 25, 23, 11),
        Sprite::new(327, 25, 23, 11),
        Sprite::new(303, 25, 23, 11),
        Sprite::new(279, 25, 23, 11),
        Sprite::new(254, 25, 23, 11),
    ],
    frame_duration: 0.1,
    should_loop: false,
};

#[derive(PartialEq, Eq, Debug)]
enum ScoreBoardState {
    Closed,
    Opening,
    Open,
    Closing,
}

#[derive(PartialEq, Eq, Debug)]
enum TurnIdentity {
    Player,
    Enemy,
    None,
}

// unfortunately i dont wanna determine the enemy every single frame, so im forced
// to save a reference to it in the struct, meaning this is the first lifetime of
// the project so far :(, if you have any ideas, hit me up
pub struct ScoreBoard {
    rng: ThreadRng,
    open_anim: SpriteAnimationInstance,
    close_anim: SpriteAnimationInstance,
    random_base_num_str: String,
    random_tally_num_str: String,
    random_multi_num_str: String,
    random_total_num_str: String,
    font_size: f32,
    font_spacing: f32,
    new_num_timer: Timer,
    state: ScoreBoardState,
    turn_identity: TurnIdentity,
}

impl ScoreBoard {
    pub fn new() -> Self {
        let mut rng = rand::rng();

        ScoreBoard {
            open_anim: SpriteAnimationInstance::new(),
            close_anim: SpriteAnimationInstance::new(),
            state: ScoreBoardState::Closed,
            turn_identity: TurnIdentity::None,
            random_base_num_str: (rng.random::<u16>() % 1000).to_string(),
            random_tally_num_str: (rng.random::<u16>() % 1000).to_string(),
            random_multi_num_str: (rng.random::<u16>() % 1000).to_string(),
            random_total_num_str: (rng.random::<u16>() % 1000).to_string(),
            font_size: 5.0,
            font_spacing: 0.5,
            rng,
            new_num_timer: Timer::new(0.25),
        }
    }

    pub fn update(&mut self, player: &mut Player, enemy: &Enemy, dt: f32) {
        match self.state {
            ScoreBoardState::Closed => {
                let is_player_turn =
                    player.state != PlayerState::WaitingForEnemy && player.state != PlayerState::Walking;
                let is_enemy_turn = enemy.get_data().state != EnemyState::WaitingForPlayer;

                let should_start_opening = is_player_turn || is_enemy_turn;

                if should_start_opening {
                    // assign proper turn
                    if is_player_turn {
                        self.turn_identity = TurnIdentity::Player;
                    } else if is_enemy_turn {
                        self.turn_identity = TurnIdentity::Enemy;
                    }

                    self.state = ScoreBoardState::Opening;
                    self.open_anim.reset();
                }
            }
            ScoreBoardState::Opening => {
                self.handle_timer_and_random_numbers(dt);

                OPEN_ANIM.update(&mut self.open_anim, dt);
                if self.open_anim.finished_playing {
                    self.state = ScoreBoardState::Open;
                    self.open_anim.reset();
                }
            }
            ScoreBoardState::Open => match self.turn_identity {
                TurnIdentity::Player => match player.state {
                    PlayerState::ChoosingDice
                    | PlayerState::RollingDice
                    | PlayerState::RerollingDice
                    | PlayerState::StoppingDice => self.handle_timer_and_random_numbers(dt),

                    PlayerState::EndTurn => {
                        self.state = ScoreBoardState::Closing;
                        self.close_anim.reset();
                    }
                    PlayerState::Dead => self.turn_identity = TurnIdentity::None,
                    _ => (),
                },
                TurnIdentity::Enemy => {
                    let enemy_state = &enemy.get_data().state;

                    match enemy_state {
                        EnemyState::StoppingDice
                        | EnemyState::ChoosingDice
                        | EnemyState::WaitingForDiceToReturnToHand
                        | EnemyState::StartDiceStopDelayTime
                        | EnemyState::StartTurn => {
                            self.handle_timer_and_random_numbers(dt);
                        }
                        EnemyState::EndTurn => {
                            self.state = ScoreBoardState::Closing;
                            self.close_anim.reset();
                        }
                        EnemyState::Dead => {
                            self.state = ScoreBoardState::Closing;
                            self.close_anim.reset();
                            self.turn_identity = TurnIdentity::None;
                        }
                        _ => (),
                    }
                }
                TurnIdentity::None => self.state = ScoreBoardState::Closing,
            },
            ScoreBoardState::Closing => {
                CLOSE_ANIM.update(&mut self.close_anim, dt);

                if self.close_anim.finished_playing {
                    self.state = ScoreBoardState::Closed;
                    self.close_anim.reset();
                }
            }
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, player: &mut Player, enemy: &Enemy, game_context: &GameContext) {
        if player.state == PlayerState::Walking {
            return;
        }

        SCOREBOARD_SPRITE.draw(d, SCOREBOARD_POS, &game_context.texture);

        match self.state {
            ScoreBoardState::Closed => {
                COVER_SPRITE.draw(d, BASE_ANIM_POS, &game_context.texture);
                COVER_SPRITE.draw(d, TALLY_ANIM_POS, &game_context.texture);
                COVER_SPRITE.draw(d, MULTI_ANIM_POS, &game_context.texture);
                COVER_SPRITE.draw(d, TOTAL_ANIM_POS, &game_context.texture);
            }
            ScoreBoardState::Opening => {
                self.draw_random_numbers(d, &game_context.font);

                OPEN_ANIM.draw(&self.open_anim, d, BASE_ANIM_POS, &game_context.texture);
                OPEN_ANIM.draw(&self.open_anim, d, TALLY_ANIM_POS, &game_context.texture);
                OPEN_ANIM.draw(&self.open_anim, d, MULTI_ANIM_POS, &game_context.texture);
                OPEN_ANIM.draw(&self.open_anim, d, TOTAL_ANIM_POS, &game_context.texture);
            }
            ScoreBoardState::Open => match self.turn_identity {
                TurnIdentity::Player => {
                    match player.state {
                        PlayerState::RollingDice
                        | PlayerState::RerollingDice
                        | PlayerState::ChoosingDice
                        | PlayerState::StartTurn
                        | PlayerState::StoppingDice => {
                            self.draw_random_numbers(d, &game_context.font);
                        }
                        _ => self.draw_box_data(d, &game_context.font, &player.dice_boxes[player.current_box_index]),
                    }

                    if enemy.get_data().state == EnemyState::Dead {
                        self.state = ScoreBoardState::Closing;
                    }
                }
                TurnIdentity::Enemy => {
                    let enemy_state = &enemy.get_data().state;

                    match enemy_state {
                        EnemyState::StartTurn
                        | EnemyState::StoppingDice
                        | EnemyState::ChoosingDice
                        | EnemyState::StartDiceStopDelayTime
                        | EnemyState::WaitingForDiceToReturnToHand => {
                            self.draw_random_numbers(d, &game_context.font);
                        }
                        _ => {
                            let enemy_data = enemy.get_data();

                            self.draw_box_data(d, &game_context.font, &enemy_data.dice_boxes[enemy_data.current_box]);
                        }
                    }
                }
                TurnIdentity::None => {}
            },
            ScoreBoardState::Closing => {
                CLOSE_ANIM.draw(&self.close_anim, d, BASE_ANIM_POS, &game_context.texture);
                CLOSE_ANIM.draw(&self.close_anim, d, TALLY_ANIM_POS, &game_context.texture);
                CLOSE_ANIM.draw(&self.close_anim, d, MULTI_ANIM_POS, &game_context.texture);
                CLOSE_ANIM.draw(&self.close_anim, d, TOTAL_ANIM_POS, &game_context.texture);
            }
        }
    }

    pub fn handle_timer_and_random_numbers(&mut self, dt: f32) {
        self.new_num_timer.track(dt);

        if self.new_num_timer.is_done() {
            self.random_base_num_str = (self.rng.random::<u16>() % 1000).to_string();
            self.random_tally_num_str = (self.rng.random::<u16>() % 1000).to_string();
            self.random_multi_num_str = (self.rng.random::<u16>() % 1000).to_string();
            self.random_total_num_str = (self.rng.random::<u16>() % 1000).to_string();

            self.new_num_timer.reset();
        }
    }

    pub fn draw_random_numbers(&self, d: &mut RaylibDrawHandle, font: &Font) {
        string_utils::draw_string_centered_on_pos(
            d,
            BASE_CENTER_POS,
            &self.random_base_num_str,
            font,
            self.font_size,
            self.font_spacing,
            Color::BLACK,
        );
        string_utils::draw_string_centered_on_pos(
            d,
            TALLY_CENTER_POS,
            &self.random_tally_num_str,
            font,
            self.font_size,
            self.font_spacing,
            Color::BLACK,
        );
        string_utils::draw_string_centered_on_pos(
            d,
            MULTI_CENTER_POS,
            &self.random_multi_num_str,
            font,
            self.font_size,
            self.font_spacing,
            Color::BLACK,
        );
        string_utils::draw_string_centered_on_pos(
            d,
            TOTAL_CENTER_POS,
            &self.random_total_num_str,
            font,
            self.font_size,
            self.font_spacing,
            Color::BLACK,
        );
    }

    pub fn draw_box_data(&self, d: &mut RaylibDrawHandle, font: &Font, dice_box: &DiceBox) {
        let data = dice_box.get_data();

        string_utils::draw_string_centered_on_pos(
            d,
            BASE_CENTER_POS,
            &data.base_multi.to_string(),
            font,
            self.font_size,
            self.font_spacing,
            data.scoreboard_info_color,
        );

        string_utils::draw_string_centered_on_pos(
            d,
            TALLY_CENTER_POS,
            &data.tally.to_string(),
            font,
            self.font_size,
            self.font_spacing,
            data.scoreboard_info_color,
        );

        string_utils::draw_string_centered_on_pos(
            d,
            MULTI_CENTER_POS,
            &data.multi.to_string(),
            font,
            self.font_size,
            self.font_spacing,
            data.scoreboard_info_color,
        );

        string_utils::draw_string_centered_on_pos(
            d,
            TOTAL_CENTER_POS,
            &data.total_points.to_string(),
            font,
            self.font_size,
            self.font_spacing,
            data.scoreboard_info_color,
        );
    }
}
