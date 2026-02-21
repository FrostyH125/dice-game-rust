use basic_raylib_core::graphics::{animation_data::AnimationData, sprite::Sprite};

use crate::entities::{attack_dice_box::AttackDiceBox, hand::Hand};

static PLAYER_WALK_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(80.0, 80.0, 32.0, 48.0),
        Sprite::new(112.0, 80.0, 32.0, 48.0),
    ],
    frame_duration: 0.5,
    should_loop: true,
};

static PLAYER_IDLE_SPRITE: Sprite = Sprite::new(144.0, 80.0, 32.0, 48.0);

pub enum PlayerState {
    Walking, // waiting for enemy
    PreparingForBattle, // setting hand and boxes to proper state
    RollingDice, // can't pick up dice until this finishes
    ChoosingDice, //selecting which dice go in which box
    Acting, // waiting for each box to finish its action
    Resetting //setting hand and box to inactive
}

pub struct player {
    attack_box: AttackDiceBox,
    hand: Hand,
    state: PlayerState
}