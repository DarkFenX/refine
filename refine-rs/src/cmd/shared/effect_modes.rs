use rc::ItemMutCommon;

use crate::{EffectId, EffectMode};

#[derive(Default)]
pub(in crate::cmd) struct EffectModes {
    data: Vec<(EffectId, EffectMode)>,
}
impl EffectModes {
    pub(in crate::cmd) const fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub(in crate::cmd) fn clear(&mut self) {
        self.data.clear();
    }
    pub(in crate::cmd) fn extend(&mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) {
        self.data.extend(effect_modes);
    }
    pub(in crate::cmd) fn apply(&self, core_item: &mut impl ItemMutCommon) {
        if !self.data.is_empty() {
            core_item.set_effect_modes(self.data.iter().map(|(k, v)| (*k, *v)));
        }
    }
}
