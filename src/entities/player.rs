use basic_raylib_core::{
    graphics::{animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance},
    system::timer::Timer,
};
use raylib::{math::Vector2, prelude::RaylibDrawHandle, text::Font, texture::Texture2D};

use crate::{
    entities::{
        dice::{Dice, DiceKind},
        enemy::{Enemy, EnemyState},
        hand::{Hand, HandState},
    },
    system::{button::Button, input_handler::InputState},
};
use crate::{
    entities::{dice::DiceState, player_dice_boxes::attack_dice_box::AttackDiceBox},
    system::input_handler::MouseState,
};

static PLAYER_WALK_ANIM: AnimationData = AnimationData {
    frames: &[Sprite::new(80.0, 80.0, 32.0, 48.0), Sprite::new(112.0, 80.0, 32.0, 48.0)],
    frame_duration: 0.5,
    should_loop: true,
};

static PLAYER_IDLE_SPRITE: Sprite = Sprite::new(144.0, 80.0, 32.0, 48.0);

#[derive(PartialEq)]
pub enum PlayerState {
    Walking,   // waiting for enemy
    StartTurn, // setting hand and boxes to proper state
    WaitingForDiceToMoveToHand,
    RollingDice,
    StoppingDice, // can't pick up dice until this finishes
    RerollingDice,
    ChoosingDice,        // selecting which dice go in which box
    TallyingAttackTotal, // wait for box to tally dice
    BeforeAttackDelay,
    Attacking, // waiting for each box to finish its action
    EndTurnDelay,
    EndTurn,
    WaitingForEnemy, // waiting for enemy turn to finish (enemy should set this for player, enemy will have reference to player)
    HitDelay,
    Dead,
}

pub struct Player {
    pub attack_box: AttackDiceBox,
    pub hand: Hand,
    attack_power: i64,
    health: i64,
    walk_anim: SpriteAnimationInstance,
    pos: raylib::math::Vector2,
    acting_timer: Timer,
    end_turn_delay_timer: Timer,
    hit_delay_timer: Timer,
    pub state: PlayerState,
    pub is_player_dragging_dice: bool,
    pub was_player_dragging_dice: bool,
}

impl Player {
    pub fn new() -> Self {
        Player {
            attack_box: AttackDiceBox::new(),
            hand: Hand::new(std::iter::repeat_with(|| Dice::new(DiceKind::D6)).take(5).collect()),
            walk_anim: SpriteAnimationInstance::new(),
            pos: Vector2 { x: 20.0, y: 150.0 },
            health: 100,
            state: PlayerState::Walking,
            acting_timer: Timer::new(1.0),
            end_turn_delay_timer: Timer::new(2.0),
            hit_delay_timer: Timer::new(1.5),
            attack_power: 0,
            is_player_dragging_dice: false,
            was_player_dragging_dice: false,
        }
    }

    pub fn update(
        &mut self,
        input_state: &InputState,
        confirm_button: &mut Button,
        stop_button: &mut Button,
        reroll_button: &mut Button,
        enemy: &Enemy,
        dt: f32,
    ) {
        self.hand.update_for_player(&mut self.is_player_dragging_dice, &mut self.was_player_dragging_dice, input_state, dt);
        if input_state.mouse_state == MouseState::Inactive {
            self.is_player_dragging_dice = false;
        }

        if self.attack_box.data.check_for_dice_being_dragged_into_box(&mut self.hand.dice) {
            self.hand.arrange_hand(false);
            self.attack_box.data.set_dice_positions();
        }
        self.attack_box.data.update_dice(&mut self.is_player_dragging_dice, &mut self.hand, input_state, dt);
        

        match self.state {
            PlayerState::Walking => {
                PLAYER_WALK_ANIM.update(&mut self.walk_anim, dt);
            }
            PlayerState::StartTurn => {
                self.reset();
                self.state = PlayerState::WaitingForDiceToMoveToHand;
                self.hand.state = HandState::RollingDice;
            }
            PlayerState::WaitingForDiceToMoveToHand => {
                
                let mut should_move_on = false;
                
                for dice in &self.hand.dice {
                    if dice.state != DiceState::Rolling {
                        continue;
                    }
                    
                    should_move_on = true;
                }
                
                if should_move_on {
                    self.state = PlayerState::RollingDice;
                }
            }
            PlayerState::RollingDice => {
                if stop_button.is_pressed(input_state) {
                    self.hand.begin_dice_stop();
                    self.state = PlayerState::StoppingDice;
                }
            }
            PlayerState::StoppingDice => {
                if self.hand.state == HandState::StoppedDice {
                    self.state = PlayerState::ChoosingDice;
                    stop_button.reset();
                }
            }
            PlayerState::ChoosingDice => {
                if self.hand.dice.len() > 0 && reroll_button.is_pressed(input_state) {
                    self.hand.reset_hand();
                    self.hand.begin_dice_stop();
                    self.state = PlayerState::RerollingDice;
                }

                if confirm_button.is_pressed(input_state) {
                    self.state = PlayerState::TallyingAttackTotal;
                }
            }
            PlayerState::RerollingDice => {
                if self.hand.state == HandState::StoppedDice {
                    reroll_button.reset();
                    self.state = PlayerState::ChoosingDice;
                }
            }
            PlayerState::TallyingAttackTotal => {
                if self.attack_box.data.dice_in_box.is_empty() {
                    self.state = PlayerState::EndTurn;
                    confirm_button.reset();
                } else if self.attack_box.data.tally_points(dt) {
                    self.state = PlayerState::BeforeAttackDelay;
                    confirm_button.reset();
                }
            }
            PlayerState::BeforeAttackDelay => {
                self.acting_timer.track(dt);

                if self.acting_timer.is_done() {
                    self.acting_timer.reset();
                    self.state = PlayerState::Attacking;
                }
            }
            PlayerState::Attacking => {
                self.attack_power = self.attack_box.data.get_value();

                println!("dealt {} damage!", self.attack_power);

                self.state = PlayerState::EndTurnDelay;
            }
            PlayerState::EndTurnDelay => {
                self.end_turn_delay_timer.track(dt);

                if self.end_turn_delay_timer.is_done() {
                    self.state = PlayerState::EndTurn;
                }
            }
            PlayerState::EndTurn => {
                // keep data, reset it at start turn
                // block and other special values will be nice to keep
                self.hand.state = HandState::Inactive;
                self.state = PlayerState::WaitingForEnemy;
            }
            PlayerState::WaitingForEnemy => {
                if enemy.get_data().state == EnemyState::WaitingForPlayer {
                    self.state = PlayerState::StartTurn;
                }
            }
            PlayerState::HitDelay => {
                self.hit_delay_timer.track(dt);
                if self.hit_delay_timer.is_done() {
                    if self.health <= 0 {
                        self.state = PlayerState::Dead
                    } else {
                        self.state = PlayerState::WaitingForEnemy;
                    }
                }
            }
            PlayerState::Dead => (),
        }

        self.was_player_dragging_dice = self.is_player_dragging_dice;
    }

    pub fn reset(&mut self) {
        self.attack_box.reset(&mut self.hand.dice);
        self.hand.reset_hand();
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        match self.state {
            PlayerState::Walking => PLAYER_WALK_ANIM.draw(&self.walk_anim, d, texture, self.pos),
            PlayerState::WaitingForEnemy => {
                PLAYER_IDLE_SPRITE.draw(d, self.pos, texture);
            }
            _ => {
                PLAYER_IDLE_SPRITE.draw(d, self.pos, texture);
                self.attack_box.draw(d, texture, font);
                self.hand.draw(d, texture);
            }
        }

        if !self.is_player_dragging_dice {
            return;
        }

        for dice in &mut self.hand.dice {
            if dice.state == DiceState::Dragging {
                dice.draw(d, texture);
            }
        }
        for dice in &mut self.attack_box.data.dice_in_box {
            if dice.state == DiceState::Dragging {
                dice.draw(d, texture);
            }
        }
    }

    pub fn hit(&mut self, damage: i64) {
        self.health -= damage;
        self.state = PlayerState::HitDelay;
    }
}
