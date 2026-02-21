use basic_raylib_core::graphics::{animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance};
use raylib::{math::Vector2, prelude::RaylibDrawHandle, texture::Texture2D};

use crate::{entities::{attack_dice_box::AttackDiceBox, confirm_button::ConfirmButton, dice_box::DiceBoxState, hand::{Hand, HandState}}, system::input_handler::InputState};

static PLAYER_WALK_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(80.0, 80.0, 32.0, 48.0),
        Sprite::new(112.0, 80.0, 32.0, 48.0),
    ],
    frame_duration: 0.5,
    should_loop: true,
};

static PLAYER_IDLE_SPRITE: Sprite = Sprite::new(144.0, 80.0, 32.0, 48.0);

#[derive(PartialEq)]
pub enum PlayerState {
    Walking, // waiting for enemy
    PreparingForBattle, // setting hand and boxes to proper state
    RollingDice, // can't pick up dice until this finishes
    ChoosingDice, //selecting which dice go in which box
    Acting, // waiting for each box to finish its action
    WaitingForEnemy, // waiting for enemy turn to finish (enemy should set this for player, enemy will have reference to player)
    Resetting //setting hand and box to inactive
}

pub struct Player {
    pub attack_box: AttackDiceBox,
    hand: Hand,
    walk_anim: SpriteAnimationInstance,
    pos: raylib::math::Vector2,
    pub state: PlayerState,
}

impl Player {
    
    pub fn new() -> Self {
        Player {
            attack_box: AttackDiceBox::new(),
            hand: Hand::new(),
            walk_anim: SpriteAnimationInstance::new(),
            pos: Vector2 { x: 20.0, y: 150.0 },
            state: PlayerState::Walking,
        }
    } 
    
    pub fn update(&mut self, input_state: &InputState, confirm_button: &mut ConfirmButton, dt: f32) {
        
        self.attack_box.update(&mut self.hand.dice, input_state, confirm_button, dt);
        self.hand.update(input_state, dt);
        
        match self.state {
            PlayerState::Walking => {
                PLAYER_WALK_ANIM.update(&mut self.walk_anim, dt);
            },
            PlayerState::PreparingForBattle => {
                self.hand.state = HandState::RollingDice;
                self.attack_box.data.state = DiceBoxState::WaitingForDice;
                self.state = PlayerState::RollingDice;
            },
            PlayerState::RollingDice => {
                if self.hand.state == HandState::StoppedDice {
                    self.state = PlayerState::ChoosingDice;
                }
            },
            PlayerState::ChoosingDice => {
                if confirm_button.is_pressed(input_state) {
                    self.state = PlayerState::Acting;
                }
            },
            PlayerState::Acting => {
                if !self.attack_box.attack(dt) {
                    return;
                }
            },
            PlayerState::WaitingForEnemy => (),
            PlayerState::Resetting => {
                self.attack_box.reset(&mut self.hand.dice);
                self.hand.reset_dice();
            },
        }
    }
    
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        match self.state {
            PlayerState::Walking => PLAYER_WALK_ANIM.draw(&self.walk_anim, d, texture, self.pos),
            _ => {
                PLAYER_IDLE_SPRITE.draw(d, self.pos, texture);
                self.hand.draw(d, texture);
                self.attack_box.draw(d, texture);
            },
        }
    }
}