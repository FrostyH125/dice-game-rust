use basic_raylib_core::{
    graphics::{animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance},
    system::timer::Timer,
};
use raylib::{
    color::Color, ffi::rlSetUniformSampler, math::Vector2, prelude::{RaylibDraw, RaylibDrawHandle}, text::{Font, RaylibFont}, texture::Texture2D
};

use crate::{
    EMPTY_SPRITE, PLAYER_UI_X_CENTER_CORD, PLAYER_UI_Y_BASE_CORD,
    entities::{
        dice::{DICE_WIDTH_HEIGHT, DiceState},
        dice_box::DiceBox,
        player_dice_boxes::broadsword_box::BroadSwordBox,
    },
    system::{input_handler::MouseState, particle_system::ParticleSystem},
};
use crate::{
    entities::{
        dice::{Dice, DiceKind},
        enemy::{Enemy, EnemyState},
        hand::Hand,
    },
    system::{button::Button, input_handler::InputState},
};

const HIT_DELAY_DURATION: f32 = 1.0;
const PLAYER_WIDTH: f32 = 32.0;
const PLAYER_HEIGHT: f32 = 48.0;
const PLAYER_POS: Vector2 = Vector2::new(84.0, 125.0);
const PLAYER_CENTER: Vector2 = Vector2::new(PLAYER_POS.x + PLAYER_WIDTH / 2.0, PLAYER_POS.y + PLAYER_HEIGHT / 2.0);
const PLAYER_HEALTH_TEXT_Y_OFFSET_FROM_BOTTOM_OF_SPRITE: f32 = 6.0;

static PLAYER_WALK_ANIM: AnimationData = AnimationData {
    frames: &[Sprite::new(80.0, 112.0, 32.0, 48.0), Sprite::new(112.0, 112.0, 32.0, 48.0)],
    frame_duration: 0.5,
    should_loop: true,
};

static PLAYER_THINKING_ANIM: AnimationData = AnimationData {
    frames: &[Sprite::new(144.0, 80.0, 32.0, 48.0), Sprite::new(176.0, 80.0, 32.0, 48.0)],
    frame_duration: 0.5,
    should_loop: true,
};

static PLAYER_WAITING_ANIM: AnimationData = AnimationData {
    frames: &[Sprite::new(144.0, 128.0, 32.0, 48.0), Sprite::new(176.0, 128.0, 32.0, 48.0)],
    frame_duration: 0.5,
    should_loop: true,
};

static PLAYER_HIT_ANIM: AnimationData = AnimationData {
    frames: &[EMPTY_SPRITE, Sprite::new(240.0, 128.0, 32.0, 48.0)],
    frame_duration: HIT_DELAY_DURATION / 4.0,
    should_loop: true,
};

pub enum PlayerState {
    Walking,
    StartTurn,
    WaitingForDiceToMoveToHand,
    RollingDice,
    StoppingDice,
    RerollingDice,
    ChoosingDice,
    TallyingCurrentBox,
    BeforeActingDelay,
    ActionVisual,
    Acting,
    EndTurnDelay,
    EndTurn,
    WaitingForEnemy,
    HitDelay,
    Dead,
}

pub struct Player {
    pub dice_boxes: Vec<DiceBox>,
    pub hand: Hand,
    power_of_current_action: i64,
    health: i64,
    current_box: usize,
    walk_anim: SpriteAnimationInstance,
    thinking_anim: SpriteAnimationInstance,
    waiting_anim: SpriteAnimationInstance,
    hit_anim: SpriteAnimationInstance,
    acting_anim: SpriteAnimationInstance,
    pos: raylib::math::Vector2,
    acting_timer: Timer,
    end_turn_delay_timer: Timer,
    hit_delay_timer: Timer,
    pub state: PlayerState,
    pub is_player_dragging_dice: bool,
    pub was_player_dragging_dice: bool,
}

impl Player {
    pub fn new() -> Self {
        Player {
            dice_boxes: Vec::new(),
            hand: Hand::new(
                std::iter::repeat_with(|| Dice::new(DiceKind::D6)).take(5).collect(),
                Vector2::new(PLAYER_UI_X_CENTER_CORD, PLAYER_UI_Y_BASE_CORD),
            ),
            walk_anim: SpriteAnimationInstance::new(),
            thinking_anim: SpriteAnimationInstance::new(),
            waiting_anim: SpriteAnimationInstance::new(),
            hit_anim: SpriteAnimationInstance::new(),
            acting_anim: SpriteAnimationInstance::new(),
            pos: PLAYER_POS,
            health: 100,
            state: PlayerState::Walking,
            acting_timer: Timer::new(1.0),
            end_turn_delay_timer: Timer::new(2.0),
            hit_delay_timer: Timer::new(HIT_DELAY_DURATION),
            power_of_current_action: 0,
            is_player_dragging_dice: false,
            was_player_dragging_dice: false,
            current_box: 0,
        }
    }

    pub fn update(
        &mut self,
        input_state: &InputState,
        confirm_button: &mut Button,
        stop_button: &mut Button,
        reroll_button: &mut Button,
        particle_system: &mut ParticleSystem,
        enemy: &mut Enemy,
        dt: f32,
    ) {
        if let MouseState::Inactive = input_state.mouse_state {
            self.is_player_dragging_dice = false;
        }

        self.hand.update_for_player(&mut self.is_player_dragging_dice, input_state, dt);

        for dice_box in &mut self.dice_boxes {
            dice_box.update_for_player(
                &mut self.is_player_dragging_dice,
                self.was_player_dragging_dice,
                &mut self.hand,
                input_state,
                dt,
            );
        }

        match self.state {
            PlayerState::Walking => {
                PLAYER_WALK_ANIM.update(&mut self.walk_anim, dt);
            }
            PlayerState::StartTurn => {
                self.reset();
                self.state = PlayerState::WaitingForDiceToMoveToHand;
                self.walk_anim.reset();
            }
            PlayerState::WaitingForDiceToMoveToHand => {
                PLAYER_WAITING_ANIM.update(&mut self.waiting_anim, dt);
                let mut should_move_on = false;

                for dice in &self.hand.dice {
                    if let DiceState::Rearranging { .. } = dice.state {
                        continue;
                    }

                    should_move_on = true;
                }

                if should_move_on {
                    self.state = PlayerState::RollingDice;
                    self.hand.roll_dice();
                }
            }
            PlayerState::RollingDice => {
                PLAYER_WAITING_ANIM.update(&mut self.waiting_anim, dt);
                if stop_button.is_pressed(input_state) {
                    self.hand.begin_dice_stop();
                    self.state = PlayerState::StoppingDice;
                    stop_button.deactivate();
                }
            }
            PlayerState::StoppingDice => {
                PLAYER_WAITING_ANIM.update(&mut self.waiting_anim, dt);
                if self.hand.stop_dice(dt) {
                    self.state = PlayerState::ChoosingDice;
                    self.waiting_anim.reset();
                    stop_button.reset();
                    confirm_button.reset();
                    reroll_button.reset();
                }
            }
            PlayerState::ChoosingDice => {
                PLAYER_THINKING_ANIM.update(&mut self.thinking_anim, dt);

                if self.hand.dice.len() > 0 && reroll_button.is_pressed(input_state) {
                    self.hand.reset_hand();
                    self.hand.begin_dice_stop();

                    confirm_button.deactivate();
                    reroll_button.deactivate();

                    self.state = PlayerState::RerollingDice;
                }

                if confirm_button.is_pressed(input_state) {
                    self.thinking_anim.reset();
                    self.state = PlayerState::TallyingCurrentBox;
                    self.hand.emit_smoke_at_each_dice(particle_system);
                    confirm_button.deactivate();
                    reroll_button.deactivate();
                }
            }
            PlayerState::RerollingDice => {
                PLAYER_WAITING_ANIM.update(&mut self.waiting_anim, dt);
                if self.hand.stop_dice(dt) {
                    self.state = PlayerState::ChoosingDice;
                    self.waiting_anim.reset();
                    reroll_button.reset();
                    confirm_button.reset();
                }
            }
            PlayerState::TallyingCurrentBox => {
                PLAYER_WAITING_ANIM.update(&mut self.waiting_anim, dt);

                if self.dice_boxes[self.current_box].get_data().dice_in_box.is_empty() {
                    self.current_box += 1;
                    if self.current_box > self.dice_boxes.len() - 1 {
                        self.state = PlayerState::EndTurn;
                    }
                } else if self.dice_boxes[self.current_box].tally(dt) {
                    self.state = PlayerState::BeforeActingDelay;
                }
            }
            PlayerState::BeforeActingDelay => {
                PLAYER_WAITING_ANIM.update(&mut self.waiting_anim, dt);

                self.acting_timer.track(dt);

                if self.acting_timer.is_done() {
                    self.acting_timer.reset();
                    self.state = PlayerState::ActionVisual;
                }
            }
            PlayerState::ActionVisual => {
                if self.dice_boxes[self.current_box].player_update_action(&mut self.acting_anim, dt) {
                    self.acting_anim.reset();
                    self.state = PlayerState::Acting;
                }
            }
            PlayerState::Acting => {
                PLAYER_WAITING_ANIM.update(&mut self.waiting_anim, dt);

                self.power_of_current_action = self.dice_boxes[self.current_box].get_data().get_value();

                self.dice_boxes[self.current_box].player_action(self.power_of_current_action, enemy);

                self.current_box += 1;

                if self.current_box > self.dice_boxes.len() - 1 {
                    self.state = PlayerState::EndTurnDelay;
                } else {
                    self.state = PlayerState::TallyingCurrentBox;
                }
            }
            PlayerState::EndTurnDelay => {
                PLAYER_WAITING_ANIM.update(&mut self.waiting_anim, dt);

                self.end_turn_delay_timer.track(dt);

                if self.end_turn_delay_timer.is_done() {
                    self.end_turn_delay_timer.reset();
                    self.state = PlayerState::EndTurn;
                }
            }
            PlayerState::EndTurn => {
                PLAYER_WAITING_ANIM.update(&mut self.waiting_anim, dt);
                for dice_box in &mut self.dice_boxes {
                    dice_box.get_mut_data().emit_smoke_at_each_dice(particle_system);
                    dice_box.reset(&mut self.hand.dice, PLAYER_CENTER + DICE_WIDTH_HEIGHT / 2.0);
                }
                self.state = PlayerState::WaitingForEnemy;
            }
            PlayerState::WaitingForEnemy => {
                PLAYER_WAITING_ANIM.update(&mut self.waiting_anim, dt);

                if let EnemyState::WaitingForPlayer = enemy.get_data().state {
                    self.state = PlayerState::StartTurn;
                    self.waiting_anim.reset();
                }
            }
            PlayerState::HitDelay => {
                self.hit_delay_timer.track(dt);
                PLAYER_HIT_ANIM.update(&mut self.hit_anim, dt);
                if self.hit_delay_timer.is_done() {
                    if self.health <= 0 {
                        self.state = PlayerState::Dead
                    } else {
                        self.state = PlayerState::WaitingForEnemy;
                    }
                    self.hit_anim.reset();
                }
            }
            PlayerState::Dead => (),
        }

        self.was_player_dragging_dice = self.is_player_dragging_dice;
    }

    pub fn reset(&mut self) {
        for dice_box in &mut self.dice_boxes {
            dice_box.reset(&mut self.hand.dice, PLAYER_CENTER + DICE_WIDTH_HEIGHT / 2.0);
        }

        self.hand.reset_hand();
        self.current_box = 0;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        match self.state {
            PlayerState::Walking => {
                PLAYER_WALK_ANIM.draw(&self.walk_anim, d, self.pos, texture);
            }
            PlayerState::WaitingForEnemy => {
                PLAYER_WAITING_ANIM.draw(&self.waiting_anim, d, self.pos, texture);
                for dice_box in &mut self.dice_boxes {
                    dice_box.draw(d, texture, font);
                }
            }
            PlayerState::HitDelay => {
                PLAYER_HIT_ANIM.draw(&mut self.hit_anim, d, self.pos, texture);
                for dice_box in &mut self.dice_boxes {
                    dice_box.draw(d, texture, font);
                }
            }
            PlayerState::ChoosingDice => {
                PLAYER_THINKING_ANIM.draw(&self.thinking_anim, d, self.pos, texture);
                for dice_box in &mut self.dice_boxes {
                    dice_box.draw(d, texture, font);
                }
                self.hand.draw(d, texture);
            }
            PlayerState::RerollingDice
            | PlayerState::RollingDice
            | PlayerState::StoppingDice
            | PlayerState::WaitingForDiceToMoveToHand => {
                PLAYER_WAITING_ANIM.draw(&self.waiting_anim, d, self.pos, texture);
                for dice_box in &mut self.dice_boxes {
                    dice_box.draw(d, texture, font);
                }
                self.hand.draw(d, texture);
            }
            PlayerState::ActionVisual => {
                self.dice_boxes[self.current_box].player_draw_action(&mut self.acting_anim, d, self.pos, texture);
                for dice_box in &mut self.dice_boxes {
                    dice_box.draw(d, texture, font);
                }
            }
            PlayerState::Dead => {
                
            }
            _ => {
                PLAYER_WAITING_ANIM.draw(&self.waiting_anim, d, self.pos, texture);
                for dice_box in &mut self.dice_boxes {
                    dice_box.draw(d, texture, font);
                }
            }
        }

        // don't draw the health if you're walking
        if let PlayerState::Walking = self.state {
            return;
        }

        let health_str = &format!("HP: {}", self.health);
        let font_size = 10.0;
        let spacing = 0.5;
        let size_of_str = font.measure_text(health_str, font_size, spacing);
        let self_width = 32.0;
        let self_height = 48.0;

        d.draw_text_ex(
            font,
            health_str,
            self.pos
                + Vector2::new(
                    self_width / 2.0 - size_of_str.x / 2.0,
                    self_height + PLAYER_HEALTH_TEXT_Y_OFFSET_FROM_BOTTOM_OF_SPRITE,
                ),
            font_size,
            spacing,
            Color::WHITE,
        );
    }

    pub fn take_hit(&mut self, damage: i64) {
        self.health -= damage;
        self.state = PlayerState::HitDelay;
    }
    
    pub fn add_box(&mut self, dice_box: DiceBox) {
        self.dice_boxes.push(dice_box);
        self.place_boxes();
    }

    pub fn place_boxes(&mut self) {
        let num_of_boxes = self.dice_boxes.len();
        let half_player_width = 32.0 / 2.0;
        let margin = 5.0;
        let dice_box_height = 16.0;

        let bottom_layer_y = self.pos.y - margin - dice_box_height;
        let top_layer_y = bottom_layer_y - margin - dice_box_height;

        // need to modify this as number of boxes moves from 1 to 4
        // eventually number of boxes may be 5+, but we will cross that bridge when we get there
        match num_of_boxes {
            1 => {
                let box_data = self.dice_boxes[0].get_mut_data();
                let half_dice_box_width = box_data.width / 2.0;
                let pos_x = self.pos.x + half_player_width - half_dice_box_width;
                let pos_y = bottom_layer_y;
                box_data.pos = Vector2::new(pos_x, pos_y);
            }
            2 => {
                let box_one_data = self.dice_boxes[0].get_mut_data();
                let first_box_width = box_one_data.width;
                box_one_data.pos = Vector2::new((self.pos.x - first_box_width) - margin, bottom_layer_y);

                let box_two_data = self.dice_boxes[1].get_mut_data();
                box_two_data.pos = Vector2::new(self.pos.x + PLAYER_WIDTH + margin, bottom_layer_y);
            } 
            3 => {
                let box_one_data = self.dice_boxes[0].get_mut_data();
                let first_box_width = box_one_data.width;
                box_one_data.pos = Vector2::new((self.pos.x - first_box_width) - margin, top_layer_y);
                
                let box_two_data = self.dice_boxes[1].get_mut_data();
                box_two_data.pos = Vector2::new(self.pos.x + PLAYER_WIDTH + margin, top_layer_y);
                
                let box_three_data = self.dice_boxes[2].get_mut_data();
                let half_dice_box_width = box_three_data.width / 2.0;
                let box_three_pos_x = self.pos.x + half_player_width - half_dice_box_width;
                let box_three_pos_y = bottom_layer_y;
                box_three_data.pos = Vector2::new(box_three_pos_x, box_three_pos_y);
            }
            4 => {
                let box_one_data = self.dice_boxes[0].get_mut_data();
                let first_box_width = box_one_data.width;
                box_one_data.pos = Vector2::new((self.pos.x - first_box_width) - margin, top_layer_y);
                
                let box_two_data = self.dice_boxes[1].get_mut_data();
                box_two_data.pos = Vector2::new(self.pos.x + PLAYER_WIDTH + margin, top_layer_y);
                
                let box_three_data = self.dice_boxes[2].get_mut_data();
                let third_box_width = box_three_data.width;
                box_three_data.pos = Vector2::new((self.pos.x - third_box_width) - margin, bottom_layer_y);
                
                let box_four_data = self.dice_boxes[3].get_mut_data();
                box_four_data.pos = Vector2::new(self.pos.x + PLAYER_WIDTH + margin, bottom_layer_y);
            }
            _ => unimplemented!("place_boxes(player) not implemented for {} boxes", num_of_boxes)
        }
        
        for dice_box in &mut self.dice_boxes {
            dice_box.adjust_collect_rect_pos_for_current_pos();
        }
    }
}
