use basic_raylib_core::{
    graphics::{animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance}, system::{sprite_particle_system::SpriteParticleSystem, timer::Timer}, utils::math_utils::{self, center_of_rect},
};
use rand::RngExt;
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::RaylibFont,
};

use crate::{
    EMPTY_SPRITE, GRAVITY, GameContext, PLAYER_UI_X_CENTER_CORD, PLAYER_UI_Y_BASE_CORD, entities::{
        dice::{DiceState}, dice_box::{DiceBox, DiceBoxResult, HitType}, 
    }, game_effects::number_battle_effect::NumberEffectType,
};
use crate::{
    entities::{
        dice::{Dice, DiceKind},
        enemy::{Enemy, EnemyState},
        hand::Hand,
    },
    system::button::Button,
};

pub const PLAYER_WIDTH: f32 = 32.0;
pub const PLAYER_HEIGHT: f32 = 48.0;
const PLAYER_POS: Vector2 = Vector2::new(84.0, 125.0);
const PLAYER_HEALTH_TEXT_Y_OFFSET_FROM_BOTTOM_OF_SPRITE: f32 = 6.0;

static PLAYER_WALK_ANIM: AnimationData = AnimationData {
    frames: &[Sprite::new(80, 112, 32, 48), Sprite::new(112, 112, 32, 48)],
    frame_duration: 0.5,
    should_loop: true,
};

static PLAYER_THINKING_ANIM: AnimationData = AnimationData {
    frames: &[Sprite::new(144, 80, 32, 48), Sprite::new(176, 80, 32, 48)],
    frame_duration: 0.5,
    should_loop: true,
};

static PLAYER_WAITING_ANIM: AnimationData = AnimationData {
    frames: &[Sprite::new(144, 128, 32, 48), Sprite::new(176, 128, 32, 48)],
    frame_duration: 0.5,
    should_loop: true,
};

static PLAYER_HIT_ANIM: AnimationData = AnimationData {
    frames: &[EMPTY_SPRITE, Sprite::new(240, 128, 32, 48), EMPTY_SPRITE, Sprite::new(240, 128, 32, 48)],
    frame_duration: 0.25,
    should_loop: false,
};

static PLAYER_BLOCK_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(0, 416, 32, 48),
        Sprite::new(32, 416, 32, 48),
        Sprite::new(64, 416, 32, 48),
        Sprite::new(96, 416, 32, 48),
        Sprite::new(128, 416, 32, 48),
        Sprite::new(128, 416, 32, 48),
        Sprite::new(128, 416, 32, 48),
        Sprite::new(128, 416, 32, 48),
        Sprite::new(128, 416, 32, 48),
        Sprite::new(128, 416, 32, 48),
    ],
    frame_duration: 0.1,
    should_loop: false,
};

static PLAYER_BLOCK_BREAK_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(0, 416, 32, 48),
        Sprite::new(32, 416, 32, 48),
        Sprite::new(64, 416, 32, 48),
        Sprite::new(96, 416, 32, 48),
        Sprite::new(128, 416, 32, 48),
        Sprite::new(160, 416, 32, 48),
        Sprite::new(192, 416, 32, 48),
        Sprite::new(224, 416, 32, 48),
        Sprite::new(224, 416, 32, 48),
        Sprite::new(224, 416, 32, 48),
    ],
    frame_duration: 0.1,
    should_loop: false,
};

static SHIELD_PIECE_SPRITES_AND_TARGET_POS: &[(Sprite, Vector2)] = &[
    (Sprite::new(250, 435, 2, 4), Vector2::new(26.0, 19.0)),
    (Sprite::new(250, 440, 3, 4), Vector2::new(26.0, 24.0)),
    (Sprite::new(250, 445, 2, 5), Vector2::new(26.0, 29.0)),
    (Sprite::new(250, 451, 3, 6), Vector2::new(26.0, 35.0)),
    (Sprite::new(253, 435, 2, 2), Vector2::new(29.0, 19.0)),
    (Sprite::new(252, 438, 3, 3), Vector2::new(28.0, 22.0)),
    (Sprite::new(253, 442, 2, 2), Vector2::new(29.0, 26.0)),
    (Sprite::new(253, 445, 2, 4), Vector2::new(29.0, 29.0)),
    (Sprite::new(253, 450, 2, 5), Vector2::new(29.0, 34.0)),
];

#[derive(PartialEq, Eq)]
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

    ///the primary action visual will be the player (or enemy since this state
    ///exists there) animation and any other effects coming from the player based
    ///on the current box's player visual methods. In the transition to this state,
    ///the secondary visual will be applied, which will be a battle effect applied over
    ///the enemy like a slash or explosion or something. The reason these battle effects
    ///are a part of a different system really boils down to usability, as the same effect
    ///can be applied to multiple boxes, unlike player animations which are planned to be
    ///mostly unique, or at the very least, if the player animation isnt unique, other effects
    ///relating to the specific box's primary action visual will be unique, such as in magic
    ///boxes, different magic projectiles being part of the visual.
    ///will definitely need 2 different timings at least to be implemented for these action visuals,
    ///one timing is fully implemented at the moment, in the transition from tallying to the action visual,
    ///the second timing will most likely be at the same time the action is applied, which is also
    ///when the number effect is applied
    ActionVisual,
    ActionResult,
    EndTurnDelay,
    EndTurn,
    WaitingForEnemy,
    HitDelay {
        hit_type: HitType,
        player_damage: i32,
        shield_damage: i32
    },
    Dead,
}

pub struct Player {
    pub dice_boxes: Vec<DiceBox>,
    pub hand: Hand,
    health: i32,
    shield_power: i32,
    pub current_box: usize,
    walk_anim: SpriteAnimationInstance,
    thinking_anim: SpriteAnimationInstance,
    waiting_anim: SpriteAnimationInstance,
    hit_anim: SpriteAnimationInstance,
    acting_anim: SpriteAnimationInstance,
    pos: raylib::math::Vector2,
    acting_timer: Timer,
    end_turn_delay_timer: Timer,
    pub state: PlayerState,
    pub is_dragging_dice: bool,
    pub was_dragging_dice: bool,
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
            shield_power: 30,
            state: PlayerState::Walking,
            acting_timer: Timer::new(1.0),
            end_turn_delay_timer: Timer::new(2.0),
            is_dragging_dice: false,
            was_dragging_dice: false,
            current_box: 0,
        }
    }

    pub fn update(
        &mut self,
        confirm_button: &mut Button,
        stop_button: &mut Button,
        reroll_button: &mut Button,
        enemy: &mut Enemy,
        game_context: &mut GameContext,
        dt: f32,
    ) {
        if !game_context.input_state.dragging {
            self.is_dragging_dice = false;
        }

        // so much cleaner than what i had before, now theres no mutable bool reference
        self.is_dragging_dice = self.are_any_dice_dragged_in_boxes() || self.hand.are_any_dice_being_dragged();

        self.hand.update_for_player(self.is_dragging_dice, &game_context.input_state, dt);

        for dice_box in &mut self.dice_boxes {
            dice_box.update_for_player(self.is_dragging_dice, &mut self.hand, &game_context.input_state, dt);
        }

        // this is here to run after all boxes have updated
        if !self.is_dragging_dice && self.was_dragging_dice {
            self.hand.arrange_hand(false);
            for dice_box in &mut self.dice_boxes {
                dice_box.get_mut_data().arrange_dice();
            }
        }

        match self.state {
            PlayerState::Walking => {
                PLAYER_WALK_ANIM.update(&mut self.walk_anim, dt);
            }
            PlayerState::StartTurn => {
                self.reset_player_dice_and_arrange_hand();
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
                if stop_button.is_pressed(&game_context.input_state) {
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

                if self.hand.dice.len() > 0 && reroll_button.is_pressed(&game_context.input_state) {
                    self.hand.reset_dice_and_arrange_hand();
                    self.hand.begin_dice_stop();

                    confirm_button.deactivate();
                    reroll_button.deactivate();

                    self.state = PlayerState::RerollingDice;
                }

                if confirm_button.is_pressed(&game_context.input_state) {
                    self.thinking_anim.reset();
                    self.state = PlayerState::TallyingCurrentBox;
                    self.hand.emit_smoke_at_each_dice(&mut game_context.sprite_particle_system);
                    confirm_button.deactivate();
                    reroll_button.deactivate();

                    let mut all_boxes_empty = true;
                    for dice_box in &self.dice_boxes {
                        if dice_box.get_data().dice_in_box.len() > 0 {
                            all_boxes_empty = false;
                        }
                    }
                    if all_boxes_empty {
                        self.state = PlayerState::EndTurn;
                    }
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
                        
                        // even though this value isnt read here, it causes problems
                        // in places like scoreboard that rely on this data
                        self.current_box = self.dice_boxes.len() - 1;
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

                    // add a battle effect to the game based on current box if applicable
                    // this method can return none, in which case it'll be skipped.
                    // I put it in this state because it should only run once and it should
                    // only run right at the same time the pre action visual would run
                    if let Some(effect_type) =
                        self.dice_boxes[self.current_box].get_battle_effect_type_pre_action_result()
                    {
                        game_context.battle_effect_manager.add_effect(effect_type, enemy.get_rect());
                    }
                }
            }
            PlayerState::ActionVisual => {
                if self.dice_boxes[self.current_box].player_update_before_action_visuals(
                    &mut self.acting_anim,
                    game_context,
                    self.pos,
                    dt,
                ) {
                    self.acting_anim.reset();
                    self.state = PlayerState::ActionResult;
                }
            }
            PlayerState::ActionResult => {
                PLAYER_WAITING_ANIM.update(&mut self.waiting_anim, dt);

                let box_result = self.dice_boxes[self.current_box].get_result();

                match box_result {
                    DiceBoxResult::BasicAttack(damage) => enemy.take_hit(damage, game_context),
                    DiceBoxResult::BasicHeal(heal_amount) => self.heal(heal_amount, game_context),
                    DiceBoxResult::ChargeShield(shield_charge) => self.shield_power += shield_charge,
                    DiceBoxResult::None => (),
                }

                self.current_box += 1;

                // make sure current box index never gets above the actual number of boxes
                // this same exact check exists in tallying current box as well as it is
                // possible for the current box to be empty and the index to be incremented there
                if self.current_box > self.dice_boxes.len() - 1 {
                    self.current_box = self.dice_boxes.len() - 1;
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

                self.reset_boxes_and_put_dice_at_center_pos(true, Some(&mut game_context.sprite_particle_system));
                self.state = PlayerState::WaitingForEnemy;
            }
            PlayerState::WaitingForEnemy => {
                PLAYER_WAITING_ANIM.update(&mut self.waiting_anim, dt);

                if let EnemyState::WaitingForPlayer = enemy.get_data().state {
                    self.state = PlayerState::StartTurn;
                    self.waiting_anim.reset();
                }
            }
            PlayerState::HitDelay { hit_type, player_damage, shield_damage } => {
                let mut should_end_hit_delay = false;

                match hit_type {
                    HitType::Unblocked => {
                        PLAYER_HIT_ANIM.update(&mut self.hit_anim, dt);
                        if self.hit_anim.finished_playing {
                            self.hit_anim.reset();
                            should_end_hit_delay = true;
                        }
                    }
                    HitType::Blocked => {
                        PLAYER_BLOCK_ANIM.update(&mut self.hit_anim, dt);
                        if self.hit_anim.finished_playing {
                            should_end_hit_delay = true;
                            self.hit_anim.reset();
                            self.shield_power -= shield_damage;
                            game_context.battle_effect_manager.add_number_effect(NumberEffectType::Block, self.get_rect(), shield_damage, &game_context.font);
                        }
                    }
                    HitType::BlockedBroken => {
                        PLAYER_BLOCK_BREAK_ANIM.update(&mut self.hit_anim, dt);
                        if self.hit_anim.finished_playing {
                            self.add_shield_pieces(game_context);
                            self.shield_power = 0;
                            self.hit_anim.reset();
                            game_context.battle_effect_manager.add_number_effect(NumberEffectType::Block, self.get_rect(), shield_damage, &game_context.font);

                            // hit player w leftover damage
                            self.manage_getting_hit_into_correct_hit_state(player_damage, game_context);
                        }
                    }
                    HitType::PerfectBreak => {
                        todo!()
                        // play the blocking animation, have a screen pause (?), and have a ton of shiny particles of lighter sparkle colors and more numerous and faster
                    }
                }

                if should_end_hit_delay {
                    if self.health <= 0 {
                        self.state = PlayerState::Dead
                    } else {
                        self.state = PlayerState::WaitingForEnemy;
                    }
                }
            }
            PlayerState::Dead => (),
        }

        self.was_dragging_dice = self.is_dragging_dice;
    }

    // this fn is basically only used in main.rs for game over and transition stuff
    pub fn reset_boxes_and_hand(&mut self) {

        let center = center_of_rect(self.get_rect());
        
        for dice_box in &mut self.dice_boxes {
            dice_box.reset_and_place_dice_at_pos_for_next_round(&mut self.hand.dice, center);
        }
        self.current_box = 0;

        self.hand.reset_dice_and_arrange_hand();
    }

    fn reset_player_dice_and_arrange_hand(&mut self) {
        self.hand.reset_dice_and_arrange_hand();
    }

    fn reset_boxes_and_put_dice_at_center_pos(&mut self, emit_smoke: bool, sprite_particle_system: Option<&mut SpriteParticleSystem>) {
        let center = center_of_rect(self.get_rect());
        
        if emit_smoke {
            let ps = sprite_particle_system.expect("particle system required when emit_smoke is true");
        
            for dice_box in &mut self.dice_boxes {
                dice_box.emit_smoke_at_each_dice(ps);
                dice_box.reset_and_place_dice_at_pos_for_next_round(&mut self.hand.dice, center);
            }
        } else {
            for dice_box in &mut self.dice_boxes {
                dice_box.reset_and_place_dice_at_pos_for_next_round(&mut self.hand.dice, center);
            }
        }
        
        self.current_box = 0;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, game_context: &GameContext) {
        match self.state {
            PlayerState::Walking => {
                PLAYER_WALK_ANIM.draw(&self.walk_anim, d, self.pos, &game_context.texture);
            }
            PlayerState::WaitingForEnemy => {
                PLAYER_WAITING_ANIM.draw(&self.waiting_anim, d, self.pos, &game_context.texture);
                for dice_box in &mut self.dice_boxes {
                    dice_box.draw(d, game_context);
                }
            }
            PlayerState::HitDelay { hit_type, .. } => {
                match hit_type {
                    HitType::Unblocked => PLAYER_HIT_ANIM.draw(&mut self.hit_anim, d, self.pos, &game_context.texture),
                    HitType::Blocked => PLAYER_BLOCK_ANIM.draw(&mut self.hit_anim, d, self.pos, &game_context.texture),
                    HitType::BlockedBroken => PLAYER_BLOCK_BREAK_ANIM.draw(&mut self.hit_anim, d, self.pos, &game_context.texture),
                    HitType::PerfectBreak => todo!(),
                }

                for dice_box in &mut self.dice_boxes {
                    dice_box.draw(d, game_context);
                }
            }
            PlayerState::ChoosingDice => {
                PLAYER_THINKING_ANIM.draw(&self.thinking_anim, d, self.pos, &game_context.texture);
                for dice_box in &mut self.dice_boxes {
                    dice_box.draw(d, game_context);
                }
                self.hand.draw(d, &game_context.texture);
            }
            PlayerState::RerollingDice
            | PlayerState::RollingDice
            | PlayerState::StoppingDice
            | PlayerState::WaitingForDiceToMoveToHand => {
                PLAYER_WAITING_ANIM.draw(&self.waiting_anim, d, self.pos, &game_context.texture);
                for dice_box in &mut self.dice_boxes {
                    dice_box.draw(d, game_context);
                }
                self.hand.draw(d, &game_context.texture);
            }
            PlayerState::ActionVisual => {
                self.dice_boxes[self.current_box].player_draw_action(
                    &mut self.acting_anim,
                    d,
                    self.pos,
                    &game_context.texture,
                );
                for dice_box in &mut self.dice_boxes {
                    dice_box.draw(d, game_context);
                }
            }
            PlayerState::Dead => {}
            _ => {
                PLAYER_WAITING_ANIM.draw(&self.waiting_anim, d, self.pos, &game_context.texture);
                for dice_box in &mut self.dice_boxes {
                    dice_box.draw(d, game_context);
                }
            }
        }

        // don't draw the health and shield power if you're walking or dead
        if let PlayerState::Walking | PlayerState::Dead = self.state {
            return;
        }

        self.draw_health_and_shield(d, game_context);
    }

    fn draw_health_and_shield(&self, d: &mut RaylibDrawHandle, game_context: &GameContext) {
        const SH_HP_STR_FONT_SIZE: f32 = 10.0;
        const SH_HP_STR_SPACING: f32 = 0.5;
        const MARGIN_BETWEEN_SH_AND_HP_STRINGS: f32 = 10.0;
        const SHIELD_STR_COLOR: Color = Color::new(180, 180, 200, 255);
        const HEALTH_STR_COLOR: Color = Color::WHITE;

        let health_str = &format!("HP:{}", self.health);
        let shield_str = &format!("SH:{}", self.shield_power);

        let size_of_hp_str = game_context.font.measure_text(health_str, SH_HP_STR_FONT_SIZE, SH_HP_STR_SPACING);
        let size_of_sh_str = game_context.font.measure_text(shield_str, SH_HP_STR_FONT_SIZE, SH_HP_STR_SPACING);

        let hp_pos_x = self.pos.x + PLAYER_WIDTH / 2.0
            - (size_of_hp_str.x + size_of_sh_str.x + MARGIN_BETWEEN_SH_AND_HP_STRINGS) / 2.0;
        let sh_pos_x = hp_pos_x + size_of_hp_str.x + MARGIN_BETWEEN_SH_AND_HP_STRINGS;
        let strings_pos_y = self.pos.y + PLAYER_HEIGHT + PLAYER_HEALTH_TEXT_Y_OFFSET_FROM_BOTTOM_OF_SPRITE;

        d.draw_text_ex(
            &game_context.font,
            health_str,
            Vector2::new(hp_pos_x, strings_pos_y),
            SH_HP_STR_FONT_SIZE,
            SH_HP_STR_SPACING,
            HEALTH_STR_COLOR,
        );

        d.draw_text_ex(
            &game_context.font,
            shield_str,
            Vector2::new(sh_pos_x, strings_pos_y),
            SH_HP_STR_FONT_SIZE,
            SH_HP_STR_SPACING,
            SHIELD_STR_COLOR,
        );
    }

    pub fn manage_getting_hit_into_correct_hit_state(&mut self, damage: i32, game_context: &mut GameContext) {
        // had no shield
        if self.shield_power == 0 {
            self.state = PlayerState::HitDelay { hit_type: HitType::Unblocked, player_damage: damage, shield_damage: 0 };
            self.health -= damage;
            game_context.battle_effect_manager.add_number_effect(
                NumberEffectType::Damage,
                self.get_rect(),
                damage,
                &game_context.font,
            );
            return;
        // had shield
        } else if self.shield_power > 0 {

            let leftover_shield_power = self.shield_power - damage;
            
            match leftover_shield_power {
                // shield blocked all damage
                1.. => {
                    self.state = PlayerState::HitDelay { hit_type: HitType::Blocked, player_damage: 0, shield_damage: damage };
                }

                // shield blocked it just perfectly with no shield to spare
                0 => {
                    self.state = PlayerState::HitDelay { hit_type: HitType::PerfectBreak, player_damage: 0, shield_damage: damage };
                }

                // shield broke and some damage came through
                ..=-1 => {
                    let overflow = leftover_shield_power.abs();
                    println!("{}", self.shield_power);
                    self.state = PlayerState::HitDelay { hit_type: HitType::BlockedBroken, player_damage: overflow, shield_damage: self.shield_power };
                }
            }
        }
    }

    pub fn heal(&mut self, heal_amount: i32, game_context: &mut GameContext) {
        self.health += heal_amount;
        game_context.battle_effect_manager.add_number_effect(
            NumberEffectType::Heal,
            self.get_rect(),
            heal_amount,
            &game_context.font,
        );
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
        let collect_rect_height = 32.0;

        let bottom_layer_y = self.pos.y - margin - dice_box_height;
        let top_layer_y = bottom_layer_y - margin - dice_box_height - collect_rect_height;

        // need to modify this as number of boxes moves from 1 to 4
        // eventually number of boxes may be 5+, but we will cross that bridge when we get there
        match num_of_boxes {
            1 => {
                let box_data = self.dice_boxes[0].get_mut_data();
                let half_dice_box_width = box_data.width / 2.0;
                let pos_x = self.pos.x + half_player_width - half_dice_box_width;
                let pos_y = bottom_layer_y;
                box_data.pos.x = pos_x;
                box_data.pos.y = pos_y;
            }
            2 => {
                let box_one_data = self.dice_boxes[0].get_mut_data();
                let box_one_width = box_one_data.width;
                let box_one_pos_x = (self.pos.x - box_one_width) - margin;
                let box_one_pos_y = bottom_layer_y;
                box_one_data.pos.x = box_one_pos_x;
                box_one_data.pos.y = box_one_pos_y;

                let box_two_data = self.dice_boxes[1].get_mut_data();
                let box_two_pos_x = self.pos.x + PLAYER_WIDTH + margin;
                let box_two_pos_y = bottom_layer_y;
                box_two_data.pos.x = box_two_pos_x;
                box_two_data.pos.y = box_two_pos_y;
            }
            3 => {
                let box_one_data = self.dice_boxes[0].get_mut_data();
                let first_box_width = box_one_data.width;
                let box_one_pos_x = (self.pos.x - first_box_width) - margin;
                let box_one_pos_y = top_layer_y;
                box_one_data.pos.x = box_one_pos_x;
                box_one_data.pos.y = box_one_pos_y;

                let box_two_data = self.dice_boxes[1].get_mut_data();
                let box_two_pos_x = self.pos.x + PLAYER_WIDTH + margin;
                let box_two_pos_y = top_layer_y;
                box_two_data.pos.x = box_two_pos_x;
                box_two_data.pos.y = box_two_pos_y;

                let box_three_data = self.dice_boxes[2].get_mut_data();
                let half_dice_box_width = box_three_data.width / 2.0;
                let box_three_pos_x = self.pos.x + half_player_width - half_dice_box_width;
                let box_three_pos_y = bottom_layer_y;
                box_three_data.pos.x = box_three_pos_x;
                box_three_data.pos.y = box_three_pos_y;
            }
            4 => {
                let box_one_data = self.dice_boxes[0].get_mut_data();
                let box_one_width = box_one_data.width;
                let box_one_pos_x = (self.pos.x - box_one_width) - margin;
                let box_one_pos_y = top_layer_y;
                box_one_data.pos.x = box_one_pos_x;
                box_one_data.pos.y = box_one_pos_y;

                let box_two_data = self.dice_boxes[1].get_mut_data();
                let box_two_pos_x = self.pos.x + PLAYER_WIDTH + margin;
                let box_two_pos_y = top_layer_y;
                box_two_data.pos.x = box_two_pos_x;
                box_two_data.pos.y = box_two_pos_y;

                let box_three_data = self.dice_boxes[2].get_mut_data();
                let box_three_width = box_three_data.width;
                let box_three_pos_x = (self.pos.x - box_three_width) - margin;
                let box_three_pos_y = bottom_layer_y;
                box_three_data.pos.x = box_three_pos_x;
                box_three_data.pos.y = box_three_pos_y;

                let box_four_data = self.dice_boxes[3].get_mut_data();
                let box_four_pos_x = self.pos.x + PLAYER_WIDTH + margin;
                let box_four_pos_y = bottom_layer_y;
                box_four_data.pos.x = box_four_pos_x;
                box_four_data.pos.y = box_four_pos_y;
            }
            _ => unimplemented!("place_boxes(player) not implemented for {} boxes", num_of_boxes),
        }

        for dice_box in &mut self.dice_boxes {
            dice_box.adjust_collect_rect_pos_for_current_pos();
            dice_box.adjust_info_hover_pos_for_current_pos();
        }
    }

    fn are_any_dice_dragged_in_boxes(&self) -> bool {
        for db in &self.dice_boxes {
            if db.get_data().are_any_dice_being_dragged() {
                return true;
            }
        }

        return false;
    }

    pub fn get_rect(&self) -> Rectangle {
        let rect = Rectangle::new(self.pos.x, self.pos.y, PLAYER_WIDTH, PLAYER_HEIGHT);
        return rect;
    }

    fn add_shield_pieces(&self, game_context: &mut GameContext) {
        let mut rng = rand::rng();

        for (sprite, pos) in SHIELD_PIECE_SPRITES_AND_TARGET_POS {
            let x_speed: f32 = rng.random_range(90.0..=120.0);
            let y_speed: f32 = rng.random_range(300.0..=360.0);
            let x_dir: f32 = rng.random_range(-1.0..=-0.75);
            let y_dir: f32 = rng.random_range(-1.0..=0.25);
            let velocity = Vector2::new(x_dir * x_speed, y_dir * y_speed);
            let acceleration = Vector2::new(-x_dir * x_speed / 100.0, GRAVITY);
            let rotation_speed: f32 = rng.random_range(-360.0..=360.0);

            game_context.sprite_particle_system.emit_ex(
                sprite,
                *pos + self.pos,
                velocity,
                acceleration,
                rotation_speed,
                0.0,
                3.0,
                true,
            );
        }
    }
}
