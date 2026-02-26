pub mod entities;
pub mod system;
pub mod utilities;
use raylib::prelude::*;

use crate::{entities::{confirm_button::ConfirmButton, player::{Player, PlayerState}, stop_button::StopButton}, system::input_handler::InputState};

const VIRTUAL_WIDTH: f32 = 480.0;
const VIRTUAL_HEIGHT: f32 = 270.0;


// snake draw method
// check player control flow and make sure player is consistent with player/box relationship
// add enemy to player flow after

// add enemy and integrate into control flow
// when theres an enemy, do back and forth loop, only breaking if enemy is dead or player is dead

fn main() {
    let (mut rl, thread) = raylib::init().size(VIRTUAL_WIDTH as i32 * 3, VIRTUAL_HEIGHT as i32 * 3).title("Dice Game").build();
    
    let font = rl.load_font(&thread, "PublicPixel.ttf").unwrap();
    
    let camera = Camera2D {
        offset: Default::default(),
        target: Default::default(),
        rotation: Default::default(),
        zoom: 3.0,
    };
    
    let mut input_state = InputState {
        mouse_pos: rl.get_mouse_position() / camera.zoom,
        click_pos: Default::default(),
        mouse_state: system::input_handler::MouseState::NotActive,
    };
    
    let sprite_sheet = rl.load_texture(&thread, "SpriteSheet.png").unwrap();
    
    let mut player = Player::new();
    let mut confirm_button = ConfirmButton::new();
    let mut stop_button = StopButton::new();
    
    while !rl.window_should_close() {
        
        rl.hide_cursor();
        
        let dt = rl.get_frame_time();
        input_state.update(&mut rl, camera.zoom);
        player.update(&input_state, &mut confirm_button, &mut stop_button, dt);
        
        //game world draw handle (will be screen space draw handle eventually)
        let mut world_handle = rl.begin_drawing(&thread);
        world_handle.clear_background(Color { r: 40, g: 40, b: 40, a: 255 });
        
        {
            let mut screen_handle = world_handle.begin_mode2D(&camera);
            player.draw(&mut screen_handle, &sprite_sheet, &font);
            if player.state != PlayerState::Walking || player.state != PlayerState::WaitingForEnemy {
                confirm_button.draw(&mut screen_handle, &sprite_sheet, &font);
                stop_button.draw(&mut screen_handle, &sprite_sheet);
            }
            input_state.draw_mouse(&mut screen_handle, &sprite_sheet);
        }
    }
}
