pub mod entities;
pub mod system;
use raylib::prelude::*;

use crate::{entities::{attack_dice_box::AttackDiceBox, confirm_button::ConfirmButton, dice_box::DiceBoxState, hand::Hand, player::{Player, PlayerState}}, system::input_handler::InputState};

const VIRTUAL_WIDTH: f32 = 480.0;
const VIRTUAL_HEIGHT: f32 = 270.0;

pub enum GameState {
    FindingEnemy,
    BattlePlayerTurn,
    BattleEnemyTurn,
    Victory,
    GameOver
}

// create boxes for dice, + a hero (player) with health
// add dice to box, once you click confirm, make all the boxes count up their dice one by one
// in hand, iterate over each dice
// if handstate == stopped, if mouse is dragging and is inside a dice, that dice is now being dragged, 
// add to enum DiceState::IsBeingDragged, and if IsBeingDragged,
// Dice.pos = mouse.pos - 8x, + 8y, and if dice intersects with box dice_collection_rect, 
// add that dice to that box, and sort the dice, and draw them. Now dice state is stopped again, should be able to drag again if needed.
// 
// i suppose in dice, if stopped and gamestate == player make selection or something

fn main() {
    let (mut rl, thread) = raylib::init().size(1920, 1080,).title("Dice Game").build();
    
    let camera = Camera2D {
        offset: Default::default(),
        target: Default::default(),
        rotation: Default::default(),
        zoom: 4.0,
    };
    
    let mut input_state = InputState {
        mouse_pos: rl.get_mouse_position() / 4.0,
        click_pos: Default::default(),
        mouse_state: system::input_handler::MouseState::NotActive,
    };
    
    let sprite_sheet = rl.load_texture(&thread, "SpriteSheet.png").unwrap();
    
    let mut player = Player::new();
    let mut confirm_button = ConfirmButton::new();
    
    while !rl.window_should_close() {
        
        rl.hide_cursor();
        
        let dt = rl.get_frame_time();
        input_state.update(&mut rl);
        player.update(&input_state, &mut confirm_button, dt);
        
        //game world draw handle (will be screen space draw handle eventually)
        let mut world_handle = rl.begin_drawing(&thread);
        world_handle.clear_background(Color { r: 40, g: 40, b: 40, a: 255 });
        
        {
            let mut screen_handle = world_handle.begin_mode2D(&camera);
            player.draw(&mut screen_handle, &sprite_sheet);
            if player.state != PlayerState::Walking {
                confirm_button.draw(&mut screen_handle, &sprite_sheet);
            }
            input_state.draw_mouse(&mut screen_handle, &sprite_sheet);
        }
        
        if player.attack_box.data.state == DiceBoxState::Acting {
            confirm_button.reset();
        }
        
        
    }
}
