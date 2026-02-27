use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{math::Vector2, prelude::RaylibDrawHandle, text::Font, texture::Texture2D};

use crate::{entities::{
    dice::{Dice, DiceKind},
    dice_box_data::DiceBoxState,
    enemy::{EnemyData, EnemyState},
    enemy_dice_boxes::snake_eyes::SnakeEyes,
    hand::{Hand, HandState}, player::{Player, PlayerState}, stop_button::StopButton,
}, system::input_handler::InputState};

const TIME_PER_DICE_CHOOSING: f32 = 1.0;
const TIME_BEFORE_STOPPING_DICE: f32 = 1.0;
static SNAKE_SPRITE: Sprite = Sprite::new(176.0, 80.0, 32.0, 48.0);

pub struct Snake {
    pub data: EnemyData,
    pub hand: Hand, // 4 D4
    pub snake_eyes_box: SnakeEyes,
    dice_add_timer: f32,
    dice_start_stopping_dice_timer: f32,
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
            dice_add_timer: 0.0,
            dice_start_stopping_dice_timer: 0.0
        }
    }

    pub fn update(&mut self, input_state: &InputState, stop_button: &mut StopButton, player: &Player, dt: f32) {
        
        self.hand.update(input_state, stop_button, dt);
        self.snake_eyes_box.update(&mut self.hand.dice, dt);
        
        match self.data.state {
            EnemyState::StartTurn => {
                self.dice_add_timer = 0.0;
                self.snake_eyes_box.data.reset_box(&mut self.hand.dice);
                self.hand.reset_hand();
                self.snake_eyes_box.data.state = DiceBoxState::WaitingForDice;
                self.hand.state = HandState::RollingDice;
                self.data.state = EnemyState::RollingDice;
            }
            EnemyState::RollingDice => {
                self.dice_start_stopping_dice_timer += dt;
                if self.dice_start_stopping_dice_timer >= TIME_BEFORE_STOPPING_DICE {
                    self.data.state = EnemyState::StoppingDice;
                    self.dice_start_stopping_dice_timer = 0.0;
                    self.hand.begin_dice_stop();
                }
            }
            EnemyState::StoppingDice => {
                if self.hand.state == HandState::StoppedDice {
                    
                    // i chose to put this check in rolling dice instead of
                    // choosing dice because i didnt want it to do a check every
                    // single frame in choosing dice and also didnt want to
                    // add a boolean to manage for if it had two dice.
                    if self.check_for_two_dice_with_value_one_in_hand() {
                        //actually add the dice
                        self.data.state = EnemyState::ChoosingDice;
                    } else {
                        //just end turn, default value for box data is 0 anyway
                        self.data.state = EnemyState::WaitingForPlayer;
                        self.hand.state = HandState::Inactive;
                        self.snake_eyes_box.data.state = DiceBoxState::Inactive;
                    }
                }
            }

            //if you got to this state, it means that theres 2 1s
            EnemyState::ChoosingDice => {
                self.dice_add_timer += dt;

                if self.dice_add_timer >= TIME_PER_DICE_CHOOSING {
                    self.add_one_die();
                    if self.snake_eyes_box.data.dice_in_box.len() == 2 {
                        self.data.state = EnemyState::TallyingTotal;
                    } else {
                        self.dice_add_timer = 0.0;
                    }
                }
            }
            EnemyState::TallyingTotal => {
                self.snake_eyes_box.data.total_value_for_current_round = self.snake_eyes_box.tally_snake_eyes();
                self.data.state = EnemyState::Acting;
            }
            EnemyState::Acting => {
                println!("Dealt {} Damage with snake eyes!", self.snake_eyes_box.data.total_value_for_current_round);
                self.data.state = EnemyState::WaitingForPlayer;
                self.hand.state = HandState::Inactive;
                self.snake_eyes_box.data.state = DiceBoxState::Inactive;
            },
            EnemyState::WaitingForPlayer => {
                if player.state == PlayerState::WaitingForEnemy {
                    self.data.state = EnemyState::StartTurn;
                }
            },
            EnemyState::Dead => ()
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
