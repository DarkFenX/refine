use crate::{
    def::ItemId,
    svc::{
        SvcCtx,
        vast::{Vast, VastFitData},
    },
    uad::UadItemKey,
    util::RSet,
};

pub struct ValNotLoadedItemFail {
    /// Item IDs of items which couldn't be loaded from current sol data source.
    pub item_ids: Vec<ItemId>,
}

impl Vast {
    pub(in crate::svc::vast) fn validate_not_loaded_item_fast(&self, kfs: &RSet<UadItemKey>) -> bool {
        validate_fast(kfs, &self.not_loaded)
    }
    pub(in crate::svc::vast) fn validate_not_loaded_item_verbose(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValNotLoadedItemFail> {
        validate_verbose(kfs, &self.not_loaded, ctx)
    }
}

impl VastFitData {
    pub(in crate::svc::vast) fn validate_not_loaded_item_fast(&self, kfs: &RSet<UadItemKey>) -> bool {
        validate_fast(kfs, &self.not_loaded)
    }
    pub(in crate::svc::vast) fn validate_not_loaded_item_verbose(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValNotLoadedItemFail> {
        validate_verbose(kfs, &self.not_loaded, ctx)
    }
}

fn validate_fast(kfs: &RSet<UadItemKey>, not_loaded: &RSet<UadItemKey>) -> bool {
    match kfs.is_empty() {
        true => not_loaded.is_empty(),
        false => not_loaded.difference(kfs).next().is_none(),
    }
}

fn validate_verbose(
    kfs: &RSet<UadItemKey>,
    not_loaded: &RSet<UadItemKey>,
    ctx: SvcCtx,
) -> Option<ValNotLoadedItemFail> {
    let item_ids: Vec<_> = not_loaded
        .iter()
        .filter(|item_key| !kfs.contains(item_key))
        .map(|item_key| ctx.uad.items.id_by_key(*item_key))
        .collect();
    match item_ids.is_empty() {
        true => None,
        false => Some(ValNotLoadedItemFail { item_ids }),
    }
}
