use basic_raylib_core::{
    graphics::{animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance},
    system::timer::Timer,
};
use raylib::{math::Vector2, prelude::RaylibDrawHandle, text::Font};

use crate::{
    EMPTY_SPRITE, GameContext, VIRTUAL_WIDTH, entities::{
        dice::{DICE_WIDTH_HEIGHT, Dice, DiceKind, DiceState},
        dice_box::DiceBox,
        enemy::{ENEMY_HAND_X_CENTER_CORD, ENEMY_HAND_Y_CORD, EnemyData, EnemyState::{self, StartTurn}},
        enemy_dice_boxes::snake_eyes::SnakeEyes,
        hand::Hand,
        player::{Player, PlayerState},
    }
};

const BEFORE_ATTACKING_TIME: f32 = 2.0;
const HIT_TIME: f32 = 1.0;
const SNAKE_POS: Vector2 = Vector2::new(VIRTUAL_WIDTH - 116.0, 125.0);
const SNAKE_WIDTH: u32 = 32;
const SNAKE_HEIGHT: u32 = 48;
const CENTER_OF_SNAKE: Vector2 = Vector2::new(SNAKE_POS.x + SNAKE_WIDTH as f32 / 2.0, SNAKE_POS.y + SNAKE_HEIGHT as f32 / 2.0);

const SNAKE_EYES_INDEX: usize = 0;

static SNAKE_IDLE_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(144, 176, 32, 48),
        Sprite::new(176, 176, 32, 48),
        Sprite::new(208, 176, 32, 48),
        Sprite::new(240, 176, 32, 48),
    ],
    frame_duration: 0.5,
    should_loop: true,
};

static SNAKE_ATTACK_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(144, 224, 32, 48),
        Sprite::new(176, 224, 32, 48),
        Sprite::new(208, 224, 32, 48),
        Sprite::new(240, 224, 32, 48),
    ],
    frame_duration: BEFORE_ATTACKING_TIME / 8.0,
    should_loop: true,
};

static SNAKE_HIT_ANIM: AnimationData = AnimationData {
    frames: &[EMPTY_SPRITE, Sprite::new(272, 176, SNAKE_WIDTH, SNAKE_HEIGHT)],
    frame_duration: HIT_TIME / 4.0,
    should_loop: true,
};

pub struct Snake {
    pub data: EnemyData,
    pub hand: Hand,
    hit_timer: Timer,
    dice_add_timer: Timer,
    before_stopping_dice_timer: Timer,
    before_tally_timer: Timer,
    before_attack_timer: Timer,
    turn_end_timer: Timer,
    idle_anim: SpriteAnimationInstance,
    attack_anim: SpriteAnimationInstance,
    hit_anim: SpriteAnimationInstance,
}

impl Snake {
    pub fn new(font: &Font) -> Self {
        Snake {
            data: EnemyData {
                health: 100.0,
                shield_power: 0.0,
                pos: SNAKE_POS,
                state: EnemyState::WaitingForPlayer,
                width: 32.0,
                height: 48.0,
                dice_boxes: vec![DiceBox::SnakeEyes { snake_eyes_box: SnakeEyes::new(font) }],
                current_box: SNAKE_EYES_INDEX,
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
            dice_add_timer: Timer::new(1.5),
            before_stopping_dice_timer: Timer::new(1.0),
            before_tally_timer: Timer::new(1.0),
            before_attack_timer: Timer::new(BEFORE_ATTACKING_TIME),
            hit_timer: Timer::new(HIT_TIME),
            turn_end_timer: Timer::new(2.0),
            idle_anim: SpriteAnimationInstance::new(),
            attack_anim: SpriteAnimationInstance::new(),
            hit_anim: SpriteAnimationInstance::new(),
        }
    }

    pub fn update(
        &mut self,
        player: &mut Player,
        game_context: &mut GameContext,
        dt: f32,
    ) {
        self.hand.update_for_enemy(dt);
        self.data.dice_boxes[SNAKE_EYES_INDEX].update_for_enemy(&game_context.input_state, dt);

        match self.data.state {
            EnemyState::StartTurn => {
                SNAKE_IDLE_ANIM.update(&mut self.idle_anim, dt);
                self.hand.reset_hand();
                self.dice_add_timer.reset();
                self.data.state = EnemyState::WaitingForDiceToReturnToHand;
            }
            EnemyState::WaitingForDiceToReturnToHand => {
                SNAKE_IDLE_ANIM.update(&mut self.idle_anim, dt);
                let mut should_move_on = false;

                for dice in &self.hand.dice {
                    if let DiceState::Rolling = dice.state {
                        should_move_on = true;
                    } else {
                        should_move_on = false;
                    }
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

                    if self.data.dice_boxes[SNAKE_EYES_INDEX].get_data().dice_in_box.len() == 2 {
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

                // if it got to this stage, it will be 11.0
                self.data.dice_boxes[SNAKE_EYES_INDEX].tally(dt);

                self.idle_anim.reset();
                self.data.state = EnemyState::BeforeActingDelay;
            }
            EnemyState::BeforeActingDelay => {
                SNAKE_ATTACK_ANIM.update(&mut self.attack_anim, dt);
                self.before_attack_timer.track(dt);

                if self.before_attack_timer.is_done() {
                    self.before_attack_timer.reset();
                    self.data.state = EnemyState::Acting
                }
            }
            EnemyState::Acting => {
                SNAKE_ATTACK_ANIM.update(&mut self.attack_anim, dt);

                let result = self.data.dice_boxes[SNAKE_EYES_INDEX].get_result();
                self.data.dice_boxes[SNAKE_EYES_INDEX].enemy_action(result, player, &mut self.data.health, &mut self.data.shield_power);

                self.data.state = EnemyState::EndTurnDelay;
                self.attack_anim.reset();
            }
            EnemyState::EndTurnDelay => {
                SNAKE_IDLE_ANIM.update(&mut self.idle_anim, dt);
                self.turn_end_timer.track(dt);

                if self.turn_end_timer.is_done() {
                    self.data.state = EnemyState::EndTurn;
                    self.turn_end_timer.reset();
                }
            }
            EnemyState::EndTurn => {
                self.data.state = EnemyState::WaitingForPlayer;
                self.data.dice_boxes[SNAKE_EYES_INDEX].get_mut_data().emit_smoke_at_each_dice(&mut game_context.sprite_particle_system);
                self.hand.emit_smoke_at_each_dice(&mut game_context.sprite_particle_system);
                self.data.dice_boxes[SNAKE_EYES_INDEX]
                    .reset(&mut self.hand.dice, CENTER_OF_SNAKE + DICE_WIDTH_HEIGHT / 2.0);
            }
            EnemyState::WaitingForPlayer => {
                SNAKE_IDLE_ANIM.update(&mut self.idle_anim, dt);

                if let PlayerState::WaitingForEnemy = player.state {
                    self.data.state = EnemyState::StartTurn;
                }
            }
            EnemyState::HitDelay => {
                SNAKE_HIT_ANIM.update(&mut self.hit_anim, dt);
                self.hit_timer.track(dt);
                if self.hit_timer.is_done() {
                    if self.data.health <= 0.0 {
                        self.data.state = EnemyState::Dead;
                    } else {

                        // needed to add this if else statement because 
                        // when the player would hit the enemy, making them go to 
                        // hit delay, when they came out of hit delay, they would 
                        // originally automatically go to waiting for player
                        // which was a problem, because the player by that time had
                        // already gone to waiting for enemy, and in that one frame
                        // of being in waiting for player, the player would, who updated next
                        // would naturally start their turn because they saw that the 
                        // enemy was waiting for them
                        // this if clause means that if the player is waiting instead,
                        // meaning their turn is over, to just go straight to starting
                        // the turn
                        if player.state == PlayerState::WaitingForEnemy {
                            self.data.state = StartTurn;
                        } else {
                            self.data.state = EnemyState::WaitingForPlayer;       
                        }
                        
                    }
                    self.hit_anim.reset();
                    self.hit_timer.reset();
                }
            }
            EnemyState::Dead => (),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, game_context: &GameContext) {
        self.data.dice_boxes[0].draw(d, game_context);

        match self.data.state {
            EnemyState::WaitingForPlayer => {
                SNAKE_IDLE_ANIM.draw(&self.idle_anim, d, self.data.pos, &game_context.texture);
                // hand not supposed to be drawn here, so thats why this exists
            }
            EnemyState::HitDelay => {
                SNAKE_HIT_ANIM.draw(&mut self.hit_anim, d, self.data.pos, &game_context.texture);
            }
            EnemyState::BeforeActingDelay | EnemyState::Acting => {
                SNAKE_ATTACK_ANIM.draw(&mut self.attack_anim, d, self.data.pos, &game_context.texture);
                self.hand.draw(d, &game_context.texture);
            }
            _ => {
                SNAKE_IDLE_ANIM.draw(&self.idle_anim, d, self.data.pos, &game_context.texture);
                self.hand.draw(d, &game_context.texture);
            }
        }
    }

    fn add_one_die(&mut self) {
        for i in (0..self.hand.dice.len()).rev() {
            if self.hand.dice[i].value == 1 {
                let dice = self.hand.remove_dice(i);
                let snake_eyes_box = &mut self.data.dice_boxes[0];
                snake_eyes_box.get_mut_data().add_dice(dice);
                snake_eyes_box.enemy_set_dice_positions();
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
