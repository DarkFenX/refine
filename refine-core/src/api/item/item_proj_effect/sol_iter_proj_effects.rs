use crate::{EntityIter, ProjEffect, SolarSystem};

impl SolarSystem {
    pub fn iter_proj_effects(&mut self) -> EntityIter<'_, ProjEffect<'_>> {
        let proj_effect_uids = self.u_data.proj_effects.iter().copied().collect();
        EntityIter::new(self, proj_effect_uids)
    }
}
