use crate::{ItemId, svc::SvcCtx, ud::UItemId, util::RSet, val::ValItemStatus};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Item which fails a validation.
#[cfg_attr(feature = "serde", derive(refine_serde::VecAsMapEntry))]
#[derive(Copy, Clone)]
pub struct ValItemInfo {
    #[cfg_attr(feature = "serde", vec_map(key))]
    pub item_id: ItemId,
    #[cfg_attr(feature = "serde", vec_map(value))]
    pub status: ValItemStatus,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ValItemInfo {
    pub(in crate::svc::vast::val) fn from_uid(kfs: &RSet<UItemId>, ctx: &SvcCtx, item_uid: UItemId) -> Self {
        Self {
            item_id: ctx.u_data.items.ext_id_by_int_id(item_uid),
            status: ValItemStatus::from_kfs(kfs, item_uid),
        }
    }
}
