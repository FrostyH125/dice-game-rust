use raylib::math::Vector2;

use crate::game_effects::attack_affinity::AttackAffinityResult;

struct AffinityResultBattleEffect {
    str: &'static str,
    velocity: Vector2,
    acceleration: Vector2,
    lifetime: f32,
    pos: Vector2,
    result_kind: AttackAffinityResult,
}