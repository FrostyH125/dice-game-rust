pub mod entities;
pub mod system;
pub mod utilities;
use std::thread::current;

use basic_raylib_core::system::timer::Timer;
use raylib::{ffi::GetRandomValue, prelude::*};

use crate::{
    entities::{
        confirm_button::ConfirmButton,
        enemy::{Enemy, EnemyState},
        player::{Player, PlayerState},
        stop_button::StopButton,
    },
    system::input_handler::InputState,
};
use rand::random_range;

const VIRTUAL_WIDTH: f32 = 480.0;
const VIRTUAL_HEIGHT: f32 = 270.0;

pub enum GameState {
    Travelling,
    Combat,
}

// add border around currently being tallied dice, in dice box data and snake eyes, snake eyes should draw the border around both dice simultaneously
// add snake eyes text
// add current tally to attack dice box
// clean player states up, much like the enemy ones, add new delay states as needed
// eventually, should have a game state enum that handles the game's state machine (travelling, in combat)

fn main() {
    let (mut rl, thread) =
        raylib::init().size(VIRTUAL_WIDTH as i32 * 3, VIRTUAL_HEIGHT as i32 * 3).title("Dice Game").build();

    let font = rl.load_font(&thread, "PublicPixel.ttf").unwrap();
    
    let mut state = GameState::Travelling;
    
    let mut next_enemy_timer = Timer::new(2.0);

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
        
        match state {
            GameState::Travelling => {
                next_enemy_timer.track(dt);
                
                if next_enemy_timer.is_done() {
                    if current_enemy.get_data().health <= 0 {
                        current_enemy = get_random_enemy();
                    }
                    
                    state = GameState::Combat;
                    player.state = PlayerState::StartTurn;
                }
            },
            GameState::Combat => {
                current_enemy.update(&input_state, &mut stop_button, &player, dt);
                
                if current_enemy.get_data().state == EnemyState::Dead {
                    player.reset();
                    state = GameState::Travelling;
                    player.state = PlayerState::Walking;
                }
            },
        }
        
        let mut handle = rl.begin_drawing(&thread);
        handle.clear_background(Color { r: 40, g: 40, b: 40, a: 255 });

        // use cam handle for basically all drawing because everything will be drawn
        // needing to be zoomed. even if it would technically be zoomed in otherwise, this is cleaner
        let mut cam_handle = handle.begin_mode2D(&camera);
        player.draw(&mut cam_handle, &sprite_sheet, &font);
        if player.state != PlayerState::Walking {
            
            current_enemy.draw(&mut cam_handle, &sprite_sheet, &font);        
            
            // if player is not walking AND not waiting for enemy
            // draw the buttons
            if player.state != PlayerState::WaitingForEnemy {
                confirm_button.draw(&mut cam_handle, &sprite_sheet, &font);
                stop_button.draw(&mut cam_handle, &sprite_sheet);
            }
        }
        input_state.draw_mouse(&mut cam_handle, &sprite_sheet);
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
