use raylib::{math::Vector2, prelude::RaylibDrawHandle, text::Font, texture::Texture2D};

use crate::{entities::{enemies::snake::Snake, player::Player, stop_button::StopButton}, system::input_handler::InputState};

#[derive(PartialEq)]
pub enum EnemyState {
    //enemy owns hand and boxes
    StartTurn,
    RollingDice,
    StoppingDice,
    ChoosingDice,
    TallyingTotal,
    Acting,
    WaitingForPlayer,
    Dead
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
    pub fn get_data(&self) -> &EnemyData {
        match self {
            Self::Snake { snake } => &snake.data,
        }
    }

    pub fn new_snake() -> Self {
        Self::Snake { snake: Snake::new() }
    }

    pub fn update(&mut self, input_state: &InputState, stop_button: &mut StopButton, player: &Player, dt: f32) {
        match self {
            Self::Snake { snake } => snake.update(input_state, stop_button, player, dt),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        match self {
            Self::Snake { snake } => snake.draw(d, texture, font),
        }
    }
}
