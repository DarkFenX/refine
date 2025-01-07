use crate::sol::{info::SolProjEffectInfo, SolarSystem};

impl SolarSystem {
    pub fn get_proj_effects(&self) -> Vec<SolProjEffectInfo> {
        self.uad
            .proj_effects
            .iter()
            .map(|v| SolProjEffectInfo::from(self.uad.items.get_item(v).unwrap().get_proj_effect().unwrap()))
            .collect()
    }
}
