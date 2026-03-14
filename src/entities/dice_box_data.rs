use std::{i8, usize};

use basic_raylib_core::{graphics::sprite::Sprite, system::timer::Timer};
use raylib::{
    math::{Rectangle, Vector2},
    prelude::RaylibDrawHandle,
    texture::Texture2D,
};

use crate::{
    LARGE_DUST_SPRITE, SMALL_DUST_SPRITE, entities::{
        dice::{DICE_WIDTH_HEIGHT, Dice, DiceKind, DiceState},
        hand::Hand,
    }, system::{info_hover::InfoHover, input_handler::InputState, particle_system::ParticleSystem}
};

pub const CURRENT_STREAK_OFFSET: Vector2 = Vector2 { x: 0.0, y: 20.0 };
pub const TOTAL_VALUE_OFFSET: Vector2 = Vector2 { x: 52.0, y: -31.0 };
pub const BASE_MULTI_OFFSET: Vector2 = Vector2 { x: 20.0, y: 7.0 };

pub const DICE_BORDER_OFFSET: Vector2 = Vector2 { x: -1.0, y: -1.0 };
pub const DICE_BORDER_SIZE: Vector2 = Vector2 {
    x: DICE_WIDTH_HEIGHT + 2.0,
    y: DICE_WIDTH_HEIGHT + 2.0,
};

pub const D6_DICE_BORDER_SPRITE: Sprite = Sprite::new(223.0, 15.0, 18.0, 18.0);
pub const D4_DICE_BORDER_SPRITE: Sprite = Sprite::new(191.0, 15.0, 18.0, 18.0);

pub const DICE_DRAW_START_OFFSET: Vector2 = Vector2 { x: 34.0, y: -15.0 };
pub const DICE_POINT_OFFSET_FOR_DETECTING_IF_INSIDE_BOX: Vector2 = Vector2 {
    x: DICE_WIDTH_HEIGHT / 2.0,
    y: DICE_WIDTH_HEIGHT / 2.0,
};

pub struct DiceBoxData {
    pub dice_in_box: Vec<Dice>,
    pub info_hover: InfoHover,
    pub current_index_of_dice_just_tallied: Option<usize>,
    pub total_tally: i64,
    pub total_multi_for_this_tally: i64,
    pub base_multi_for_this_dice_box: i64,
    pub total_value_for_current_round: i64,
    pub pos: Vector2,
    pub dice_collect_rect: Rectangle,
    pub timer_for_tallying_dice: Timer,
    pub previous_dice_value: i8,
    pub current_streak: i8,
}

impl DiceBoxData {
    pub fn new(pos: Vector2, dice_collect_rect: Rectangle, info_hover: InfoHover) -> Self {
        DiceBoxData {
            dice_in_box: Vec::new(),
            info_hover,
            current_index_of_dice_just_tallied: None,
            total_tally: 0,
            total_multi_for_this_tally: 1,
            base_multi_for_this_dice_box: 1,
            total_value_for_current_round: 0,
            pos,
            dice_collect_rect,
            timer_for_tallying_dice: Timer::new(1.5),
            previous_dice_value: i8::MAX,
            current_streak: 0,
        }
    }
}

impl DiceBoxData {
    pub fn pull_in_dragged_dice(&mut self, dice_in_hand: &mut Vec<Dice>) {
        for i in (0..dice_in_hand.len()).rev() {
            match dice_in_hand[i].state {
                DiceState::Stopped => {
                    if self
                        .dice_collect_rect
                        .check_collision_point_rec(dice_in_hand[i].pos + DICE_POINT_OFFSET_FOR_DETECTING_IF_INSIDE_BOX)
                    {
                        let dice_to_add = dice_in_hand.remove(i);
                        self.dice_in_box.push(dice_to_add);
                        self.dice_in_box.sort_by(|a, b| a.value.cmp(&b.value));
                    }
                }
                _ => (),
            }
        }
    }
    
    pub fn update_dice_for_enemy(&mut self, dt: f32) {
        for dice in &mut self.dice_in_box {
            dice.update_for_enemy(dt);
        }
    }

    //dice box being empty handled by call site
    pub fn tally_points(&mut self, dt: f32) -> bool {
        let is_first = self.current_index_of_dice_just_tallied == None;
        self.timer_for_tallying_dice.track(dt);

        if self.timer_for_tallying_dice.is_done() || is_first {
            match &mut self.current_index_of_dice_just_tallied {
                None => self.current_index_of_dice_just_tallied = Some(0),
                Some(index) => *index += 1,
            };

            self.timer_for_tallying_dice.reset();
            let current_dice = &self.dice_in_box[self.current_index_of_dice_just_tallied.unwrap()];

            let continue_streak = self.previous_dice_value == current_dice.value;

            let should_reset_streak = !is_first && !continue_streak;

            self.total_tally += current_dice.value as i64;

            if continue_streak {
                self.current_streak += 1;
                self.total_multi_for_this_tally += 1;
            }

            if should_reset_streak {
                self.current_streak = 1;
            }

            println!(
                "Current tally: {}, Current Multi: {}, Current Streak: {}, value of the dice just tallied: {}",
                self.total_tally, self.total_multi_for_this_tally, self.current_streak, current_dice.value
            );

            if self.current_index_of_dice_just_tallied.unwrap() == self.dice_in_box.len() - 1 {
                return true;
            }

            self.previous_dice_value = current_dice.value;
        }

        return false;
    }

    pub fn reset_box(&mut self, hand_dice: &mut Vec<Dice>) {
        while let Some(dice) = self.dice_in_box.pop() {
            hand_dice.push(dice);
        }

        self.total_value_for_current_round = 0;
        self.current_index_of_dice_just_tallied = None;
        self.current_streak = 0;
        self.previous_dice_value = i8::MAX;
        self.timer_for_tallying_dice.reset();
        self.total_multi_for_this_tally = 1;
        self.total_tally = 0;
    }

    pub fn set_dice_positions(&mut self) {
        let mut target_pos = self.pos + DICE_DRAW_START_OFFSET;
        let mut times_increased_x = 0;

        for i in (0..self.dice_in_box.len()).rev() {
            if self.dice_in_box[i].state != DiceState::Dragging {
                let old_pos = self.dice_in_box[i].pos;
                self.dice_in_box[i].state = DiceState::Rearranging {
                    old_pos,
                    target_pos,
                    should_roll_after: false,
                };
            }
            target_pos.x -= DICE_WIDTH_HEIGHT;
            times_increased_x += 1;
            if times_increased_x == 3 {
                target_pos.x += DICE_WIDTH_HEIGHT * 3.0;
                target_pos.y -= DICE_WIDTH_HEIGHT;
                times_increased_x = 0;
            }
        }
    }

    pub fn get_value(&self) -> i64 {
        return self.total_tally * self.base_multi_for_this_dice_box * self.total_multi_for_this_tally;
    }

    pub fn draw_dice(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let mut dice_being_dragged: Option<&mut Dice> = None;

        for dice in &mut self.dice_in_box {
            dice.draw(d, texture);
            match dice.state {
                DiceState::Dragging => {
                    dice_being_dragged = Some(dice);
                }
                _ => (),
            }
        }

        if let Some(dragged_dice) = dice_being_dragged {
            dragged_dice.draw(d, texture);
        }
    }

    pub fn draw_border_around_current_dice(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        if self.current_index_of_dice_just_tallied == None {
            return;
        }

        let sprite = match self.dice_in_box[self.current_index_of_dice_just_tallied.unwrap()].kind {
            DiceKind::D4 => &D4_DICE_BORDER_SPRITE,
            DiceKind::D6 => &D6_DICE_BORDER_SPRITE,
        };

        let pos = self.dice_in_box[self.current_index_of_dice_just_tallied.unwrap()].pos + DICE_BORDER_OFFSET;

        sprite.draw(d, pos, texture);
    }

    pub fn handle_dragging_dice(
        &mut self,
        is_player_dragging_any_dice: &mut bool,
        hand: &mut Hand,
        input_state: &InputState,
        dt: f32,
    ) {
        let hand_stopped = hand.all_dice_stopped_passive_check();

        for i in (0..self.dice_in_box.len()).rev() {
            self.dice_in_box[i].update_for_player(is_player_dragging_any_dice, hand_stopped, input_state, dt);
            if !self
                .dice_collect_rect
                .check_collision_point_rec(self.dice_in_box[i].pos + DICE_POINT_OFFSET_FOR_DETECTING_IF_INSIDE_BOX)
                && self.dice_in_box[i].state == DiceState::Stopped
            {
                let dice = self.dice_in_box.remove(i);
                hand.dice.push(dice);
                hand.arrange_hand(false);
            }
        }
    }

    pub fn emit_smoke_at_each_dice(&mut self, particle_system: &mut ParticleSystem) {
        for dice in &mut self.dice_in_box {
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
