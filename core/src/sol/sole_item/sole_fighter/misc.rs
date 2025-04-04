use crate::sol::{SolarSystem, info::FighterInfo, uad::item::Fighter};

impl SolarSystem {
    pub(in crate::sol) fn make_fighter_info(&self, fighter: &Fighter) -> FighterInfo {
        let mut autocharges = std::collections::HashMap::new();
        for (a_effect_id, autocharge_id) in fighter.get_autocharges().iter() {
            if let Ok(autocharge_info) = self.get_autocharge(autocharge_id) {
                autocharges.insert(a_effect_id.into(), autocharge_info);
            }
        }
        FighterInfo::from_fighter_and_autocharges(fighter, autocharges)
    }
}
