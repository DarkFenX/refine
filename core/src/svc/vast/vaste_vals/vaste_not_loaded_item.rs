use crate::{
    def::ItemId,
    svc::{
        SvcCtx,
        vast::{Vast, VastFitData},
    },
    ud::UItemId,
    util::RSet,
};

pub struct ValNotLoadedItemFail {
    /// Item IDs of items which couldn't be loaded from current sol data source.
    pub item_ids: Vec<ItemId>,
}

impl Vast {
    pub(in crate::svc::vast) fn validate_not_loaded_item_fast(&self, kfs: &RSet<UItemId>) -> bool {
        validate_fast(kfs, &self.not_loaded)
    }
    pub(in crate::svc::vast) fn validate_not_loaded_item_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValNotLoadedItemFail> {
        validate_verbose(kfs, &self.not_loaded, ctx)
    }
}

impl VastFitData {
    pub(in crate::svc::vast) fn validate_not_loaded_item_fast(&self, kfs: &RSet<UItemId>) -> bool {
        validate_fast(kfs, &self.not_loaded)
    }
    pub(in crate::svc::vast) fn validate_not_loaded_item_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValNotLoadedItemFail> {
        validate_verbose(kfs, &self.not_loaded, ctx)
    }
}

fn validate_fast(kfs: &RSet<UItemId>, not_loaded: &RSet<UItemId>) -> bool {
    match kfs.is_empty() {
        true => not_loaded.is_empty(),
        false => not_loaded.difference(kfs).next().is_none(),
    }
}

fn validate_verbose(kfs: &RSet<UItemId>, not_loaded: &RSet<UItemId>, ctx: SvcCtx) -> Option<ValNotLoadedItemFail> {
    let item_ids: Vec<_> = not_loaded
        .iter()
        .filter(|item_key| !kfs.contains(item_key))
        .map(|item_key| ctx.u_data.items.eid_by_iid(*item_key))
        .collect();
    match item_ids.is_empty() {
        true => None,
        false => Some(ValNotLoadedItemFail { item_ids }),
    }
}
