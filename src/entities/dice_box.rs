// ok heres the deal for anyone reading this code
// this could absolutely be done in a more explicit, cleaner, less error prone way
// however, i figured this was still a decent way to have dice boxes that can be
// either implemented exclusively for player, exclusively for enemy, or for both
// while still keeping their api in one spot and treating them as the same object
// i think it would get messy quick if i had different enums for enemy dice boxes
// and player dice boxes, when the same box implemented for either is actually the exact same,
// its just used slightly differently (updating to check for dice being picked up and
// dice boxes having different animations for player and enemy being the main differences)


use basic_raylib_core::{graphics::sprite_animation::SpriteAnimationInstance, system::input_handler::InputState};
use raylib::{
    math::{Rectangle, Vector2}, prelude::RaylibDrawHandle, text::Font, texture::Texture2D
};


use crate::{
    GameContext, entities::{
        dice::Dice, dice_box_data::DiceBoxData, enemy_dice_boxes::snake_eyes::SnakeEyes, hand::Hand, player::Player, player_dice_boxes::{broadsword_box::BroadSwordBox, heal_box::HealBox, shield_box::ShieldBox}
    }, game_effects::{battle_effect::BattleEffectType, number_battle_effect::{NumberEffect, NumberEffectType}}
};

pub enum DiceBox {
    BroadSwordBox { broadsword_box: BroadSwordBox },
    HealBox { heal_box: HealBox },
    ShieldBox { shield_box: ShieldBox},
    SnakeEyes { snake_eyes_box: SnakeEyes },
}

pub enum DiceBoxResult {
    BasicAttack(i32),
    BasicHeal(i32),
    ChargeShield(i32),
    None,
}

impl DiceBoxResult {
    pub fn get_num_effect_type(&self) -> Option<NumberEffectType> {

        let num_effect_type = match self {
            DiceBoxResult::BasicAttack(_) => NumberEffectType::Damage,
            DiceBoxResult::BasicHeal(_) => NumberEffectType::Heal,
            DiceBoxResult::ChargeShield(_) => return None,
            DiceBoxResult::None => return None,
        };

        return Some(num_effect_type);
    }
}

impl DiceBox {

    /// handles dice dragging, updating dice, and returning dice to hand
    /// also sets dice positions and updates the info hover
    pub fn update_for_player(
        &mut self,
        is_player_dragging_dice: bool,
        hand: &mut Hand,
        input_state: &InputState,
        dt: f32,
    ) {
        let data = self.get_mut_data();
        let hand_stopped = hand.all_dice_stopped_passive_check();

        data.pull_in_dragged_dice(hand);
        data.update_dice_for_player(is_player_dragging_dice, hand_stopped, input_state, dt);
        data.check_if_any_dice_need_to_go_back_to_hand(hand);

        data.info_hover.update(input_state, dt);
    }

    pub fn update_for_enemy(&mut self, input_state: &InputState, dt: f32) {
        let data = self.get_mut_data();

        data.update_dice_for_enemy(dt);
        data.info_hover.update(input_state, dt);
    }
    
    // since player dice boxes are going to be fairly standard in size and shape, and enemy dice boxes dont depend on input 
    // im going to keep the setting dice positions right here for them, in a convenient location
    // please note that i should have put this method specifically in dice_box_data.rs right next to its buddy, dice_box_data.set_dice_positions
    // for normal sized enemy boxes, ill simply use that method in this match statement, like if i had a cleave box, or something, it would look like
    // DiceBox::CleaveBox { cleave_box } => cleave_box.data.set_dice_positions();
    pub fn enemy_set_dice_positions(&mut self) {
        match self {
            DiceBox::BroadSwordBox { .. } => unimplemented!(),
            DiceBox::HealBox { .. } => unimplemented!(),
            DiceBox::ShieldBox { .. } => unimplemented!(),
            DiceBox::SnakeEyes { snake_eyes_box } => snake_eyes_box.snake_eyes_set_dice_positions(),
        }
    }

    pub fn adjust_collect_rect_pos_for_current_pos(&mut self) {
        let data = self.get_mut_data();
        data.dice_collect_rect.x = data.pos.x + data.collect_rect_offset_x;
        data.dice_collect_rect.y = data.pos.y + data.collect_rect_offset_y;
    }

    pub fn adjust_info_hover_pos_for_current_pos(&mut self) {
        let data = self.get_mut_data();
        data.info_hover.activation_rect.x = data.pos.x;
        data.info_hover.activation_rect.y = data.pos.y;
    }

    pub fn tally(&mut self, dt: f32) -> bool {
        match self {
            Self::BroadSwordBox { broadsword_box } => broadsword_box.data.tally_points(dt),
            Self::HealBox { heal_box } => heal_box.data.tally_points(dt),
            Self::ShieldBox { shield_box } => shield_box.data.tally_points(dt),
            Self::SnakeEyes { snake_eyes_box } => snake_eyes_box.check_if_two_ones(),
        }
    }

    pub fn get_result(&self) -> DiceBoxResult {
        match self {
            DiceBox::BroadSwordBox { broadsword_box } => DiceBoxResult::BasicAttack(broadsword_box.data.get_value()),
            DiceBox::HealBox { heal_box } => DiceBoxResult::BasicHeal(heal_box.data.get_value()),
            DiceBox::ShieldBox { shield_box } => DiceBoxResult::ChargeShield(shield_box.data.get_value()),
            DiceBox::SnakeEyes { snake_eyes_box } => {
                if snake_eyes_box.data.dice_in_box.len() == 2 {
                    DiceBoxResult::BasicAttack(11)
                } else {
                    // tbh because snake is pretty deterministic this is pretty unlikely, but if i ever decide to give player access
                    // to snake eyes, then i need this to be a thing
                    DiceBoxResult::None
                }
            }
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, game_context: &GameContext) {
        match self {
            Self::BroadSwordBox { broadsword_box } => broadsword_box.draw_box_and_dice(d, game_context),
            Self::HealBox { heal_box } => heal_box.draw_box_and_dice(d, game_context),
            Self::ShieldBox { shield_box } => shield_box.draw_box_and_dice(d, game_context),
            Self::SnakeEyes { snake_eyes_box } => snake_eyes_box.draw(d, game_context),
        }

        self.get_data().info_hover.draw(d, game_context);
    }

    pub fn get_data(&self) -> &DiceBoxData {        
        match self {
            Self::BroadSwordBox { broadsword_box: dice_box } => &dice_box.data,
            Self::HealBox { heal_box: dice_box } => &dice_box.data,
            Self::ShieldBox { shield_box: dice_box } => &dice_box.data,
            Self::SnakeEyes { snake_eyes_box: dice_box } => &dice_box.data,
        }
    }

    pub fn get_mut_data(&mut self) -> &mut DiceBoxData {
        match self {
            Self::BroadSwordBox { broadsword_box: dice_box } => &mut dice_box.data,
            Self::HealBox { heal_box: dice_box } => &mut dice_box.data,
            Self::ShieldBox { shield_box: dice_box } => &mut dice_box.data,
            Self::SnakeEyes { snake_eyes_box: dice_box } => &mut dice_box.data,
        }
    }

    pub fn enemy_action(&self, result: DiceBoxResult, player: &mut Player, enemy_health: &mut i32, enemy_shield_power: &mut i32) {   
        match result {
            DiceBoxResult::BasicAttack(damage) => Self::enemy_basic_attack(damage, player),
            DiceBoxResult::BasicHeal(heal_amount) => *enemy_health += heal_amount,
            DiceBoxResult::ChargeShield(charge_amount) => *enemy_shield_power += charge_amount,
            DiceBoxResult::None => (),
        }
    }

    pub fn enemy_basic_attack(power: i32, player: &mut Player) {
        player.take_hit(power);
    }

    pub fn reset(&mut self, dice_in_hand: &mut Vec<Dice>, dice_origin_pos: Vector2) {
        self.get_mut_data().reset_box(dice_in_hand, dice_origin_pos);
    }

    pub fn player_draw_action(
        &self,
        anim: &mut SpriteAnimationInstance,
        d: &mut RaylibDrawHandle,
        pos: Vector2,
        texture: &Texture2D,
    ) {
        match self {
            Self::BroadSwordBox { .. } => BroadSwordBox::player_draw_attack(d, anim, pos, texture),
            Self::HealBox { .. } => HealBox::player_draw_heal(d, anim, pos, texture),
            Self::ShieldBox { .. } => ShieldBox::player_draw_put_shield_up(d, anim, pos, texture),
            Self::SnakeEyes { .. } => unimplemented!(),
        }
    }

    pub fn player_update_before_action_visuals(&mut self, anim: &mut SpriteAnimationInstance, game_context: &mut GameContext, player_pos: Vector2, dt: f32) -> bool {
        match self {
            Self::BroadSwordBox { .. } => BroadSwordBox::player_update_attack(anim, dt),
            Self::HealBox { heal_box } => heal_box.player_update_heal(anim, game_context, player_pos, dt),
            Self::ShieldBox { .. } => ShieldBox::player_update_put_shield_up(anim, dt),
            Self::SnakeEyes { .. } => unimplemented!(),
        }
    }

    /// this function returns any battle effect type that should happen with the battle
    /// action visual, not after it. there will be another function which will return
    /// battle results that should happen after the battle action visual, hence, 
    /// get_battle_effect_type_post_action_result(&self) (this will be used moreso
    /// for things like spells where the explosion or whatever shouldnt happen until after
    /// the action visual finishes, (which would involve some projectile hitting the enemy/player))
    pub fn get_battle_effect_type_pre_action_result(&self) -> Option<BattleEffectType> {
        match self {
            DiceBox::BroadSwordBox { .. } => Some(BattleEffectType::Slash),
            DiceBox::HealBox { .. } => None,
            DiceBox::ShieldBox { .. } => None,
            DiceBox::SnakeEyes { .. } => todo!("add bite effect"), 
        }
    }
}
