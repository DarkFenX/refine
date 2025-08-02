use crate::sol::{
    SolarSystem,
    api::{MutIter, ProjEffect, ProjEffectMut},
};

impl SolarSystem {
    pub fn iter_proj_effects(&self) -> impl ExactSizeIterator<Item = ProjEffect<'_>> {
        let proj_effect_keys = self.u_data.proj_effects.iter();
        proj_effect_keys.map(|proj_effect_key| ProjEffect::new(self, *proj_effect_key))
    }
    pub fn iter_proj_effects_mut(&mut self) -> MutIter<'_, ProjEffectMut<'_>> {
        let proj_effect_keys = self.u_data.proj_effects.iter().copied().collect();
        MutIter::new(self, proj_effect_keys)
    }
}
