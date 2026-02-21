use crate::system::input_handler::{InputState, MouseState};

use self::DiceState::*;
use basic_raylib_core::graphics::{
    animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance,
};
use rand::random_range;
use raylib::prelude::*;

pub const DICE_WIDTH_HEIGHT: f32 = 16.0;

pub static ROLL_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(0.0, 0.0, 16.0, 16.0),
        Sprite::new(16.0, 0.0, 16.0, 16.0),
        Sprite::new(32.0, 0.0, 16.0, 16.0),
        Sprite::new(48.0, 0.0, 16.0, 16.0),
        Sprite::new(64.0, 0.0, 16.0, 16.0),
        Sprite::new(80.0, 0.0, 16.0, 16.0),
    ],
    frame_duration: 0.2,
    should_loop: true,
};

#[derive(PartialEq)]
pub enum DiceState {
    Stopped,
    Rolling,
    Dragging,
}

pub struct Dice {
    pub pos: Vector2,
    roll_anim: SpriteAnimationInstance,
    pub value: i8,
    pub state: DiceState,
}

impl Dice {
    pub fn new() -> Dice {
        Dice {
            pos: Default::default(),
            roll_anim: SpriteAnimationInstance::default(),
            value: Default::default(),
            state: Rolling,
        }
    }

    pub fn update(&mut self, other_dice_dragged: &mut bool, input_state: &InputState, dt: f32) {
        match self.state {
            Stopped => {
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
                    // decided to do it this way so it is o(n) and not o(n * n) where each dice checks every other dice
                    // also didnt want to update the bool by checking every other dice 
                    // from the call site on every dice update iteration either, as that would also just be o(n * n)
                    // it may seem messy to pass the bool in like this and change it inside here, but i couldn't
                    // see another clean way to do it and this is the fastest
                    *other_dice_dragged = true;
                    self.state = Dragging;
                }
            }
            Rolling => ROLL_ANIM.update_roll_anim_random(&mut self.roll_anim, dt),
            Dragging => {
                if input_state.mouse_state == MouseState::Dragging {
                    self.pos = input_state.mouse_pos - Vector2 { x: DICE_WIDTH_HEIGHT / 2.0 , y: DICE_WIDTH_HEIGHT / 2.0};  
                }
                else {
                    self.state = Stopped;
                }
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
