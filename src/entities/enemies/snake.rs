use raylib::{math::Vector2, prelude::RaylibDrawHandle, texture::Texture2D};

use crate::entities::{
    dice::{Dice, DiceKind},
    dice_box_data::DiceBoxState,
    enemy::{EnemyData, EnemyState},
    enemy_dice_boxes::snake_eyes::SnakeEyes,
    hand::{Hand, HandState}, player::{Player, PlayerState},
};

const TIME_PER_DICE_CHOOSING: f32 = 1.0;

pub struct Snake {
    pub data: EnemyData,
    pub hand: Hand, // 4 D4
    pub snake_eyes_box: SnakeEyes,
    dice_add_timer: f32,
}

impl Snake {
    pub fn new() -> Self {
        let pos = Vector2 { x: 300.0, y: 20.0 };

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
        }
    }

    pub fn update(&mut self, dt: f32, player: Player) {
        match self.data.state {
            EnemyState::StartTurn => {
                self.snake_eyes_box.data.state = DiceBoxState::WaitingForDice;
                self.hand.state = HandState::RollingDice;
            }
            EnemyState::RollingDice => {
                if self.hand.state == HandState::StoppedDice {
                    if self.check_for_two_dice_with_value_one_in_hand() {
                        //actually add the dice
                        self.data.state = EnemyState::ChoosingDice;
                    } else {
                        //just end turn, default value for box data is 0 anyway
                        self.data.state = EnemyState::WaitingForPlayer;
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
                self.data.state = EnemyState::WaitingForPlayer
            },
            EnemyState::WaitingForPlayer => {
                if player.state == PlayerState::WaitingForEnemy {
                    self.data.state = EnemyState::Resetting;
                }
            },
            EnemyState::Resetting => {
                self.dice_add_timer = 0.0;
                self.snake_eyes_box.data.reset_box(&mut self.hand.dice);
                self.hand.reset_hand();
                self.data.state = EnemyState::StartTurn;
            },
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        
    }

    fn add_one_die(&mut self) {
        for i in 0..self.hand.dice.len() {
            if self.hand.dice[i].value == 1 {
                let dice = self.hand.dice.remove(i);
                self.snake_eyes_box.data.dice_in_box.push(dice);
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
