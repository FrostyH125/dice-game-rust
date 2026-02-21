use basic_raylib_core::graphics::{
    animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance,
};
use raylib::{math::Vector2, prelude::RaylibDrawHandle, texture::Texture2D};

use crate::{
    entities::{
        attack_dice_box::AttackDiceBox,
        confirm_button::ConfirmButton,
        dice_box::DiceBoxState,
        hand::{Hand, HandState},
    },
    system::input_handler::InputState,
};

static PLAYER_WALK_ANIM: AnimationData = AnimationData {
    frames: &[Sprite::new(80.0, 80.0, 32.0, 48.0), Sprite::new(112.0, 80.0, 32.0, 48.0)],
    frame_duration: 0.5,
    should_loop: true,
};

static PLAYER_IDLE_SPRITE: Sprite = Sprite::new(144.0, 80.0, 32.0, 48.0);

#[derive(PartialEq)]
pub enum PlayerState {
    Walking,            // waiting for enemy
    PreparingForBattle, // setting hand and boxes to proper state
    RollingDice,        // can't pick up dice until this finishes
    ChoosingDice,       // selecting which dice go in which box
    TallyingTotal,      // wait for box to tally dice
    Acting,             // waiting for each box to finish its action
    WaitingForEnemy, // waiting for enemy turn to finish (enemy should set this for player, enemy will have reference to player)
    Resetting,       //setting hand and box to inactive
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
                if self.walk_anim.current_frame_index == 1 && self.walk_anim.current_frame_time > 0.49 {
                    self.state = PlayerState::PreparingForBattle;
                    self.walk_anim.reset();
                }
            }
            PlayerState::PreparingForBattle => {
                self.hand.state = HandState::RollingDice;
                self.attack_box.data.state = DiceBoxState::WaitingForDice;
                self.state = PlayerState::RollingDice;
            }
            PlayerState::RollingDice => {
                if self.hand.state == HandState::StoppedDice {
                    self.state = PlayerState::ChoosingDice;
                }
            }
            PlayerState::ChoosingDice => {  
                
                let waiting = self.attack_box.data.state == DiceBoxState::WaitingForDice;
                let any_box_has_dice = self.attack_box.data.state == DiceBoxState::TallyingPoints;
                
                if any_box_has_dice && !waiting {
                    self.state = PlayerState::TallyingTotal;
                } else if !waiting && !any_box_has_dice {
                    self.state = PlayerState::Resetting;
                }
            }
            PlayerState::TallyingTotal => {
                let attack_box_acting = self.attack_box.data.state == DiceBoxState::Acting;
                if attack_box_acting {
                    self.state = PlayerState::Acting;
                }
            }
            PlayerState::Acting => {
                if !self.attack_box.attack(dt) {
                    return;
                }
                self.state = PlayerState::Resetting;
            }
            PlayerState::WaitingForEnemy => (),
            PlayerState::Resetting => {
                self.attack_box.reset(&mut self.hand.dice);
                self.hand.reset_dice();
                self.state = PlayerState::Walking;
            }
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        match self.state {
            PlayerState::Walking => PLAYER_WALK_ANIM.draw(&self.walk_anim, d, texture, self.pos),
            _ => {
                PLAYER_IDLE_SPRITE.draw(d, self.pos, texture);
                self.hand.draw(d, texture);
                self.attack_box.draw(d, texture);
            }
        }
    }
}
