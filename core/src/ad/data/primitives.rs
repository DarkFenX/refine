use std::sync::Arc;

use crate::{
    ad::{AAttr, ABuff, AEffectRt, AItem, AMuta},
    def::OF,
};

// Arcs to entities returned by handler
pub type ArcAttr = Arc<AAttr>;
pub type ArcBuff = Arc<ABuff>;
pub type ArcEffectRt = Arc<AEffectRt>;
pub type ArcItem = Arc<AItem>;
pub type ArcMuta = Arc<AMuta>;
// Entity IDs
pub type AAttrId = i32;
pub type ABuffId = i32;
pub type ADogmaEffectId = i32;
pub type ACustomEffectId = i32;
pub type AEffectCatId = i32;
pub type AItemId = i32;
pub type AItemGrpId = i32;
pub type AItemCatId = i32;
// Misc
pub type AAttrVal = OF<f64>;
pub type ACount = u32;
pub type ASlotIndex = i32;
