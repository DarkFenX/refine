use crate::{rd::REffectId, util::SmallRSet};

/// Running effects of an item.
#[derive(Clone)]
pub(crate) struct REffs {
    data: SmallRSet<REffectId, 4>,
}
impl REffs {
    pub(in crate::ud::item) fn new() -> Self {
        Self { data: SmallRSet::new() }
    }
    pub(crate) fn iter(&self) -> std::slice::Iter<'_, REffectId> {
        self.data.iter()
    }
    pub(crate) fn contains(&self, effect_rid: &REffectId) -> bool {
        self.data.contains(effect_rid)
    }
    pub(in crate::ud::item) fn len(&self) -> usize {
        self.data.len()
    }
    // Modification methods
    pub(in crate::ud::item) fn extend(&mut self, effect_rids: impl IntoIterator<Item = REffectId>) {
        self.data.extend(effect_rids)
    }
    pub(in crate::ud::item) fn remove(&mut self, effect_rid: &REffectId) -> bool {
        self.data.remove(effect_rid)
    }
    pub(in crate::ud::item) fn drain(&mut self) -> impl ExactSizeIterator<Item = REffectId> {
        self.data.drain()
    }
}
