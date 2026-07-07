#[derive(PartialEq)]
pub enum AttackAffinity {
    None,
    Phys,
    Fire,
}

impl AttackAffinity {
    fn get_affinity_result(&self, weaknesses: &[AttackAffinity], resistances: &[AttackAffinity]) -> AttackAffinityResult {
        for w in weaknesses {
            if self == w {
                return AttackAffinityResult::Weak;
            }
        }

        for r in resistances {
            if self == r {
                return AttackAffinityResult::Resist;
            }
        }

        return AttackAffinityResult::None;
    }

    pub fn get_final_damage(&self, damage: i32, weaknesses: &[AttackAffinity], resistances: &[AttackAffinity]) -> i32 {
        let result = self.get_affinity_result(weaknesses, resistances);

        match result {
            AttackAffinityResult::None => damage,
            AttackAffinityResult::Weak => damage * 2,
            AttackAffinityResult::Resist => damage / 2,
        }
    }
}

pub enum AttackAffinityResult {
    None,
    Weak,
    Resist,
}
