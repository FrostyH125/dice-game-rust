use basic_raylib_core::graphics::sprite::Sprite;
use raylib::{ffi::ImageKernelConvolution, math::{Rectangle, Vector2}, prelude::RaylibDrawHandle, texture::Texture2D};

use crate::{
    entities::dice::{Dice, DiceState},
    system::{button::Button, input_handler::InputState},
};

pub enum DiceBoxState {
    WaitingForDice,
    TallyingPoints,
    Acting,
}

// with any luck, the only things youll need from this mod are update() and draw(), the rest should happen automatically.
// all that is left is checking data, doing something when it changes, and thats it

pub struct DiceBoxData {
    dice_in_box: Vec<Dice>,
    current_index_dice_being_tallied: usize,
    total_tally: i64,
    total_multi_for_this_tally: i64,
    base_multi_for_this_dice_box: i64,
    pos: Vector2,
    state: DiceBoxState,
    dice_collection_rect: Rectangle,
    timer_for_tallying_dice: f32,
    previous_dice_value: i8,
    current_streak: i8,
}

fn update(
    dice_box_data: &mut DiceBoxData,
    dice_in_hand: &mut Vec<Dice>,
    input_state: &InputState,
    confirm_button: &mut Button,
    dt: f32
) {
    match dice_box_data.state {
        DiceBoxState::WaitingForDice => {
            for i in 0..dice_in_hand.len() {
                if dice_in_hand[i].state == DiceState::Dragging
                    && dice_box_data.dice_collection_rect.check_collision_point_rec(dice_in_hand[i].pos)
                {
                    let dice_to_add = dice_in_hand.remove(i);
                    dice_box_data.dice_in_box.push(dice_to_add);
                    break;
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
        _ => ()
    }
}

//dice box being empty handled by call site
fn tally_points(data: &mut DiceBoxData, dt: f32) -> bool {
    let time_between_dice = 1.0;

    data.timer_for_tallying_dice += dt;

    if data.current_index_dice_being_tallied == 0 || data.timer_for_tallying_dice >= time_between_dice
    {
        data.timer_for_tallying_dice = 0.0;
        let current_dice = &data.dice_in_box[data.current_index_dice_being_tallied];
        
        let is_last_dice = data.current_index_dice_being_tallied == data.dice_in_box.len() - 1;
        let continue_streak = data.previous_dice_value == current_dice.value;
        

        data.total_tally += current_dice.value as i64;

        if continue_streak {
            data.current_streak += 1;
        }
        
        // if you dont skip the first dice, the other statement will always return true
        if ((is_last_dice || !continue_streak) && data.current_streak > 1) && data.current_index_dice_being_tallied != 0  {
            data.total_multi_for_this_tally *= data.current_streak as i64;
            data.current_streak = 1;
        }   
        
        if data.current_index_dice_being_tallied == data.dice_in_box.len() - 1 {
            //data will reset after all things are used in a reset() function
            return true;
        }
        
        data.previous_dice_value = current_dice.value;
        data.current_index_dice_being_tallied += 1;
    }
    return false;
}

fn draw(d: &mut RaylibDrawHandle, texture: &Texture2D, dice_box_data: &DiceBoxData, dice_box_sprite: &Sprite, dice_sprites: &[Sprite]) {
    dice_box_sprite.draw(d, dice_box_data.pos, texture);
    draw_dice(dice_box_data, dice_sprites);
    draw_multi();
    draw_base_multi();
    draw_border_around_currently_being_tallied_dice();
}

fn draw_dice(dice_box_data: &DiceBoxData, dice_sprites: &[Sprite]) {
    todo!("need to implement drawing the dice")
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
