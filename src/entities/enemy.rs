use raylib::{math::Vector2, prelude::RaylibDrawHandle, text::Font, texture::Texture2D};
use basic_raylib_core::system::timer::Timer;
use crate::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH, entities::{enemies::snake::Snake, player::Player}, system::{input_handler::InputState, particle_system::ParticleSystem}};


pub const ENEMY_HAND_X_CENTER_CORD: f32 = VIRTUAL_WIDTH - 100.0;
pub const ENEMY_HAND_Y_CORD: f32 = 195.0;

#[derive(PartialEq)]
pub enum EnemyState {
    
    // reset hands and boxes
    StartTurn,
    
    // waits for hand to recieve all dice physically before continuing
    WaitingForDiceToReturnToHand,
    
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
    
    BeforeActingDelay, // used for special visuals depending on the action
    
    Acting,
    
    // handles being hit, animation for being hit, other effects for being hit, before turning back to waiting for player
    HitDelay,
    
    //the delay before fully ending turn for seamless, sensible transitions 
    EndTurnDelay,
    
    // should be a simple check to see if player is waiting for enemy, and then
    // if so, start turn
    WaitingForPlayer,
    Dead
}

pub struct EnemyData {
    pub health: i64,
    pub pos: Vector2,
    pub state: EnemyState,
    pub hit_timer: Timer,
}

pub enum Enemy {
    Snake { snake: Snake },
}

impl Enemy { 
    fn get_mut_data(&mut self) -> &mut EnemyData {
        match self {
            Self::Snake { snake } => &mut snake.data
        }
    }
    
    pub fn get_data(&self) -> &EnemyData {
        match self {
            Self::Snake { snake } => &snake.data,
        }
    }
    
    pub fn take_hit(&mut self, damage: i64) {
        self.get_mut_data().health -= damage;
        self.get_mut_data().state = EnemyState::HitDelay;
    }


    pub fn update(&mut self, input_state: &InputState, player: &mut Player, particle_system: &mut ParticleSystem, dt: f32) {
        match self {
            Self::Snake { snake } => snake.update(input_state, player, particle_system, dt),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        match self {
            Self::Snake { snake } => snake.draw(d, texture, font),
        }
    }
}
