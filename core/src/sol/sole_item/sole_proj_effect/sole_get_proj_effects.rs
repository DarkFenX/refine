use crate::sol::{SolarSystem, info::ProjEffectInfo};

impl SolarSystem {
    pub fn get_proj_effects(&self) -> Vec<ProjEffectInfo> {
        self.uad
            .proj_effects
            .iter()
            .map(|v| ProjEffectInfo::from(self.uad.items.get_by_id(v).unwrap().get_proj_effect().unwrap()))
            .collect()
    }
}
