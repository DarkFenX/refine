//! ND stands for eNtity Data.
//!
//! This module is a place for almost all the hardcoded customizations applied to various EVE
//! entities, and entities derived from them. It should've been called customization data, but
//! prefix "C" is used by cacheable entities (the ones stored in adapted data cacher).

pub(crate) use attr::{N_ATTR_MAP, NAttr};
pub(crate) use buff::{N_BUFF_MAP, NBuff};
pub(crate) use effect::{
    N_EFFECT_MAP, NBaseOutputGetter, NChargeMultGetter, NEffect, NEffectBreacherDmgGetter, NEffectCalcCustomizer,
    NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeDeplCrystal, NEffectChargeLoc,
    NEffectDmgKind, NEffectDmgKindGetter, NEffectLocalOpcSpec, NEffectProjMultGetter, NEffectProjOpcSpec,
    NEffectProjecteeFilter, NEffectResist, NEffectSpoolAttrs,
};
pub(crate) use item_list::{N_ITEM_LIST_MAP, NItemList};

mod attr;
mod buff;
mod effect;
mod item_list;
