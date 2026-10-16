use itertools::Itertools;

use crate::{
    ItemId, ModRack,
    svc::{SvcCtx, vast::VastFitData},
    ud::{UItemId, UModule},
    util::RSet,
    val::DetectedItemKind,
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Validators
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_item_kind_fast(&self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.item_kind.is_empty(),
            false => self.item_kind.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast::val) fn validate_item_kind_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(in crate::svc::vast) struct ValItemKindItemStored {
    pub(in crate::svc::vast) kind: Option<DetectedItemKind>,
    pub(in crate::svc::vast) expected_kind: DetectedItemKind,
}

impl VastFitData {
    pub(in crate::svc::vast) fn item_kind_add(
        &mut self,
        item_uid: UItemId,
        item_kind: Option<DetectedItemKind>,
        expected_kind: DetectedItemKind,
    ) {
        if item_kind != Some(expected_kind) {
            self.item_kind.insert(
                item_uid,
                ValItemKindItemStored {
                    kind: item_kind,
                    expected_kind,
                },
            );
        }
    }
    pub(in crate::svc::vast) fn item_kind_remove(
        &mut self,
        item_uid: &UItemId,
        item_kind: Option<DetectedItemKind>,
        expected_kind: DetectedItemKind,
    ) {
        if item_kind != Some(expected_kind) {
            self.item_kind.remove(item_uid);
        }
    }
    pub(in crate::svc::vast) fn module_item_kind_add(
        &mut self,
        item_uid: UItemId,
        item_kind: Option<DetectedItemKind>,
        module: &UModule,
    ) {
        self.item_kind_add(item_uid, item_kind, get_module_expected_kind(module));
    }
    pub(in crate::svc::vast) fn module_item_kind_remove(
        &mut self,
        item_uid: &UItemId,
        item_kind: Option<DetectedItemKind>,
        module: &UModule,
    ) {
        self.item_kind_remove(item_uid, item_kind, get_module_expected_kind(module));
    }
}

fn get_module_expected_kind(module: &UModule) -> DetectedItemKind {
    match module.get_rack() {
        ModRack::High => DetectedItemKind::ModuleHigh,
        ModRack::Mid => DetectedItemKind::ModuleMid,
        ModRack::Low => DetectedItemKind::ModuleLow,
    }
}
