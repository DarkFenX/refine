use crate::{defs::SolItemId, sol::svc::vast::SolVastFitData, util::StSet};

#[derive(Clone)]
pub struct SolValNotLoadedItemFail {
    pub item_id: SolItemId,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_not_loaded_item_fast(&self, kfs: &StSet<SolItemId>) -> bool {
        match kfs.is_empty() {
            true => self.not_loaded.is_empty(),
            false => self.not_loaded.difference(kfs).nth(0).is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_not_loaded_item_verbose(
        &self,
        kfs: &StSet<SolItemId>,
    ) -> Vec<SolValNotLoadedItemFail> {
        self.not_loaded
            .iter()
            .filter(|v| !kfs.contains(v))
            .map(|v| SolValNotLoadedItemFail { item_id: *v })
            .collect()
    }
}
