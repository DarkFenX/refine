use crate::{
    sol::{ItemId, svc::vast::VastFitData},
    util::RSet,
};

pub struct ValNotLoadedItemFail {
    pub item_id: ItemId,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_not_loaded_item_fast(&self, kfs: &RSet<ItemId>) -> bool {
        match kfs.is_empty() {
            true => self.not_loaded.is_empty(),
            false => self.not_loaded.difference(kfs).nth(0).is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_not_loaded_item_verbose(
        &self,
        kfs: &RSet<ItemId>,
    ) -> Vec<ValNotLoadedItemFail> {
        self.not_loaded
            .iter()
            .filter(|v| !kfs.contains(v))
            .map(|v| ValNotLoadedItemFail { item_id: *v })
            .collect()
    }
}
