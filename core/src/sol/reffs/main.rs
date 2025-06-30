use crate::{ad, def::ItemKey, util::RMapRSet};

/// Running effects by item.
#[derive(Clone)]
pub(crate) struct REffs {
    pub(super) data: RMapRSet<ItemKey, ad::AEffectId>,
}
impl REffs {
    pub(crate) fn new() -> Self {
        Self { data: RMapRSet::new() }
    }
    // Query methods
    pub(crate) fn is_running(&self, item_key: &ItemKey, a_effect_id: &ad::AEffectId) -> bool {
        self.data.contains_entry(item_key, a_effect_id)
    }
    pub(crate) fn iter_running(&self, item_key: &ItemKey) -> impl ExactSizeIterator<Item = &ad::AEffectId> + use<'_> {
        self.data.get(item_key)
    }
    // Modification methods
    pub(crate) fn effects_started(
        &mut self,
        item_key: ItemKey,
        a_effect_ids: impl ExactSizeIterator<Item = ad::AEffectId>,
    ) {
        self.data.extend_entries(item_key, a_effect_ids);
    }
    pub(crate) fn effects_stopped<'a>(
        &mut self,
        item_key: &ItemKey,
        a_effect_ids: impl Iterator<Item = &'a ad::AEffectId>,
    ) {
        self.data.drain_entries(item_key, a_effect_ids);
    }
    pub(crate) fn extract_running(&mut self, item_key: &ItemKey) -> Option<impl Iterator<Item = ad::AEffectId>> {
        self.data.remove_key(item_key)
    }
}
