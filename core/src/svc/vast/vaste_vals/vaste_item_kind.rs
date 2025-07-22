use std::collections::HashMap;

use crate::{
    ad,
    def::ItemId,
    svc::{SvcCtx, vast::VastFitData},
    uad::UadItemKey,
    util::RSet,
};

pub struct ValItemKindFail {
    /// Map between item IDs and info about failed validation.
    pub item_kinds: HashMap<ItemId, ValItemKindItemInfo>,
}

#[derive(Copy, Clone)]
pub struct ValItemKindItemInfo {
    /// Detected item kind.
    pub kind: Option<ad::AItemKind>,
    /// Expected item kind for position it was put in.
    pub expected_kind: ad::AItemKind,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_item_kind_fast(&self, kfs: &RSet<UadItemKey>) -> bool {
        match kfs.is_empty() {
            true => self.item_kind.is_empty(),
            false => self.item_kind.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_item_kind_verbose(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValItemKindFail> {
        let item_kinds: HashMap<_, _> = self
            .item_kind
            .iter()
            .filter(|(item_key, _)| !kfs.contains(item_key))
            .map(|(item_key, item_info)| (ctx.uad.items.id_by_key(*item_key), *item_info))
            .collect();
        match item_kinds.is_empty() {
            true => None,
            false => Some(ValItemKindFail { item_kinds }),
        }
    }
}
