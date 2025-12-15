use crate::{
    def::OF,
    ed::{EAbilId, EAttrId, EBuffId, EEffectCatId, EEffectId, EItemCatId, EItemGrpId, EItemId, EItemListId},
};

// Entity IDs
pub type AAbilId = EAbilId;
pub type AAttrId = EAttrId;
pub type ABuffId = EBuffId;
pub type ADogmaEffectId = EEffectId;
pub type ACustomEffectId = i32;
pub type AEffectCatId = EEffectCatId;
pub type AItemId = EItemId;
pub type AItemGrpId = EItemGrpId;
pub type AItemCatId = EItemCatId;
pub type AEveItemListId = EItemListId;
pub type ACustomItemListId = i32;
// Misc
pub type AAttrVal = OF<f64>;
pub type ACount = u32;
pub(crate) type ASlotIndex = i32;
