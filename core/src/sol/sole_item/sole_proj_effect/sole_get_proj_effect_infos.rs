use crate::sol::{item_info::SolProjEffectInfo, SolarSystem};

impl SolarSystem {
    pub fn get_proj_effect_infos(&self) -> Vec<SolProjEffectInfo> {
        self.proj_effects
            .iter()
            .map(|v| SolProjEffectInfo::from(self.items.get_item(v).unwrap().get_proj_effect().unwrap()))
            .collect()
    }
}
