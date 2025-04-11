use crate::sol::{SolarSystem, info::ProjEffectInfo};

impl SolarSystem {
    pub fn get_proj_effect_infos(&self) -> Vec<ProjEffectInfo> {
        self.uad
            .proj_effects
            .iter()
            .map(|item_key| self.get_proj_effect_info_internal(*item_key).unwrap())
            .collect()
    }
}
