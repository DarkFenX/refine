//! Adapted data handler and data types.
//!
//! Adapted data types are built from the EVE data provided by an EVE data handler and are optimized
//! for ease of use by various reefast components. Adapted data types are assumed to be read-only by
//! the components, anything mutable is built on top of them.

pub(crate) use data::AEffectXt;
pub use data::{
    AAttr, AAttrId, AAttrVal, ABuff, ABuffAffecteeFilter, ABuffAggrMode, ABuffId, ABuffModifier, ACount,
    ACustomEffectId, AData, ADogmaEffectId, AEffect, AEffectAffecteeFilter, AEffectBuffInfo, AEffectBuffScope,
    AEffectBuffSrc, AEffectBuffSrcCustom, AEffectCatId, AEffectChargeInfo, AEffectId, AEffectLocation,
    AEffectModBuildStatus, AEffectModifier, AEffectRt, AItem, AItemCatId, AItemChargeLimit, AItemEffectData,
    AItemExtras, AItemGrpId, AItemId, AItemKind, AItemShipLimit, AModifierSrq, AMuta, AMutaAttrRange, AOp,
    AShipDroneLimit, AShipKind, ASkillLevel, ASlotIndex, AState, ArcAttr, ArcBuff, ArcEffectRt, ArcItem, ArcMuta,
};
pub use handler::AdaptedDataHandler;
pub use result::AResult;

pub(crate) mod consts;
mod data;
mod handler;
mod result;
