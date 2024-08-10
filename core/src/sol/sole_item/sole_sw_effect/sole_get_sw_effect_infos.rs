use crate::sol::{item_info::SolSwEffectInfo, SolarSystem};

impl SolarSystem {
    pub fn get_sw_effect_infos(&self) -> Vec<SolSwEffectInfo> {
        self.sw_effects
            .iter()
            .map(|v| SolSwEffectInfo::from(self.items.get_item(v).unwrap().get_sw_effect().unwrap()))
            .collect()
    }
}
