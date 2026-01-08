use crate::{
    api::{MutIter, ProjEffect, ProjEffectMut},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn iter_proj_effects(&self) -> impl ExactSizeIterator<Item = ProjEffect<'_>> {
        let proj_effect_uids = self.u_data.proj_effects.iter();
        proj_effect_uids.map(|proj_effect_uid| ProjEffect::new(self, *proj_effect_uid))
    }
    pub fn iter_proj_effects_mut(&mut self) -> MutIter<'_, ProjEffectMut<'_>> {
        let proj_effect_uids = self.u_data.proj_effects.iter().copied().collect();
        MutIter::new(self, proj_effect_uids)
    }
}
