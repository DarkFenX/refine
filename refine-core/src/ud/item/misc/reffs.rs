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
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = REffectId> {
        self.data.iter().copied()
    }
    pub(crate) fn contains(&self, effect_rid: REffectId) -> bool {
        self.data.contains(&effect_rid)
    }
    pub(in crate::ud::item) fn len(&self) -> usize {
        self.data.len()
    }
    // Modification methods
    pub(in crate::ud::item) fn extend(&mut self, effect_rids: impl IntoIterator<Item = REffectId>) {
        self.data.extend(effect_rids)
    }
    // Takes distinct running effects
    pub(in crate::ud::item) fn remove_running(&mut self, effect_rids: &[REffectId]) {
        // All of them, e.g. when an item goes to ghost state
        if effect_rids.len() == self.data.len() {
            self.data.clear();
            return;
        }
        for effect_rid in effect_rids {
            self.data.remove(effect_rid);
        }
    }
}
