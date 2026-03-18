use basic_raylib_core::{
    graphics::{animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance},
    system::timer::Timer,
};
use raylib::{math::Vector2, prelude::RaylibDrawHandle, text::Font, texture::Texture2D};

use crate::{
    entities::{
        dice::{Dice, DiceKind, DiceState},
        enemy::{ENEMY_HAND_X_CENTER_CORD, ENEMY_HAND_Y_CORD, EnemyData, EnemyState},
        enemy_dice_boxes::snake_eyes::SnakeEyes,
        hand::Hand,
        player::{Player, PlayerState},
        player_dice_boxes::dice_box::DiceBox,
    },
    system::{input_handler::InputState, particle_system::ParticleSystem},
};

static SNAKE_IDLE_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(144.0, 176.0, 32.0, 48.0),
        Sprite::new(176.0, 176.0, 32.0, 48.0),
        Sprite::new(208.0, 176.0, 32.0, 48.0),
        Sprite::new(240.0, 176.0, 32.0, 48.0),
    ],
    frame_duration: 0.5,
    should_loop: true,
};

static SNAKE_ATTACK_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(144.0, 224.0, 32.0, 48.0),
        Sprite::new(176.0, 224.0, 32.0, 48.0),
        Sprite::new(208.0, 224.0, 32.0, 48.0),
        Sprite::new(240.0, 224.0, 32.0, 48.0),
    ],
    frame_duration: 0.25,
    should_loop: true,
};

pub struct Snake {
    pub data: EnemyData,
    pub hand: Hand,
    pub snake_eyes_box: DiceBox,
    dice_add_timer: Timer,
    before_stopping_dice_timer: Timer,
    before_tally_timer: Timer,
    before_act_timer: Timer,
    turn_end_timer: Timer,
    idle_anim: SpriteAnimationInstance,
    attack_anim: SpriteAnimationInstance,
}

impl Snake {
    pub fn new(font: &Font) -> Self {
        let pos = Vector2 { x: 400.0, y: 150.0 };

        Snake {
            data: EnemyData {
                health: 100,
                pos: pos,
                state: EnemyState::WaitingForPlayer,
                hit_timer: Timer::new(1.5),
            },
            hand: Hand::new(
                vec![
                    Dice::new(DiceKind::D4),
                    Dice::new(DiceKind::D4),
                    Dice::new(DiceKind::D4),
                    Dice::new(DiceKind::D4),
                ],
                Vector2::new(ENEMY_HAND_X_CENTER_CORD, ENEMY_HAND_Y_CORD),
            ),
            snake_eyes_box: DiceBox::SnakeEyes {
                snake_eyes_box: SnakeEyes::new(pos - Vector2::new(40.0, 0.0), font),
            },
            dice_add_timer: Timer::new(1.5),
            before_stopping_dice_timer: Timer::new(1.0),
            before_tally_timer: Timer::new(1.0),
            before_act_timer: Timer::new(2.0),
            turn_end_timer: Timer::new(2.0),
            idle_anim: SpriteAnimationInstance::new(),
            attack_anim: SpriteAnimationInstance::new(),
        }
    }

    pub fn update(&mut self, input_state: &InputState, player: &Player, particle_system: &mut ParticleSystem, dt: f32) {
        self.hand.update_for_enemy(dt);
        self.snake_eyes_box.update_for_enemy(input_state, dt);

        match self.data.state {
            EnemyState::StartTurn => {
                SNAKE_IDLE_ANIM.update(&mut self.idle_anim, dt);
                self.hand.reset_hand();
                self.dice_add_timer.reset();
                self.snake_eyes_box.reset(&mut self.hand.dice);
                self.data.state = EnemyState::WaitingForDiceToReturnToHand;
            }
            EnemyState::WaitingForDiceToReturnToHand => {
                SNAKE_IDLE_ANIM.update(&mut self.idle_anim, dt);
                let mut should_move_on = false;

                for dice in &self.hand.dice {
                    if dice.state != DiceState::Rolling {
                        continue;
                    }

                    should_move_on = true;
                }

                if should_move_on {
                    self.data.state = EnemyState::StartDiceStopDelayTime;
                    self.hand.roll_dice();
                }
            }
            EnemyState::StartDiceStopDelayTime => {
                SNAKE_IDLE_ANIM.update(&mut self.idle_anim, dt);
                self.before_stopping_dice_timer.track(dt);
                if self.before_stopping_dice_timer.is_done() {
                    self.before_stopping_dice_timer.reset();
                    self.data.state = EnemyState::StoppingDice;
                    self.hand.begin_dice_stop();
                }
            }
            EnemyState::StoppingDice => {
                SNAKE_IDLE_ANIM.update(&mut self.idle_anim, dt);
                if self.hand.stop_dice(dt) {
                    self.data.state = EnemyState::EvaluateRoll;
                }
            }

            EnemyState::EvaluateRoll => {
                SNAKE_IDLE_ANIM.update(&mut self.idle_anim, dt);
                if self.check_for_two_dice_with_value_one_in_hand() {
                    self.data.state = EnemyState::ChoosingDice;
                } else {
                    self.data.state = EnemyState::EndTurnDelay;
                }
            }

            //if you got to this state, it means that theres 2 1s
            EnemyState::ChoosingDice => {
                SNAKE_IDLE_ANIM.update(&mut self.idle_anim, dt);
                self.dice_add_timer.track(dt);

                if self.dice_add_timer.is_done() {
                    self.dice_add_timer.reset();

                    self.add_one_die();

                    if self.snake_eyes_box.get_data().dice_in_box.len() == 2 {
                        self.data.state = EnemyState::BeforeTallyDelay;
                    }
                }
            }
            EnemyState::BeforeTallyDelay => {
                SNAKE_IDLE_ANIM.update(&mut self.idle_anim, dt);
                self.before_tally_timer.track(dt);

                if self.before_tally_timer.is_done() {
                    self.data.state = EnemyState::TallyingTotal;
                    self.before_tally_timer.reset();
                }
            }

            EnemyState::TallyingTotal => {
                SNAKE_IDLE_ANIM.update(&mut self.idle_anim, dt);
                if self.snake_eyes_box.tally(dt) {
                    self.snake_eyes_box.get_mut_data().total_value_for_current_round = 11;
                }

                self.idle_anim.reset();
                self.data.state = EnemyState::BeforeActingDelay;
            }
            EnemyState::BeforeActingDelay => {
                SNAKE_ATTACK_ANIM.update(&mut self.attack_anim, dt);
                self.before_act_timer.track(dt);

                if self.before_act_timer.is_done() {
                    self.before_act_timer.reset();
                    self.data.state = EnemyState::Acting
                }
            }
            EnemyState::Acting => {
                SNAKE_ATTACK_ANIM.update(&mut self.attack_anim, dt);
                println!(
                    "Dealt {} Damage with snake eyes!",
                    self.snake_eyes_box.get_data().total_value_for_current_round
                );
                self.data.state = EnemyState::EndTurnDelay;
                self.attack_anim.reset();
            }
            EnemyState::EndTurnDelay => {
                SNAKE_IDLE_ANIM.update(&mut self.idle_anim, dt);
                self.turn_end_timer.track(dt);

                if self.turn_end_timer.is_done() {
                    self.turn_end_timer.reset();
                    self.data.state = EnemyState::WaitingForPlayer;
                    self.snake_eyes_box.get_mut_data().emit_smoke_at_each_dice(particle_system);
                    self.hand.emit_smoke_at_each_dice(particle_system);
                    self.snake_eyes_box.get_mut_data().reset_box(&mut self.hand.dice);
                }
            }
            EnemyState::WaitingForPlayer => {
                SNAKE_IDLE_ANIM.update(&mut self.idle_anim, dt);
                if player.state == PlayerState::WaitingForEnemy {
                    self.data.state = EnemyState::StartTurn;
                }
            }
            EnemyState::HitDelay => {
                self.data.hit_timer.track(dt);
                if self.data.hit_timer.is_done() {
                    if self.data.health <= 0 {
                        self.data.state = EnemyState::Dead;
                    } else {
                        self.data.state = EnemyState::WaitingForPlayer;
                    }
                }
            }
            EnemyState::Dead => (),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        self.snake_eyes_box.draw(d, texture, font);

        match self.data.state {
            EnemyState::WaitingForPlayer => {
                SNAKE_IDLE_ANIM.draw(&self.idle_anim, d, self.data.pos, texture);
                // hand not supposed to be drawn here, so thats why this exists
            }
            EnemyState::HitDelay => {
                //draw hit here
            }
            EnemyState::BeforeActingDelay | EnemyState::Acting => {
                SNAKE_ATTACK_ANIM.draw(&mut self.attack_anim, d, self.data.pos, texture);
            }
            _ => {
                SNAKE_IDLE_ANIM.draw(&self.idle_anim, d, self.data.pos, texture);
                self.hand.draw(d, texture);
            }
        }
    }

    fn add_one_die(&mut self) {
        for i in (0..self.hand.dice.len()).rev() {
            if self.hand.dice[i].value == 1 {
                let dice = self.hand.dice.remove(i);
                self.snake_eyes_box.get_mut_data().dice_in_box.push(dice);

                if let DiceBox::SnakeEyes { snake_eyes_box: dice_box } = &mut self.snake_eyes_box {
                    dice_box.snake_eyes_set_dice_positions();
                }

                return;
            }
        }
    }

    fn check_for_two_dice_with_value_one_in_hand(&self) -> bool {
        let mut num_of_ones = 0;

        for dice in &self.hand.dice {
            if dice.value == 1 {
                num_of_ones += 1;
            }
        }

        if num_of_ones >= 2 {
            return true;
        } else {
            return false;
        }
    }
}
