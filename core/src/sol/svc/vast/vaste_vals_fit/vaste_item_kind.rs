use crate::{
    ad,
    sol::{ItemId, svc::vast::VastFitData},
    util::RSet,
};

#[derive(Copy, Clone)]
pub struct ValItemKindFail {
    pub item_id: ItemId,
    pub kind: Option<ad::AItemKind>,
    pub expected_kind: ad::AItemKind,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_item_kind_fast(&self, kfs: &RSet<ItemId>) -> bool {
        match kfs.is_empty() {
            true => self.item_kind.is_empty(),
            false => self.item_kind.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_item_kind_verbose(&self, kfs: &RSet<ItemId>) -> Vec<ValItemKindFail> {
        self.item_kind
            .values()
            .filter(|v| !kfs.contains(&v.item_id))
            .copied()
            .collect()
    }
}
