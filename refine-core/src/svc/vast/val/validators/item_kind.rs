use itertools::Itertools;

use crate::{
    ItemId,
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemId,
    util::RSet,
    val::DetectedItemKind,
};

#[derive(Copy, Clone)]
pub(in crate::svc::vast) struct ValItemKindItemStored {
    pub(in crate::svc::vast) kind: Option<DetectedItemKind>,
    pub(in crate::svc::vast) expected_kind: DetectedItemKind,
}

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
#[derive(Clone)]
pub struct ValItemKindFail {
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::KeyValueMap<_>"))]
    pub item_kinds: Vec<ValItemKindItemInfo>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ValItemKindItemInfo {
    /// Item which failed validation
    pub item_id: ItemId,
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
            .filter_map(|(item_uid, item_info)| match kfs.contains(item_uid) {
                true => None,
                false => Some(ValItemKindItemInfo {
                    item_id: ctx.u_data.items.ext_id_by_int_id(*item_uid),
                    kind: item_info.kind,
                    expected_kind: item_info.expected_kind,
                }),
            })
            .collect_vec();
        match item_kinds.is_empty() {
            true => None,
            false => Some(ValItemKindFail { item_kinds }),
        }
    }
}
