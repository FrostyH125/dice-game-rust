use super::dice::{Dice, DICE_WIDTH_HEIGHT};
use raylib::prelude::*;

use crate::{entities::{dice::{DiceKind, DiceState}, stop_button::StopButton}, system::input_handler::InputState};
use super::super::{VIRTUAL_WIDTH, VIRTUAL_HEIGHT};

pub const DICE_Y_OFFSET: f32 = 72.0;
const HAND_MARGIN_BETWEEN_DICE: f32 = 10.0;

#[derive(PartialEq)]
pub enum HandState {
    Inactive,
    RollingDice,
    StoppingDice,
    StoppedDice,
}

pub struct Hand {
    pub dice: Vec<Dice>,
    current_index_of_dice_stopping: usize,
    dice_stop_time_per_dice: f32,
    dice_stop_timer: f32,
    pub state: HandState,
    is_any_dice_dragged: bool,
}

impl Hand {
    pub fn new(dice: Vec<Dice>) -> Self {
        Hand {
            dice,
            current_index_of_dice_stopping: Default::default(),
            dice_stop_time_per_dice: Default::default(),
            dice_stop_timer: Default::default(),
            state: HandState::RollingDice,
            is_any_dice_dragged: false
        }
    }
    
    
    pub fn update(&mut self, input_state: &InputState, stop_button: &mut StopButton, dt: f32) {
        
        self.is_any_dice_dragged = self.dice.iter().any(|dice| dice.state == DiceState::Dragging);
        
        // iterating backwards prevents the dice under other dice to be the first one
        // dragged, which is not the desired behavior
        // now the topmost dice gets updated (and subsequently dragged) first
        for i in (0..self.dice.len()).rev() {
            self.dice[i].update(&mut self.is_any_dice_dragged, &self.state, input_state, dt);
        }     
        
        match self.state {
            HandState::RollingDice => {
                if stop_button.is_pressed(input_state) {
                    self.begin_dice_stop();
                    self.state = HandState::StoppingDice;
                }
                self.set_dice_positions();
            },
            HandState::StoppingDice => {
                if self.stop_dice(dt) {
                    self.state = HandState::StoppedDice;
                    stop_button.reset();
                }
            }
            HandState::StoppedDice => {
              // check if stop button is pressed again
              // if so, handstate == rolling dice again, except the ones that youre saving 
            },
            _ => (), 
        }
    }

    pub fn set_dice_positions(&mut self) {
        let num_of_dice = self.dice.len() as f32;
        let number_of_margins = num_of_dice - 1.0;
        
        let total_width = DICE_WIDTH_HEIGHT * num_of_dice + number_of_margins * HAND_MARGIN_BETWEEN_DICE;
        
        let start_pos_x = VIRTUAL_WIDTH / 2.0 - total_width / 2.0;
        let pos_y = VIRTUAL_HEIGHT - DICE_Y_OFFSET;
        let mut pos_x = start_pos_x;
        
        for i in 0..num_of_dice as usize {
            self.dice[i].pos.x = pos_x;
            self.dice[i].pos.y = pos_y;
            
            pos_x += DICE_WIDTH_HEIGHT + HAND_MARGIN_BETWEEN_DICE;
        }
    }
    
    fn begin_dice_stop(&mut self) {
        self.dice_stop_time_per_dice = 2.0 / self.dice.len() as f32;
        self.current_index_of_dice_stopping = 0;
        self.dice_stop_timer = 0.0;
        self.state = HandState::StoppingDice;
    }
    
    fn stop_dice(&mut self, dt: f32) -> bool {
        self.dice_stop_timer += dt;
        
        if self.dice_stop_timer >= self.dice_stop_time_per_dice {
            self.stop_current_dice();
            self.dice_stop_timer = 0.0;
            
            if self.all_dice_stopped() {
                return true;
            }
        }
        
        false
    }

    fn stop_current_dice(&mut self) {
        self.dice[self.current_index_of_dice_stopping].stop();
        self.current_index_of_dice_stopping += 1;
    }
    
    fn all_dice_stopped(&self) -> bool {
        self.current_index_of_dice_stopping >= self.dice.len()
    }
    
    pub fn reset_hand(&mut self) {
        for i in 0..self.dice.len() {
            self.dice[i].reset();
        }
        
        self.state = HandState::Inactive;
    }
    
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        for i in 0..self.dice.len() {
            self.dice[i].draw(d, texture);
        }
    }
}
