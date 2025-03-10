use crate::{ad, defs::SolItemId, sol::svc::vast::SolVastFitData, util::StSet};

#[derive(Copy, Clone)]
pub struct SolValItemKindFail {
    pub item_id: SolItemId,
    pub kind: Option<ad::AItemKind>,
    pub expected_kind: ad::AItemKind,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_item_kind_fast(&self, kfs: &StSet<SolItemId>) -> bool {
        match kfs.is_empty() {
            true => self.item_kind.is_empty(),
            false => self.item_kind.difference(kfs).nth(0).is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_item_kind_verbose(
        &self,
        kfs: &StSet<SolItemId>,
    ) -> Vec<SolValItemKindFail> {
        self.item_kind
            .values()
            .filter(|v| !kfs.contains(&v.item_id))
            .copied()
            .collect()
    }
}
