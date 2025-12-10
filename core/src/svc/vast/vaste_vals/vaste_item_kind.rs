use std::collections::HashMap;

use crate::{
    def::ItemId,
    misc::ItemKind,
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemKey,
    util::RSet,
};

pub struct ValItemKindFail {
    /// Map between item IDs and info about failed validation.
    pub item_kinds: HashMap<ItemId, ValItemKindItemInfo>,
}

#[derive(Copy, Clone)]
pub struct ValItemKindItemInfo {
    /// Detected item kind.
    pub kind: Option<ItemKind>,
    /// Expected item kind for position it was put in.
    pub expected_kind: ItemKind,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_item_kind_fast(&self, kfs: &RSet<UItemKey>) -> bool {
        match kfs.is_empty() {
            true => self.item_kind.is_empty(),
            false => self.item_kind.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_item_kind_verbose(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValItemKindFail> {
        let item_kinds: HashMap<_, _> = self
            .item_kind
            .iter()
            .filter(|(item_key, _)| !kfs.contains(item_key))
            .map(|(item_key, item_info)| (ctx.u_data.items.id_by_key(*item_key), *item_info))
            .collect();
        match item_kinds.is_empty() {
            true => None,
            false => Some(ValItemKindFail { item_kinds }),
        }
    }
}
