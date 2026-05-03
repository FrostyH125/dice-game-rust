use std::{i8, usize};

use basic_raylib_core::{graphics::sprite::Sprite, system::{timer::Timer, sprite_particle_system::SpriteParticleSystem}};
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::Font,
    texture::Texture2D,
};

// define the closing anim
// have the anim play

use crate::{
    LARGE_DUST_SPRITE, SMALL_DUST_SPRITE, VIRTUAL_HEIGHT, VIRTUAL_WIDTH,
    entities::{
        dice::{DICE_WIDTH_HEIGHT, Dice, DiceKind, DiceState},
        hand::Hand,
    },
    system::{info_hover::InfoHover, input_handler::InputState},
};

pub const CURRENT_STREAK_OFFSET: Vector2 = Vector2::new(0.0, 20.0);
pub const TOTAL_VALUE_OFFSET: Vector2 = Vector2::new(60.0, -31.0);
pub const BASE_MULTI_OFFSET: Vector2 = Vector2::new(30.0, 7.0);
pub const DICE_CENTER_OF_SCREEN_POS: Vector2 =
    Vector2::new(VIRTUAL_WIDTH / 2.0 - DICE_WIDTH_HEIGHT / 2.0, VIRTUAL_HEIGHT / 2.0 - DICE_WIDTH_HEIGHT / 2.0);

pub const DICE_BORDER_OFFSET: Vector2 = Vector2::new(-1.0, -1.0);
pub const DICE_BORDER_SIZE: Vector2 = Vector2::new(DICE_WIDTH_HEIGHT + 2.0, DICE_WIDTH_HEIGHT + 2.0);

pub const D6_DICE_BORDER_SPRITE: Sprite = Sprite::new(223.0, 15.0, 18.0, 18.0);
pub const D4_DICE_BORDER_SPRITE: Sprite = Sprite::new(191.0, 15.0, 18.0, 18.0);

pub const DICE_DRAW_START_OFFSET: Vector2 = Vector2::new(34.0, -15.0);
pub const DICE_POINT_OFFSET_FOR_DETECTING_IF_INSIDE_BOX: Vector2 =
    Vector2::new(DICE_WIDTH_HEIGHT / 2.0, DICE_WIDTH_HEIGHT / 2.0);

// standard measurements for the average box size (small boxes will have different measurements)
pub const STANDARD_BOX_COLLECT_RECT_OFFSET_X: f32 = 2.0;
pub const STANDARD_BOX_COLLECT_RECT_OFFSET_Y: f32 = -31.0;
pub const STANDARD_BOX_COLLECT_RECT_WIDTH: f32 = 48.0;
pub const STANDARD_BOX_COLLECT_RECT_HEIGHT: f32 = 32.0;
pub const STANDARD_BOX_WIDTH: f32 = 52.0;
pub const STANDARD_BOX_HEIGHT: f32 = 16.0;

pub struct DiceBoxData {
    pub dice_in_box: Vec<Dice>,
    pub info_hover: InfoHover,
    pub current_index_of_dice_just_tallied: Option<usize>,
    pub total_tally: f64,
    pub total_multi_for_this_tally: f64,
    pub base_multi_for_this_dice_box: f64,
    pub total_value_for_current_round: f64,
    pub pos: Vector2,
    pub width: f32,
    pub height: f32,
    pub dice_collect_rect: Rectangle,
    pub timer_for_tallying_dice: Timer,
    pub previous_dice_value: i8,
    pub current_streak: i8,
    pub collect_rect_offset_x: f32,
    pub collect_rect_offset_y: f32,
}

impl DiceBoxData {
    pub fn new(collect_rect_offset_x: f32, collect_rect_offset_y: f32, collect_rect_width: f32, collect_rect_height: f32, dice_box_width: f32, dice_box_height: f32, info_hover: InfoHover) -> Self {
        
        DiceBoxData {
            dice_in_box: Vec::new(),
            info_hover,
            current_index_of_dice_just_tallied: None,
            total_tally: 0.0,
            total_multi_for_this_tally: 1.0,
            base_multi_for_this_dice_box: 1.0,
            
            // some boxes use this value in equality, im making sure it is actually equal no matter what, since floats are weird
            total_value_for_current_round: 0.0f64.floor(),
            pos: Vector2::zero(),
            dice_collect_rect: Rectangle::new(collect_rect_offset_x, collect_rect_offset_y, collect_rect_width, collect_rect_height),
            width: dice_box_width,
            height: dice_box_height,
            timer_for_tallying_dice: Timer::new(1.5),
            previous_dice_value: i8::MAX,
            current_streak: 1,
            collect_rect_offset_x,
            collect_rect_offset_y,
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

    pub fn update_dice_for_player(
        &mut self,
        is_player_dragging_any_dice: &mut bool,
        hand_stopped: bool,
        input_state: &InputState,
        dt: f32,
    ) {
        for i in 0..self.dice_in_box.len() {
            self.dice_in_box[i].update_for_player(is_player_dragging_any_dice, hand_stopped, input_state, dt);
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

            self.total_tally += current_dice.value as f64;

            if continue_streak {
                self.current_streak += 1;
                self.total_multi_for_this_tally += 1.0;
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

    pub fn reset_box(&mut self, hand_dice: &mut Vec<Dice>, dice_origin_pos: Vector2) {
        while let Some(mut dice) = self.dice_in_box.pop() {
            // this is so when the hand arranges itself, the dice will come
            // from this spot. they wont be drawn or reset until the turn starts
            // though
            dice.pos = dice_origin_pos;
            hand_dice.push(dice);
        }

        self.total_value_for_current_round = 0.0f64.floor();
        self.current_index_of_dice_just_tallied = None;
        self.current_streak = 1;
        self.previous_dice_value = i8::MAX;
        self.timer_for_tallying_dice.reset();
        self.total_multi_for_this_tally = 1.0;
        self.total_tally = 0.0;
    }

    pub fn set_dice_positions(&mut self) {
        let mut target_pos = self.pos + DICE_DRAW_START_OFFSET;
        let mut times_increased_x = 0;

        for i in (0..self.dice_in_box.len()).rev() {
            match self.dice_in_box[i].state {
                // do NOT reorganize this dice when its being dragged
                DiceState::Dragging => (),
                _ => {
                    let old_pos = self.dice_in_box[i].pos;
                    self.dice_in_box[i].state = DiceState::Rearranging {
                        old_pos,
                        target_pos,
                        should_roll_after: false,
                    };
                }
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

    pub fn get_value(&self) -> f64 {
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

    pub fn handle_dragging_dice(&mut self, hand: &mut Hand) {
        for i in (0..self.dice_in_box.len()).rev() {
            if !self
                .dice_collect_rect
                .check_collision_point_rec(self.dice_in_box[i].pos + DICE_POINT_OFFSET_FOR_DETECTING_IF_INSIDE_BOX)
                && matches!(self.dice_in_box[i].state, DiceState::Stopped)
            {
                let dice = self.dice_in_box.remove(i);
                hand.dice.push(dice);
                hand.arrange_hand(false);
            }
        }
    }

    pub fn emit_smoke_at_each_dice(&mut self, particle_system: &mut SpriteParticleSystem) {
        for dice in &mut self.dice_in_box {
            let cycles_for_this_dice = rand::random_range(15..=25);

            for _ in 0..=cycles_for_this_dice {
                let sprite = match rand::random_bool(0.5) {
                    true => &SMALL_DUST_SPRITE,
                    false => &LARGE_DUST_SPRITE,
                };

                let particle_pos_x = rand::random_range(dice.pos.x..=dice.pos.x + DICE_WIDTH_HEIGHT);
                let particle_pos_y =
                    rand::random_range(dice.pos.y + DICE_WIDTH_HEIGHT - 4.0..=dice.pos.y + DICE_WIDTH_HEIGHT);
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

    // these draw methods are the rebirth of my old api for this
    // now, theyre not required for implementing a new dice box, simply a tool
    // to use if the dice box doesnt deviate much and wants to use it
    // like the enemy and player attack methods in dice_box.rs
    pub fn draw_base_multi(&self, d: &mut RaylibDrawHandle, font: &Font, color: Color) {
        d.draw_text_ex(
            font,
            &format!("x{}", self.base_multi_for_this_dice_box),
            self.pos + BASE_MULTI_OFFSET,
            3.0,
            0.0,
            color,
        );
    }

    pub fn draw_info_sprite_and_information(&self, d: &mut RaylibDrawHandle, font: &Font, color: Color) {
        let no_dice_counted_yet = self.current_index_of_dice_just_tallied == None;
        if no_dice_counted_yet {
            return;
        }

        let base = self.base_multi_for_this_dice_box;
        let tally = self.total_tally;
        let multi = self.total_multi_for_this_tally;

        d.draw_text_ex(
            font,
            &format!(
                "total:\n{} tally\n* {} multi \n* {} base\n= {}!",
                tally,
                multi,
                base,
                tally * multi * base
            ),
            self.pos + TOTAL_VALUE_OFFSET,
            8.0,
            0.0,
            color,
        );
    }

    pub fn draw_current_streak(&self, d: &mut RaylibDrawHandle, font: &Font, color: Color) {
        let streak = self.current_streak;

        if streak <= 1 {
            return;
        }

        d.draw_text_ex(font, &format!("Streak {} !", streak), self.pos + CURRENT_STREAK_OFFSET, 8.0, 0.0, color);
    }
}
