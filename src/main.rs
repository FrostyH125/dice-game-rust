pub mod entities;
pub mod system;
pub mod utilities;
pub mod game_effects;

use raylib::prelude::*;

use basic_raylib_core::{
    graphics::sprite::Sprite,
    system::{input_handler::InputState, sprite_particle_system::SpriteParticleSystem, timer::Timer},
};

use crate::{
    entities::{
        dice::DICE_WIDTH_HEIGHT,
        dice_box::DiceBox,
        enemies::snake::Snake,
        enemy::{Enemy, EnemyState},
        player::{Player, PlayerState},
        player_dice_boxes::{broadsword_box::BroadSwordBox, heal_box::HealBox, shield_box::ShieldBox},
        scoreboard::ScoreBoard,
    }, game_effects::battle_effects_manager::{BattleEffectsManager}, system::{button::Button, dialogue_system::DialogueSystem}
};
use rand::random_range;

pub static SMALL_DUST_SPRITE: Sprite = Sprite::new(0, 32, 1, 1);
pub static LARGE_DUST_SPRITE: Sprite = Sprite::new(1, 32, 3, 3);

static MOUSE_SPRITE: Sprite = Sprite::new(0, 16, 16, 16);

pub const EMPTY_SPRITE: Sprite = Sprite::new(0, 0, 0, 0);

const VIRTUAL_WIDTH: f32 = 480.0;
const VIRTUAL_HEIGHT: f32 = 270.0;
const PLAYER_UI_Y_BASE_CORD: f32 = VIRTUAL_HEIGHT - 75.0;
const PLAYER_UI_X_CENTER_CORD: f32 = 100.0;

pub const GRAVITY: f32 = 160.0;

pub enum GameState {
    Travelling,
    Combat,
    GameOver,
}

// shield box, make the player hold out shield when attacked when they still have defense, make it break perfectly if damage equals shield power, if damage exceeds
// shield power, make it shatter and make player take damage with flashing animation, different pose than normal one though
// ACTUAL IMPLEMENTATION
//  have a BlockType enum with { None, Blocked, Break, PerfectBreak } 
//  in player.takehit() assign the enum
//  then match it in Player::TakeHit to properly do the visual 

// make CombatEffectManager also responsible for having damage numbers fly off of hits and blocks
// disable input if the dialogue is running


// if a function needs one of these fields, pass the field itself by reference
// if a function needs more than one of these fields, pass the struct itself by reference
// this might seem like a questionable decision to have one of these structs at all, but as
// more systems are added to the game, i noticed the amount of parameters reaching
// uncomfortable amounts
pub struct GameContext {
    sprite_particle_system: SpriteParticleSystem,
    dialogue_system: DialogueSystem,
    battle_effect_manager: BattleEffectsManager,
    input_state: InputState,
    texture: Texture2D,
    font: Font,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum HitType {
    Unblocked,
    Blocked,
    BlockedBroken,
    PerfectBreak
}

fn main() {
    let (mut rl, thread) =
        raylib::init().size(VIRTUAL_WIDTH as i32 * 3, VIRTUAL_HEIGHT as i32 * 3).title("Dice Game").build();

    let sprite_particle_system = SpriteParticleSystem::new(1000);
    let dialogue_system = DialogueSystem::new();
    let battle_effect_manager = BattleEffectsManager::new();
    let camera = Camera2D {
        offset: Default::default(),
        target: Default::default(),
        rotation: Default::default(),
        zoom: 3.0,
    };
    let input_state = InputState::new();
    let texture = rl.load_texture(&thread, "SpriteSheet.png").unwrap();
    let font = rl.load_font(&thread, "PublicPixel.ttf").unwrap();

    let mut game_context = GameContext {
        sprite_particle_system,
        dialogue_system,
        battle_effect_manager,
        input_state,
        texture,
        font,
    };

    let mut state = GameState::Travelling;

    let mut next_enemy_timer = Timer::new(2.0);

    let mut player = Player::new();
    player.add_box(DiceBox::BroadSwordBox {
        broadsword_box: BroadSwordBox::new(&game_context.font),
    });
    player.add_box(DiceBox::HealBox {
        heal_box: HealBox::new(&game_context.font),
    });
    player.add_box(DiceBox::ShieldBox { shield_box: ShieldBox::new(&game_context.font) });

    let mut scoreboard = ScoreBoard::new();

    let mut confirm_button = Button::new(
        Rectangle::new(PLAYER_UI_X_CENTER_CORD + 2.0, PLAYER_UI_Y_BASE_CORD + DICE_WIDTH_HEIGHT + 8.0, 64.0, 32.0),
        Sprite::new(80, 16, 64, 32),
        Sprite::new(80, 48, 64, 32),
        Sprite::new(80, 80, 64, 32),
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
        Sprite::new(16, 176, 128, 32),
        Sprite::new(16, 208, 128, 32),
        Sprite::new(16, 240, 128, 32),
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
        Sprite::new(16, 16, 64, 32),
        Sprite::new(16, 48, 64, 32),
        Sprite::new(16, 80, 64, 32),
        Some("Reroll"),
        Some(Vector2::new(2.0, 10.0)),
    );

    let mut current_enemy = get_random_enemy(&game_context.font);

    while !rl.window_should_close() {
        rl.hide_cursor();
        let dt = rl.get_frame_time();
        let total_time = rl.get_time() as f32;

        game_context.input_state.update(&mut rl, camera.zoom);
        player.update(
            &mut confirm_button,
            &mut stop_button,
            &mut reroll_button,
            &mut current_enemy,
            &mut game_context,
            dt,
        );

        game_context.sprite_particle_system.update(dt);
        game_context.dialogue_system.update(&game_context.input_state);
        game_context.battle_effect_manager.update(dt, total_time);
        
        scoreboard.update(&mut player, &current_enemy, dt);

        match state {
            GameState::Travelling => {
                next_enemy_timer.track(dt);

                if next_enemy_timer.is_done() {
                    if current_enemy.get_data().health <= 0 {
                        current_enemy = get_random_enemy(&game_context.font);
                    }

                    state = GameState::Combat;
                    player.state = PlayerState::StartTurn;
                    next_enemy_timer.reset();
                }
            }
            GameState::Combat => {
                current_enemy.update(&mut player, &mut game_context, dt);

                if rl.is_key_pressed(KeyboardKey::KEY_A) {
                    player.take_hit(100);
                }

                if let EnemyState::Dead = current_enemy.get_data().state {
                    player.reset();
                    state = GameState::Travelling;
                    player.state = PlayerState::Walking;
                }

                if let PlayerState::Dead = player.state {
                    state = GameState::GameOver;
                }
            }
            GameState::GameOver => {
                let quit_pressed = rl.is_key_pressed(KeyboardKey::KEY_Q);
                let restart_pressed = rl.is_key_pressed(KeyboardKey::KEY_R);

                if quit_pressed {
                    break;
                }

                if restart_pressed {
                    state = GameState::Travelling;
                    player.reset();
                    player.state = PlayerState::Walking;
                    current_enemy = get_random_enemy(&game_context.font);
                }
            }
        }

        let mut handle = rl.begin_drawing(&thread);
        handle.clear_background(Color { r: 40, g: 40, b: 40, a: 255 });

        let mut cam_handle = handle.begin_mode2D(&camera);

        // so far player is always drawn regardless of state, that will eventually change but it doesnt need to at the moment
        player.draw(&mut cam_handle, &game_context);

        match state {
            GameState::Combat => {
                current_enemy.draw(&mut cam_handle, &game_context);
            }
            GameState::GameOver => {
                // Draw game over text here, add a replay button, and a quit button

                let game_over_string = "Game Over!";

                let replay_string = "Press [R] to replay";
                let quit_string = "Press [Q] to quit";

                // handle the game over text first
                let game_over_string_length = game_context.font.measure_text(game_over_string, 10.0, 0.5);
                let game_over_string_y = (VIRTUAL_HEIGHT / 2.0) - 40.0 - game_over_string_length.y / 2.0;
                let game_over_string_x = VIRTUAL_WIDTH / 2.0 - game_over_string_length.x / 2.0;
                cam_handle.draw_text_ex(
                    &game_context.font,
                    game_over_string,
                    Vector2::new(game_over_string_x, game_over_string_y),
                    10.0,
                    0.5,
                    Color::PALEVIOLETRED,
                );

                // now handle the replay and quit texts
                let replay_string_length = game_context.font.measure_text(replay_string, 8.0, 0.0);
                let margin_from_middle = 10.0;

                let replay_and_quit_str_y = VIRTUAL_HEIGHT / 2.0;
                let replay_and_quit_str_x_start = VIRTUAL_WIDTH / 2.0;

                let replay_string_x = replay_and_quit_str_x_start - replay_string_length.x - margin_from_middle;
                let quit_string_x = replay_and_quit_str_x_start + margin_from_middle;

                cam_handle.draw_text_ex(
                    &game_context.font,
                    replay_string,
                    Vector2::new(replay_string_x, replay_and_quit_str_y),
                    8.0,
                    0.0,
                    Color::WHITE,
                );

                cam_handle.draw_text_ex(
                    &game_context.font,
                    quit_string,
                    Vector2::new(quit_string_x, replay_and_quit_str_y),
                    8.0,
                    0.0,
                    Color::WHITE,
                );
            }
            _ => (),
        }

        // handle buttons specifically
        match player.state {
            PlayerState::RollingDice | PlayerState::StoppingDice => {
                stop_button.draw(&mut cam_handle, &game_context);
            }
            PlayerState::WaitingForEnemy
            | PlayerState::StartTurn
            | PlayerState::WaitingForDiceToMoveToHand
            | PlayerState::HitDelay {..}
            | PlayerState::Dead
            | PlayerState::Walking => (),
            _ => {
                confirm_button.draw_with_text(&mut cam_handle, &game_context);

                if player.hand.dice.len() > 0 {
                    reroll_button.draw_with_text(&mut cam_handle, &game_context);
                }
            }
        }

        game_context.battle_effect_manager.draw(&mut cam_handle, &game_context.texture, &game_context.font);
        game_context.sprite_particle_system.draw(&mut cam_handle, &game_context.texture);
        game_context.dialogue_system.draw(&mut cam_handle, &game_context.font);

        if player.state != PlayerState::Dead {
            scoreboard.draw(&mut cam_handle, &mut player, &current_enemy, &game_context);
        }

        draw_mouse(&mut cam_handle, &game_context);
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

pub fn draw_mouse(d: &mut RaylibDrawHandle, game_context: &GameContext) {
    // here, yes, i pass 2 arguments needing the game info,
    // this is a rare case and im not changing sprite::draw() just
    // to accomodate the one instance where i need 2 fields from the struct
    MOUSE_SPRITE.draw(d, game_context.input_state.mouse_pos, &game_context.texture);
}
