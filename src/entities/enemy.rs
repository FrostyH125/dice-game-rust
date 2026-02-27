use raylib::{math::Vector2, prelude::RaylibDrawHandle, text::Font, texture::Texture2D};

use crate::{entities::{enemies::snake::Snake, player::Player, stop_button::StopButton}, system::input_handler::InputState};

#[derive(PartialEq)]
pub enum EnemyState {
    
    // reset hands and boxes
    StartTurn,
    
    // exists only to smoothly transition from start turn to
    // actually letting the dice stop. Should have a timer, and
    // once the timer goes off, put all the hands to start stopping
    // their dice
    StartDiceStopDelayTime,
    
    // waits for the dice to be stopped
    StoppingDice,
    
    // once the hand is stopped, chooses to either choose dice based on the 
    // roll (mostly important for special condition boxes), or go straight to
    // ending the turn
    EvaluateRoll,
    
    // actually chooses which dice to add to their box
    ChoosingDice,
    
    // some transition time between choosing the final die, and tallying
    BeforeTallyDelay,
    
    TallyingTotal,
    Acting,
    
    // exists purely for visual cohesiveness, every enemy will need it
    // should be used as the primary location (exclusively, which i know is possible)
    // of turning hands and boxes inactive. These elements should be visible all the
    // way until the enemy is no longer active, and turn is passed to player
    TurnEndDelayTime,
    
    // should be a simple check to see if player is waiting for enemy, and then
    // if so, start turn
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
