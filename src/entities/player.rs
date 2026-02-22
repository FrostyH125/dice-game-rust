use basic_raylib_core::graphics::{
    animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance,
};
use raylib::{math::Vector2, prelude::RaylibDrawHandle, texture::Texture2D};

use crate::{
    entities::{
        attack_dice_box::AttackDiceBox,
        confirm_button::ConfirmButton,
        dice_box::DiceBoxState,
        hand::{Hand, HandState}, stop_button::StopButton,
    },
    system::input_handler::InputState,
};

static PLAYER_WALK_ANIM: AnimationData = AnimationData {
    frames: &[Sprite::new(80.0, 80.0, 32.0, 48.0), Sprite::new(112.0, 80.0, 32.0, 48.0)],
    frame_duration: 0.5,
    should_loop: true,
};

static PLAYER_IDLE_SPRITE: Sprite = Sprite::new(144.0, 80.0, 32.0, 48.0);
const TIME_BETWEEN_ACTIONS: f32 = 1.0;

#[derive(PartialEq)]
pub enum PlayerState {
    Walking,            // waiting for enemy
    PreparingForRound, // setting hand and boxes to proper state
    RollingDice,        // can't pick up dice until this finishes
    ChoosingDice,       // selecting which dice go in which box
    TallyingTotal,      // wait for box to tally dice
    Acting,             // waiting for each box to finish its action
    WaitingForEnemy, // waiting for enemy turn to finish (enemy should set this for player, enemy will have reference to player)
    Resetting,       //setting hand and box to inactive
}

pub struct Player {
    pub attack_box: AttackDiceBox,
    pub hand: Hand,
    attack_power: i64,
    walk_anim: SpriteAnimationInstance,
    pos: raylib::math::Vector2,
    pub state: PlayerState,
    acting_timer: f32,
    attacked: bool,
}

impl Player {
    pub fn new() -> Self {
        Player {
            attack_box: AttackDiceBox::new(),
            hand: Hand::new(),
            walk_anim: SpriteAnimationInstance::new(),
            pos: Vector2 { x: 20.0, y: 150.0 },
            state: PlayerState::Walking,
            acting_timer: 0.0,
            attack_power: 0,
            attacked: false,
        }
    }

    pub fn update(&mut self, input_state: &InputState, confirm_button: &mut ConfirmButton, stop_button: &mut StopButton, dt: f32) {
        self.attack_box.update(&mut self.hand.dice, dt);
        self.hand.update(input_state, stop_button, dt);

        match self.state {
            PlayerState::Walking => {
                PLAYER_WALK_ANIM.update(&mut self.walk_anim, dt);
                if self.walk_anim.current_frame_index == 1 && self.walk_anim.current_frame_time > 0.49 {
                    self.state = PlayerState::PreparingForRound;
                    self.walk_anim.reset();
                }
            }
            PlayerState::PreparingForRound => {
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
                if confirm_button.is_pressed(input_state) {
                    self.state = PlayerState::TallyingTotal;
                    self.attack_box.data.state = DiceBoxState::TallyingPoints;
                }
            }
            PlayerState::TallyingTotal => {
                
                // extend this to being "if any of the boxes are waiting for action"
                if self.attack_box.data.state == DiceBoxState::WaitingForAction {
                    self.state = PlayerState::Acting;
                    confirm_button.reset();
                }
                
                // extend this to being "if all of the boxes are inactive"
                if self.attack_box.data.state == DiceBoxState::Inactive {
                    self.state = PlayerState::Resetting;
                    confirm_button.reset();
                }
            }
            PlayerState::Acting => {
                self.acting_timer += dt;
                
                if !self.attacked {
                    if self.acting_timer >= TIME_BETWEEN_ACTIONS {
                        //attack enemy
                        self.attack_power = self.attack_box.data.total_value_for_current_round;
                        
                        println!("dealt {} damage!", self.attack_power);
                        
                        self.acting_timer = 0.0;
                        self.attacked = true;
                    }
                }
                
                // if all boxes acted (if self.attacked && self.healed && self.block_calculated && self.special)
                if self.attacked {
                    self.state = PlayerState::WaitingForEnemy;
                    self.attack_box.data.state = DiceBoxState::Inactive;
                }
            }
            PlayerState::WaitingForEnemy => (),
                // if enemy.data.state == waitingforplayer {
                //  self.state == Resetting
                // },
            PlayerState::Resetting => {
                self.attacked = false;
                self.attack_box.reset(&mut self.hand.dice);
                self.hand.reset_hand();
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
