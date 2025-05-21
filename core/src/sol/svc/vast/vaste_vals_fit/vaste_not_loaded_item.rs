use crate::{
    sol::{ItemId, ItemKey, svc::vast::VastFitData, uad::Uad},
    util::RSet,
};

pub struct ValNotLoadedItemFail {
    /// Item IDs of items which couldn't be loaded from current sol data source.
    pub item_ids: Vec<ItemId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_not_loaded_item_fast(&self, kfs: &RSet<ItemKey>) -> bool {
        match kfs.is_empty() {
            true => self.not_loaded.is_empty(),
            false => self.not_loaded.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_not_loaded_item_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
    ) -> Option<ValNotLoadedItemFail> {
        let item_ids: Vec<_> = self
            .not_loaded
            .iter()
            .filter(|item_key| !kfs.contains(item_key))
            .map(|item_key| uad.items.id_by_key(*item_key))
            .collect();
        match item_ids.is_empty() {
            true => None,
            false => Some(ValNotLoadedItemFail { item_ids }),
        }
    }
}
