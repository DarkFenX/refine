use itertools::Itertools;

use crate::{
    svc::{SvcCtx, Vast, vast::VastFitData},
    ud::UItemId,
    util::RSet,
    val::{ValItemInfo, ValResult},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Result
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
#[derive(Clone)]
pub struct ValNotLoadedItemFail {
    /// Items which couldn't be loaded from current sol data source.
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub items: Vec<ValItemInfo>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Validators
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Vast {
    pub(in crate::svc::vast::val) fn validate_not_loaded_item_fast(&self, kfs: &RSet<UItemId>) -> bool {
        validate_fast(kfs, &self.not_loaded)
    }
    pub(in crate::svc::vast::val) fn validate_not_loaded_item_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
    ) -> ValResult<ValNotLoadedItemFail> {
        validate_verbose(kfs, &self.not_loaded, ctx)
    }
}

impl VastFitData {
    pub(in crate::svc::vast::val) fn validate_not_loaded_item_fast(&self, kfs: &RSet<UItemId>) -> bool {
        validate_fast(kfs, &self.not_loaded)
    }
    pub(in crate::svc::vast::val) fn validate_not_loaded_item_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
    ) -> ValResult<ValNotLoadedItemFail> {
        validate_verbose(kfs, &self.not_loaded, ctx)
    }
}

fn validate_fast(kfs: &RSet<UItemId>, not_loaded: &RSet<UItemId>) -> bool {
    match kfs.is_empty() {
        true => not_loaded.is_empty(),
        false => not_loaded.difference(kfs).next().is_none(),
    }
}

fn validate_verbose(kfs: &RSet<UItemId>, not_loaded: &RSet<UItemId>, ctx: &SvcCtx) -> ValResult<ValNotLoadedItemFail> {
    let items = not_loaded
        .iter()
        .map(|&item_uid| ValItemInfo::from_uid(kfs, ctx, item_uid))
        .collect_vec();
    match items.is_empty() {
        true => ValResult::Passed,
        false => ValResult::from_fail(items.iter().all(|v| v.status.is_kf()), ValNotLoadedItemFail { items }),
    }
}
