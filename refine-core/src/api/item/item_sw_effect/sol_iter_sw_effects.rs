use crate::{EntityIter, Sol, SwEffect};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn iter_sw_effects(&mut self) -> EntityIter<'_, SwEffect<'_>> {
        let core = self.inner_mut();
        let sw_effect_uids = core.u_data.sw_effects.iter().copied().collect();
        EntityIter::new(core, sw_effect_uids)
    }
}
