use crate::{
    GameContext, VIRTUAL_WIDTH, entities::{
        dice::DiceState, dice_box::{DiceBox, HitType}, enemies::snake::Snake, hand::Hand, player::Player,
    }, game_effects::attack_affinity::AttackAffinity,
};
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::RaylibFont,
};

pub const ENEMY_HAND_X_CENTER_CORD: f32 = VIRTUAL_WIDTH - 100.0;
pub const ENEMY_HAND_Y_CORD: f32 = 195.0;
pub const ENEMY_HEALTH_TEXT_Y_OFFSET_FROM_BOTTOM_OF_SPRITE: f32 = 6.0;

#[derive(PartialEq, Eq)]
pub enum EnemyState {
    ///reset hands and boxes
    StartTurn,

    ///waits for hand to recieve all dice physically before continuing
    WaitingForDiceToReturnToHand,

    ///exists only to smoothly transition from start turn to
    ///actually letting the dice stop. Should have a timer, and
    ///once the timer goes off, put all the hands to start stopping
    ///their dice
    StartDiceStopDelayTime,

    ///waits for the dice to be stopped
    StoppingDice,

    ///once the hand is stopped, chooses to either choose dice based on the
    ///roll, or go straight to ending the turn
    EvaluateRoll,

    ///actually chooses which dice to add to their box
    ChoosingDice,

    ///some transition time between choosing the final die, and tallying
    BeforeTallyDelay,

    /// can either be tallying on by one like in the player boxes or tally all at once based on a condition like snake eyes
    TallyingTotal,

    ///for the animation of the action
    BeforeActingDelay,

    ///handles the actual acting logic (should only be one frame long)
    Acting,

    ///handles being hit, animation for being hit, other effects for being hit, before turning back to waiting for player
    HitDelay {
        hit_type: HitType,
        enemy_damage: i32,
        shield_damage: i32
    },

    ///the delay before fully ending turn for seamless, sensible transitions
    EndTurnDelay,

    ///everything that needs to happen before going into the waiting state
    EndTurn,

    ///should be a simple check to see if player is waiting for enemy, and then
    ///if so, start turn
    WaitingForPlayer,

    Dead,
}

pub struct EnemyData {
    pub weaknesses: Vec<AttackAffinity>,
    pub resistances: Vec<AttackAffinity>,
    
    pub health: i32,
    pub shield_power: i32,
    pub pos: Vector2,
    pub state: EnemyState,

    // added these to position things properly depending on enemy
    pub width: f32,
    pub height: f32,

    // added the boxes as official enemy data for the scoreboard to function
    pub dice_boxes: Vec<DiceBox>,
    pub hand: Hand,
    pub current_box: usize,
}

impl EnemyData {
    pub fn get_rect(&self) -> Rectangle {
        return Rectangle::new(self.pos.x, self.pos.y, self.width, self.height);
    }

    pub fn are_dice_back_in_hand(&self) -> bool {
        
        for dice in &self.hand.dice {
            match dice.state {
                // only move on if every dice is rolling
                DiceState::Rolling => continue,
                _ => return false
            }
        }

        // can only make it here if each dice is rolling
        return true;
    }
}

pub enum Enemy {
    Snake { snake: Snake },
}

impl Enemy {
    fn get_mut_data(&mut self) -> &mut EnemyData {
        match self {
            Self::Snake { snake } => &mut snake.data,
        }
    }

    pub fn get_data(&self) -> &EnemyData {
        match self {
            Self::Snake { snake } => &snake.data,
        }
    }

    pub fn take_hit(&mut self, damage: i32, affinity: AttackAffinity, game_context: &mut GameContext) {
        let data = self.get_mut_data();

        let final_damage = affinity.get_final_damage(damage, &data.weaknesses, &data.resistances);

        game_context.battle_effect_manager.add_number_effect(
            crate::game_effects::number_battle_effect::NumberEffectType::Damage,
            data.get_rect(),
            final_damage,
            &game_context.font,
        );

        // had no shield
        if data.shield_power == 0 {
            data.health -= final_damage;
            data.state = EnemyState::HitDelay { hit_type: HitType::Unblocked, enemy_damage: final_damage, shield_damage: 0 };
            return;
        // had shield
        } else if data.shield_power > 0 {
            data.shield_power -= final_damage;

            match data.shield_power {
                // shield blocked all damage
                1.. => {
                    data.state = EnemyState::HitDelay { hit_type: HitType::Blocked, enemy_damage: 0, shield_damage: final_damage };
                }

                // shield blocked it just perfectly with no shield to spare
                0 => {
                    data.state = EnemyState::HitDelay { hit_type: HitType::PerfectBreak, enemy_damage: 0, shield_damage: final_damage };
                }

                // shield broke and some damage came through
                ..=-1 => {
                    let overflow = data.shield_power.abs();
                    data.shield_power = 0;
                    data.state = EnemyState::HitDelay { hit_type: HitType::BlockedBroken, enemy_damage: overflow, shield_damage: final_damage - overflow };
                }
            }
        }
    }

    pub fn update(&mut self, player: &mut Player, game_context: &mut GameContext, dt: f32) {
        match self {
            Self::Snake { snake } => snake.update(player, game_context, dt),
        }
    }

    pub fn place_boxes(&mut self) {
        let margin = 5.0;
        let dice_box_height = 16.0;
        let self_width = self.get_data().width;
        let self_pos = self.get_data().pos;

        let boxes = self.get_mut_data().dice_boxes.as_mut_slice();

        let num_of_boxes = boxes.len();

        match num_of_boxes {
            1 => {
                let self_half_width = self_width / 2.0;

                let box_data = boxes[0].get_mut_data();
                let half_dice_box_width = box_data.width / 2.0;
                let box_x_pos = self_pos.x + self_half_width - half_dice_box_width;
                let box_y_pos = self_pos.y - margin - dice_box_height;
                box_data.pos = Vector2::new(box_x_pos, box_y_pos);
            }
            _ => unimplemented!("place_boxes(enemy) not implemented for {} boxes", num_of_boxes),
        }

        for dice_box in boxes {
            dice_box.adjust_info_hover_pos_for_current_pos();
            dice_box.adjust_collect_rect_pos_for_current_pos();
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, game_context: &GameContext) {
        match self {
            Self::Snake { snake } => snake.draw(d, game_context),
        }

        let pos = self.get_data().pos;
        let font_size = 10.0;
        let spacing = 0.5;
        let health_str = &format!("HP: {}", self.get_data().health);
        let size_of_health_str = game_context.font.measure_text(health_str, font_size, spacing);

        d.draw_text_ex(
            &game_context.font,
            health_str,
            pos + Vector2::new(
                self.get_data().width / 2.0 - size_of_health_str.x / 2.0,
                self.get_data().height + ENEMY_HEALTH_TEXT_Y_OFFSET_FROM_BOTTOM_OF_SPRITE,
            ),
            font_size,
            spacing,
            Color::WHITE,
        );
    }

    pub fn get_rect(&self) -> Rectangle {
        let data = self.get_data();
        let rect = Rectangle::new(data.pos.x, data.pos.y, data.width, data.height);
        return rect;
    }
}
