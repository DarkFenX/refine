use crate::sol::{info::SolSwEffectInfo, SolarSystem};

impl SolarSystem {
    pub fn get_sw_effects(&self) -> Vec<SolSwEffectInfo> {
        self.uad
            .sw_effects
            .iter()
            .map(|v| SolSwEffectInfo::from(self.uad.items.get_item(v).unwrap().get_sw_effect().unwrap()))
            .collect()
    }
}
