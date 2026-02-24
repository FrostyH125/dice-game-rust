use raylib::{math::Vector2, prelude::RaylibDrawHandle, rgui::RaylibDrawGui, texture::Texture2D};

use crate::entities::{
    dice::{Dice, DiceKind}, enemy_data::{EnemyData, EnemyState}, enemy_dice_boxes::snake_eyes::SnakeEyes, hand::Hand
};

pub struct Snake {
    data: EnemyData,
    hand: Hand, // 3 D4
    snake_eyes_box: SnakeEyes,
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
            hand: Hand::new(vec![Dice::new(DiceKind::D4), Dice::new(DiceKind::D4), Dice::new(DiceKind::D4)]),
            snake_eyes_box: SnakeEyes::new(pos - Vector2 { x: 40.0, y: 0.0 })
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        match self.data.state {
            EnemyState::StartTurn => todo!("set box to waiting and hand to rolling, then set hand to stopping"),
            EnemyState::RollingDice => todo!("if hand is stopped, set self to choosing dice"),
            EnemyState::ChoosingDice => todo!("if there are 2 ones in the hand, put them in snake eyes, otherwise, state = waiting for player"),
            EnemyState::TallyingTotal => todo!("count through the two ones, finish by resulting in 25, get data"),
            EnemyState::Acting => todo!("attack with snake eyes"),
            EnemyState::WaitingForPlayer => todo!("if player is waiting, reset"),
            EnemyState::Resetting => todo!("reset, start turn"),
            
            // WHEN DONE: refactor attack_dice_box into controlling its own flow, remove the control flow of dice_box_data before its too late.
            // boxes should control their own flow explicitely, not much can be generalized
        }
    }
    
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        
    }
    
    //update, draw
}
