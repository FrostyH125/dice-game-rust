use crate::{
    entities::hand::HandState,
    system::input_handler::{InputState, MouseState},
};

use self::DiceState::*;
use basic_raylib_core::graphics::{
    animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance,
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
    should_loop: true
};

#[derive(PartialEq)]
pub enum DiceState {
    Stopped,
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
    
    pub fn anim(&self) -> &AnimationData {
        match self {
            DiceKind::D4 => &D4_ROLL_ANIM,
            DiceKind::D6 => &D6_ROLL_ANIM,
        }
    }
}

pub struct Dice {
    pub pos: Vector2,
    pub roll_anim: SpriteAnimationInstance,
    pub value: i8,
    pub state: DiceState,
    pub kind: DiceKind,
}

impl Dice {
    pub fn new(kind: DiceKind) -> Dice {
        Dice {
            pos: Default::default(),
            roll_anim: SpriteAnimationInstance::default(),
            value: Default::default(),
            state: Rolling,
            kind,
        }
    }
    
    pub fn update_for_enemy(&mut self, dt: f32) {
        match self.state {
            DiceState::Rolling => self.update_roll_anim_random(dt),
            _ => ()
        }
    }
    

    pub fn update_for_player(&mut self, other_dice_dragged: &mut bool, hand_state: &HandState, input_state: &InputState, dt: f32) {
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
                    if rect.check_collision_point_rec(input_state.mouse_pos) { true } else { false }
                };

                if mouse_dragging && mouse_over_this && !*other_dice_dragged {
                    *other_dice_dragged = true;
                    self.state = DiceState::Dragging;
                }
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
        
        let anim = self.kind.anim();
        
        match self.state {
            Stopped => {
                let frame_to_draw = self.value as usize - 1;
                anim.frames[frame_to_draw].draw(d, self.pos, texture)
            }
            Rolling => anim.draw(&self.roll_anim, d, texture, self.pos),
            Dragging => {
                let frame_to_draw = self.value as usize - 1;
                anim.frames[frame_to_draw].draw(d, self.pos, texture)
            }
        }
    }

    pub fn stop(&mut self) {
        let new_value = random_range(1..=self.kind.num_of_sides());
        self.value = new_value;
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
