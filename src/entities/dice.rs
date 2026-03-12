use crate::{
    entities::{
        dice_box_data::{D4_DICE_BORDER_SPRITE, D6_DICE_BORDER_SPRITE},
    },
    system::input_handler::{InputState, MouseState},
};

use self::DiceState::*;
use basic_raylib_core::{
    graphics::{animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance},
    system::timer::Timer,
    utils::math_utils::smooth_lerp,
};
use rand::random_range;
use raylib::prelude::*;

pub const DICE_WIDTH_HEIGHT: f32 = 16.0;
pub const DICE_ROLL_FRAME_DURATION: f32 = 0.2;

pub static D6_ROLL_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(0.0, 0.0, DICE_WIDTH_HEIGHT, DICE_WIDTH_HEIGHT),
        Sprite::new(16.0, 0.0, DICE_WIDTH_HEIGHT, DICE_WIDTH_HEIGHT),
        Sprite::new(32.0, 0.0, DICE_WIDTH_HEIGHT, DICE_WIDTH_HEIGHT),
        Sprite::new(48.0, 0.0, DICE_WIDTH_HEIGHT, DICE_WIDTH_HEIGHT),
        Sprite::new(64.0, 0.0, DICE_WIDTH_HEIGHT, DICE_WIDTH_HEIGHT),
        Sprite::new(80.0, 0.0, DICE_WIDTH_HEIGHT, DICE_WIDTH_HEIGHT),
    ],
    frame_duration: DICE_ROLL_FRAME_DURATION,
    should_loop: true,
};

pub static D4_ROLL_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(96.0, 0.0, DICE_WIDTH_HEIGHT, DICE_WIDTH_HEIGHT),
        Sprite::new(112.0, 0.0, DICE_WIDTH_HEIGHT, DICE_WIDTH_HEIGHT),
        Sprite::new(128.0, 0.0, DICE_WIDTH_HEIGHT, DICE_WIDTH_HEIGHT),
        Sprite::new(144.0, 0.0, DICE_WIDTH_HEIGHT, DICE_WIDTH_HEIGHT),
    ],
    frame_duration: DICE_ROLL_FRAME_DURATION,
    should_loop: true,
};

#[derive(PartialEq, Copy, Clone)]
pub enum DiceState {
    Stopped,
    Rearranging { old_pos: Vector2, target_pos: Vector2, should_roll_after: bool },
    Rolling,
    Dragging,
}

pub enum DiceKind {
    D4,
    D6,
}

impl DiceKind {
    pub fn num_of_sides(&self) -> i8 {
        match self {
            DiceKind::D4 => 4,
            DiceKind::D6 => 6,
        }
    }

    pub fn roll_anim(&self) -> &AnimationData {
        match self {
            DiceKind::D4 => &D4_ROLL_ANIM,
            DiceKind::D6 => &D6_ROLL_ANIM,
        }
    }

    pub fn outline_sprite(&self) -> &Sprite {
        match self {
            DiceKind::D4 => &D4_DICE_BORDER_SPRITE,
            DiceKind::D6 => &D6_DICE_BORDER_SPRITE,
        }
    }
}

pub struct Dice {
    stopped_frame_to_draw: usize,
    pub pos: Vector2,
    pub roll_anim: SpriteAnimationInstance,
    rearranging_timer: Timer,
    pub state: DiceState,
    pub kind: DiceKind,
    pub value: i8,
}

impl Dice {
    pub fn new(kind: DiceKind) -> Dice {
        Dice {
            pos: Default::default(),
            roll_anim: SpriteAnimationInstance::default(),
            value: Default::default(),
            state: Rolling,
            kind,
            rearranging_timer: Timer::new(0.25),
            stopped_frame_to_draw: Default::default(),
        }
    }

    pub fn update_for_enemy(&mut self, dt: f32) {
        match self.state {
            DiceState::Rolling => self.update_roll_anim_random(dt),
            Rearranging { old_pos, target_pos , should_roll_after } => {
                self.rearranging_timer.track(dt);

                if self.rearranging_timer.is_done() {
                    
                    let next_state = match should_roll_after {
                        true => DiceState::Rolling,
                        false => DiceState::Stopped,
                    };
                    
                    self.state = next_state;
                    self.pos = target_pos;
                    self.rearranging_timer.reset();

                    //if you dont return here, the value of the timer will be 0.0, and the pos will get set to old pos
                    return;
                }

                let current_time = self.rearranging_timer.current_time;
                let total_duration = self.rearranging_timer.duration;

                self.pos.x = smooth_lerp(old_pos.x, target_pos.x, current_time, total_duration);
                self.pos.y = smooth_lerp(old_pos.y, target_pos.y, current_time, total_duration)
            }
            _ => (),
        }
    }

    pub fn update_for_player(
        &mut self,
        other_dice_dragged: &mut bool,
        hand_stopped: bool,
        input_state: &InputState,
        dt: f32,
    ) {
        match self.state {
            DiceState::Stopped => {
                if !hand_stopped {
                    return;
                }

                let mouse_dragging = input_state.mouse_state == MouseState::Dragging;
                let mouse_over_this = {
                    let rect = Rectangle {
                        x: self.pos.x,
                        y: self.pos.y,
                        width: DICE_WIDTH_HEIGHT,
                        height: DICE_WIDTH_HEIGHT,
                    };
                    if rect.check_collision_point_rec(input_state.mouse_pos) { true } else { false }
                };

                if mouse_dragging && mouse_over_this && !*other_dice_dragged {
                    *other_dice_dragged = true;
                    self.state = DiceState::Dragging;
                }
            }
            Rearranging { old_pos, target_pos , should_roll_after } => {
                self.rearranging_timer.track(dt);

                if self.rearranging_timer.is_done() {
                    
                    let next_state = match should_roll_after {
                        true => DiceState::Rolling,
                        false => DiceState::Stopped,
                    };
                    
                    self.state = next_state;
                    self.pos = target_pos;
                    self.rearranging_timer.reset();

                    //if you dont return here, the value of the timer will be 0.0, and the pos will get set to old pos
                    return;
                }

                let current_time = self.rearranging_timer.current_time;
                let total_duration = self.rearranging_timer.duration;

                self.pos.x = smooth_lerp(old_pos.x, target_pos.x, current_time, total_duration);
                self.pos.y = smooth_lerp(old_pos.y, target_pos.y, current_time, total_duration)
            }
            DiceState::Dragging => {
                if input_state.mouse_state == MouseState::Dragging {
                    self.pos = input_state.mouse_pos
                        - Vector2 {
                            x: DICE_WIDTH_HEIGHT / 2.0,
                            y: DICE_WIDTH_HEIGHT / 2.0,
                        };
                } else {
                    self.state = DiceState::Stopped;
                }
            }
            DiceState::Rolling => self.update_roll_anim_random(dt),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let anim = self.kind.roll_anim();

        match self.state {
            Rolling => anim.draw(&self.roll_anim, d, self.pos, texture),
            _ => anim.frames[self.stopped_frame_to_draw].draw(d, self.pos, texture),
        }
    }

    pub fn stop(&mut self) {
        let new_value = random_range(1..=self.kind.num_of_sides());
        self.value = new_value;
        self.stopped_frame_to_draw = self.value as usize - 1;
        self.state = Stopped;
    }

    pub fn reset(&mut self) {
        self.state = Rolling;
    }

    pub fn update_roll_anim_random(&mut self, dt: f32) {
        self.roll_anim.current_frame_time += dt;

        while self.roll_anim.current_frame_time >= DICE_ROLL_FRAME_DURATION {
            let new_frame_index = random_range(0..=self.kind.num_of_sides() as u8 - 1);
            self.roll_anim.current_frame_index = new_frame_index;
            self.roll_anim.current_frame_time -= DICE_ROLL_FRAME_DURATION;
        }
    }
}
