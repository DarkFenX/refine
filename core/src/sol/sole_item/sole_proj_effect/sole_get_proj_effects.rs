use crate::sol::{SolarSystem, info::ProjEffectInfo};

impl SolarSystem {
    pub fn get_proj_effects(&self) -> Vec<ProjEffectInfo> {
        self.uad
            .proj_effects
            .iter()
            .map(|item_key| self.get_proj_effect_internal(*item_key).unwrap())
            .collect()
    }
}
