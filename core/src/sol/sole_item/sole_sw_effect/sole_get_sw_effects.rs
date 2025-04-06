use crate::sol::{SolarSystem, info::SwEffectInfo};

impl SolarSystem {
    pub fn get_sw_effects(&self) -> Vec<SwEffectInfo> {
        self.uad
            .sw_effects
            .iter()
            .map(|v| SwEffectInfo::from(self.uad.items.get_by_id(v).unwrap().get_sw_effect().unwrap()))
            .collect()
    }
}
