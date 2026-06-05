use std::{i8, usize};

use basic_raylib_core::{
    graphics::sprite::Sprite,
    system::{input_handler::InputState, sprite_particle_system::SpriteParticleSystem, timer::Timer},
};
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
    system::info_hover::InfoHover,
};

pub const CURRENT_STREAK_OFFSET: Vector2 = Vector2::new(0.0, 20.0);
pub const TOTAL_VALUE_OFFSET: Vector2 = Vector2::new(60.0, -31.0);
pub const BASE_MULTI_OFFSET: Vector2 = Vector2::new(30.0, 7.0);
pub const DICE_CENTER_OF_SCREEN_POS: Vector2 =
    Vector2::new(VIRTUAL_WIDTH / 2.0 - DICE_WIDTH_HEIGHT / 2.0, VIRTUAL_HEIGHT / 2.0 - DICE_WIDTH_HEIGHT / 2.0);

pub const DICE_BORDER_OFFSET: Vector2 = Vector2::new(-1.0, -1.0);
pub const DICE_BORDER_SIZE: Vector2 = Vector2::new(DICE_WIDTH_HEIGHT + 2.0, DICE_WIDTH_HEIGHT + 2.0);

pub const D6_DICE_BORDER_SPRITE: Sprite = Sprite::new(223, 15, 18, 18);
pub const D4_DICE_BORDER_SPRITE: Sprite = Sprite::new(191, 15, 18, 18);

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

const NO_DICE_COUNTED_INDEX: usize = usize::MAX;

pub struct DiceBoxData {
    pub dice_in_box: Vec<Dice>,
    pub info_hover: InfoHover,
    pub current_dice_index: usize,
    pub tally: f64,
    pub multi: f64,
    pub base_multi: f64,
    pub total_points: f64,
    pub scoreboard_info_color: Color,
    pub pos: Vector2,
    pub width: f32,
    pub height: f32,
    pub dice_collect_rect: Rectangle,
    pub dice_tally_timer: Timer,
    pub previous_dice_value: i8,
    pub current_streak: i8,
    pub collect_rect_offset_x: f32,
    pub collect_rect_offset_y: f32,
}

impl DiceBoxData {
    pub fn new(
        collect_rect_offset_x: f32,
        collect_rect_offset_y: f32,
        collect_rect_width: f32,
        collect_rect_height: f32,
        dice_box_width: f32,
        dice_box_height: f32,
        info_hover: InfoHover,
        scoreboard_info_color: Color,
        base_multi: f64,
    ) -> Self {
        DiceBoxData {
            dice_in_box: Vec::new(),
            info_hover,
            scoreboard_info_color,
            current_dice_index: NO_DICE_COUNTED_INDEX,
            tally: 0.0,
            multi: 1.0,
            base_multi,
            total_points: 0.0f64,
            pos: Vector2::zero(),
            dice_collect_rect: Rectangle::new(
                collect_rect_offset_x,
                collect_rect_offset_y,
                collect_rect_width,
                collect_rect_height,
            ),
            width: dice_box_width,
            height: dice_box_height,
            dice_tally_timer: Timer::new(1.5),
            previous_dice_value: i8::MAX,
            current_streak: 1,
            collect_rect_offset_x,
            collect_rect_offset_y,
        }
    }
}

impl DiceBoxData {
    pub fn pull_in_dragged_dice(&mut self, hand: &mut Hand) {
        
        for i in (0..hand.dice.len()).rev() {
            match hand.dice[i].state {
                DiceState::Stopped => {
                    if self
                        .dice_collect_rect
                        .check_collision_point_rec(hand.dice[i].pos + DICE_POINT_OFFSET_FOR_DETECTING_IF_INSIDE_BOX)
                    {
                        let dice_to_add = hand.remove_dice(i);
                        self.add_dice(dice_to_add);
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
        is_player_dragging_any_dice: bool,
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
        self.dice_tally_timer.track(dt);

        let is_first = self.current_dice_index == NO_DICE_COUNTED_INDEX;

        if self.dice_tally_timer.is_done() || is_first {
            self.dice_tally_timer.reset();

            if self.current_dice_index == NO_DICE_COUNTED_INDEX {
                self.current_dice_index = 0;
            } else {
                self.current_dice_index += 1;
            }

            let current_dice = &self.dice_in_box[self.current_dice_index];

            let continue_streak = self.previous_dice_value == current_dice.value;

            let should_reset_streak = !is_first && !continue_streak;

            self.tally += current_dice.value as f64;

            if continue_streak {
                self.current_streak += 1;
                self.multi += 1.0;
            }

            if should_reset_streak {
                self.current_streak = 1;
            }

            self.total_points = self.get_value();
            
            println!(
                "Current tally: {}, Current Multi: {}, Current Streak: {}, value of the dice just tallied: {}",
                self.tally, self.multi, self.current_streak, current_dice.value
            );

            if self.current_dice_index == self.dice_in_box.len() - 1 {
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

        self.total_points = 0.0f64;
        self.current_dice_index = NO_DICE_COUNTED_INDEX;
        self.current_streak = 1;
        self.previous_dice_value = i8::MAX;
        self.dice_tally_timer.reset();
        self.multi = 1.0;
        self.tally = 0.0;
    }


    // this function may need to be altered in the future to allow for more specialize dice arrangements,
    // that would be perfectly fine to do as long as you avoid messes
    pub fn arrange_dice(&mut self) {
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

    /// returns total points at the dice box data's current values
    pub fn get_value(&self) -> f64 {
        return self.tally * self.base_multi * self.multi;
    }

    pub fn draw_dice(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let mut dice_being_dragged: Option<&Dice> = None;

        for dice in &self.dice_in_box {
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

    pub fn draw_border_around_current_dice(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        if self.current_dice_index == NO_DICE_COUNTED_INDEX {
            return;
        }

        let sprite = match self.dice_in_box[self.current_dice_index].kind {
            DiceKind::D4 => &D4_DICE_BORDER_SPRITE,
            DiceKind::D6 => &D6_DICE_BORDER_SPRITE,
        };

        let pos = self.dice_in_box[self.current_dice_index].pos + DICE_BORDER_OFFSET;

        sprite.draw(d, pos, texture);
    }

    pub fn check_if_any_dice_need_to_go_back_to_hand(&mut self, hand: &mut Hand) {
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
        d.draw_text_ex(font, &format!("x{}", self.base_multi), self.pos + BASE_MULTI_OFFSET, 3.0, 0.0, color);
    }

    pub fn are_any_dice_being_dragged(&self) -> bool {
        for dice in &self.dice_in_box {
            if let DiceState::Dragging = dice.state {
                return true;
            }
        }
        return false;
    }

    pub fn place(&mut self, pos: Vector2) {
        let dice_collect_rect_offset_x = self.dice_collect_rect.x - pos.x;
        let dice_collect_rect_offset_y = self.dice_collect_rect.y - pos.y;

        self.pos = pos;
        self.dice_collect_rect = Rectangle {
            x: pos.x + dice_collect_rect_offset_x,
            y: pos.y + dice_collect_rect_offset_y,
            width: self.dice_collect_rect.width,
            height: self.dice_collect_rect.height,
        };
        self.info_hover.activation_rect =
            Rectangle::new(pos.x, pos.y, self.info_hover.activation_rect.width, self.info_hover.activation_rect.height);
    }

    pub fn add_dice(&mut self, dice: Dice) {
        self.dice_in_box.push(dice);
        self.arrange_dice();
    }

    pub fn remove_dice(&mut self, index_to_remove: usize) -> Dice {
        let dice = self.dice_in_box.remove(index_to_remove);
        self.arrange_dice();
        return dice;
    }
}
