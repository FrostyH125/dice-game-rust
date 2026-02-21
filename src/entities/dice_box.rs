use std::i8;

use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
};

use crate::{
    entities::{
        confirm_button::ConfirmButton,
        dice::{DICE_WIDTH_HEIGHT, Dice, DiceState},
    },
    system::input_handler::InputState,
};

#[derive(PartialEq, Debug)]
pub enum DiceBoxState {
    WaitingForDice,
    TallyingPoints,
    Acting,
    Inactive,
}

const DICE_DRAW_START_OFFSET: Vector2 = Vector2 { x: 34.0, y: -15.0 };

// with any luck, the only things youll need from this mod are update() and draw(), the rest should happen automatically.
// all that is left is checking data, doing something when it changes, and thats it

pub struct DiceBoxData {
    pub dice_in_box: Vec<Dice>,
    current_index_dice_being_tallied: usize,
    pub total_tally: i64,
    pub total_multi_for_this_tally: i64,
    pub base_multi_for_this_dice_box: i64,
    pos: Vector2,
    pub state: DiceBoxState,
    dice_collection_rect: Rectangle,
    timer_for_tallying_dice: f32,
    previous_dice_value: i8,
    current_streak: i8,
}

impl DiceBoxData {
    pub fn new(pos: Vector2) -> Self {
        DiceBoxData {
            dice_in_box: Vec::new(),
            current_index_dice_being_tallied: 0,
            total_tally: 0,
            total_multi_for_this_tally: 1,
            base_multi_for_this_dice_box: 1,
            pos,
            state: DiceBoxState::WaitingForDice,
            dice_collection_rect: Rectangle {
                x: pos.x + 2.0,
                y: pos.y - 63.0,
                width: 48.0,
                height: 64.0,
            },
            timer_for_tallying_dice: 0.0,
            previous_dice_value: i8::MAX,
            current_streak: 1,
        }
    }
}

pub fn update(
    dice_box_data: &mut DiceBoxData,
    dice_in_hand: &mut Vec<Dice>,
    input_state: &InputState,
    confirm_button: &mut ConfirmButton,
    dt: f32,
) {
    set_dice_positions(dice_box_data);

    match dice_box_data.state {
        DiceBoxState::WaitingForDice => {
            for i in (0..dice_in_hand.len()).rev() {
                if dice_in_hand[i].state == DiceState::Stopped
                    && dice_box_data.dice_collection_rect.check_collision_point_rec(dice_in_hand[i].pos)
                {
                    let dice_to_add = dice_in_hand.remove(i);
                    dice_box_data.dice_in_box.push(dice_to_add);
                    dice_box_data.dice_in_box.sort_by(|a, b| a.value.cmp(&b.value));
                }
            }

            if confirm_button.is_pressed(input_state) {
                dice_box_data.state = DiceBoxState::TallyingPoints;
            }
        }
        DiceBoxState::TallyingPoints => {
            if !dice_box_data.dice_in_box.is_empty() {
                if tally_points(dice_box_data, dt) {
                    dice_box_data.state = DiceBoxState::Acting;
                }
            }
        }
        _ => (),
    }
}

//dice box being empty handled by call site
fn tally_points(data: &mut DiceBoxData, dt: f32) -> bool {
    let time_between_dice = 1.0;

    data.timer_for_tallying_dice += dt;

    if data.current_index_dice_being_tallied == 0 || data.timer_for_tallying_dice >= time_between_dice {
        data.timer_for_tallying_dice = 0.0;
        let current_dice = &data.dice_in_box[data.current_index_dice_being_tallied];

        let is_last_dice = data.current_index_dice_being_tallied == data.dice_in_box.len() - 1;
        let continue_streak = data.previous_dice_value == current_dice.value;

        data.total_tally += current_dice.value as i64;

        if continue_streak {
            data.current_streak += 1;
        }

        // if you dont skip the first dice, the other statement will always return true
        if ((is_last_dice || !continue_streak) && data.current_streak > 1) && data.current_index_dice_being_tallied != 0
        {
            data.total_multi_for_this_tally *= data.current_streak as i64;
            data.current_streak = 1;
        }

        if data.current_index_dice_being_tallied == data.dice_in_box.len() - 1 {
            //data will reset after all things are used in a reset() function
            return true;
        }

        data.previous_dice_value = current_dice.value;
        data.current_index_dice_being_tallied += 1;

        println!(
            "Current tally: {}, Current Multi: {}, value of the dice just tallied: {}",
            data.total_tally, data.total_multi_for_this_tally, current_dice.value
        );
    }
    return false;
}

pub fn reset_box(data: &mut DiceBoxData, hand_dice: &mut Vec<Dice>) {
    while let Some(dice) = data.dice_in_box.pop() {
        hand_dice.push(dice);
    }

    data.state = DiceBoxState::WaitingForDice;
    data.current_index_dice_being_tallied = 0;
    data.current_streak = 1;
    data.previous_dice_value = i8::MAX;
    data.timer_for_tallying_dice = 0.0;
    data.total_multi_for_this_tally = 1;
    data.total_tally = 0;
}

fn set_dice_positions(dice_box_data: &mut DiceBoxData) {
    let mut draw_pos = dice_box_data.pos + DICE_DRAW_START_OFFSET;
    let mut times_increased_x = 0;

    for i in (0..dice_box_data.dice_in_box.len()).rev() {
        dice_box_data.dice_in_box[i].pos = draw_pos;
        draw_pos.x -= DICE_WIDTH_HEIGHT;
        times_increased_x += 1;
        if times_increased_x == 3 {
            draw_pos.x += DICE_WIDTH_HEIGHT * 3.0;
            draw_pos.y -= DICE_WIDTH_HEIGHT;
            times_increased_x = 0;
        }
    }
}

pub fn draw(d: &mut RaylibDrawHandle, texture: &Texture2D, dice_box_data: &mut DiceBoxData, dice_box_sprite: &Sprite) {
    match dice_box_data.state {
        DiceBoxState::Inactive => return,
        _ => {
            dice_box_sprite.draw(d, dice_box_data.pos, texture);
            d.draw_rectangle_lines(
                dice_box_data.dice_collection_rect.x as i32,
                dice_box_data.dice_collection_rect.y as i32,
                dice_box_data.dice_collection_rect.width as i32,
                dice_box_data.dice_collection_rect.height as i32,
                Color::WHITE,
            );
            draw_dice(dice_box_data, d, texture);
            //draw_multi();
            //draw_base_multi();
            //draw_border_around_currently_being_tallied_dice();
        }
    }
}

fn draw_dice(data: &mut DiceBoxData, d: &mut RaylibDrawHandle, texture: &Texture2D) {
    for i in 0..data.dice_in_box.len() {
        data.dice_in_box[i].draw(d, texture);
    }
}

fn draw_multi() {
    todo!("need to implement drawing the multiplier number")
}

fn draw_base_multi() {
    todo!("need to implement drawing the base multiplier for this dice box")
}

fn draw_border_around_currently_being_tallied_dice() {
    todo!("need to draw border around currently being tallied dice")
}

//need to update dice in order to check if theyre being picked up or not
fn update_dice() {
    todo!("need to impl update dice")
}
