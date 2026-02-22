use crate::{entities::hand::HandState, system::input_handler::{InputState, MouseState}};

use self::DiceState::*;
use basic_raylib_core::graphics::{
    animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance,
};
use rand::random_range;
use raylib::prelude::*;

pub const DICE_WIDTH_HEIGHT: f32 = 16.0;

#[derive(PartialEq)]
pub enum DiceState {
    Stopped,
    Rolling,
    Dragging,
}

pub enum DiceKind {
    SixSided,
}

pub struct Dice {
    pub pos: Vector2,
    pub roll_anim: SpriteAnimationInstance,
    pub value: i8,
    pub state: DiceState,
    kind: DiceKind,
}

impl Dice {
    pub fn new(kind: DiceKind) -> Dice {
        Dice {
            pos: Default::default(),
            roll_anim: SpriteAnimationInstance::default(),
            value: Default::default(),
            state: Rolling,
            kind
        }
    }

    pub fn update(&mut self, other_dice_dragged: &mut bool, hand_state: &HandState, input_state: &InputState, dt: f32) {
        match self.state {
            DiceState::Stopped => {
                if *hand_state != HandState::StoppedDice {
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
                    if rect.check_collision_point_rec(input_state.mouse_pos) {
                        true
                    } else {
                        false
                    }
                };

                if mouse_dragging && mouse_over_this && !*other_dice_dragged {
                    *other_dice_dragged = true;
                    self.state = DiceState::Rolling;
                }
            }
            DiceState::Dragging => {
                if input_state.mouse_state == MouseState::Dragging {
                    self.pos = input_state.mouse_pos - Vector2 { x: DICE_WIDTH_HEIGHT / 2.0 , y: DICE_WIDTH_HEIGHT / 2.0};
                }
                else {
                    self.state = DiceState::Stopped;
                }
            }
            DiceState::Rolling => {
                match self.kind {
                    DiceKind::SixSided => self.update_six_sided(other_dice_dragged, hand_state, input_state, dt),
                }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        match self.state {
            Stopped => {
                let frame_to_draw = self.value as usize - 1;
                ROLL_ANIM.frames[frame_to_draw].draw(d, self.pos, texture)
            }
            Rolling => ROLL_ANIM.draw(&self.roll_anim, d, texture, self.pos),
            Dragging => {
                let frame_to_draw = self.value as usize - 1;
                ROLL_ANIM.frames[frame_to_draw].draw(d, self.pos, texture)
            }
        }
    }

    pub fn stop(&mut self) {
        let new_value = random_range(1..=6);
        self.value = new_value;
        self.state = Stopped;
    }

    pub fn reset(&mut self) {
        self.state = Rolling;
    }
}

//only using because i cant actually implement it on the sprite animation instance itself
trait RandomDiceAnimUpdate {
    fn update_roll_anim_random(&self, animation_instance: &mut SpriteAnimationInstance, dt: f32);
}

impl RandomDiceAnimUpdate for AnimationData {
    fn update_roll_anim_random(&self, animation_instance: &mut SpriteAnimationInstance, dt: f32) {
        animation_instance.current_frame_time += dt;

        while animation_instance.current_frame_time >= self.frame_duration {
            let new_frame_index = random_range(0..=5);
            animation_instance.current_frame_index = new_frame_index;
            animation_instance.current_frame_time -= self.frame_duration;
        }
    }
}
