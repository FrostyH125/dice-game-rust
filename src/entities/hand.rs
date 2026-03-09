use super::dice::{DICE_WIDTH_HEIGHT, Dice};
use basic_raylib_core::system::timer::Timer;
use raylib::prelude::*;

use super::super::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH};
use crate::{entities::dice::DiceState, system::input_handler::InputState};

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
    dice_stop_timer: Timer,
    pub state: HandState,
}

impl Hand {
    pub fn new(dice: Vec<Dice>) -> Self {
        Hand {
            dice,
            current_index_of_dice_stopping: Default::default(),
            dice_stop_timer: Timer::new(1.0),
            state: HandState::Inactive,
        }
    }

    // doesnt require any player input
    pub fn update_for_enemy(&mut self, dt: f32) {
        for i in (0..self.dice.len()).rev() {
            self.dice[i].update_for_enemy(dt);
        }

        match self.state {
            HandState::StoppingDice => {
                if self.stop_dice(dt) {
                    self.state = HandState::StoppedDice;
                }
            }
            _ => (),
        }
    }

    pub fn update_for_player(
        &mut self,
        player_dragging_any_dice: &mut bool,
        was_player_dragging_dice: &bool,
        input_state: &InputState,
        dt: f32,
    ) {
        if !*player_dragging_any_dice && *was_player_dragging_dice {
            self.arrange_hand();
        }

        for i in (0..self.dice.len()).rev() {
            self.dice[i].update_for_player(player_dragging_any_dice, &self.state, input_state, dt);
        }

        match self.state {
            HandState::StoppingDice => {
                if self.stop_dice(dt) {
                    self.state = HandState::StoppedDice;
                }
            }
            _ => (),
        }
    }

    //rolling variable says whether the dice should go to rolling after moving to its new location or not
    pub fn arrange_hand(&mut self) {
        let num_of_dice = self.dice.len() as f32;
        let number_of_margins = num_of_dice - 1.0;

        let total_width = DICE_WIDTH_HEIGHT * num_of_dice + number_of_margins * HAND_MARGIN_BETWEEN_DICE;

        let start_pos_x = VIRTUAL_WIDTH / 2.0 - total_width / 2.0;
        let pos_y = VIRTUAL_HEIGHT - DICE_Y_OFFSET;
        let mut pos_x = start_pos_x;

        for i in 0..num_of_dice as usize {
            
            if self.dice[i].state == DiceState::WaitingToBeAssigned {
                pos_x += DICE_WIDTH_HEIGHT + HAND_MARGIN_BETWEEN_DICE;                                  
                continue;
            }
            
            let old_pos = self.dice[i].pos;
            let target_pos = Vector2 { x: pos_x, y: pos_y };

            self.dice[i].old_state = self.dice[i].state;
            self.dice[i].state = DiceState::Rearranging { old_pos, target_pos };
            pos_x += DICE_WIDTH_HEIGHT + HAND_MARGIN_BETWEEN_DICE;
        }
    }

    pub fn begin_dice_stop(&mut self) {
        self.dice_stop_timer.duration = 2.0 / self.dice.len() as f32;
        self.current_index_of_dice_stopping = 0;
        self.dice_stop_timer.reset();
        self.state = HandState::StoppingDice;
    }

    fn stop_dice(&mut self, dt: f32) -> bool {
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

    pub fn reset_hand(&mut self) {
        for i in 0..self.dice.len() {
            self.dice[i].reset();
        }

        self.state = HandState::Inactive;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        if self.state == HandState::Inactive {
            return;
        }

        for i in 0..self.dice.len() {
            self.dice[i].draw(d, texture);
        }
    }
}
