use crate::{EntityIter, ProjEffect, Sol};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn iter_proj_effects(&mut self) -> EntityIter<'_, ProjEffect<'_>> {
        let core = self.inner_mut();
        let proj_effect_uids = core.u_data.proj_effects.iter().copied().collect();
        EntityIter::new(core, proj_effect_uids)
    }
}
