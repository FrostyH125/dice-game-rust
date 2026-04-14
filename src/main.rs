pub mod entities;
pub mod system;
pub mod utilities;

use basic_raylib_core::{graphics::sprite::Sprite, system::timer::Timer};
use raylib::prelude::*;

use crate::{
    entities::{
        dice::DICE_WIDTH_HEIGHT, dice_box::DiceBox, enemies::snake::Snake, enemy::{Enemy, EnemyState}, player::{Player, PlayerState}, player_dice_boxes::broadsword_box::BroadSwordBox
    },
    system::{button::Button, dialogue_system::{Dialogue, DialogueSystem}, input_handler::InputState, particle_system::ParticleSystem},
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
    GameOver,
}

// impl gameover state
// combine dialogue system, particle system, and input state as a GlobalState struct

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
    player.add_box(DiceBox::BroadSwordBox { broadsword_box: BroadSwordBox::new(&font)});
    
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
    
    let mut dialogue_system = DialogueSystem::new();
    dialogue_system.add_dialogue(Dialogue { text_blocks: vec![String::from("this is test dialogue"), String::from("I added a second one just to test"), String::from("blah blah blah")] });
    
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
        dialogue_system.update(&input_state);

        match state {
            GameState::Travelling => {
                next_enemy_timer.track(dt);

                if next_enemy_timer.is_done() {
                    if current_enemy.get_data().health <= 0 {
                        current_enemy = get_random_enemy(&font);
                    }

                    state = GameState::Combat;
                    player.state = PlayerState::StartTurn;
                    next_enemy_timer.reset();
                }
            }
            GameState::Combat => {
                current_enemy.update(&input_state, &mut player, &mut particle_system, dt);
                
                if rl.is_key_pressed(KeyboardKey::KEY_A) {
                    player.take_hit(100);
                }
                
                if let EnemyState::Dead = current_enemy.get_data().state  {
                    player.reset();
                    state = GameState::Travelling;
                    player.state = PlayerState::Walking;
                }
            }
            GameState::GameOver => {
                todo!("Draw game over text here, add a replay button, and a quit button")
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
            | PlayerState::Dead
            | PlayerState::Walking => (),
            _ => {
                confirm_button.draw_with_text(&mut cam_handle, &sprite_sheet, &font, &input_state);

                if player.hand.dice.len() > 0 {
                    reroll_button.draw_with_text(&mut cam_handle, &sprite_sheet, &font, &input_state);
                }
            }
        }

        particle_system.draw(&mut cam_handle, &sprite_sheet);
        dialogue_system.draw(&mut cam_handle);
        input_state.draw_mouse(&mut cam_handle, &sprite_sheet);
    }
}

fn get_random_enemy(font: &Font) -> Enemy {
    let mut enemy = match random_range(0..1) {
        0 => Enemy::Snake { snake: Snake::new(font) },
        _ => {
            println!("number out of range for spawning enemy");
            Enemy::Snake { snake: Snake::new(font) }
        }
    };
    
    enemy.place_boxes();
    
    return enemy;
}
