use raylib::{prelude::RaylibDrawHandle, text::Font, texture::Texture2D};

use crate::{
    entities::{
        dice::Dice, dice_box_data::DiceBoxData, enemy::Enemy, enemy_dice_boxes::snake_eyes::SnakeEyes, hand::Hand, player::Player, player_dice_boxes::broadsword_box::BroadSwordBox
    },
    system::input_handler::InputState,
};

pub enum DiceBox {
    BroadSwordBox { broadsword_box: BroadSwordBox },
    SnakeEyes { snake_eyes_box: SnakeEyes },
}

impl DiceBox {
    // really, the only updating a box needs is updating dice and updating the info hover
    // this is the reason i just kind merged them into one thing
    // every enemy and the player should be calling this every frame.
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
            hand.arrange_hand(false);
            data.set_dice_positions();
        }

        data.info_hover.update(input_state, dt);
    }

    pub fn update_for_enemy(&mut self, input_state: &InputState, dt: f32) {
        let data = self.get_mut_data();

        data.update_dice_for_enemy(dt);
        data.info_hover.update(input_state, dt);
    }

    pub fn tally(&mut self, dt: f32) -> bool {
        match self {
            DiceBox::BroadSwordBox { broadsword_box } => broadsword_box.data.tally_points(dt),
            DiceBox::SnakeEyes { snake_eyes_box } => snake_eyes_box.tally_snake_eyes(),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        match self {
            DiceBox::BroadSwordBox { broadsword_box } => broadsword_box.draw(d, texture, font),
            DiceBox::SnakeEyes { snake_eyes_box } => snake_eyes_box.draw(d, texture, font),
        }

        self.get_data().info_hover.draw(d, font, texture);
    }

    pub fn get_data(&self) -> &DiceBoxData {
        match self {
            Self::BroadSwordBox { broadsword_box: dice_box } => &dice_box.data,
            Self::SnakeEyes { snake_eyes_box: dice_box } => &dice_box.data,
        }
    }

    pub fn get_mut_data(&mut self) -> &mut DiceBoxData {
        match self {
            Self::BroadSwordBox { broadsword_box: dice_box } => &mut dice_box.data,
            Self::SnakeEyes { snake_eyes_box: dice_box } => &mut dice_box.data,
        }
    }
    
    // player and enemy action differentiated so i can only have to pass in player or enemy, not both
    // otherwise, if i wanted them in the same one, id make them take in an Option<&mut T> of both, which is just noisy
    pub fn player_action(&self, power: i64, enemy: &mut Enemy) {
        match self {
            Self::BroadSwordBox { .. } => Self::player_basic_attack(power, enemy),
            _ => panic!(
                "'player_action()' not implemented for this dice box. Perhaps its an enemy box you're trying to use it with?"
            ),
        }
    }
    
    pub fn enemy_action(&self, power: i64, player: &mut Player) {
        match self {
            Self::SnakeEyes { .. } => Self::enemy_basic_attack(power, player),
            _ => panic!(
                "'enemy_action()' not implemented for this dice box. Perhaps its a player box you're trying to use it with?"
            ),
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

    pub fn reset(&mut self, dice_in_hand: &mut Vec<Dice>) {
        self.get_mut_data().reset_box(dice_in_hand);
    }
}
