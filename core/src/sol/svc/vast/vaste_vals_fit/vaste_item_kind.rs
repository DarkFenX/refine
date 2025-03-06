use crate::{ad, defs::SolItemId, sol::svc::vast::SolVastFitData};

#[derive(Copy, Clone)]
pub struct SolValItemKindFail {
    pub item_id: SolItemId,
    pub kind: Option<ad::AItemKind>,
    pub expected_kind: ad::AItemKind,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_item_kind_fast(&self) -> bool {
        self.item_kind.is_empty()
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_item_kind_verbose(&self) -> Vec<SolValItemKindFail> {
        self.item_kind.values().copied().collect()
    }
}
