//! Adapted data handler and data types.
//!
//! Adapted data types are built from the EVE data provided by an EVE data handler and are optimized
//! for ease of use by various reefast components. Adapted data types are assumed to be read-only by
//! the components, anything mutable is built on top of them.

pub use cacher::AdaptedDataCacher;
pub(crate) use data::ASlotIndex;
pub use data::{
    AAbil, AAbilId, AAttr, AAttrId, AAttrVal, ABuff, ABuffAffecteeFilter, ABuffAggrMode, ABuffId, ABuffModifier,
    ACount, ACustomEffectId, AData, ADogmaEffectId, AEffect, AEffectAffecteeFilter, AEffectBuffInfo, AEffectBuffScope,
    AEffectBuffSrc, AEffectBuffSrcCustom, AEffectCatId, AEffectId, AEffectLocation, AEffectModifier, AItem, AItemCatId,
    AItemEffectData, AItemGrpId, AItemId, AItemListId, AModifierSrq, AMuta, AMutaAttrRange, AOp, ASkillLevel, AState,
};
pub use result::AResult;

mod cacher;
pub(crate) mod consts;
mod data;
mod result;
