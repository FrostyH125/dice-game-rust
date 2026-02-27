use std::i8;

use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{
    color::Color, math::{Rectangle, Vector2}, prelude::{RaylibDraw, RaylibDrawHandle}, text::Font, texture::Texture2D
};

use crate::entities::dice::{DICE_WIDTH_HEIGHT, Dice, DiceKind, DiceState};

#[derive(PartialEq, Debug)]
pub enum DiceBoxState {
    WaitingForDice,
    TallyingPoints,
    WaitingForAction,
    Inactive,
}

// big multi number that gets bigger as the number gets bigger, "x2, x3, x4," etc
pub const CURRENT_MULTI_OFFSET: Vector2 = Vector2 { x: 60.0, y: -40.0 };

// smaller number set inside of the box itself, denotes the current base multiplier
// most likely will be determined by weapon
// will also likely have a symbol next to it denoting the type of damage it is
// "slash, blunt, pierce, fire, lightning, etc"
// will also be symbols for healing and blocking as well
pub const BASE_MULTI_OFFSET: Vector2 = Vector2 { x: 20.0, y: 7.0 };

// planning for this to be under the scoring box
// will continuously count a streak if its over 1
// "2 streak!, 3 streak!, 4 streak!"
// will likely get bigger as the streak gets larger as well
pub const CURRENT_STREAK_OFFSET: Vector2 = Vector2 { x: 0.0, y: 18.0 };

// where the border needs to be drawn relative to the dice
pub const CURRENT_DICE_BORDER_OFFSET: Vector2 = Vector2 { x: -1.0, y: -1.0 };
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
    pub current_index_dice_being_tallied: usize,
    pub total_tally: i64,
    pub total_multi_for_this_tally: i64,
    pub base_multi_for_this_dice_box: i64,
    pub total_value_for_current_round: i64,
    pub pos: Vector2,
    pub state: DiceBoxState,
    pub dice_collect_rect: Rectangle,
    pub timer_for_tallying_dice: f32,
    pub previous_dice_value: i8,
    pub current_streak: i8,
}

impl DiceBoxData {
    pub fn new(pos: Vector2, dice_collect_rect: Rectangle) -> Self {
        DiceBoxData {
            dice_in_box: Vec::new(),
            current_index_dice_being_tallied: 0,
            total_tally: 0,
            total_multi_for_this_tally: 1,
            base_multi_for_this_dice_box: 1,
            total_value_for_current_round: 0,
            pos,
            state: DiceBoxState::Inactive,
            dice_collect_rect,
            timer_for_tallying_dice: 0.0,
            previous_dice_value: i8::MAX,
            current_streak: 1,
        }
    }
}

impl DiceBoxData {
    pub fn check_for_dice_being_dragged_into_box(&mut self, dice_in_hand: &mut Vec<Dice>) {
        for i in (0..dice_in_hand.len()).rev() {
            if dice_in_hand[i].state == DiceState::Stopped
                && self
                    .dice_collect_rect
                    .check_collision_point_rec(dice_in_hand[i].pos + DICE_POINT_OFFSET_FOR_DETECTING_IF_INSIDE_BOX)
            {
                let dice_to_add = dice_in_hand.remove(i);
                self.dice_in_box.push(dice_to_add);
                self.dice_in_box.sort_by(|a, b| a.value.cmp(&b.value));
            }
        }
    }

    //dice box being empty handled by call site
    pub fn tally_points(&mut self, dt: f32) -> bool {
        let time_between_dice = 1.0;

        self.timer_for_tallying_dice += dt;

        if self.current_index_dice_being_tallied == 0 || self.timer_for_tallying_dice >= time_between_dice {
            self.timer_for_tallying_dice = 0.0;
            let current_dice = &self.dice_in_box[self.current_index_dice_being_tallied];

            let is_last_dice = self.current_index_dice_being_tallied == self.dice_in_box.len() - 1;
            let continue_streak = self.previous_dice_value == current_dice.value;

            let is_first = self.current_index_dice_being_tallied == 0;
            let should_finalize = !is_first && (!continue_streak || is_last_dice);
            
            self.total_tally += current_dice.value as i64;

            if continue_streak {
                self.current_streak += 1;
            }

            // if you dont skip the first dice, the other statement will always return true
            
            if should_finalize {
                if self.current_streak > 1 {
                    self.total_multi_for_this_tally += self.current_streak as i64;
                }
            
                if !is_last_dice {
                    self.current_streak = 1;
                }
            }

            println!(
                "Current tally: {}, Current Multi: {}, value of the dice just tallied: {}",
                self.total_tally, self.total_multi_for_this_tally, current_dice.value
            );

            if self.current_index_dice_being_tallied == self.dice_in_box.len() - 1 {
                //self will reset after all things are used in a reset() function
                return true;
            }

            self.previous_dice_value = current_dice.value;
            self.current_index_dice_being_tallied += 1;
            self.timer_for_tallying_dice = 0.0;
        }
        return false;
    }

    pub fn reset_box(&mut self, hand_dice: &mut Vec<Dice>) {
        while let Some(dice) = self.dice_in_box.pop() {
            hand_dice.push(dice);
        }

        self.total_value_for_current_round = 0;
        self.state = DiceBoxState::WaitingForDice;
        self.current_index_dice_being_tallied = 0;
        self.current_streak = 1;
        self.previous_dice_value = i8::MAX;
        self.timer_for_tallying_dice = 0.0;
        self.total_multi_for_this_tally = 1;
        self.total_tally = 0;
    }

    pub fn set_dice_positions(&mut self) {
        let mut draw_pos = self.pos + DICE_DRAW_START_OFFSET;
        let mut times_increased_x = 0;

        for i in (0..self.dice_in_box.len()).rev() {
            self.dice_in_box[i].pos = draw_pos;
            draw_pos.x -= DICE_WIDTH_HEIGHT;
            times_increased_x += 1;
            if times_increased_x == 3 {
                draw_pos.x += DICE_WIDTH_HEIGHT * 3.0;
                draw_pos.y -= DICE_WIDTH_HEIGHT;
                times_increased_x = 0;
            }
        }
    }

    pub fn get_value(&self) -> i64 {
        return self.total_tally * self.base_multi_for_this_dice_box * self.total_multi_for_this_tally;
    }
    
    //UNUSED FUNCTION BEING PREPPED FOR POTENTIAL FUTURE USAGE. THE DRAWING CAN MAYBE BE SIMPLIFIED INTO ONE FUNCTION EASILY
    // WITH NO COMPLEXITY
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, dice_box_sprite: &Sprite, font: &Font, color: Color) {
        match self.state {
            DiceBoxState::Inactive => return,
            _ => {
                dice_box_sprite.draw(d, self.pos, texture);
                d.draw_rectangle_lines(
                    self.dice_collect_rect.x as i32,
                    self.dice_collect_rect.y as i32,
                    self.dice_collect_rect.width as i32,
                    self.dice_collect_rect.height as i32,
                    Color::WHITE,
                );
                //draw_multi(font, color);
                //draw_base_multi(font, color);
                //draw_border_around_currently_being_tallied_dice();
            }
        }
    }
    pub fn draw_dice(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        for i in 0..self.dice_in_box.len() {
            self.dice_in_box[i].draw(d, texture);
        }
    }
    
    pub fn draw_border_around_current_dice(&mut self) {
        
        let sprite = match self.dice_in_box[self.current_index_dice_being_tallied].kind {
            DiceKind::D4 => &D4_DICE_BORDER_SPRITE,
            DiceKind::D6 => &D6_DICE_BORDER_SPRITE,
        };
        
        todo!("need to draw border around currently being tallied dice")
    }
    
    pub fn draw_arrow_to_current_dice() {
        todo!("need to draw arrow to currently being tallied dice, arrow should bob up and down i think")
    }
}

//need to update dice in order to check if theyre being picked up or not
pub fn update_dice() {
    todo!("need to impl update dice")
}
