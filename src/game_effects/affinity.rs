#[derive(PartialEq)]
pub enum AttackAffinity {
    None,
    Phys,
    Fire,
}

impl AttackAffinity {
    fn get_affinity_result(&self, weaknesses: &[AttackAffinity], resistances: &[AttackAffinity]) -> AffinityResult {
        for w in weaknesses {
            if self == w {
                return AffinityResult::Weak;
            }
        }

        for r in resistances {
            if self == r {
                return AffinityResult::Resist;
            }
        }

        return AffinityResult::None;
    }

    pub fn get_final_damage(&self, damage: i32, weaknesses: &[AttackAffinity], resistances: &[AttackAffinity]) -> i32 {
        let result = self.get_affinity_result(weaknesses, resistances);

        match result {
            AffinityResult::None => damage,
            AffinityResult::Weak => damage * 2,
            AffinityResult::Resist => damage / 2,
        }
    }
}

pub enum AffinityResult {
    None,
    Weak,
    Resist,
}
