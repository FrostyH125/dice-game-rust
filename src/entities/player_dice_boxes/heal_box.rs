use basic_raylib_core::{
    graphics::{animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance},
    system::timer::Timer,
};
use rand::{RngExt, rngs::ThreadRng};
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::Font,
    texture::Texture2D,
};

use crate::{
    GameContext,
    entities::{
        dice_box_data::{
            DiceBoxData, STANDARD_BOX_COLLECT_RECT_HEIGHT, STANDARD_BOX_COLLECT_RECT_OFFSET_X,
            STANDARD_BOX_COLLECT_RECT_OFFSET_Y, STANDARD_BOX_COLLECT_RECT_WIDTH, STANDARD_BOX_HEIGHT,
            STANDARD_BOX_WIDTH,
        },
        player::{PLAYER_HEIGHT, PLAYER_WIDTH},
    },
    system::info_hover::InfoHover,
};

static HEAL_BOX_SPRITE: Sprite = Sprite::new(14, 144, 52, 16);
static PLAYER_HEAL_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(0, 320, 32, 48),
        Sprite::new(32, 320, 32, 48),
        Sprite::new(64, 320, 32, 48),
        Sprite::new(96, 320, 32, 48),
        Sprite::new(128, 320, 32, 48),
        Sprite::new(160, 320, 32, 48),
        Sprite::new(192, 320, 32, 48),
        Sprite::new(64, 320, 32, 48),
        Sprite::new(96, 320, 32, 48),
        Sprite::new(128, 320, 32, 48),
        Sprite::new(160, 320, 32, 48),
        Sprite::new(192, 320, 32, 48),
        Sprite::new(224, 320, 32, 48),
    ],
    frame_duration: 0.20,
    should_loop: false,
};

static HEAL_PARTICLE_SPRITE_SMALL: Sprite = Sprite::new(0, 144, 3, 3);
static HEAL_PARTICLE_SPRITE_LARGE: Sprite = Sprite::new(0, 148, 6, 6);

pub struct HealBox {
    pub data: DiceBoxData,
    particle_timer: Timer,
    rng: ThreadRng,
}

impl HealBox {
    pub fn new(font: &Font) -> Self {
        let data = DiceBoxData::new(
            STANDARD_BOX_COLLECT_RECT_OFFSET_X,
            STANDARD_BOX_COLLECT_RECT_OFFSET_Y,
            STANDARD_BOX_COLLECT_RECT_WIDTH,
            STANDARD_BOX_COLLECT_RECT_HEIGHT,
            STANDARD_BOX_WIDTH,
            STANDARD_BOX_HEIGHT,
            InfoHover::new(
                "Heal:\n a basic healing spell, it will heal you with a quarter of the value placed inside",
                Rectangle::new(0.0, 0.0, STANDARD_BOX_WIDTH, STANDARD_BOX_HEIGHT),
                font,
                5.0,
                0.5,
            ),
            Color::MEDIUMBLUE,
            0.25
        );
        
        return HealBox {
            data,
            particle_timer: Timer::new(0.05),
            rng: rand::rng(),
        };
    }

    pub fn draw_box_and_dice(&self, d: &mut RaylibDrawHandle, game_context: &GameContext) {

        const BASE_MULTI_TEXT_COLOR: Color = Color::new(146, 215, 200, 255);
        
        HEAL_BOX_SPRITE.draw(d, self.data.pos, &game_context.texture);
        d.draw_rectangle_lines(
            self.data.dice_collect_rect.x as i32,
            self.data.dice_collect_rect.y as i32,
            self.data.dice_collect_rect.width as i32,
            self.data.dice_collect_rect.height as i32,
            Color::WHITE,
        );
        self.data.draw_dice(d, &game_context.texture);
        self.data.draw_border_around_current_dice(d, &game_context.texture);
        self.data.draw_base_multi(d, &game_context.font, BASE_MULTI_TEXT_COLOR);
    }

    pub fn player_draw_heal(
        d: &mut RaylibDrawHandle,
        anim: &SpriteAnimationInstance,
        pos: Vector2,
        texture: &Texture2D,
    ) {
        PLAYER_HEAL_ANIM.draw(anim, d, pos, texture);
    }

    pub fn player_update_heal(
        &mut self,
        anim: &mut SpriteAnimationInstance,
        game_context: &mut GameContext,
        player_pos: Vector2,
        dt: f32,
    ) -> bool {
        // handle particles
        self.particle_timer.track(dt);
        if self.particle_timer.is_done() {
            self.particle_timer.reset();
            HealBox::emit_random_heal_particle(game_context, &mut self.rng, player_pos);
        }

        // handle anim
        PLAYER_HEAL_ANIM.update(anim, dt);

        return anim.finished_playing;
    }

    fn emit_random_heal_particle(game_context: &mut GameContext, rng: &mut ThreadRng, player_pos: Vector2) {
        let is_big = rng.random::<bool>();
        let position_x: f32 = rng.random_range(player_pos.x..=player_pos.x + PLAYER_WIDTH);
        let position_y: f32 = player_pos.y + PLAYER_HEIGHT;
        let left_start = rng.random::<bool>();
        let velocity_x = if left_start {
            -20.0
        } else {
            20.0
        };
        
        let velocity_y = rng.random_range(-40.0..=-30.0);
        let velocity: Vector2 = Vector2::new(velocity_x, velocity_y);
        let acceleration_x = if left_start {
            20.0
        } else {
            -20.0
        };
        
        let acceleration: Vector2 = Vector2::new(acceleration_x, 0.0);
        let lifetime: f32 = rng.random_range(1.5..=2.0);
        let sprite = if is_big { &HEAL_PARTICLE_SPRITE_LARGE } else { &HEAL_PARTICLE_SPRITE_SMALL };

        game_context.sprite_particle_system.emit(
            sprite,
            Vector2::new(position_x, position_y),
            velocity,
            acceleration,
            lifetime,
        );
    }
}
