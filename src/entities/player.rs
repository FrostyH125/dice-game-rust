use basic_raylib_core::{
    graphics::{animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance},
    system::timer::Timer,
};
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::RaylibFont,
};

use crate::{
    EMPTY_SPRITE, GameContext, HitType, PLAYER_UI_X_CENTER_CORD, PLAYER_UI_Y_BASE_CORD,
    entities::{
        dice::{DICE_WIDTH_HEIGHT, DiceState},
        dice_box::{DiceBox, DiceBoxResult},
    },
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
const PLAYER_CENTER: Vector2 = Vector2::new(PLAYER_POS.x + PLAYER_WIDTH / 2.0, PLAYER_POS.y + PLAYER_HEIGHT / 2.0);
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
            shield_power: 0,
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
                    self.hand.reset_hand();
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

                // check if theres a number effect type for this box result
                if let Some(num_effect_type) = box_result.get_num_effect_type() {
                    // if there is a box result, find the proper rectangle (player or enemy)
                    // find the value of the action as well to display
                    // i kinda wanted to see if this process could be generalized, so if you
                    // happen to be doing a code review for me, perhaps its just something
                    // to consider, since doing this every time i need to do a number effect
                    // just seems to smell a little bit even if theres technically nothing wrong
                    // with doing so
                    let (num_effect_rect, value) = match box_result {
                        DiceBoxResult::BasicAttack(num) => (enemy.get_rect(), num),
                        DiceBoxResult::BasicHeal(num) => (self.get_rect(), num),
                        DiceBoxResult::ChargeShield(_) => panic!("shouldn't have a num effect rect"),
                        DiceBoxResult::None => panic!("shouldn't have a num effect rect"),
                    };

                    game_context.battle_effect_manager.add_number_effect(
                        num_effect_type,
                        num_effect_rect,
                        value,
                        &game_context.font,
                    );
                    println!("I got added")
                }

                match box_result {
                    DiceBoxResult::BasicAttack(damage) => enemy.take_hit(damage),
                    DiceBoxResult::BasicHeal(heal_amount) => self.heal(heal_amount),
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
                for dice_box in &mut self.dice_boxes {
                    dice_box.get_mut_data().emit_smoke_at_each_dice(&mut game_context.sprite_particle_system);
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
            PlayerState::HitDelay { hit_type } => {
                PLAYER_HIT_ANIM.update(&mut self.hit_anim, dt);
                if self.hit_anim.finished_playing {
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

        self.was_dragging_dice = self.is_dragging_dice;
    }

    pub fn reset(&mut self) {
        for dice_box in &mut self.dice_boxes {
            dice_box.reset(&mut self.hand.dice, PLAYER_CENTER + DICE_WIDTH_HEIGHT / 2.0);
        }

        self.hand.reset_hand();
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
            PlayerState::HitDelay { hit_type } => {
                PLAYER_HIT_ANIM.draw(&mut self.hit_anim, d, self.pos, &game_context.texture);
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

        // don't draw the health if you're walking or dead
        if let PlayerState::Walking | PlayerState::Dead = self.state {
            return;
        }

        let health_str = &format!("HP: {}", self.health);
        let font_size = 10.0;
        let spacing = 0.5;
        let size_of_str = game_context.font.measure_text(health_str, font_size, spacing);
        let self_width = 32.0;
        let self_height = 48.0;

        d.draw_text_ex(
            &game_context.font,
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

    pub fn take_hit(&mut self, damage: i32) {
        // had no shield
        if self.shield_power == 0 {
            self.health -= damage;
            self.state = PlayerState::HitDelay { hit_type: HitType::Unblocked };
            return;
        // had shield
        } else if self.shield_power > 0 {
            self.shield_power -= damage;

            match self.shield_power {
                // shield blocked all damage
                1.. => {
                    self.shield_power = 0;
                    self.state = PlayerState::HitDelay { hit_type: HitType::Blocked };
                }

                // shield blocked it just perfectly with no shield to spare
                0 => {
                    self.state = PlayerState::HitDelay { hit_type: HitType::PerfectBreak };
                }

                // shield broke and some damage came through
                ..=-1 => {
                    let overflow = self.shield_power.abs();
                    self.health -= overflow;
                    self.shield_power = 0;
                    self.state = PlayerState::HitDelay { hit_type: HitType::BlockedBroken };
                }
            }
        }
    }

    pub fn heal(&mut self, heal_amount: i32) {
        self.health += heal_amount;
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

    fn get_rect(&self) -> Rectangle {
        let rect = Rectangle::new(self.pos.x, self.pos.y, PLAYER_WIDTH, PLAYER_HEIGHT);
        return rect;
    }
}
