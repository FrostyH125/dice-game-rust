use crate::{
    VIRTUAL_WIDTH,
    entities::{dice_box::DiceBox, enemies::snake::Snake, player::Player},
    system::input_handler::InputState,
};
use basic_raylib_core::system::sprite_particle_system::SpriteParticleSystem;
use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::{Font, RaylibFont},
    texture::Texture2D,
};

pub const ENEMY_HAND_X_CENTER_CORD: f32 = VIRTUAL_WIDTH - 100.0;
pub const ENEMY_HAND_Y_CORD: f32 = 195.0;
pub const ENEMY_HEALTH_TEXT_Y_OFFSET_FROM_BOTTOM_OF_SPRITE: f32 = 6.0;

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
    HitDelay,

    ///the delay before fully ending turn for seamless, sensible transitions
    EndTurnDelay,

    ///should be a simple check to see if player is waiting for enemy, and then
    ///if so, start turn
    WaitingForPlayer,
    
    Dead,
}

pub struct EnemyData {
    pub health: i64,
    pub pos: Vector2,
    pub state: EnemyState,

    // added these to position things properly depending on enemy
    pub width: f32,
    pub height: f32,
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

    pub fn take_hit(&mut self, damage: i64) {
        self.get_mut_data().health -= damage;
        self.get_mut_data().state = EnemyState::HitDelay;
    }

    pub fn update(
        &mut self,
        input_state: &InputState,
        player: &mut Player,
        particle_system: &mut SpriteParticleSystem,
        dt: f32,
    ) {
        match self {
            Self::Snake { snake } => snake.update(input_state, player, particle_system, dt),
        }
    }
    
    pub fn place_boxes(&mut self) {
        
        let margin = 5.0;
        let dice_box_height = 16.0;
        let self_width = self.get_data().width;
        let self_pos = self.get_data().pos;
        
        let mut boxes: Vec<&mut DiceBox> = match self {
            Self::Snake { snake } => vec![&mut snake.snake_eyes_box],
        };
        
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
            _ => unimplemented!("place_boxes(enemy) not implemented for {} boxes", num_of_boxes)
        }
        
        for dice_box in &mut boxes {
            dice_box.adjust_info_hover_pos_for_current_pos();
            dice_box.adjust_collect_rect_pos_for_current_pos();
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, texture: &Texture2D, font: &Font) {
        
        match self {
            Self::Snake { snake } => snake.draw(d, texture, font),
        }
        
        let pos = self.get_data().pos;
        let font_size = 10.0;
        let spacing = 0.5;
        let health_str = &format!("HP: {}", self.get_data().health);
        let size_of_health_str = font.measure_text(health_str, font_size, spacing);

        d.draw_text_ex(
            font,
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
}
