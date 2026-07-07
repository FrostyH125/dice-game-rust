use raylib::math::Vector2;

use crate::game_effects::affinity::AffinityResult;

struct AffinityResultBattleEffect {
    str: &'static str,
    velocity: Vector2,
    Acceleration: Vector2,
    lifetime: f32,
    pos: Vector2,
    result_kind: AffinityResult,
}