use crate::sol::{item_info::SolSwEffectInfo, SolarSystem};

impl SolarSystem {
    pub fn get_sw_effects(&self) -> Vec<SolSwEffectInfo> {
        self.sw_effects
            .iter()
            .map(|v| SolSwEffectInfo::from(self.items.get_item(v).unwrap().get_sw_effect().unwrap()))
            .collect()
    }
}
