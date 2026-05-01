use basic_raylib_core::graphics::sprite_animation::SpriteAnimationInstance;
use raylib::{
    math::{Rectangle, Vector2},
    prelude::RaylibDrawHandle,
    text::Font,
    texture::Texture2D,
};

// ok heres the deal for anyone reading this code
// this could absolutely be done in a more explicit, cleaner, less error prone way
// however, i figured this was still a decent way to have dice boxes that can be
// either implemented exclusively for player, exclusively for enemy, or for both
// while still keeping their api in one spot and treating them as the same object
// i think it would get messy quick if i had different enums for enemy dice boxes
// and player dice boxes, when the same box implemented for either is actually the exact same,
// its just used slightly differently (updating to check for dice being picked up and
// dice boxes having different animations for player and enemy being the main differences)

use crate::{
    entities::{
        dice::Dice, dice_box_data::DiceBoxData, enemy::Enemy, enemy_dice_boxes::snake_eyes::SnakeEyes, hand::Hand,
        player::Player, player_dice_boxes::{broadsword_box::BroadSwordBox, heal_box::HealBox},
    },
    system::input_handler::InputState,
};

pub enum DiceBox {
    BroadSwordBox { broadsword_box: BroadSwordBox },
    HealBox { heal_box: HealBox },
    SnakeEyes { snake_eyes_box: SnakeEyes },
}

impl DiceBox {
    pub fn update_for_player(
        &mut self,
        is_player_dragging_dice: &mut bool,
        was_player_dragging_dice: bool,
        hand: &mut Hand,
        input_state: &InputState,
        dt: f32,
    ) {
        let data = self.get_mut_data();
        let hand_stopped = hand.all_dice_stopped_passive_check();

        data.pull_in_dragged_dice(&mut hand.dice);
        data.update_dice_for_player(is_player_dragging_dice, hand_stopped, input_state, dt);
        data.handle_dragging_dice(hand);

        if !*is_player_dragging_dice && was_player_dragging_dice {
            data.set_dice_positions();
        }

        data.info_hover.update(input_state, dt);
    }

    pub fn update_for_enemy(&mut self, input_state: &InputState, dt: f32) {
        let data = self.get_mut_data();

        data.update_dice_for_enemy(dt);
        data.info_hover.update(input_state, dt);
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
            Self::SnakeEyes { snake_eyes_box } => snake_eyes_box.tally_snake_eyes(),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        match self {
            Self::BroadSwordBox { broadsword_box } => broadsword_box.draw(d, texture, font),
            Self::HealBox { heal_box } => heal_box.draw(d, texture, font),
            Self::SnakeEyes { snake_eyes_box } => snake_eyes_box.draw(d, texture, font),
        }

        self.get_data().info_hover.draw(d, font, texture);
    }

    pub fn get_data(&self) -> &DiceBoxData {
        
        match self {
            Self::BroadSwordBox { broadsword_box: dice_box } => &dice_box.data,
            Self::HealBox { heal_box: dice_box } => &dice_box.data,
            Self::SnakeEyes { snake_eyes_box: dice_box } => &dice_box.data,
        }
    }

    pub fn get_mut_data(&mut self) -> &mut DiceBoxData {
        match self {
            Self::BroadSwordBox { broadsword_box: dice_box } => &mut dice_box.data,
            Self::HealBox { heal_box: dice_box } => &mut dice_box.data,
            Self::SnakeEyes { snake_eyes_box: dice_box } => &mut dice_box.data,
        }
    }

    // player and enemy action differentiated so i can only have to pass in player or enemy, not both
    // otherwise, if i wanted them in the same one, id make them take in an Option<&mut T> of both, which is just noisy
    pub fn player_action(&self, power: i64, enemy: &mut Enemy, player_health: &mut i64) {
        match self {
            Self::BroadSwordBox { .. } => Self::player_basic_attack(power, enemy),
            Self::HealBox { .. } => *player_health += power,
            Self::SnakeEyes { .. } => unimplemented!(),
        }
    }

    pub fn enemy_action(&self, power: i64, player: &mut Player) {
        match self {
            Self::SnakeEyes { .. } => Self::enemy_basic_attack(power, player),
            Self::HealBox { .. } => unimplemented!(),
            Self::BroadSwordBox { .. } => unimplemented!(),
        }
    }

    // free method to use for dice boxes whos only gimmick is higher power, and no special abilities
    // otherwise, dice box structs are free to implement their own actions and act accordingly
    pub fn player_basic_attack(power: i64, enemy: &mut Enemy) {
        enemy.take_hit(power);
    }

    pub fn enemy_basic_attack(power: i64, player: &mut Player) {
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
            Self::SnakeEyes { .. } => unimplemented!(),
        }
    }

    pub fn player_update_before_action_visuals(&self, anim: &mut SpriteAnimationInstance, dt: f32) -> bool {
        match self {
            Self::BroadSwordBox { .. } => BroadSwordBox::player_update_attack(anim, dt),
            Self::HealBox { .. } => HealBox::player_update_heal(anim, dt),
            Self::SnakeEyes { .. } => unimplemented!(),
        }
    }

    pub fn place(&mut self, pos: Vector2) {
        let data = self.get_mut_data();
        let dice_collect_rect_offset_x = data.dice_collect_rect.x - pos.x;
        let dice_collect_rect_offset_y = data.dice_collect_rect.y - pos.y;

        data.pos = pos;
        data.dice_collect_rect = Rectangle {
            x: pos.x + dice_collect_rect_offset_x,
            y: pos.y + dice_collect_rect_offset_y,
            width: data.dice_collect_rect.width,
            height: data.dice_collect_rect.height,
        };
        data.info_hover.activation_rect =
            Rectangle::new(pos.x, pos.y, data.info_hover.activation_rect.width, data.info_hover.activation_rect.height);
    }
}
