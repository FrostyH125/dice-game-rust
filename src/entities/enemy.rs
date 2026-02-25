use raylib::{math::Vector2, prelude::RaylibDrawHandle, texture::Texture2D};

use crate::entities::{
    dice::{Dice, DiceKind},
    enemies::snake::{self, Snake},
    enemy_dice_boxes::snake_eyes::SnakeEyes,
    hand::Hand,
};

pub enum EnemyState {
    //enemy owns hand and boxes
    StartTurn,
    RollingDice,
    ChoosingDice,
    TallyingTotal,
    Acting,
    WaitingForPlayer,
    Resetting,
}

pub struct EnemyData {
    pub health: i64,
    pub pos: Vector2,
    pub state: EnemyState,
}

pub enum Enemy {
    Snake { snake: Snake },
}

impl Enemy {
    pub fn new_snake() -> Self {
        Self::Snake { snake: Snake::new() }
    }
    
    pub fn update(&mut self, dt: f32) {
        match self {
            Self::Snake { snake } => snake.update(dt),
        }
    }
    
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        match self {
            Self::Snake { snake } => snake.draw(d, texture),
        }
    }
}
