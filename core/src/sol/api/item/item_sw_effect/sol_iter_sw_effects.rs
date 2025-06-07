use crate::sol::{
    SolarSystem,
    api::{MutIter, SwEffect, SwEffectMut},
};

impl SolarSystem {
    pub fn iter_sw_effects(&self) -> impl ExactSizeIterator<Item = SwEffect<'_>> {
        self.uad
            .sw_effects
            .iter()
            .map(|item_key| SwEffect::new(self, *item_key))
    }
    pub fn iter_sw_effects_mut(&mut self) -> MutIter<'_, SwEffectMut<'_>> {
        let sw_effect_keys = self.uad.sw_effects.iter().copied().collect();
        MutIter::new(self, sw_effect_keys)
    }
}
