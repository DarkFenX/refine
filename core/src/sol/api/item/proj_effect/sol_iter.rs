use crate::sol::{SolarSystem, api::ProjEffect};

impl SolarSystem {
    pub fn iter_proj_effects(&self) -> impl ExactSizeIterator<Item = ProjEffect> {
        self.uad
            .proj_effects
            .iter()
            .map(|item_key| ProjEffect::new(self, *item_key))
    }
}
