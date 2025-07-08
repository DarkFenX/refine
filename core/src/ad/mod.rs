//! Adapted data handler and data types.
//!
//! Adapted data types are built from the EVE data provided by an EVE data handler and are optimized
//! for ease of use by various reefast components. Adapted data types are assumed to be read-only by
//! the components, anything mutable is built on top of them.

pub use data::{
    AAttr, AAttrId, AAttrVal, ABuff, ABuffAffecteeFilter, ABuffAggrMode, ABuffId, ABuffModifier, ACount,
    ACustomEffectId, AData, ADogmaEffectId, AEffect, AEffectAffecteeFilter, AEffectBuffInfo, AEffectBuffScope,
    AEffectBuffSrc, AEffectBuffSrcCustom, AEffectCatId, AEffectId, AEffectLocation, AEffectModBuildStatus,
    AEffectModifier, AEffectRt, AItem, AItemCatId, AItemEffectData, AItemGrpId, AItemId, AItemKind, AItemRt,
    AModifierSrq, AMuta, AMutaAttrRange, AOp, ASkillLevel, AState, ArcAttr, ArcBuff, ArcEffectRt, ArcItemRt, ArcMuta,
};
pub(crate) use data::{
    AEffectXt, AItemChargeLimit, AItemContainerLimit, AItemShipLimit, AItemXt, AShipDroneLimit, AShipKind, ASlotIndex,
};
pub use handler::AdaptedDataHandler;
pub use result::AResult;

pub(crate) mod consts;
mod data;
mod handler;
mod result;
