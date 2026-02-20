use raylib::math::Rectangle;

use crate::{
    entities::dice::{Dice, DiceState},
    system::{button::Button, input_handler::InputState},
};

pub enum DiceBoxState {
    WaitingForDice,
    TallyingPoints,
    Acting,
}

fn update(
    state: &mut DiceBoxState,
    dice_in_hand: &mut Vec<Dice>,
    dice_in_box: &mut Vec<Dice>,
    dice_collect_rect: Rectangle,
    input_state: &InputState,
    confirm_button: &mut Button
) {
    match state {
        DiceBoxState::WaitingForDice => {
            for i in 0..dice_in_hand.len() {
                if dice_in_hand[i].state == DiceState::Dragging
                    && dice_collect_rect.check_collision_point_rec(dice_in_hand[i].pos)
                {
                    let dice_to_add = dice_in_hand.remove(i);
                    dice_in_box.push(dice_to_add);
                    break;
                }
            }
            
            if confirm_button.is_pressed(input_state) {
                *state = DiceBoxState::TallyingPoints;
            }
        }
        DiceBoxState::TallyingPoints => {
            // go one by one, with a dice timer, 
            // and each time it encounters one dice of the same number in a row, 
            // it adds 1 to a multiplier until it hits another dice
            // who has a different value, then add a multiplier to a value (probably on a specific button struct,
            // who will display it properly)
            // the game will also continuously add the value to a total, and also update that total as the dice are counted
            // to which then, when it gets to the end, it all gets passed to the function inside of that struct when its done,
            // the struct itself will handle this call once the box_state == acting
        }
        DiceBoxState::Acting => {
            //maybe will do nothing here, but in the struct, will act (unless i wanna pass a function pointer or soemthing)
        },
    }
}

fn draw() {}
