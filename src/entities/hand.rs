use super::dice::{DICE_WIDTH_HEIGHT, Dice};
use basic_raylib_core::system::timer::Timer;
use raylib::prelude::*;

use super::super::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH};
use crate::{LARGE_DUST_SPRITE, SMALL_DUST_SPRITE, entities::dice::DiceState, system::{input_handler::InputState, particle_system::ParticleSystem}};

pub const DICE_Y_OFFSET: f32 = 72.0;
const HAND_MARGIN_BETWEEN_DICE: f32 = 10.0;

pub struct Hand {
    pub dice: Vec<Dice>,
    current_index_of_dice_stopping: usize,
    dice_stop_timer: Timer,
}

impl Hand {
    pub fn new(dice: Vec<Dice>) -> Self {
        Hand {
            dice,
            current_index_of_dice_stopping: Default::default(),
            dice_stop_timer: Timer::new(1.0),
        }
    }

    // doesnt require any player input
    pub fn update_for_enemy(&mut self, dt: f32) {
        for i in (0..self.dice.len()).rev() {
            self.dice[i].update_for_enemy(dt);
        }
    }

    pub fn update_for_player(
        &mut self,
        player_dragging_any_dice: &mut bool,
        input_state: &InputState,
        dt: f32,
    ) {
        let hand_stopped = self.all_dice_stopped_passive_check();

        for i in (0..self.dice.len()).rev() {
            self.dice[i].update_for_player(player_dragging_any_dice, hand_stopped, input_state, dt);
        }
    }

    //rolling variable says whether the dice should go to rolling after moving to its new location or not
    pub fn arrange_hand(&mut self, should_roll_after: bool) {
        
        if self.dice.is_empty() {
            return;
        }
        
        let num_of_dice = self.dice.len();
        let number_of_margins = num_of_dice - 1;

        let total_width = DICE_WIDTH_HEIGHT * num_of_dice as f32 + number_of_margins as f32 * HAND_MARGIN_BETWEEN_DICE;

        let start_pos_x = VIRTUAL_WIDTH / 2.0 - total_width / 2.0;
        let pos_y = VIRTUAL_HEIGHT - DICE_Y_OFFSET;
        let mut pos_x = start_pos_x;

        for i in 0..num_of_dice {
            
            let old_pos = self.dice[i].pos;
            let target_pos = Vector2 { x: pos_x, y: pos_y };

            self.dice[i].state = DiceState::Rearranging { old_pos, target_pos, should_roll_after };
            pos_x += DICE_WIDTH_HEIGHT + HAND_MARGIN_BETWEEN_DICE;
        }
    }

    pub fn begin_dice_stop(&mut self) {
        self.dice_stop_timer.duration = 2.0 / self.dice.len() as f32;
        self.current_index_of_dice_stopping = 0;
        self.dice_stop_timer.reset();
    }

    pub fn stop_dice(&mut self, dt: f32) -> bool {
        self.dice_stop_timer.track(dt);

        if self.dice_stop_timer.is_done() {
            self.dice_stop_timer.reset();

            self.dice[self.current_index_of_dice_stopping].stop();
            self.current_index_of_dice_stopping += 1;

            //is done
            if self.current_index_of_dice_stopping >= self.dice.len() {
                return true;
            }
        }
        return false;
    }
    
    pub fn roll_dice(&mut self) {
        for dice in &mut self.dice {
            dice.state = DiceState::Rolling;
        }
    }

    pub fn reset_hand(&mut self) {
        for i in 0..self.dice.len() {
            self.dice[i].reset();
        }
        
        self.arrange_hand(true);
    }
    
    pub fn all_dice_stopped_passive_check(&self) -> bool {
        for dice in &self.dice {
            if dice.state == DiceState::Rolling {
                return false;
            }
        }
        
        return true;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        
        let mut dice_being_dragged: Option<&mut Dice> = None;

        for dice in &mut self.dice {
            
            dice.draw(d, texture);
            
            match dice.state {
                DiceState::Dragging => {
                    dice_being_dragged = Some(dice);
                }
                _ => ()
            }
        }
        
        if let Some(dragged_dice) = dice_being_dragged {
            dragged_dice.draw(d, texture);
        }
    }
    
    pub fn emit_smoke_at_each_dice(&mut self, particle_system: &mut ParticleSystem) {
        for dice in &mut self.dice {
            let cycles_for_this_dice = rand::random_range(15..=25);

            for _ in 0..=cycles_for_this_dice {
                
                let sprite = match rand::random_bool(0.5) {
                    true => &SMALL_DUST_SPRITE,
                    false => &LARGE_DUST_SPRITE,
                };

                let particle_pos_x = rand::random_range(dice.pos.x..=dice.pos.x + DICE_WIDTH_HEIGHT);
                let particle_pos_y = rand::random_range(dice.pos.y + DICE_WIDTH_HEIGHT - 4.0..=dice.pos.y + DICE_WIDTH_HEIGHT);
                let position = Vector2::new(particle_pos_x, particle_pos_y);

                let velocity_y = rand::random_range(1.0..=15.0);
                let velocity = Vector2::new(0.0, velocity_y);

                let acceleration_x = rand::random_range(-5.0..=5.0);
                let acceleration_y = rand::random_range(-60.0..=-40.0);
                let acceleration = Vector2::new(acceleration_x, acceleration_y);
                
                let lifetime = rand::random_range(1.0..=2.0);

                particle_system.emit(sprite, position, velocity, acceleration, lifetime);
            }
        }
    }
}
