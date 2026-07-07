use basic_raylib_core::graphics::{
    animation_data::AnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance,
};
use raylib::{drawing::RaylibDrawHandle, math::{Rectangle, Vector2}, texture::Texture2D};

static SLASH_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(400, 0, 32, 16),
        Sprite::new(432, 0, 32, 16),
        Sprite::new(464, 0, 32, 16),
        Sprite::new(496, 0, 32, 16),
    ],
    frame_duration: 0.15,
    should_loop: false,
};

static SNAKE_BITE_ANIM: AnimationData = AnimationData {
    frames: &[
        Sprite::new(272, 224, 16, 16),
        Sprite::new(272, 240, 16, 16),
        Sprite::new(272, 256, 16, 16),
        Sprite::new(288, 224, 16, 16),
        Sprite::new(288, 240, 16, 16),
        Sprite::new(288, 256, 16, 16),
    ],
    frame_duration: 0.20,
    should_loop: false
};

#[derive(Clone, Copy)]
pub enum AttackVisualEffectType {
    None,
    Slash,
    SnakeBite
}

impl AttackVisualEffectType {
    #[inline]
    pub fn get_anim(&self) -> &AnimationData {
        match self {
            Self::Slash => &SLASH_ANIM,
            Self::SnakeBite => &SNAKE_BITE_ANIM,
            Self::None => panic!("should never ask for animation on AttackVisualEffect::None")
        }
    }
}

///battle effects are used for effects like slashes or fire bursts that should
///be visually run simultaneous to the visual of the action it is related to
///examples being a slash over the enemy being attacked, or fire being placed over
///something just hit with a fireball
pub struct AttackVisualEffect {
    effect_type: AttackVisualEffectType,
    pos: Vector2,
    anim_instance: SpriteAnimationInstance,
}

impl AttackVisualEffect {
    /// target pos rect is used as the rectangle of the target in order to
    /// properly center the effect without having to do the math
    /// every single time you wanna make a new effect
    pub fn new(effect_type: AttackVisualEffectType, target_pos_rect: Rectangle) -> Self {

        if let AttackVisualEffectType::None = effect_type {
            panic!("something is calling AttackVisualEffect::new() on AttackVisualEffectType::None")
        }
        
        let anim = effect_type.get_anim();
        let anim_width = anim.frames[0].src_rect.width;
        let anim_height = anim.frames[0].src_rect.height;

        let center_pos_of_target = Vector2::new(
            target_pos_rect.x + target_pos_rect.width / 2.0,
            target_pos_rect.y + target_pos_rect.height / 2.0,
        );

        let pos_of_animation =
            Vector2::new(center_pos_of_target.x - anim_width / 2.0, center_pos_of_target.y - anim_height / 2.0);

        return AttackVisualEffect {
            effect_type,
            pos: pos_of_animation,
            anim_instance: SpriteAnimationInstance::new(),
        };
    }

    pub fn update(&mut self, dt: f32) {
        let anim = self.effect_type.get_anim();

        anim.update(&mut self.anim_instance, dt);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        let anim = self.effect_type.get_anim();
        anim.draw(&self.anim_instance, d, self.pos, texture);
    }

    #[inline]
    pub fn is_done(&self) -> bool {
        return self.anim_instance.finished_playing;
    }
}
