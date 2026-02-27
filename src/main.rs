pub mod entities;
pub mod system;
pub mod utilities;
use raylib::prelude::*;

use crate::{
    entities::{
        confirm_button::ConfirmButton,
        enemy::Enemy,
        player::{Player, PlayerState},
        stop_button::StopButton,
    },
    system::input_handler::InputState,
};
use rand::random_range;

const VIRTUAL_WIDTH: f32 = 480.0;
const VIRTUAL_HEIGHT: f32 = 270.0;

// add border around currently being tallied dice, in dice box data and snake eyes, snake eyes should draw the border around both dice simultaneously
// add snake eyes text
// clean player states up, much like the enemy ones, add new delay states as needed

fn main() {
    let (mut rl, thread) =
        raylib::init().size(VIRTUAL_WIDTH as i32 * 3, VIRTUAL_HEIGHT as i32 * 3).title("Dice Game").build();

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

    let mut current_enemy = get_random_enemy();

    while !rl.window_should_close() {
        rl.hide_cursor();

        let dt = rl.get_frame_time();

        input_state.update(&mut rl, camera.zoom);

        player.update(&input_state, &mut confirm_button, &mut stop_button, &current_enemy, dt);
        
        // reset the player after the enemy dies (may clean this into)
        // its own function eventually
        if player.state == PlayerState::Walking {
            if player.time_to_walk_this_cycle == 0.0 {
                player.time_to_walk_this_cycle = random_range(2.0..=3.0);  
            }
            
            player.walk_timer += dt;
            
            if player.walk_timer >= player.time_to_walk_this_cycle {
                current_enemy = get_random_enemy();
                player.time_to_walk_this_cycle = 0.0;
                player.walk_timer = 0.0;
                player.state = PlayerState::StartTurn;
            }
        }
        
        current_enemy.update(&input_state, &mut stop_button, &player, dt);

        //game world draw handle (will be screen space draw handle eventually)
        let mut screen_handle = rl.begin_drawing(&thread);
        screen_handle.clear_background(Color { r: 40, g: 40, b: 40, a: 255 });

        {
            // handle for drawing world objects (even if theyre the same so far)
            let mut world_handle = screen_handle.begin_mode2D(&camera);

            player.draw(&mut world_handle, &sprite_sheet, &font);

            if player.state != PlayerState::Walking {
                current_enemy.draw(&mut world_handle, &sprite_sheet, &font);

                // if player is not walking AND not waiting for enemy
                // draw the buttons
                if player.state != PlayerState::WaitingForEnemy {
                    confirm_button.draw(&mut world_handle, &sprite_sheet, &font);
                    stop_button.draw(&mut world_handle, &sprite_sheet);
                }
            }

            input_state.draw_mouse(&mut world_handle, &sprite_sheet);
        }
    }
}

fn get_random_enemy() -> Enemy {
    match random_range(0..1) {
        0 => Enemy::new_snake(),
        _ => {
            println!("number out of range for spawning enemy");
            Enemy::new_snake()
        }
    }
}
