//! Adapted data handler and data types.
//!
//! Adapted data types are built from the EVE data provided by an EVE data handler and are optimized
//! for ease of use by various refine components. Since adapted types can be persisted, it helps to
//! avoid processing every time data is loaded.

pub use cacher::AdaptedDataCacher;
pub use data::{
    AAbil, AAbilId, AAbils, AAttr, AAttrId, AAttrs, ABuff, ABuffAffecteeFilter, ABuffAggrMode, ABuffId, ABuffModifier,
    ABuffModifiers, ABuffs, ACount, ACustomAttrId, ACustomBuffId, ACustomEffectId, ACustomItemListId, AData,
    ADogmaEffectId, AEffect, AEffectAffecteeFilter, AEffectBuff, AEffectBuffAttrMerge, AEffectBuffDuration,
    AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength, AEffectCatId, AEffectId, AEffectLocation, AEffectModifier,
    AEffectModifiers, AEffectStopIds, AEffects, AEveAttrId, AEveBuffId, AEveItemListId, AItem, AItemAbils, AItemAttr,
    AItemAttrs, AItemBuffItemLists, AItemCapUseAttrs, AItemCatId, AItemEffect, AItemEffectData, AItemEffects,
    AItemGrpId, AItemId, AItemList, AItemListId, AItemListItemIds, AItemLists, AItemSkillReq, AItemSkillReqs, AItems,
    AModifierSrq, AMuta, AMutaAttr, AMutaAttrRange, AMutaAttrs, AMutaItemConv, AMutaItemConvs, AMutas, AOp,
    ASkillLevel, AState, AValue,
};
pub(crate) use generator::generate_adapted_data;
pub use result::AResult;

mod cacher;
mod consts;
mod data;
pub mod err;
mod generator;
mod result;
