pub mod entities;
pub mod system;
pub mod utilities;

use basic_raylib_core::{graphics::sprite::Sprite, system::timer::Timer};
use raylib::prelude::*;

use crate::{
    entities::{
        dice::DICE_WIDTH_HEIGHT,
        enemies::snake::Snake,
        enemy::{Enemy, EnemyState},
        player::{Player, PlayerState},
    },
    system::{button::Button, input_handler::InputState, particle_system::ParticleSystem},
};
use rand::random_range;

pub static SMALL_DUST_SPRITE: Sprite = Sprite::new(0.0, 32.0, 1.0, 1.0);
pub static LARGE_DUST_SPRITE: Sprite = Sprite::new(1.0, 32.0, 3.0, 3.0);

pub const EMPTY_SPRITE: Sprite = Sprite::new(0.0, 0.0, 0.0, 0.0);

const VIRTUAL_WIDTH: f32 = 480.0;
const VIRTUAL_HEIGHT: f32 = 270.0;
const PLAYER_UI_Y_BASE_CORD: f32 = VIRTUAL_HEIGHT - 75.0;
const PLAYER_UI_X_CENTER_CORD: f32 = 100.0;

pub enum GameState {
    Travelling,
    Combat,
}

// player has a box placing algorithm that
// if 1 box, places them directly above player, centered
// if 2 boxes, places them on directly above player on either side, with the empty space in the middle
// if 3 boxes, places 2 boxes in the same arrangement as in the 2 box condition, except one box height higher, 
// and then places the 3rd in the exact same spot as the 1 box arrangement
// if 4 boxes, places both rows of boxes in same arrangement as in the 2 box condition, stacked on top of eachother

// eventually broadsword box will need to take in a pos to place, and when more dice boxes, player
// will have to arrange dice boxes

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

    let mut player = Player::new(&font);
    let mut confirm_button = Button::new(
        Rectangle::new(PLAYER_UI_X_CENTER_CORD + 2.0, PLAYER_UI_Y_BASE_CORD + DICE_WIDTH_HEIGHT + 8.0, 64.0, 32.0),
        Sprite::new(80.0, 16.0, 64.0, 32.0),
        Sprite::new(80.0, 48.0, 64.0, 32.0),
        Sprite::new(80.0, 80.0, 64.0, 32.0),
        Some("Tally"),
        Some(Vector2::new(5.0, 10.0)),
    );
    let mut stop_button = Button::new(
        Rectangle::new(
            PLAYER_UI_X_CENTER_CORD - 128.0 / 2.0,
            PLAYER_UI_Y_BASE_CORD + DICE_WIDTH_HEIGHT + 8.0,
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
            PLAYER_UI_X_CENTER_CORD - 64.0 - 2.0,
            PLAYER_UI_Y_BASE_CORD + DICE_WIDTH_HEIGHT + 8.0,
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

    let mut particle_system = ParticleSystem::new();

    while !rl.window_should_close() {
        rl.hide_cursor();
        let dt = rl.get_frame_time();
        input_state.update(&mut rl, camera.zoom);
        player.update(
            &input_state,
            &mut confirm_button,
            &mut stop_button,
            &mut reroll_button,
            &mut particle_system,
            &mut current_enemy,
            dt,
        );
        particle_system.update(dt);

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
                current_enemy.update(&input_state, &mut player, &mut particle_system, dt);
                
                if let EnemyState::Dead = current_enemy.get_data().state  {
                    player.reset();
                    state = GameState::Travelling;
                    player.state = PlayerState::Walking;
                }
            }
        }

        let mut handle = rl.begin_drawing(&thread);
        handle.clear_background(Color { r: 40, g: 40, b: 40, a: 255 });

        let mut cam_handle = handle.begin_mode2D(&camera);
        player.draw(&mut cam_handle, &sprite_sheet, &font);
        
        if let GameState::Combat = state {
            current_enemy.draw(&mut cam_handle, &sprite_sheet, &font);
        }

        match player.state {
            PlayerState::RollingDice | PlayerState::StoppingDice => {
                stop_button.draw(&mut cam_handle, &sprite_sheet, &input_state);
            }
            PlayerState::WaitingForEnemy
            | PlayerState::StartTurn
            | PlayerState::WaitingForDiceToMoveToHand
            | PlayerState::HitDelay
            | PlayerState::Walking => (),
            _ => {
                confirm_button.draw_with_text(&mut cam_handle, &sprite_sheet, &font, &input_state);

                if player.hand.dice.len() > 0 {
                    reroll_button.draw_with_text(&mut cam_handle, &sprite_sheet, &font, &input_state);
                }
            }
        }

        particle_system.draw(&mut cam_handle, &sprite_sheet);
        input_state.draw_mouse(&mut cam_handle, &sprite_sheet);
    }
}

fn get_random_enemy(font: &Font) -> Enemy {
    match random_range(0..1) {
        0 => Enemy::Snake { snake: Snake::new(font) },
        _ => {
            println!("number out of range for spawning enemy");
            Enemy::Snake { snake: Snake::new(font) }
        }
    }
}
