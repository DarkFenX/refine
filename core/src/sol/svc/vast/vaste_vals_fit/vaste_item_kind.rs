use crate::{
    ad,
    sol::{ItemId, svc::vast::VastFitData},
    util::StSet,
};

#[derive(Copy, Clone)]
pub struct ValItemKindFail {
    pub item_id: ItemId,
    pub kind: Option<ad::AItemKind>,
    pub expected_kind: ad::AItemKind,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_item_kind_fast(&self, kfs: &StSet<ItemId>) -> bool {
        match kfs.is_empty() {
            true => self.item_kind.is_empty(),
            false => self.item_kind.difference(kfs).nth(0).is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_item_kind_verbose(&self, kfs: &StSet<ItemId>) -> Vec<ValItemKindFail> {
        self.item_kind
            .values()
            .filter(|v| !kfs.contains(&v.item_id))
            .copied()
            .collect()
    }
}
