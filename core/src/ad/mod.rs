//! Adapted data handler and data types.
//!
//! Adapted data types are built from the EVE data provided by an EVE data handler and are optimized
//! for ease of use by various refine components. Since adapted types can be persisted, it helps to
//! avoid processing every time data is loaded.

pub use cacher::AdaptedDataCacher;
pub use data::{
    AAbil, AAbilId, AAttr, AAttrId, ABuff, ABuffAffecteeFilter, ABuffAggrMode, ABuffId, ABuffModifier, ACount,
    ACustomAttrId, ACustomBuffId, ACustomEffectId, ACustomItemListId, AData, ADogmaEffectId, AEffect,
    AEffectAffecteeFilter, AEffectBuff, AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope,
    AEffectBuffStrength, AEffectCatId, AEffectId, AEffectLocation, AEffectModifier, AEveAttrId, AEveBuffId,
    AEveItemListId, AItem, AItemCatId, AItemEffectData, AItemGrpId, AItemId, AItemList, AItemListId, AModifierSrq,
    AMuta, AMutaAttrRange, AOp, ASkillLevel, AState, AValue,
};
pub(crate) use generator::generate_adapted_data;
pub use result::AResult;

mod cacher;
pub(crate) mod consts;
mod data;
pub mod err;
mod generator;
mod result;
