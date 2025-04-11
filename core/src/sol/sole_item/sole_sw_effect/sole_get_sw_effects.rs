use crate::sol::{SolarSystem, info::SwEffectInfo};

impl SolarSystem {
    pub fn get_sw_effects(&self) -> Vec<SwEffectInfo> {
        self.uad
            .sw_effects
            .iter()
            .map(|item_key| self.get_sw_effect_info_internal(*item_key).unwrap())
            .collect()
    }
}
