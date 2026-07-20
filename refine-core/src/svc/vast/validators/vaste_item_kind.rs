use itertools::Itertools;

use crate::{
    misc::DetectedItemKind,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::RSet,
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
pub struct ValItemKindFail {
    /// Items and info about failed validation.
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::Map<_, _>"))]
    pub item_kinds: Vec<(ItemId, ValItemKindItemInfo)>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ValItemKindItemInfo {
    /// Detected item kind.
    pub kind: Option<DetectedItemKind>,
    /// Expected item kind for position it was put in.
    pub expected_kind: DetectedItemKind,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_item_kind_fast(&self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.item_kind.is_empty(),
            false => self.item_kind.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_item_kind_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValItemKindFail> {
        let item_kinds = self
            .item_kind
            .iter()
            .filter(|(item_uid, _)| !kfs.contains(item_uid))
            .map(|(item_uid, item_info)| (ctx.u_data.items.ext_id_by_int_id(*item_uid), *item_info))
            .collect_vec();
        match item_kinds.is_empty() {
            true => None,
            false => Some(ValItemKindFail { item_kinds }),
        }
    }
}
