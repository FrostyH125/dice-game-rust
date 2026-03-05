pub mod entities;
pub mod system;
pub mod utilities;

use basic_raylib_core::system::timer::Timer;
use raylib::prelude::*;

use crate::{
    entities::{
        confirm_button::ConfirmButton,
        enemy::{Enemy, EnemyState},
        player::{Player, PlayerState},
        reroll_button::RerollButton,
        stop_button::StopButton,
    },
    system::input_handler::InputState,
};
use rand::random_range;

const VIRTUAL_WIDTH: f32 = 480.0;
const VIRTUAL_HEIGHT: f32 = 270.0;

#[derive(PartialEq)]
pub enum GameState {
    Travelling,
    Combat,
}

// add snake eyes text
// add drawing current tally to attack dice box
// make player and enemy actually attack eachother for real
// add reroll button that gets added in old place of the stop button once the hand has been rolled once
// particle system, sprite particle should have a 'sprite: &'static Sprite' field
// if hand has no dice, disable reroll button
// add dice border around snake eyes dice, after both dice are in, but before they disappear, right as its being tallied {DURING BEFORE ATTACK DELAY}
// snake animation + snake attack animation during tally delay

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
        mouse_state: system::input_handler::MouseState::Inactive,
    };

    let sprite_sheet = rl.load_texture(&thread, "SpriteSheet.png").unwrap();
    sprite_sheet.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);

    let mut player = Player::new();
    let mut confirm_button = ConfirmButton::new();
    let mut stop_button = StopButton::new();
    let mut reroll_button = RerollButton::new();

    let mut current_enemy = get_random_enemy(&font);

    while !rl.window_should_close() {
        rl.hide_cursor();
        let dt = rl.get_frame_time();
        input_state.update(&mut rl, camera.zoom);
        player.update(&input_state, &mut confirm_button, &mut stop_button, &mut reroll_button, &current_enemy, dt);

        match state {
            GameState::Travelling => {
                next_enemy_timer.track(dt);

                if next_enemy_timer.is_done() {
                    if current_enemy.get_data().health <= 0 {
                        current_enemy = get_random_enemy(&font);
                    }

                    state = GameState::Combat;
                    player.state = PlayerState::StartTurn;
                }
            }
            GameState::Combat => {
                current_enemy.update(&input_state, &player, dt);

                if current_enemy.get_data().state == EnemyState::Dead {
                    player.reset();
                    state = GameState::Travelling;
                    player.state = PlayerState::Walking;
                }
            }
        }

        let mut handle = rl.begin_drawing(&thread);
        handle.clear_background(Color { r: 40, g: 40, b: 40, a: 255 });

        // use cam handle for basically all drawing because everything will be drawn
        // needing to be zoomed. even if it would technically be zoomed in otherwise, this is cleaner
        let mut cam_handle = handle.begin_mode2D(&camera);
        player.draw(&mut cam_handle, &sprite_sheet, &font);

        if state == GameState::Combat {
            current_enemy.draw(&mut cam_handle, &sprite_sheet, &font);
        }

        if player.state == PlayerState::RollingDice || player.state == PlayerState::StoppingDice {
            stop_button.draw(&mut cam_handle, &sprite_sheet);
        }

        if player.state == PlayerState::ChoosingDice {
            confirm_button.draw(&mut cam_handle, &sprite_sheet, &font);
            reroll_button.draw(&mut cam_handle, &sprite_sheet, &font);
        }

        input_state.draw_mouse(&mut cam_handle, &sprite_sheet);
    }
}

fn get_random_enemy(font: &Font) -> Enemy {
    match random_range(0..1) {
        0 => Enemy::new_snake(font),
        _ => {
            println!("number out of range for spawning enemy");
            Enemy::new_snake(font)
        }
    }
}
