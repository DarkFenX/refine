use crate::{
    sol::{item::SolFighter, item_info::SolFighterInfo, SolarSystem},
    util::StMap,
};

impl SolarSystem {
    pub(in crate::sol) fn make_fighter_info(&self, fighter: &SolFighter) -> SolFighterInfo {
        let mut autocharges = StMap::new();
        for (effect_id, autocharge_item_id) in fighter.autocharges.iter() {
            if let Ok(autocharge_info) = self.get_autocharge(&autocharge_item_id) {
                autocharges.insert(*effect_id, autocharge_info);
            }
        }
        SolFighterInfo::from_fighter_and_autocharges(fighter, autocharges)
    }
}
