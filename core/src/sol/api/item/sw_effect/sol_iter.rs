use crate::sol::{SolarSystem, api::SwEffect};

impl SolarSystem {
    pub fn iter_sw_effects(&self) -> impl ExactSizeIterator<Item = SwEffect> {
        self.uad
            .sw_effects
            .iter()
            .map(|item_key| SwEffect::new(self, *item_key))
    }
}
