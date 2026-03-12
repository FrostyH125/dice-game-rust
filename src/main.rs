pub mod entities;
pub mod system;
pub mod utilities;

use basic_raylib_core::{graphics::sprite::Sprite, system::timer::Timer};
use raylib::prelude::*;

use crate::{
    entities::{
        dice::DICE_WIDTH_HEIGHT,
        enemy::{Enemy, EnemyState},
        hand::DICE_Y_OFFSET,
        player::{Player, PlayerState},
    },
    system::{button::Button, input_handler::InputState},
};
use rand::random_range;

const VIRTUAL_WIDTH: f32 = 480.0;
const VIRTUAL_HEIGHT: f32 = 270.0;

#[derive(PartialEq)]
pub enum GameState {
    Travelling,
    Combat,
}


// what if we just got rid of handstate? its basically being driven like a zombie anyway

// player attack animation and getting hit animation
// snake animation + snake attack animation during tally delay

// particle system, sprite particle should have a 'sprite: &'static Sprite' field
// make dice emit smoke particles when they disappear back to the hand

// make player and enemy actually attack eachother for real

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

    let mut input_state = InputState::new();

    let sprite_sheet = rl.load_texture(&thread, "SpriteSheet.png").unwrap();
    sprite_sheet.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);

    let mut player = Player::new();
    let mut confirm_button = Button::new(
        Rectangle::new(VIRTUAL_WIDTH / 2.0 + 2.0, VIRTUAL_HEIGHT - DICE_Y_OFFSET + DICE_WIDTH_HEIGHT + 8.0, 64.0, 32.0),
        Sprite::new(80.0, 16.0, 64.0, 32.0),
        Sprite::new(80.0, 48.0, 64.0, 32.0),
        Sprite::new(80.0, 80.0, 64.0, 32.0),
        Some("Tally"),
        Some(Vector2::new(5.0, 10.0)),
    );
    let mut stop_button = Button::new(
        Rectangle::new(
            VIRTUAL_WIDTH / 2.0 - 128.0 / 2.0,
            VIRTUAL_HEIGHT - DICE_Y_OFFSET + DICE_WIDTH_HEIGHT + 8.0,
            128.0,
            32.0,
        ),
        Sprite::new(16.0, 176.0, 128.0, 32.0),
        Sprite::new(16.0, 208.0, 128.0, 32.0),
        Sprite::new(16.0, 240.0, 128.0, 32.0),
        None,
        None,
    );
    let mut reroll_button = Button::new(
        Rectangle::new(
            VIRTUAL_WIDTH / 2.0 - 64.0 - 2.0,
            VIRTUAL_HEIGHT - DICE_Y_OFFSET + DICE_WIDTH_HEIGHT + 8.0,
            64.0,
            32.0,
        ),
        Sprite::new(16.0, 16.0, 64.0, 32.0),
        Sprite::new(16.0, 48.0, 64.0, 32.0),
        Sprite::new(16.0, 80.0, 64.0, 32.0),
        Some("Reroll"),
        Some(Vector2::new(2.0, 10.0)),
    );

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
            stop_button.draw(&mut cam_handle, &sprite_sheet, &input_state);
        }

        if player.state == PlayerState::ChoosingDice || player.state == PlayerState::RerollingDice {
            confirm_button.draw_with_text(&mut cam_handle, &sprite_sheet, &font, &input_state);

            if player.hand.dice.len() > 0 {
                reroll_button.draw_with_text(&mut cam_handle, &sprite_sheet, &font, &input_state);
            }
        }

        input_state.draw_mouse(&mut cam_handle, &sprite_sheet);
        cam_handle.draw_text(&format!("{:?}", input_state.mouse_state), 0, 0, 10, Color::WHITE);
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
