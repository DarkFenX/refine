use crate::{EntityIter, SolarSystem, SwEffect};

impl SolarSystem {
    pub fn iter_sw_effects(&mut self) -> EntityIter<'_, SwEffect<'_>> {
        let sw_effect_uids = self.u_data.sw_effects.iter().copied().collect();
        EntityIter::new(self, sw_effect_uids)
    }
}
