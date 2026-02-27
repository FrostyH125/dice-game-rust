use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{math::Vector2, prelude::RaylibDrawHandle, text::Font, texture::Texture2D};

use crate::{
    entities::{
        dice::{Dice, DiceKind},
        dice_box_data::DiceBoxState,
        enemy::{EnemyData, EnemyState},
        enemy_dice_boxes::snake_eyes::SnakeEyes,
        hand::{Hand, HandState},
        player::{Player, PlayerState},
        stop_button::StopButton,
    },
    system::{input_handler::InputState, timer::Timer},
};

static SNAKE_SPRITE: Sprite = Sprite::new(176.0, 80.0, 32.0, 48.0);

pub struct Snake {
    pub data: EnemyData,
    pub hand: Hand, // 4 D4
    pub snake_eyes_box: SnakeEyes,
    dice_add_timer: Timer,
    before_stopping_dice_timer: Timer,
    before_tally_timer: Timer,
    turn_end_timer: Timer,
}

impl Snake {
    pub fn new() -> Self {
        let pos = Vector2 { x: 350.0, y: 150.0 };

        Snake {
            data: EnemyData {
                health: 100,
                pos: pos,
                state: EnemyState::WaitingForPlayer,
            },
            hand: Hand::new(vec![
                Dice::new(DiceKind::D4),
                Dice::new(DiceKind::D4),
                Dice::new(DiceKind::D4),
                Dice::new(DiceKind::D4),
            ]),
            snake_eyes_box: SnakeEyes::new(pos - Vector2 { x: 40.0, y: 0.0 }),
            dice_add_timer: Timer::new(1.0),
            before_stopping_dice_timer: Timer::new(1.0),
            before_tally_timer: Timer::new(1.0),
            turn_end_timer: Timer::new(1.0),
        }
    }

    pub fn update(&mut self, input_state: &InputState, stop_button: &mut StopButton, player: &Player, dt: f32) {
        self.hand.update(input_state, stop_button, dt);
        self.snake_eyes_box.update(&mut self.hand.dice, dt);

        match self.data.state {
            EnemyState::StartTurn => {
                self.dice_add_timer.reset();
                self.snake_eyes_box.data.reset_box(&mut self.hand.dice);
                self.hand.reset_hand();
                self.snake_eyes_box.data.state = DiceBoxState::WaitingForDice;
                self.hand.state = HandState::RollingDice;
                self.data.state = EnemyState::StartDiceStopDelayTime;
            }
            EnemyState::StartDiceStopDelayTime => {
                self.before_stopping_dice_timer.track(dt);
                if self.before_stopping_dice_timer.is_done() {
                    self.before_stopping_dice_timer.reset();
                    self.data.state = EnemyState::StoppingDice;
                    self.hand.begin_dice_stop();
                }
            }
            EnemyState::StoppingDice => {
                if self.hand.state == HandState::StoppedDice { 
                    self.data.state = EnemyState::EvaluateRoll;
                }
            }
            
            EnemyState::EvaluateRoll => {
                if self.check_for_two_dice_with_value_one_in_hand() {
                    self.data.state = EnemyState::ChoosingDice;
                } else {
                    self.data.state = EnemyState::TurnEndDelayTime;
                    self.turn_end_timer.reset();
                }
            }

            //if you got to this state, it means that theres 2 1s
            EnemyState::ChoosingDice => {
                self.dice_add_timer.track(dt);

                if self.dice_add_timer.is_done() {
                    self.dice_add_timer.reset();
                    
                    self.add_one_die();
                    
                    if self.snake_eyes_box.data.dice_in_box.len() == 2 {
                        self.data.state = EnemyState::BeforeTallyDelay;
                    } 
                }
            }
            EnemyState::BeforeTallyDelay => {
                self.before_tally_timer.track(dt);
                
                if self.before_tally_timer.is_done() {
                    self.data.state = EnemyState::TallyingTotal;
                    self.before_tally_timer.reset();
                }
            }
            
            EnemyState::TallyingTotal => {
                self.snake_eyes_box.data.total_value_for_current_round = self.snake_eyes_box.tally_snake_eyes();
                self.data.state = EnemyState::Acting;
            }
            EnemyState::Acting => {
                println!("Dealt {} Damage with snake eyes!", self.snake_eyes_box.data.total_value_for_current_round);
                self.data.state = EnemyState::TurnEndDelayTime;
            }
            EnemyState::TurnEndDelayTime => {
                self.turn_end_timer.track(dt);
                
                if self.turn_end_timer.is_done() {
                    self.hand.state = HandState::Inactive;
                    self.snake_eyes_box.data.state = DiceBoxState::Inactive;
                    self.data.state = EnemyState::WaitingForPlayer;
                }
            }
            EnemyState::WaitingForPlayer => {
                if player.state == PlayerState::WaitingForEnemy {
                    self.data.state = EnemyState::StartTurn;
                }
            }
            EnemyState::Dead => (),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        self.hand.draw(d, texture);
        self.snake_eyes_box.draw(d, texture, font);
        SNAKE_SPRITE.draw(d, self.data.pos, texture);
    }

    fn add_one_die(&mut self) {
        for i in (0..self.hand.dice.len()).rev() {
            if self.hand.dice[i].value == 1 {
                let dice = self.hand.dice.remove(i);
                self.snake_eyes_box.data.dice_in_box.push(dice);
                return;
            }
        }
    }

    fn check_for_two_dice_with_value_one_in_hand(&self) -> bool {
        let mut num_of_ones = 0;

        for dice in &self.hand.dice {
            if dice.value == 1 {
                num_of_ones += 1;
            }
        }

        if num_of_ones >= 2 {
            return true;
        } else {
            return false;
        }
    }
}
