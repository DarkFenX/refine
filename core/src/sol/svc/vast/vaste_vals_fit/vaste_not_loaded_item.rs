use crate::{defs::SolItemId, sol::svc::vast::SolVastFitData};

#[derive(Clone)]
pub struct SolValNotLoadedItemFail {
    pub item_id: SolItemId,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_not_loaded_item_fast(&self) -> bool {
        self.not_loaded.is_empty()
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_not_loaded_item_verbose(&self) -> Vec<SolValNotLoadedItemFail> {
        self.not_loaded
            .iter()
            .map(|v| SolValNotLoadedItemFail { item_id: *v })
            .collect()
    }
}
