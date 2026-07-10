use crate::{
    api::{MutIter, SwEffect, SwEffectMut},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn iter_sw_effects(&self) -> impl ExactSizeIterator<Item = SwEffect<'_>> {
        let sw_effect_uids = self.u_data.sw_effects.iter();
        sw_effect_uids.map(|sw_effect_uid| SwEffect::new(self, *sw_effect_uid))
    }
    pub fn iter_sw_effects_mut(&mut self) -> MutIter<'_, SwEffectMut<'_>> {
        let sw_effect_uids = self.u_data.sw_effects.iter().copied().collect();
        MutIter::new(self, sw_effect_uids)
    }
}
