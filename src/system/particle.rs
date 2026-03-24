use basic_raylib_core::graphics::sprite::Sprite;
use raylib::math::Vector2;

static DUMMY_SPRITE: Sprite = Sprite::new(0.0, 0.0, 0.0, 0.0);

pub struct Particle {
    pub sprite: &'static Sprite,
    pub position: Vector2,
    pub velocity: Vector2,
    pub acceleration: Vector2,
    pub lifetime: f32,
}

impl Particle {
    pub fn new_default() -> Self {
        Particle {
            sprite: &DUMMY_SPRITE,
            position: Default::default(),
            velocity: Default::default(),
            acceleration: Default::default(),
            lifetime: Default::default(),
        }
    }
}
