use crate::sol::{
    SolarSystem,
    api::{MutIter, ProjEffect, mut_iter::ProjEffectMutGenerator},
};

impl SolarSystem {
    pub fn iter_proj_effects(&self) -> impl ExactSizeIterator<Item = ProjEffect> {
        self.uad
            .proj_effects
            .iter()
            .map(|item_key| ProjEffect::new(self, *item_key))
    }
    pub fn iter_proj_effects_mut(&mut self) -> MutIter<'_, ProjEffectMutGenerator> {
        let proj_effect_keys = self.uad.proj_effects.iter().copied().collect();
        MutIter::new(self, proj_effect_keys)
    }
}
